use std::{collections::HashMap, future::Future};

use agui_protocol::events::AgUiEvent;
use futures::{Stream, StreamExt};

use crate::{openai::RuntimeError, provider::ProviderDelta};

#[derive(Debug)]
pub enum RuntimeEngineError {
    Message(String),
    Provider(RuntimeError),
    Json(serde_json::Error),
}

impl std::fmt::Display for RuntimeEngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(message) => f.write_str(message),
            Self::Provider(error) => write!(f, "{error}"),
            Self::Json(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for RuntimeEngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Message(_) => None,
            Self::Provider(error) => Some(error),
            Self::Json(error) => Some(error),
        }
    }
}

impl From<RuntimeError> for RuntimeEngineError {
    fn from(value: RuntimeError) -> Self {
        Self::Provider(value)
    }
}

impl From<serde_json::Error> for RuntimeEngineError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
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
                    let message_id = current_msg_id.clone().ok_or_else(|| {
                        RuntimeEngineError::Message(
                            "assistant text chunk arrived before message start".to_string(),
                        )
                    })?;
                    record_event(
                        &mut all_events,
                        &mut emit_event,
                        AgUiEvent::TextMessageContent {
                            message_id,
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

        if !assistant_text.trim().is_empty() {
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::stream;
    use tokio::sync::Mutex;

    use crate::provider::ProviderDelta;

    use super::run_chat_rounds;

    #[tokio::test]
    async fn tool_round_ignores_whitespace_only_assistant_history() {
        let emitted = Arc::new(Mutex::new(Vec::new()));
        let emitted_clone = emitted.clone();
        let round = Arc::new(Mutex::new(0usize));
        let round_clone = round.clone();

        let result = run_chat_rounds(
            vec![serde_json::json!({
                "role": "user",
                "content": "count issues"
            })],
            vec![serde_json::json!({"name": "list_issues"})],
            move |_messages, _tools| {
                let round = round_clone.clone();
                async move {
                    let mut current_round = round.lock().await;
                    let deltas = if *current_round == 0 {
                        *current_round += 1;
                        vec![
                            Ok(ProviderDelta::Text("\n".into())),
                            Ok(ProviderDelta::ToolStart {
                                id: "call_1".into(),
                                name: "list_issues".into(),
                            }),
                            Ok(ProviderDelta::ToolArgs {
                                id: "call_1".into(),
                                delta: "{\"project_id\":37}".into(),
                            }),
                            Ok(ProviderDelta::ToolEnd {
                                id: "call_1".into(),
                            }),
                            Ok(ProviderDelta::Done),
                        ]
                    } else {
                        vec![Ok(ProviderDelta::Done)]
                    };

                    Ok(stream::iter(deltas))
                }
            },
            |_tool_name, _args| async move { Ok(serde_json::json!([])) },
            move |event| {
                let emitted = emitted_clone.clone();
                let event = event.clone();
                async move {
                    emitted.lock().await.push(event);
                    Ok(())
                }
            },
        )
        .await
        .unwrap_or_else(|error| panic!("run_chat_rounds should succeed: {error}"));

        assert!(result.assistant_messages.is_empty());
        assert!(!emitted.lock().await.is_empty());
    }
}
