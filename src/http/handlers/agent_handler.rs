use std::{collections::VecDeque, convert::Infallible, time::Duration};

use agui_axum::sse::encode_persisted_event;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::sse::{Event, Sse},
};
use futures::stream::Stream;

use crate::{
    agent::{
        models::{
            AgentSessionDetail, AgentSessionRow, CreateRunResponse, PersistedRunInput,
            RenameSessionRequest, RunAgentRequest, RunEventsQuery,
        },
        runs, sessions,
    },
    error::AppError,
    http::routes::AppState,
    session::Session,
};

pub async fn list_sessions(
    State(state): State<AppState>,
    session: Session,
    Path(workbench_id): Path<i64>,
) -> Result<Json<Vec<AgentSessionRow>>, AppError> {
    let rows = sessions::list_sessions(&state.pool, session.user_id, workbench_id)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok(Json(rows))
}

pub async fn create_session(
    State(state): State<AppState>,
    session: Session,
    Path(workbench_id): Path<i64>,
) -> Result<(StatusCode, Json<AgentSessionRow>), AppError> {
    let row = sessions::create_session(&state.pool, session.user_id, workbench_id)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok((StatusCode::CREATED, Json(row)))
}

pub async fn get_session(
    State(state): State<AppState>,
    session: Session,
    Path((workbench_id, id)): Path<(i64, String)>,
) -> Result<Json<AgentSessionDetail>, AppError> {
    let s = sessions::get_session(&state.pool, session.user_id, &id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            other => AppError::Internal(other.into()),
        })?;

    if s.workbench_id != workbench_id {
        return Err(AppError::NotFound);
    }

    let messages = sessions::list_messages(&state.pool, &id)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(Json(AgentSessionDetail {
        session: s,
        messages,
    }))
}

pub async fn rename_session(
    State(state): State<AppState>,
    session: Session,
    Path((_workbench_id, id)): Path<(i64, String)>,
    Json(payload): Json<RenameSessionRequest>,
) -> Result<StatusCode, AppError> {
    sessions::rename_session(&state.pool, session.user_id, &id, &payload.title)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_session(
    State(state): State<AppState>,
    session: Session,
    Path((_workbench_id, id)): Path<(i64, String)>,
) -> Result<StatusCode, AppError> {
    sessions::delete_session(&state.pool, session.user_id, &id)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_run(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<RunAgentRequest>,
) -> Result<(StatusCode, Json<CreateRunResponse>), AppError> {
    let _s = sessions::get_session(&state.pool, session.user_id, &payload.thread_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            other => AppError::Internal(other.into()),
        })?;

    // persist user messages
    for msg in &payload.messages {
        let content = msg
            .get("content")
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_default();
        sqlx::query(
            "INSERT INTO agent_messages (session_id, role, message_kind, content, created_at) VALUES (?, 'user', 'text', ?, CURRENT_TIMESTAMP)",
        )
        .bind(&payload.thread_id)
        .bind(&content)
        .execute(&state.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    }

    let thread_id = payload.thread_id.clone();
    let input_payload = serde_json::to_string(&PersistedRunInput {
        request: payload,
        access_token: session.access_token,
    })
    .unwrap_or_default();
    let run = runs::create_run(&state.pool, &thread_id, None, &input_payload)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok((
        StatusCode::OK,
        Json(CreateRunResponse {
            run_id: run.id.clone(),
            thread_id,
            status: run.status,
        }),
    ))
}

pub async fn subscribe_run_events(
    State(state): State<AppState>,
    session: Session,
    Path(run_id): Path<String>,
    Query(query): Query<RunEventsQuery>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let _ = session;

    let stream = futures::stream::unfold(
        RunEventsStreamState::new(state.pool.clone(), run_id, query.after_seq.unwrap_or(0)),
        |mut state| async move {
            loop {
                if let Some(event) = state.pending.pop_front() {
                    return Some((Ok(event), state));
                }

                if state.finished {
                    return None;
                }

                match runs::list_events_after(&state.pool, &state.run_id, state.after_seq).await {
                    Ok(events) if !events.is_empty() => {
                        state.after_seq = events
                            .last()
                            .map(|(seq, _, _)| *seq)
                            .unwrap_or(state.after_seq);
                        state.pending.extend(events.into_iter().map(
                            |(_seq, event_type, payload_str)| {
                                encode_persisted_event(&event_type, &payload_str)
                            },
                        ));
                    }
                    Ok(_) => match runs::get_run(&state.pool, &state.run_id).await {
                        Ok(run) if is_terminal_run_status(&run.status) => {
                            state.finished = true;
                        }
                        Ok(_) => tokio::time::sleep(Duration::from_millis(20)).await,
                        Err(_) => state.finished = true,
                    },
                    Err(_) => state.finished = true,
                }
            }
        },
    );

    Ok(Sse::new(stream))
}

struct RunEventsStreamState {
    pool: crate::db::DbPool,
    run_id: String,
    after_seq: i64,
    pending: VecDeque<Event>,
    finished: bool,
}

impl RunEventsStreamState {
    fn new(pool: crate::db::DbPool, run_id: String, after_seq: i64) -> Self {
        Self {
            pool,
            run_id,
            after_seq,
            pending: VecDeque::new(),
            finished: false,
        }
    }
}

fn is_terminal_run_status(status: &str) -> bool {
    matches!(status, "completed" | "failed" | "cancelled")
}
