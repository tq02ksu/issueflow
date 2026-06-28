use std::convert::Infallible;

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
            AgentSessionDetail, AgentSessionRow, CreateRunResponse, RenameSessionRequest,
            RunAgentRequest, RunEventsQuery,
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

    let input_payload = serde_json::to_string(&payload).unwrap_or_default();
    let run = runs::create_run(&state.pool, &payload.thread_id, None, &input_payload)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok((
        StatusCode::OK,
        Json(CreateRunResponse {
            run_id: run.id.clone(),
            thread_id: payload.thread_id,
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

    let after_seq = query.after_seq.unwrap_or(0);
    let past = runs::list_events_after(&state.pool, &run_id, after_seq)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let events: Vec<Result<Event, Infallible>> = past
        .into_iter()
        .map(|(_seq, event_type, payload_str)| {
            let event = build_sse_event(&event_type, &payload_str);
            Ok(event)
        })
        .collect();

    let stream = futures::stream::iter(events);
    Ok(Sse::new(stream))
}

fn build_sse_event(event_type: &str, payload: &str) -> Event {
    match event_type {
        "CUSTOM" => {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(payload) {
                Event::default().event("custom").json_data(v).unwrap()
            } else {
                Event::default().data(payload)
            }
        }
        _ => Event::default().event(event_type).data(payload),
    }
}
