use std::sync::Arc;

use tokio::sync::RwLock;

use issueflow::{config::Config, db::DbPool, http::routes::AppState};

pub async fn test_pool() -> DbPool {
    sqlx::any::install_default_drivers();
    let db_url = format!("sqlite:test-{}.db?mode=rwc", std::process::id());
    let pool = sqlx::AnyPool::connect(&db_url).await.unwrap();
    issueflow::db::run_migrations(&pool, &db_url)
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
