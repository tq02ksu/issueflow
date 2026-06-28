use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawConfig {
    pub server: Option<RawServerConfig>,
    pub git: Option<RawGitConfig>,
    pub oidc: Option<RawOidcConfig>,
    pub agent: Option<RawAgentConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawServerConfig {
    pub listen_addr: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawGitConfig {
    pub webhook_secret: Option<String>,
    pub base_url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawOidcConfig {
    pub enabled: Option<bool>,
    pub issuer: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub state_signing_secret: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawAgentConfig {
    pub openai_base_url: Option<String>,
    pub openai_api_key: Option<String>,
    pub model: Option<String>,
    pub max_tool_rounds: Option<u32>,
}

impl RawConfig {
    pub fn merge(mut self, other: Self) -> Self {
        self.server = merge_server(self.server, other.server);
        self.git = merge_git(self.git, other.git);
        self.oidc = merge_oidc(self.oidc, other.oidc);
        self.agent = merge_agent(self.agent, other.agent);
        self
    }
}

fn merge_server(
    current: Option<RawServerConfig>,
    incoming: Option<RawServerConfig>,
) -> Option<RawServerConfig> {
    match (current, incoming) {
        (Some(mut current), Some(incoming)) => {
            if incoming.listen_addr.is_some() {
                current.listen_addr = incoming.listen_addr;
            }
            Some(current)
        }
        (None, Some(incoming)) => Some(incoming),
        (current, None) => current,
    }
}

fn merge_git(
    current: Option<RawGitConfig>,
    incoming: Option<RawGitConfig>,
) -> Option<RawGitConfig> {
    match (current, incoming) {
        (Some(mut current), Some(incoming)) => {
            if incoming.webhook_secret.is_some() {
                current.webhook_secret = incoming.webhook_secret;
            }
            if incoming.base_url.is_some() {
                current.base_url = incoming.base_url;
            }
            Some(current)
        }
        (None, Some(incoming)) => Some(incoming),
        (current, None) => current,
    }
}

fn merge_oidc(
    current: Option<RawOidcConfig>,
    incoming: Option<RawOidcConfig>,
) -> Option<RawOidcConfig> {
    match (current, incoming) {
        (Some(mut current), Some(incoming)) => {
            if incoming.enabled.is_some() {
                current.enabled = incoming.enabled;
            }
            if incoming.issuer.is_some() {
                current.issuer = incoming.issuer;
            }
            if incoming.client_id.is_some() {
                current.client_id = incoming.client_id;
            }
            if incoming.client_secret.is_some() {
                current.client_secret = incoming.client_secret;
            }
            if incoming.redirect_uri.is_some() {
                current.redirect_uri = incoming.redirect_uri;
            }
            if incoming.scopes.is_some() {
                current.scopes = incoming.scopes;
            }
            if incoming.state_signing_secret.is_some() {
                current.state_signing_secret = incoming.state_signing_secret;
            }
            Some(current)
        }
        (None, Some(incoming)) => Some(incoming),
        (current, None) => current,
    }
}

fn merge_agent(
    current: Option<RawAgentConfig>,
    incoming: Option<RawAgentConfig>,
) -> Option<RawAgentConfig> {
    match (current, incoming) {
        (Some(mut current), Some(incoming)) => {
            if incoming.openai_base_url.is_some() {
                current.openai_base_url = incoming.openai_base_url;
            }
            if incoming.openai_api_key.is_some() {
                current.openai_api_key = incoming.openai_api_key;
            }
            if incoming.model.is_some() {
                current.model = incoming.model;
            }
            if incoming.max_tool_rounds.is_some() {
                current.max_tool_rounds = incoming.max_tool_rounds;
            }
            Some(current)
        }
        (None, Some(incoming)) => Some(incoming),
        (current, None) => current,
    }
}
