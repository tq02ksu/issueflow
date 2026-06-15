pub mod raw;
pub mod sources;

use crate::oidc::OidcConfig;

#[derive(Clone, Debug)]
pub struct GitConfig {
    pub webhook_secret: String,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub listen_addr: String,
    pub git: GitConfig,
    pub oidc: OidcConfig,
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
            .and_then(|git| git.webhook_secret)
            .ok_or("missing required configuration: git.webhook_secret")?;
        let oidc = OidcConfig::from_raw(raw.oidc.unwrap_or_default()).await?;

        Ok(Self {
            listen_addr,
            git: GitConfig {
                webhook_secret,
            },
            oidc,
        })
    }

    pub fn for_tests(secret: &str) -> Self {
        Self {
            listen_addr: "127.0.0.1:0".to_string(),
            git: GitConfig {
                webhook_secret: secret.to_string(),
            },
            oidc: OidcConfig::disabled(),
        }
    }

    pub fn with_oidc(mut self, oidc: OidcConfig) -> Self {
        self.oidc = oidc;
        self
    }
}
