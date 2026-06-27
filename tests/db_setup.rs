#[tokio::test]
async fn migration_creates_users_and_workbenches_tables() {
    sqlx::any::install_default_drivers();
    let pool = sqlx::AnyPool::connect("sqlite::memory:").await.unwrap();

    let result = issueflow::db::run_migrations(&pool, "sqlite::memory:").await;
    assert!(result.is_ok(), "run_migrations failed: {:?}", result.err());

    let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(user_count.0, 0);

    let wb_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM workbenches")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(wb_count.0, 0);
}
