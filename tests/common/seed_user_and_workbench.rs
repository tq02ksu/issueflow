use issueflow::db::{self, DbPool};

pub async fn seed_user_and_workbench(
    pool: &DbPool,
    user_sub: &str,
    project_id: i64,
    project_path: &str,
) -> (i64, i64) {
    let user = db::upsert_user(pool, user_sub, "Test User", "test@example.com")
        .await
        .unwrap();

    let row: (i64,) = sqlx::query_as(
        "INSERT INTO workbenches (user_id, project_id, project_name, project_path, name)
         VALUES (?, ?, ?, ?, ?)
         RETURNING id",
    )
    .bind(user.id)
    .bind(project_id)
    .bind(project_path)
    .bind(project_path)
    .bind("Test Workbench")
    .fetch_one(pool)
    .await
    .unwrap();

    (user.id, row.0)
}
