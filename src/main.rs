use issueflow::{config::Config, db, http::server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let config = match Config::load().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(%e, "failed to load configuration");
            std::process::exit(1);
        }
    };

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:issueflow.db?mode=rwc".to_string());
    let pool = match db::open(&database_url).await {
        Ok(p) => p,
        Err(e) => {
            tracing::error!(%e, "failed to open database");
            std::process::exit(1);
        }
    };

    if let Err(e) = server::serve(config, pool).await {
        tracing::error!(%e, "server error");
        std::process::exit(1);
    }
}
