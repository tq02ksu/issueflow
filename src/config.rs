use crate::oauth::OAuthConfig;

#[derive(Clone, Debug)]
pub struct Config {
    pub listen_addr: String,
    pub gitlab_webhook_secret: String,
    pub oauth: OAuthConfig,
}

impl Config {
    pub fn from_env() -> Self {
        let listen_addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let gitlab_webhook_secret = std::env::var("GITLAB_WEBHOOK_SECRET")
            .expect("GITLAB_WEBHOOK_SECRET must be set");
        let oauth = OAuthConfig::from_env();

        Self {
            listen_addr,
            gitlab_webhook_secret,
            oauth,
        }
    }

    pub fn for_tests(secret: &str) -> Self {
        Self {
            listen_addr: "127.0.0.1:0".to_string(),
            gitlab_webhook_secret: secret.to_string(),
            oauth: OAuthConfig::disabled(),
        }
    }

    pub fn with_oauth(mut self, oauth: OAuthConfig) -> Self {
        self.oauth = oauth;
        self
    }
}
