#[derive(Clone, Debug)]
pub struct Config {
    pub listen_addr: String,
    pub gitlab_webhook_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        let listen_addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let gitlab_webhook_secret = std::env::var("GITLAB_WEBHOOK_SECRET")
            .unwrap_or_else(|_| "development-secret".to_string());

        Self {
            listen_addr,
            gitlab_webhook_secret,
        }
    }

    pub fn for_tests(secret: &str) -> Self {
        Self {
            listen_addr: "127.0.0.1:0".to_string(),
            gitlab_webhook_secret: secret.to_string(),
        }
    }
}
