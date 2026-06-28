use crate::{
    agent::{
        events::AgUiEvent,
        gitlab_tools,
        models::{AgentRunRow, PersistedRunInput},
        prompt, runs, sessions,
    },
    error::AppError,
    http::routes::AppState,
    session::Session,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn process_run(state: AppState, run: &AgentRunRow) -> Result<(), AppError> {
    let session_row = sessions::get_session_by_id(&state.pool, &run.session_id)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let mut seq = 0i64;

    let started = serde_json::to_string(&AgUiEvent::RunStarted {
        thread_id: run.session_id.clone(),
        run_id: run.id.clone(),
    })
    .unwrap_or_default();
    runs::append_event(&state.pool, &run.id, seq, "RUN_STARTED", &started).await?;
    seq += 1;

    let messages = sessions::list_messages(&state.pool, &run.session_id)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let persisted_messages: Vec<serde_json::Value> = messages
        .iter()
        .map(|m| serde_json::json!({"role": m.role, "content": m.content}))
        .collect();
    let prompt_context = load_prompt_context(&state, session_row.workbench_id).await?;
    let system_prompt =
        prompt::render_system_prompt(&prompt_context).map_err(|e| AppError::Internal(e.into()))?;
    let model_messages = prompt::build_model_messages(&system_prompt, persisted_messages);

    let tools = gitlab_tools::tool_definitions();
    let client =
        agui_runtime::openai::OpenAiClient::new(agui_runtime::openai::OpenAiClientConfig {
            base_url: state
                .config
                .agent
                .openai_base_url
                .clone()
                .unwrap_or_else(|| "https://api.openai.com/v1".into()),
            api_key: state
                .config
                .agent
                .openai_api_key
                .clone()
                .unwrap_or_default(),
            model: state
                .config
                .agent
                .model
                .clone()
                .unwrap_or_else(|| "gpt-4o".into()),
        });
    let persisted_input = parse_persisted_run_input(run)?;

    let stub_session = Session {
        user_id: session_row.user_id,
        sub: String::new(),
        access_token: persisted_input.access_token,
    };
    let seq_ref = Arc::new(Mutex::new(seq));
    let event_state = state.clone();
    let event_run_id = run.id.clone();
    let event_seq_ref = seq_ref.clone();

    let engine_result = agui_runtime::engine::run_chat_rounds(
        model_messages,
        tools,
        |messages, tools| async {
            client
                .stream_chat(messages, tools)
                .await
                .map_err(|e| agui_runtime::engine::RuntimeEngineError::Message(e.to_string()))
        },
        |tool_name, args| {
            let tool_name = tool_name.to_string();
            let state = state.clone();
            let stub_session = stub_session.clone();
            async move {
                gitlab_tools::execute_tool(&tool_name, args, &state, &stub_session)
                    .await
                    .map_err(|e| agui_runtime::engine::RuntimeEngineError::Message(e.to_string()))
            }
        },
        move |event| {
            let event_state = event_state.clone();
            let event_run_id = event_run_id.clone();
            let event = event.clone();
            let event_seq_ref = event_seq_ref.clone();
            async move {
                let mut seq = event_seq_ref.lock().await;
                append(&event_state, &event_run_id, &mut seq, &event)
                    .await
                    .map_err(|e| agui_runtime::engine::RuntimeEngineError::Message(e.to_string()))
            }
        },
    )
    .await;

    let engine_result = match engine_result {
        Ok(result) => result,
        Err(error) => {
            append(
                &state,
                &run.id,
                &mut seq,
                &AgUiEvent::RunError {
                    message: format!("tool execution failed: {error}"),
                    code: Some("TOOL_ERROR".into()),
                },
            )
            .await?;
            return Err(AppError::Internal(error.to_string().into()));
        }
    };

    for assistant_text in engine_result.assistant_messages {
        sqlx::query(
            "INSERT INTO agent_messages (session_id, run_id, role, message_kind, content, created_at) VALUES (?, ?, 'assistant', 'text', ?, CURRENT_TIMESTAMP)",
        )
        .bind(&run.session_id)
        .bind(&run.id)
        .bind(&assistant_text)
        .execute(&state.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    }

    seq = *seq_ref.lock().await;

    append(
        &state,
        &run.id,
        &mut seq,
        &AgUiEvent::RunFinished {
            thread_id: run.session_id.clone(),
            run_id: run.id.clone(),
        },
    )
    .await?;

    Ok(())
}

fn parse_persisted_run_input(run: &AgentRunRow) -> Result<PersistedRunInput, AppError> {
    let payload = run
        .input_payload
        .as_deref()
        .ok_or_else(|| AppError::Internal("agent run is missing input payload".into()))?;

    serde_json::from_str(payload)
        .map_err(|e| AppError::Internal(format!("invalid agent run input payload: {e}").into()))
}

async fn append(
    state: &AppState,
    run_id: &str,
    seq: &mut i64,
    event: &AgUiEvent,
) -> Result<(), AppError> {
    let payload = serde_json::to_string(event).map_err(|e| AppError::Internal(e.into()))?;
    runs::append_event(&state.pool, run_id, *seq, event_type(event), &payload).await?;
    *seq += 1;
    Ok(())
}

fn event_type(event: &AgUiEvent) -> &'static str {
    match event {
        AgUiEvent::RunStarted { .. } => "RUN_STARTED",
        AgUiEvent::RunFinished { .. } => "RUN_FINISHED",
        AgUiEvent::RunError { .. } => "RUN_ERROR",
        AgUiEvent::StepStarted { .. } => "STEP_STARTED",
        AgUiEvent::StepFinished { .. } => "STEP_FINISHED",
        AgUiEvent::TextMessageStart { .. } => "TEXT_MESSAGE_START",
        AgUiEvent::TextMessageContent { .. } => "TEXT_MESSAGE_CONTENT",
        AgUiEvent::TextMessageEnd { .. } => "TEXT_MESSAGE_END",
        AgUiEvent::ToolCallStart { .. } => "TOOL_CALL_START",
        AgUiEvent::ToolCallArgs { .. } => "TOOL_CALL_ARGS",
        AgUiEvent::ToolCallEnd { .. } => "TOOL_CALL_END",
        AgUiEvent::ToolCallResult { .. } => "TOOL_CALL_RESULT",
        AgUiEvent::StateSnapshot { .. } => "STATE_SNAPSHOT",
        AgUiEvent::MessagesSnapshot { .. } => "MESSAGES_SNAPSHOT",
        AgUiEvent::Custom { .. } => "CUSTOM",
    }
}

#[derive(sqlx::FromRow)]
struct WorkbenchPromptRow {
    project_id: i64,
    project_name: String,
    project_path: String,
    name: String,
}

async fn load_prompt_context(
    state: &AppState,
    workbench_id: i64,
) -> Result<prompt::PromptContext, AppError> {
    let row: WorkbenchPromptRow = sqlx::query_as(
        "SELECT project_id, project_name, project_path, name FROM workbenches WHERE id = ?",
    )
    .bind(workbench_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    Ok(prompt::PromptContext {
        workbench_name: row.name,
        project_id: row.project_id,
        project_name: row.project_name,
        project_path: row.project_path,
    })
}
