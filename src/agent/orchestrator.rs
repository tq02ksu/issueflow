use std::collections::HashMap;

use futures::StreamExt;

use crate::{
    agent::{
        events::AgUiEvent, gitlab_tools, models::AgentRunRow, openai::ProviderDelta, runs, sessions,
    },
    error::AppError,
    http::routes::AppState,
    session::Session,
};

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

    let mut model_messages: Vec<serde_json::Value> = messages
        .iter()
        .map(|m| serde_json::json!({"role": m.role, "content": m.content}))
        .collect();

    let tools = gitlab_tools::tool_definitions();
    let client = crate::agent::openai::OpenAiClient::new(&state.config.agent);

    let stub_session = Session {
        user_id: session_row.user_id,
        sub: String::new(),
        access_token: state.config.git.token.clone().unwrap_or_default(),
    };

    let mut tool_name_map: HashMap<String, String> = HashMap::new();
    let mut tool_args_map: HashMap<String, String> = HashMap::new();

    for _round in 0..state.config.agent.max_tool_rounds {
        let mut stream = client
            .stream_chat(model_messages.clone(), tools.clone())
            .await?;

        let mut current_msg_id: Option<String> = None;
        let mut assistant_text = String::new();
        let mut produced_tool = false;

        while let Some(delta) = stream.next().await {
            let delta = delta?;
            match delta {
                ProviderDelta::Text(text) => {
                    if current_msg_id.is_none() {
                        let mid = uuid::Uuid::new_v4().to_string();
                        current_msg_id = Some(mid.clone());
                        append(
                            &state,
                            &run.id,
                            &mut seq,
                            &AgUiEvent::TextMessageStart {
                                message_id: mid,
                                role: "assistant".into(),
                            },
                        )
                        .await?;
                    }
                    assistant_text.push_str(&text);
                    append(
                        &state,
                        &run.id,
                        &mut seq,
                        &AgUiEvent::TextMessageContent {
                            message_id: current_msg_id.clone().unwrap(),
                            delta: text,
                        },
                    )
                    .await?;
                }
                ProviderDelta::ToolStart { id, name } => {
                    if let Some(mid) = current_msg_id.take() {
                        append(
                            &state,
                            &run.id,
                            &mut seq,
                            &AgUiEvent::TextMessageEnd { message_id: mid },
                        )
                        .await?;
                    }
                    tool_name_map.insert(id.clone(), name.clone());
                    append(
                        &state,
                        &run.id,
                        &mut seq,
                        &AgUiEvent::ToolCallStart {
                            tool_call_id: id.clone(),
                            tool_call_name: name,
                            parent_message_id: None,
                        },
                    )
                    .await?;
                }
                ProviderDelta::ToolArgs { id, delta } => {
                    tool_args_map
                        .entry(id.clone())
                        .or_default()
                        .push_str(&delta);
                    append(
                        &state,
                        &run.id,
                        &mut seq,
                        &AgUiEvent::ToolCallArgs {
                            tool_call_id: id,
                            delta,
                        },
                    )
                    .await?;
                }
                ProviderDelta::ToolEnd { id } => {
                    produced_tool = true;
                    append(
                        &state,
                        &run.id,
                        &mut seq,
                        &AgUiEvent::ToolCallEnd {
                            tool_call_id: id.clone(),
                        },
                    )
                    .await?;

                    let args_str = tool_args_map.remove(&id).unwrap_or_default();
                    let args: serde_json::Value =
                        serde_json::from_str(&args_str).unwrap_or(serde_json::Value::Null);
                    let tname = tool_name_map.remove(&id).unwrap_or_default();

                    let result =
                        gitlab_tools::execute_tool(&tname, args, &state, &stub_session).await;

                    match result {
                        Ok(content) => {
                            let mid = uuid::Uuid::new_v4().to_string();
                            append(
                                &state,
                                &run.id,
                                &mut seq,
                                &AgUiEvent::ToolCallResult {
                                    message_id: mid,
                                    tool_call_id: id.clone(),
                                    content: content.clone(),
                                    role: "tool".into(),
                                },
                            )
                            .await?;

                            model_messages.push(serde_json::json!({
                                "role": "assistant",
                                "tool_calls": [{"id": &id, "type": "function", "function": {"name": &tname, "arguments": &args_str}}]
                            }));
                            model_messages.push(serde_json::json!({
                                "role": "tool", "tool_call_id": &id, "content": content.to_string(),
                            }));
                        }
                        Err(e) => {
                            append(
                                &state,
                                &run.id,
                                &mut seq,
                                &AgUiEvent::RunError {
                                    message: format!("tool execution failed: {e}"),
                                    code: Some("TOOL_ERROR".into()),
                                },
                            )
                            .await?;
                            return Err(e);
                        }
                    }
                }
                ProviderDelta::Done => {
                    if let Some(mid) = current_msg_id.take() {
                        append(
                            &state,
                            &run.id,
                            &mut seq,
                            &AgUiEvent::TextMessageEnd { message_id: mid },
                        )
                        .await?;
                    }
                    break;
                }
            }
        }

        if !assistant_text.is_empty() {
            sqlx::query(
                "INSERT INTO agent_messages (session_id, run_id, role, message_kind, content, created_at) VALUES (?, ?, 'assistant', 'text', ?, CURRENT_TIMESTAMP)",
            )
            .bind(&run.session_id).bind(&run.id).bind(&assistant_text)
            .execute(&state.pool).await
            .map_err(|e| AppError::Internal(e.into()))?;
        }

        if !produced_tool {
            break;
        }
    }

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
