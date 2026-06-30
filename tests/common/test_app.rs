use std::sync::Arc;

use issueflow::{config::Config, http::routes::AppState};
use tokio::sync::RwLock;

async fn test_pool() -> issueflow::db::DbPool {
    sqlx::any::install_default_drivers();
    let pool = sqlx::pool::PoolOptions::<sqlx::Any>::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    issueflow::db::run_migrations(&pool, "sqlite::memory:")
        .await
        .unwrap();
    pool
}

pub async fn test_app(config: Config) -> axum::Router {
    let pool = test_pool().await;
    issueflow::http::routes::router(AppState {
        config,
        pool,
        oidc_metadata: Arc::new(RwLock::new(None)),
    })
}
