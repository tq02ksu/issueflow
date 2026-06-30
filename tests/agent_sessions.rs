#[path = "common/test_app.rs"]
mod test_app_support;
#[path = "common/test_app_with_pool.rs"]
mod test_app_with_pool_support;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode, header},
};
use issueflow::db::DbPool;
use issueflow::session::{build_claims, sign_token};
use tower::ServiceExt;

fn auth_header() -> (header::HeaderName, String) {
    let claims = build_claims(1, "user-sub", "gitlab-access-token");
    let token = sign_token(&claims, "test-jwt-secret").unwrap();
    (header::AUTHORIZATION, format!("Bearer {token}"))
}

async fn isolated_memory_pool() -> DbPool {
    sqlx::any::install_default_drivers();
    let db_url = format!(
        "sqlite:file:{}?mode=memory&cache=shared",
        uuid::Uuid::new_v4()
    );
    let pool = sqlx::AnyPool::connect(&db_url).await.unwrap();
    issueflow::db::run_migrations(&pool, &db_url).await.unwrap();
    pool
}

#[tokio::test]
async fn create_and_list_agent_sessions() {
    let pool = isolated_memory_pool().await;
    sqlx::query("INSERT OR IGNORE INTO users (sub, name, email) VALUES ('user-sub', 'Test', 'test@test.com')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT OR IGNORE INTO workbenches (user_id, project_id, project_name, project_path) VALUES (1, 42, 'test-project', 'group/test')")
        .execute(&pool).await.unwrap();

    let app = test_app_with_pool_support::test_app_with_pool(
        issueflow::config::Config::for_tests("secret"),
        pool,
    )
    .await;
    let (auth_name, auth_value) = auth_header();

    let create = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/workbenches/1/agent-sessions")
                .header(auth_name.clone(), auth_value.clone())
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create.status(), StatusCode::CREATED);

    let list = app
        .oneshot(
            Request::builder()
                .uri("/api/workbenches/1/agent-sessions")
                .header(auth_name, auth_value)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(list.status(), StatusCode::OK);
}

#[tokio::test]
async fn unauthenticated_request_is_rejected() {
    let app = test_app_support::test_app(issueflow::config::Config::for_tests("secret")).await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/workbenches/1/agent-sessions")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_session_rejects_stale_session_when_user_row_is_missing() {
    let pool = isolated_memory_pool().await;
    sqlx::query(
        "INSERT INTO workbenches (user_id, project_id, project_name, project_path, name)
         VALUES (1, 42, 'test-project', 'group/test', 'test-project')",
    )
    .execute(&pool)
    .await
    .unwrap();

    let app = test_app_with_pool_support::test_app_with_pool(
        issueflow::config::Config::for_tests("test-jwt-secret"),
        pool,
    )
    .await;
    let (auth_name, auth_value) = auth_header();

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/workbenches/1/agent-sessions")
                .header(auth_name, auth_value)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn delete_session_removes_dependent_agent_data() {
    let pool = isolated_memory_pool().await;
    sqlx::query("INSERT OR IGNORE INTO users (sub, name, email) VALUES ('user-sub', 'Test', 'test@test.com')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT OR IGNORE INTO workbenches (user_id, project_id, project_name, project_path) VALUES (1, 42, 'test-project', 'group/test')")
        .execute(&pool).await.unwrap();
    let session_id = uuid::Uuid::new_v4().to_string();
    let run_id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO agent_sessions (id, user_id, workbench_id, title, last_message_at, created_at, updated_at)
         VALUES (?, 1, 1, 'Test', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
    )
    .bind(&session_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO agent_runs (id, session_id, status, attempt_count, input_payload, started_at)
         VALUES (?, ?, 'completed', 1, '{}', CURRENT_TIMESTAMP)",
    )
    .bind(&run_id)
    .bind(&session_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO agent_messages (session_id, run_id, role, message_kind, content, created_at)
         VALUES (?, ?, 'assistant', 'text', 'hello', CURRENT_TIMESTAMP)",
    )
    .bind(&session_id)
    .bind(&run_id)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "INSERT INTO agent_run_events (run_id, seq, event_type, payload, created_at)
         VALUES (?, 0, 'RUN_STARTED', '{}', CURRENT_TIMESTAMP)",
    )
    .bind(&run_id)
    .execute(&pool)
    .await
    .unwrap();

    let app = test_app_with_pool_support::test_app_with_pool(
        issueflow::config::Config::for_tests("secret"),
        pool.clone(),
    )
    .await;
    let (auth_name, auth_value) = auth_header();

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri(format!("/api/workbenches/1/agent-sessions/{session_id}"))
                .header(auth_name, auth_value)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let session_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_sessions WHERE id = ?")
        .bind(&session_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let run_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_runs WHERE id = ?")
        .bind(&run_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    let message_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM agent_messages WHERE session_id = ?")
            .bind(&session_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    let event_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM agent_run_events WHERE run_id = ?")
            .bind(&run_id)
            .fetch_one(&pool)
            .await
            .unwrap();

    assert_eq!(session_count.0, 0);
    assert_eq!(run_count.0, 0);
    assert_eq!(message_count.0, 0);
    assert_eq!(event_count.0, 0);
}
