#[path = "common/test_pool.rs"]
mod test_pool_support;

#[tokio::test]
async fn migrations_create_all_agent_tables() {
    let pool = test_pool_support::test_pool().await;

    let _: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_sessions")
        .fetch_one(&pool)
        .await
        .unwrap();
    let _: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_runs")
        .fetch_one(&pool)
        .await
        .unwrap();
    let _: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_messages")
        .fetch_one(&pool)
        .await
        .unwrap();
    let _: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_run_events")
        .fetch_one(&pool)
        .await
        .unwrap();
}
