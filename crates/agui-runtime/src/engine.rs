use std::{collections::HashMap, future::Future};

use agui_protocol::events::AgUiEvent;
use futures::{Stream, StreamExt};

use crate::{openai::RuntimeError, provider::ProviderDelta};

#[derive(Debug, thiserror::Error)]
pub enum RuntimeEngineError {
    #[error("{0}")]
    Message(String),
    #[error("{0}")]
    Provider(#[from] RuntimeError),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct ChatRunResult {
    pub model_messages: Vec<serde_json::Value>,
    pub assistant_messages: Vec<String>,
    pub events: Vec<AgUiEvent>,
}

pub async fn run_chat_rounds<
    StartStream,
    StartFut,
    DeltaStream,
    ExecuteTool,
    ExecuteToolFut,
    EmitEvent,
    EmitEventFut,
>(
    mut model_messages: Vec<serde_json::Value>,
    tools: Vec<serde_json::Value>,
    mut start_stream: StartStream,
    mut execute_tool: ExecuteTool,
    mut emit_event: EmitEvent,
) -> Result<ChatRunResult, RuntimeEngineError>
where
    StartStream: FnMut(Vec<serde_json::Value>, Vec<serde_json::Value>) -> StartFut,
    StartFut: Future<Output = Result<DeltaStream, RuntimeEngineError>>,
    DeltaStream: Stream<Item = Result<ProviderDelta, RuntimeError>> + Unpin,
    ExecuteTool: FnMut(&str, serde_json::Value) -> ExecuteToolFut,
    ExecuteToolFut: Future<Output = Result<serde_json::Value, RuntimeEngineError>>,
    EmitEvent: FnMut(&AgUiEvent) -> EmitEventFut,
    EmitEventFut: Future<Output = Result<(), RuntimeEngineError>>,
{
    let mut all_events = Vec::new();
    let mut assistant_messages = Vec::new();

    loop {
        let mut stream = start_stream(model_messages.clone(), tools.clone()).await?;
        let mut current_msg_id: Option<String> = None;
        let mut assistant_text = String::new();
        let mut produced_tool = false;
        let mut tool_name_map: HashMap<String, String> = HashMap::new();
        let mut tool_args_map: HashMap<String, String> = HashMap::new();

        while let Some(delta) = stream.next().await {
            match delta? {
                ProviderDelta::Text(text) => {
                    if current_msg_id.is_none() {
                        let mid = uuid::Uuid::new_v4().to_string();
                        current_msg_id = Some(mid.clone());
                        record_event(
                            &mut all_events,
                            &mut emit_event,
                            AgUiEvent::TextMessageStart {
                                message_id: mid,
                                role: "assistant".into(),
                            },
                        )
                        .await?;
                    }

                    assistant_text.push_str(&text);
                    record_event(
                        &mut all_events,
                        &mut emit_event,
                        AgUiEvent::TextMessageContent {
                            message_id: current_msg_id.clone().unwrap(),
                            delta: text,
                        },
                    )
                    .await?;
                }
                ProviderDelta::ToolStart { id, name } => {
                    if let Some(mid) = current_msg_id.take() {
                        record_event(
                            &mut all_events,
                            &mut emit_event,
                            AgUiEvent::TextMessageEnd { message_id: mid },
                        )
                        .await?;
                    }

                    tool_name_map.insert(id.clone(), name.clone());
                    record_event(
                        &mut all_events,
                        &mut emit_event,
                        AgUiEvent::ToolCallStart {
                            tool_call_id: id,
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
                    record_event(
                        &mut all_events,
                        &mut emit_event,
                        AgUiEvent::ToolCallArgs {
                            tool_call_id: id,
                            delta,
                        },
                    )
                    .await?;
                }
                ProviderDelta::ToolEnd { id } => {
                    produced_tool = true;
                    record_event(
                        &mut all_events,
                        &mut emit_event,
                        AgUiEvent::ToolCallEnd {
                            tool_call_id: id.clone(),
                        },
                    )
                    .await?;

                    let args_str = tool_args_map.remove(&id).unwrap_or_default();
                    let args: serde_json::Value =
                        serde_json::from_str(&args_str).unwrap_or(serde_json::Value::Null);
                    let tool_name = tool_name_map.remove(&id).unwrap_or_default();
                    let content = execute_tool(&tool_name, args).await?;

                    record_event(
                        &mut all_events,
                        &mut emit_event,
                        AgUiEvent::ToolCallResult {
                            message_id: uuid::Uuid::new_v4().to_string(),
                            tool_call_id: id.clone(),
                            content: content.clone(),
                            role: "tool".into(),
                        },
                    )
                    .await?;

                    model_messages.push(serde_json::json!({
                        "role": "assistant",
                        "tool_calls": [{"id": &id, "type": "function", "function": {"name": &tool_name, "arguments": &args_str}}]
                    }));
                    model_messages.push(serde_json::json!({
                        "role": "tool",
                        "tool_call_id": &id,
                        "content": content.to_string(),
                    }));
                }
                ProviderDelta::Done => {
                    if let Some(mid) = current_msg_id.take() {
                        record_event(
                            &mut all_events,
                            &mut emit_event,
                            AgUiEvent::TextMessageEnd { message_id: mid },
                        )
                        .await?;
                    }
                    break;
                }
            }
        }

        if !assistant_text.is_empty() {
            assistant_messages.push(assistant_text);
        }

        if !produced_tool {
            break;
        }
    }

    Ok(ChatRunResult {
        model_messages,
        assistant_messages,
        events: all_events,
    })
}

async fn record_event<EmitEvent, EmitEventFut>(
    events: &mut Vec<AgUiEvent>,
    emit_event: &mut EmitEvent,
    event: AgUiEvent,
) -> Result<(), RuntimeEngineError>
where
    EmitEvent: FnMut(&AgUiEvent) -> EmitEventFut,
    EmitEventFut: Future<Output = Result<(), RuntimeEngineError>>,
{
    emit_event(&event).await?;
    events.push(event);
    Ok(())
}
