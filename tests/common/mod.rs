use issueflow::{config::Config, db::DbPool, http::routes::AppState};

pub async fn test_pool() -> DbPool {
    sqlx::any::install_default_drivers();
    let pool = sqlx::AnyPool::connect("sqlite::memory:").await.unwrap();
    issueflow::db::run_migrations(&pool, "sqlite::memory:").await.unwrap();
    pool
}

pub async fn test_app(config: Config) -> axum::Router {
    let pool = test_pool().await;
    issueflow::http::routes::router(AppState { config, pool })
}
