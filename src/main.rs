use std::sync::Arc;

use tokio::sync::RwLock;

use issueflow::{
    config::Config,
    db,
    http::{routes::AppState, server},
};

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

    let state = AppState {
        config,
        pool,
        oidc_metadata: Arc::new(RwLock::new(None)),
    };

    // start background worker loop
    let worker_state = state.clone();
    tokio::spawn(async move {
        if let Err(e) = issueflow::agent::worker::run_loop(worker_state).await {
            tracing::error!(%e, "agent worker loop failed");
        }
    });

    if let Err(e) = server::serve_with_state(state).await {
        tracing::error!(%e, "server error");
        std::process::exit(1);
    }
}
