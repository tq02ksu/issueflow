use crate::{config::AgentConfig, error::AppError};

pub use agui_runtime::provider::ProviderDelta;

pub struct OpenAiClient {
    inner: agui_runtime::openai::OpenAiClient,
}

impl OpenAiClient {
    pub fn new(config: &AgentConfig) -> Self {
        let inner =
            agui_runtime::openai::OpenAiClient::new(agui_runtime::openai::OpenAiClientConfig {
                base_url: config
                    .openai_base_url
                    .clone()
                    .unwrap_or_else(|| "https://api.openai.com/v1".into()),
                api_key: config.openai_api_key.clone().unwrap_or_default(),
                model: config.model.clone().unwrap_or_else(|| "gpt-4o".into()),
            });

        Self { inner }
    }

    pub async fn stream_chat(
        &self,
        messages: Vec<serde_json::Value>,
        tools: Vec<serde_json::Value>,
    ) -> Result<impl futures::Stream<Item = Result<ProviderDelta, AppError>>, AppError> {
        let stream = self.inner.stream_chat(messages, tools).await?;

        use futures::StreamExt;

        Ok(stream.map(|delta| delta.map_err(AppError::from)))
    }
}
