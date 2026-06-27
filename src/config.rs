pub mod raw;
pub mod sources;

use crate::oidc::OidcConfig;

#[derive(Clone, Debug)]
pub struct GitConfig {
    pub webhook_secret: String,
    pub base_url: Option<String>,
    pub token: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub listen_addr: String,
    pub git: GitConfig,
    pub oidc: OidcConfig,
    pub jwt_secret: String,
}

impl Config {
    pub async fn load() -> Result<Self, String> {
        let raw = sources::load_raw_config()?;
        let listen_addr = raw
            .server
            .and_then(|server| server.listen_addr)
            .unwrap_or_else(|| "127.0.0.1:8080".to_string());
        let webhook_secret = raw
            .git
            .as_ref()
            .and_then(|git| git.webhook_secret.as_deref())
            .map(str::to_string)
            .ok_or("missing required configuration: git.webhook_secret")?;
        let base_url = raw
            .git
            .as_ref()
            .and_then(|git| git.base_url.as_deref())
            .map(str::to_string);
        let token = raw
            .git
            .as_ref()
            .and_then(|git| git.token.as_deref())
            .map(str::to_string);
        let oidc = OidcConfig::from_raw(raw.oidc.unwrap_or_default()).await?;

        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "issueflow-default-jwt-secret".to_string());

        Ok(Self {
            listen_addr,
            git: GitConfig {
                webhook_secret,
                base_url,
                token,
            },
            oidc,
            jwt_secret,
        })
    }

    pub fn for_tests(secret: &str) -> Self {
        Self {
            listen_addr: "127.0.0.1:0".to_string(),
            git: GitConfig {
                webhook_secret: secret.to_string(),
                base_url: None,
                token: None,
            },
            oidc: OidcConfig::disabled(),
            jwt_secret: "test-jwt-secret".to_string(),
        }
    }

    pub fn with_oidc(mut self, oidc: OidcConfig) -> Self {
        self.oidc = oidc;
        self
    }

    pub fn with_gitlab_api(mut self, base_url: &str, token: &str) -> Self {
        self.git.base_url = Some(base_url.to_string());
        self.git.token = Some(token.to_string());
        self
    }
}
