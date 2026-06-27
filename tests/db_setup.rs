#[tokio::test]
async fn migration_creates_users_and_workbenches_tables() {
    sqlx::any::install_default_drivers();
    let pool = sqlx::AnyPool::connect("sqlite::memory:").await.unwrap();

    for sql in [
        "CREATE TABLE IF NOT EXISTS users (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            sub        TEXT NOT NULL UNIQUE,
            name       TEXT NOT NULL DEFAULT '',
            email      TEXT NOT NULL DEFAULT '',
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        "CREATE TABLE IF NOT EXISTS workbenches (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id      INTEGER NOT NULL,
            project_id   INTEGER NOT NULL,
            project_name TEXT NOT NULL,
            project_path TEXT NOT NULL,
            created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(user_id, project_id)
        )",
    ] {
        sqlx::query(sql).execute(&pool).await.unwrap();
    }

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
