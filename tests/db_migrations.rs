use issueflow::db;

#[tokio::test]
async fn sqlite_migrations_create_tracking_rows_and_do_not_rerun() {
    sqlx::any::install_default_drivers();
    let db_url = format!(
        "sqlite:file:{}?mode=memory&cache=shared",
        uuid::Uuid::new_v4()
    );
    let pool = sqlx::AnyPool::connect(&db_url).await.unwrap();

    db::run_migrations(&pool, &db_url).await.unwrap();

    let first_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM _sqlx_migrations")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(first_count.0, 6);

    db::run_migrations(&pool, &db_url).await.unwrap();

    let second_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM _sqlx_migrations")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(second_count.0, 6);
}
