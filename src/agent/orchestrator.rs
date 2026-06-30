use crate::{
    actions::store as action_store,
    agent::{
        events::AgUiEvent,
        gitlab_tools, issue_preparation,
        models::{AgentRunRow, ExecutePendingActionInput, PersistedRunInput},
        prompt, runs, sessions,
    },
    error::AppError,
    gitlab::issues,
    http::routes::AppState,
    session::Session,
};
use std::{collections::HashMap, sync::Arc};
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

    if let Some(action_input) = parse_execute_pending_action_input(run)? {
        process_pending_action_run(&state, run, &action_input).await?;

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

        return Ok(());
    }

    let messages = sessions::list_messages(&state.pool, &run.session_id)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;
    let persisted_input = parse_persisted_run_input(run)?;

    if let Some(command) = latest_prepare_issue_command(&messages) {
        let assistant_text = match prepare_issue_from_command(
            &state,
            run,
            &session_row,
            &persisted_input.access_token,
            command.issue_iid,
        )
        .await
        {
            Ok(outcome) => outcome.assistant_message,
            Err(error) => format!(
                "I couldn't prepare issue #{} yet: {}",
                command.issue_iid,
                user_facing_prepare_issue_error(&error)
            ),
        };

        persist_assistant_text_message(&state, run, &mut seq, &assistant_text).await?;
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

        return Ok(());
    }

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

    persist_tool_call_messages(&state, run, &engine_result.events).await?;

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

fn parse_execute_pending_action_input(
    run: &AgentRunRow,
) -> Result<Option<ExecutePendingActionInput>, AppError> {
    let Some(payload) = run.input_payload.as_deref() else {
        return Ok(None);
    };

    match serde_json::from_str::<ExecutePendingActionInput>(payload) {
        Ok(input) => Ok(Some(input)),
        Err(_) => Ok(None),
    }
}

async fn process_pending_action_run(
    state: &AppState,
    run: &AgentRunRow,
    input: &ExecutePendingActionInput,
) -> Result<(), AppError> {
    let action = action_store::get_pending_action(&state.pool, &input.pending_action_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            other => AppError::from(other),
        })?;

    action_store::mark_running(&state.pool, &action.id, &run.id).await?;

    let result = execute_pending_action(state, &action, &input.access_token).await;

    match result {
        Ok(()) => {
            action_store::mark_completed(&state.pool, &action.id, &run.id).await?;
            Ok(())
        }
        Err(error) => {
            action_store::mark_failed(&state.pool, &action.id, &run.id).await?;
            Err(error)
        }
    }
}

