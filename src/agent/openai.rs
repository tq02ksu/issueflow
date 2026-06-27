use serde::Serialize;

use crate::config::AgentConfig;

#[derive(Debug, Serialize)]
pub struct OpenAiChatRequest {
    pub model: String,
    pub stream: bool,
    pub messages: Vec<serde_json::Value>,
    pub tools: Vec<serde_json::Value>,
}

#[derive(Debug)]
pub enum ProviderDelta {
    Text(String),
    ToolStart { id: String, name: String },
    ToolArgs { id: String, delta: String },
    ToolEnd { id: String },
    Done,
}

pub struct OpenAiClient {
    base_url: String,
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl OpenAiClient {
    pub fn new(config: &AgentConfig) -> Self {
        Self {
            base_url: config
                .openai_base_url
                .clone()
                .unwrap_or_else(|| "https://api.openai.com/v1".into()),
            api_key: config.openai_api_key.clone().unwrap_or_default(),
            model: config.model.clone().unwrap_or_else(|| "gpt-4o".into()),
            client: reqwest::Client::new(),
        }
    }

    pub async fn stream_chat(
        &self,
        messages: Vec<serde_json::Value>,
        tools: Vec<serde_json::Value>,
    ) -> Result<
        impl futures::Stream<Item = Result<ProviderDelta, crate::error::AppError>>,
        crate::error::AppError,
    > {
        let req = OpenAiChatRequest {
            model: self.model.clone(),
            stream: true,
            messages,
            tools,
        };

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&req)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(crate::error::AppError::Internal(
                format!("openai api returned {status}: {body}").into(),
            ));
        }

        let stream = resp.bytes_stream();

        use futures::StreamExt;
        use tokio::sync::mpsc;

        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(async move {
            let mut buf = String::new();
            let mut stream = stream;
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        buf.push_str(&String::from_utf8_lossy(&bytes));
                        while let Some(pos) = buf.find('\n') {
                            let line = buf[..pos].trim().to_string();
                            buf = buf[pos + 1..].to_string();

                            if line.is_empty() {
                                continue;
                            }
                            let line = line.strip_prefix("data: ").unwrap_or(&line);
                            if line == "[DONE]" {
                                let _ = tx.send(Ok(ProviderDelta::Done)).await;
                                continue;
                            }
                            if let Ok(delta) = serde_json::from_str::<serde_json::Value>(line)
                                && let Some(choices) = delta["choices"].as_array()
                            {
                                for choice in choices {
                                    if let Some(d) = translate_delta(choice) {
                                        let _ = tx.send(Ok(d)).await;
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx
                            .send(Err(crate::error::AppError::Internal(e.into())))
                            .await;
                    }
                }
            }
        });

        Ok(tokio_stream::wrappers::ReceiverStream::new(rx))
    }
}

fn translate_delta(choice: &serde_json::Value) -> Option<ProviderDelta> {
    if let Some(content) = choice["delta"]["content"].as_str()
        && !content.is_empty()
    {
        return Some(ProviderDelta::Text(content.to_string()));
    }
    if let Some(tool_calls) = choice["delta"]["tool_calls"].as_array() {
        for tc in tool_calls {
            let id = tc["id"].as_str().unwrap_or("").to_string();
            let name = tc["function"]["name"].as_str().unwrap_or("").to_string();
            if !id.is_empty() && !name.is_empty() {
                return Some(ProviderDelta::ToolStart { id, name });
            }
            let args = tc["function"]["arguments"].as_str().unwrap_or("");
            if !id.is_empty() && !args.is_empty() {
                return Some(ProviderDelta::ToolArgs {
                    id,
                    delta: args.to_string(),
                });
            }
        }
    }
    if choice["finish_reason"].as_str().is_some()
        && let Some(tc) = choice["delta"]["tool_calls"].as_array()
        && let Some(first) = tc.first()
        && let Some(id) = first["id"].as_str()
    {
        return Some(ProviderDelta::ToolEnd { id: id.to_string() });
    }
    None
}
