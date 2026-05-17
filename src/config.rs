#[derive(Clone, Debug)]
pub struct Config {
    pub listen_addr: String,
}

impl Config {
    pub fn from_env() -> Self {
        let listen_addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        Self { listen_addr }
    }
}