async fn execute_pending_action(
    state: &AppState,
    action: &crate::actions::models::PendingActionRow,
    access_token: &str,
) -> Result<(), AppError> {
    match action.action_type.as_str() {
        "update_gitlab_issue" => {
            let base_url =
                state.config.git.base_url.as_deref().ok_or_else(|| {
                    AppError::Internal("missing git.base_url configuration".into())
                })?;
            let payload: UpdateGitlabIssuePayload =
                serde_json::from_str(&action.payload).map_err(|e| {
                    AppError::BadRequest(format!("invalid update_gitlab_issue payload: {e}"))
                })?;
            let update_result = issues::update_issue(
                base_url,
                access_token,
                action.project_id as u64,
                payload.issue_iid,
                &payload.rendered_content,
            )
            .await;

            match update_result {
                Ok(_) => {}
                Err(error) if should_fallback_to_comment(&error) => {
                    let comment_body = payload
                        .fallback_comment_body
                        .as_deref()
                        .unwrap_or(&payload.rendered_content);
                    let _ = issues::create_issue_note(
                        base_url,
                        access_token,
                        action.project_id as u64,
                        payload.issue_iid,
                        comment_body,
                    )
                    .await
                    .map_err(|e| AppError::Internal(e.into()))?;
                }
                Err(error) => return Err(AppError::Internal(error.into())),
            }
            Ok(())
        }
        "publish_gitlab_comment" => {
            let base_url =
                state.config.git.base_url.as_deref().ok_or_else(|| {
                    AppError::Internal("missing git.base_url configuration".into())
                })?;
            let payload: PublishGitlabCommentPayload = serde_json::from_str(&action.payload)
                .map_err(|e| {
                    AppError::BadRequest(format!("invalid publish_gitlab_comment payload: {e}"))
                })?;
            let _ = issues::create_issue_note(
                base_url,
                access_token,
                action.project_id as u64,
                payload.issue_iid,
                &payload.body,
            )
            .await
            .map_err(|e| AppError::Internal(e.into()))?;
            Ok(())
        }
        "refresh_memory" => Ok(()),
        other => Err(AppError::BadRequest(format!(
            "unsupported action type: {other}"
        ))),
    }
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

async fn persist_assistant_text_message(
    state: &AppState,
    run: &AgentRunRow,
    seq: &mut i64,
    text: &str,
) -> Result<(), AppError> {
    let message_id = uuid::Uuid::new_v4().to_string();
    append(
        state,
        run.id.as_str(),
        seq,
        &AgUiEvent::TextMessageStart {
            message_id: message_id.clone(),
            role: "assistant".to_string(),
        },
    )
    .await?;
    append(
        state,
        run.id.as_str(),
        seq,
        &AgUiEvent::TextMessageContent {
            message_id: message_id.clone(),
            delta: text.to_string(),
        },
    )
    .await?;
    append(
        state,
        run.id.as_str(),
        seq,
        &AgUiEvent::TextMessageEnd { message_id },
    )
    .await?;

    sqlx::query(
        "INSERT INTO agent_messages (session_id, run_id, role, message_kind, content, created_at)
         VALUES (?, ?, 'assistant', 'text', ?, CURRENT_TIMESTAMP)",
    )
    .bind(&run.session_id)
    .bind(&run.id)
    .bind(text)
    .execute(&state.pool)
    .await
    .map_err(|e| AppError::Internal(e.into()))?;

    Ok(())
}

async fn persist_tool_call_messages(
    state: &AppState,
    run: &AgentRunRow,
    events: &[AgUiEvent],
) -> Result<(), AppError> {
    let mut in_flight: HashMap<String, (String, String)> = HashMap::new();

    for event in events {
        match event {
            AgUiEvent::ToolCallStart {
                tool_call_id,
                tool_call_name,
                ..
            } => {
                in_flight.insert(
                    tool_call_id.clone(),
                    (tool_call_name.clone(), String::new()),
                );
            }
            AgUiEvent::ToolCallArgs {
                tool_call_id,
                delta,
            } => {
                if let Some((_, args)) = in_flight.get_mut(tool_call_id) {
                    args.push_str(delta);
                }
            }
            AgUiEvent::ToolCallResult {
                tool_call_id,
                content,
                ..
            } => {
                let (name, args) = in_flight
                    .remove(tool_call_id)
                    .unwrap_or_else(|| (String::new(), String::new()));
                sqlx::query(
                    "INSERT INTO agent_messages (session_id, run_id, role, message_kind, content, created_at)
                     VALUES (?, ?, 'tool', 'tool_call', ?, CURRENT_TIMESTAMP)",
                )
                .bind(&run.session_id)
                .bind(&run.id)
                .bind(tool_call_message_content(tool_call_id, &name, &args, content))
                .execute(&state.pool)
                .await
                .map_err(|e| AppError::Internal(e.into()))?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn tool_call_message_content(
    tool_call_id: &str,
    tool_name: &str,
    args: &str,
    result: &serde_json::Value,
) -> String {
    serde_json::json!({
        "toolCallId": tool_call_id,
        "name": tool_name,
        "args": args,
        "result": result,
    })
    .to_string()
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

#[cfg(test)]
mod tests {
    use super::{should_fallback_to_comment, tool_call_message_content};

    #[test]
    fn tool_call_message_content_keeps_name_args_and_result() {
        let content = tool_call_message_content(
            "call_1",
            "list_issues",
            "{\"project_id\":37}",
            &serde_json::json!([{"id": 1}]),
        );

        let parsed: serde_json::Value = serde_json::from_str(&content)
            .unwrap_or_else(|error| panic!("tool call content should be valid JSON: {error}"));
        assert_eq!(parsed["toolCallId"], "call_1");
        assert_eq!(parsed["name"], "list_issues");
        assert_eq!(parsed["args"], "{\"project_id\":37}");
        assert_eq!(parsed["result"], serde_json::json!([{"id": 1}]));
    }

    #[test]
    fn should_fallback_to_comment_for_permission_errors() {
        assert!(should_fallback_to_comment(
            "gitlab api request failed with 403 Forbidden: nope"
        ));
        assert!(should_fallback_to_comment(
            "gitlab api request failed with 401 Unauthorized: nope"
        ));
        assert!(!should_fallback_to_comment(
            "gitlab api request failed with 500 Internal Server Error: nope"
        ));
    }
}

#[derive(sqlx::FromRow)]
struct WorkbenchPromptRow {
    project_id: i64,
    project_name: String,
    project_path: String,
    name: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateGitlabIssuePayload {
    issue_iid: u64,
    rendered_content: String,
    #[serde(default)]
    fallback_comment_body: Option<String>,
}

#[derive(serde::Deserialize)]
struct PublishGitlabCommentPayload {
    issue_iid: u64,
    body: String,
}

fn should_fallback_to_comment(error: &str) -> bool {
    error.contains(" 401")
        || error.contains(" 403")
        || error.to_ascii_lowercase().contains("forbidden")
        || error.to_ascii_lowercase().contains("unauthorized")
}

fn latest_prepare_issue_command(
    messages: &[crate::agent::models::AgentMessageRow],
) -> Option<issue_preparation::PrepareIssueCommand> {
    messages
        .iter()
        .rev()
        .find(|message| message.role == "user")
        .and_then(|message| issue_preparation::parse_prepare_issue_command(&message.content))
}

async fn prepare_issue_from_command(
    state: &AppState,
    run: &AgentRunRow,
    session_row: &crate::agent::models::AgentSessionRow,
    access_token: &str,
    issue_iid: u64,
) -> Result<issue_preparation::PreparedIssueOutcome, AppError> {
    let base_url = state.config.git.base_url.as_deref().ok_or_else(|| {
        AppError::BadRequest("git.base_url must be configured to prepare issue drafts".into())
    })?;
    let prompt_context = load_prompt_context(state, session_row.workbench_id).await?;
    let issue = issues::get_issue(
        base_url,
        access_token,
        prompt_context.project_id as u64,
        issue_iid,
    )
    .await
    .map_err(AppError::BadRequest)?;

    issue_preparation::persist_issue_preparation(
        &state.pool,
        session_row.workbench_id,
        &run.session_id,
        session_row.user_id,
        &issue,
    )
    .await
}

fn user_facing_prepare_issue_error(error: &AppError) -> String {
    match error {
        AppError::BadRequest(message) => message.clone(),
        AppError::NotFound => "the requested issue or workbench was not found".to_string(),
        AppError::Conflict => {
            "the draft could not be created because the current state changed".to_string()
        }
        AppError::Unauthorized => "authentication is required".to_string(),
        AppError::Forbidden => "permission was denied".to_string(),
        AppError::ServiceUnavailable(message) => message.clone(),
        AppError::Internal(_) => "an internal error occurred while preparing the draft".to_string(),
    }
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
