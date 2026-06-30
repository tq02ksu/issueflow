pub async fn test_pool() -> issueflow::db::DbPool {
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
