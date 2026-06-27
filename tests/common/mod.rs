use std::sync::{Arc, atomic::AtomicU64};

use tokio::sync::RwLock;

use issueflow::{config::Config, db::DbPool, http::routes::AppState};

static DB_COUNTER: AtomicU64 = AtomicU64::new(0);

pub async fn test_pool() -> DbPool {
    sqlx::any::install_default_drivers();
    let n = DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let db_url = format!("sqlite:test-{}-{}.db?mode=rwc", std::process::id(), n);
    let pool = sqlx::AnyPool::connect(&db_url).await.unwrap();
    issueflow::db::run_migrations(&pool, &db_url).await.unwrap();
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

pub async fn test_app_with_pool(config: Config, pool: DbPool) -> axum::Router {
    issueflow::http::routes::router(AppState {
        config,
        pool,
        oidc_metadata: Arc::new(RwLock::new(None)),
    })
}
