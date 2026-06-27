mod common;

#[tokio::test]
async fn migrations_create_all_agent_tables() {
    let pool = common::test_pool().await;

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
