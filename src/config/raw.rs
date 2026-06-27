use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawConfig {
    pub server: Option<RawServerConfig>,
    pub git: Option<RawGitConfig>,
    pub oidc: Option<RawOidcConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawServerConfig {
    pub listen_addr: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawGitConfig {
    pub webhook_secret: Option<String>,
    pub base_url: Option<String>,
    pub token: Option<String>,
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

impl RawConfig {
    pub fn merge(mut self, other: Self) -> Self {
        self.server = merge_server(self.server, other.server);
        self.git = merge_git(self.git, other.git);
        self.oidc = merge_oidc(self.oidc, other.oidc);
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

fn merge_git(current: Option<RawGitConfig>, incoming: Option<RawGitConfig>) -> Option<RawGitConfig> {
    match (current, incoming) {
        (Some(mut current), Some(incoming)) => {
            if incoming.webhook_secret.is_some() {
                current.webhook_secret = incoming.webhook_secret;
            }
            if incoming.base_url.is_some() {
                current.base_url = incoming.base_url;
            }
            if incoming.token.is_some() {
                current.token = incoming.token;
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
