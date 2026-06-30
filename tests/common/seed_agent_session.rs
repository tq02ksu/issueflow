use issueflow::db::DbPool;

pub async fn seed_agent_session(pool: &DbPool, user_id: i64, workbench_id: i64) -> String {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO agent_sessions (
            id, user_id, workbench_id, title, last_message_at, created_at, updated_at
         ) VALUES (?, ?, ?, 'Test Session', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
    )
    .bind(&id)
    .bind(user_id)
    .bind(workbench_id)
    .execute(pool)
    .await
    .unwrap();

    id
}
