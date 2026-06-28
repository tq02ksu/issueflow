use async_openai::{Client, config::OpenAIConfig};

use crate::provider::ProviderDelta;

#[derive(Debug, Clone)]
pub struct OpenAiClientConfig {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("{0}")]
    OpenAi(#[from] async_openai::error::OpenAIError),
}

pub struct OpenAiClient {
    client: Client<OpenAIConfig>,
    model: String,
}

impl OpenAiClient {
    pub fn new(config: OpenAiClientConfig) -> Self {
        let client = Client::with_config(
            OpenAIConfig::new()
                .with_api_base(config.base_url)
                .with_api_key(config.api_key),
        );

        Self {
            client,
            model: config.model,
        }
    }

    pub async fn stream_chat(
        &self,
        messages: Vec<serde_json::Value>,
        tools: Vec<serde_json::Value>,
    ) -> Result<impl futures::Stream<Item = Result<ProviderDelta, RuntimeError>>, RuntimeError>
    {
        let request = serde_json::json!({
            "model": self.model,
            "stream": true,
            "messages": messages,
            "tools": tools,
        });

        let stream = self
            .client
            .chat()
            .create_stream_byot::<_, serde_json::Value>(request)
            .await?;

        use futures::StreamExt;
        use tokio::sync::mpsc;

        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(async move {
            let mut stream = stream;
            let mut active_tool_call_id = None;
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(delta) => {
                        if let Some(choices) = delta["choices"].as_array() {
                            for choice in choices {
                                for d in collect_deltas(choice, &mut active_tool_call_id) {
                                    let _ = tx.send(Ok(d)).await;
                                }
                            }
                        }
                    }
                    Err(error) => {
                        let _ = tx.send(Err(RuntimeError::from(error))).await;
                    }
                }
            }
        });

        Ok(tokio_stream::wrappers::ReceiverStream::new(rx))
    }
}

fn collect_deltas(
    choice: &serde_json::Value,
    active_tool_call_id: &mut Option<String>,
) -> Vec<ProviderDelta> {
    let mut deltas = Vec::new();

    if let Some(content) = choice["delta"]["content"].as_str()
        && !content.is_empty()
    {
        deltas.push(ProviderDelta::Text(content.to_string()));
    }

    if let Some(tool_calls) = choice["delta"]["tool_calls"].as_array() {
        for tc in tool_calls {
            if let Some(id) = tc["id"].as_str().filter(|id| !id.is_empty()) {
                *active_tool_call_id = Some(id.to_string());
            }

            let current_id = tc["id"]
                .as_str()
                .filter(|id| !id.is_empty())
                .map(str::to_string)
                .or_else(|| active_tool_call_id.clone());

            let name = tc["function"]["name"].as_str().unwrap_or("");
            if let Some(id) = current_id.clone()
                && !name.is_empty()
            {
                deltas.push(ProviderDelta::ToolStart {
                    id,
                    name: name.to_string(),
                });
            }

            let args = tc["function"]["arguments"].as_str().unwrap_or("");
            if let Some(id) = current_id
                && !args.is_empty()
            {
                deltas.push(ProviderDelta::ToolArgs {
                    id,
                    delta: args.to_string(),
                });
            }
        }
    }

    if choice["finish_reason"].as_str() == Some("tool_calls")
        && let Some(id) = active_tool_call_id.take()
    {
        deltas.push(ProviderDelta::ToolEnd { id });
    }

    if choice["finish_reason"].as_str() == Some("stop") {
        deltas.push(ProviderDelta::Done);
    }

    deltas
}

#[cfg(test)]
mod tests {
    use crate::provider::ProviderDelta;

    use super::collect_deltas;

    #[test]
    fn collect_deltas_emits_done_for_stop_finish_reason() {
        let choice = serde_json::json!({
            "delta": {},
            "finish_reason": "stop"
        });

        let mut active_tool_call_id = None;
        let deltas = collect_deltas(&choice, &mut active_tool_call_id);

        assert!(
            deltas
                .iter()
                .any(|delta| matches!(delta, ProviderDelta::Done))
        );
    }

    #[test]
    fn collect_deltas_emits_tool_end_for_finish_reason_without_tool_payload() {
        let start = serde_json::json!({
            "delta": {
                "tool_calls": [{
                    "id": "call_123",
                    "function": { "name": "list_issues" }
                }]
            },
            "finish_reason": null
        });
        let choice = serde_json::json!({
            "delta": {},
            "finish_reason": "tool_calls"
        });
        let mut active_tool_call_id = None;

        let start_deltas = collect_deltas(&start, &mut active_tool_call_id);
        let end_deltas = collect_deltas(&choice, &mut active_tool_call_id);

        assert!(start_deltas.iter().any(|delta| matches!(
            delta,
            ProviderDelta::ToolStart { id, name } if id == "call_123" && name == "list_issues"
        )));

        assert!(end_deltas.iter().any(|delta| matches!(
            delta,
            ProviderDelta::ToolEnd { id } if id == "call_123"
        )));
    }
}
