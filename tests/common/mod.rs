use std::sync::Arc;

use tokio::sync::RwLock;

use issueflow::{config::Config, db::DbPool, http::routes::AppState};

const DB_URL: &str = "sqlite::memory:?cache=shared";

pub async fn test_pool() -> DbPool {
    sqlx::any::install_default_drivers();
    let pool = sqlx::AnyPool::connect(DB_URL).await.unwrap();
    issueflow::db::run_migrations(&pool, DB_URL).await.unwrap();
    pool
}

#[allow(dead_code)]
pub async fn test_app(config: Config) -> axum::Router {
    let pool = test_pool().await;
    issueflow::http::routes::router(AppState {
        config,
        pool,
        oidc_metadata: Arc::new(RwLock::new(None)),
    })
}

#[allow(dead_code)]
pub async fn test_app_with_pool(config: Config, pool: DbPool) -> axum::Router {
    issueflow::http::routes::router(AppState {
        config,
        pool,
        oidc_metadata: Arc::new(RwLock::new(None)),
    })
}
