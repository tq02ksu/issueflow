mod common;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode, header},
};
use issueflow::{
    db::DbPool,
    session::{build_claims, sign_token},
};
use tokio::time::{Duration, timeout};
use tower::ServiceExt;

fn auth_header() -> (header::HeaderName, String) {
    let claims = build_claims(1, "user-sub", "gitlab-access-token");
    let token = sign_token(&claims, "test-jwt-secret").unwrap();
    (header::AUTHORIZATION, format!("Bearer {token}"))
}

async fn seed_session(pool: &DbPool) -> String {
    sqlx::query("INSERT OR IGNORE INTO users (sub, name, email) VALUES ('user-sub', 'Test', 'test@test.com')")
        .execute(pool).await.unwrap();
    sqlx::query("INSERT OR IGNORE INTO workbenches (user_id, project_id, project_name, project_path) VALUES (1, 42, 'test', 'g/t')")
        .execute(pool).await.unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO agent_sessions (id, user_id, workbench_id, title, last_message_at, created_at, updated_at) VALUES (?, 1, 1, 'Test', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
    ).bind(&id).execute(pool).await.unwrap();
    id
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
async fn run_creation_requires_auth() {
    let app = common::test_app(issueflow::config::Config::for_tests("secret")).await;
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/agent/runs")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{"threadId":"x","workbenchId":1,"messages":[]}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_run_returns_durable_metadata() {
    let pool = isolated_memory_pool().await;
    let session_id = seed_session(&pool).await;
    let app =
        common::test_app_with_pool(issueflow::config::Config::for_tests("secret"), pool).await;
    let (auth_name, auth_value) = auth_header();

    let response = app.oneshot(
        Request::builder().method(Method::POST).uri("/api/agent/runs")
            .header(auth_name, auth_value).header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(format!(r#"{{"threadId":"{}","workbenchId":1,"messages":[{{"role":"user","content":"hello"}}]}}"#, session_id))).unwrap(),
    ).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("runId").is_some());
    assert_eq!(
        json.get("threadId").and_then(|v| v.as_str()),
        Some(session_id.as_str())
    );
    assert_eq!(json.get("status").and_then(|v| v.as_str()), Some("queued"));
}

#[tokio::test]
async fn subscribe_events_returns_event_stream() {
    let pool = isolated_memory_pool().await;
    let session_id = seed_session(&pool).await;
    let input =
        serde_json::json!({"threadId": &session_id, "workbenchId": 1, "messages": []}).to_string();
    let run_id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO agent_runs (id, session_id, status, attempt_count, input_payload, started_at) VALUES (?, ?, 'queued', 0, ?, CURRENT_TIMESTAMP)")
        .bind(&run_id).bind(&session_id).bind(&input).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO agent_run_events (run_id, seq, event_type, payload, created_at) VALUES (?, 1, 'RUN_STARTED', '{}', CURRENT_TIMESTAMP)")
        .bind(&run_id).execute(&pool).await.unwrap();

    let app =
        common::test_app_with_pool(issueflow::config::Config::for_tests("secret"), pool).await;
    let (auth_name, auth_value) = auth_header();
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!("/api/agent/runs/{run_id}/events?after_seq=0"))
                .header(auth_name, auth_value)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers()[header::CONTENT_TYPE],
        "text/event-stream"
    );
}

#[tokio::test]
async fn subscribe_events_waits_for_later_events_before_finishing() {
    let pool = isolated_memory_pool().await;
    let session_id = seed_session(&pool).await;
    let input =
        serde_json::json!({"threadId": &session_id, "workbenchId": 1, "messages": []}).to_string();
    let run_id = uuid::Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO agent_runs (id, session_id, status, attempt_count, input_payload, started_at) VALUES (?, ?, 'running', 0, ?, CURRENT_TIMESTAMP)")
        .bind(&run_id).bind(&session_id).bind(&input).execute(&pool).await.unwrap();

    let writer_pool = pool.clone();
    let writer_run_id = run_id.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(50)).await;
        sqlx::query("INSERT INTO agent_run_events (run_id, seq, event_type, payload, created_at) VALUES (?, 1, 'RUN_STARTED', '{\"type\":\"RUN_STARTED\"}', CURRENT_TIMESTAMP)")
            .bind(&writer_run_id)
            .execute(&writer_pool)
            .await
            .unwrap();
        sqlx::query(
            "UPDATE agent_runs SET status = 'completed', finished_at = CURRENT_TIMESTAMP WHERE id = ?",
        )
        .bind(&writer_run_id)
        .execute(&writer_pool)
        .await
        .unwrap();
    });

    let app =
        common::test_app_with_pool(issueflow::config::Config::for_tests("secret"), pool).await;
    let (auth_name, auth_value) = auth_header();
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!("/api/agent/runs/{run_id}/events?after_seq=0"))
                .header(auth_name, auth_value)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = timeout(
        Duration::from_secs(1),
        axum::body::to_bytes(response.into_body(), usize::MAX),
    )
    .await
    .unwrap()
    .unwrap();
    let text = String::from_utf8(body.to_vec()).unwrap();

    assert!(text.contains("event: RUN_STARTED"));
}
