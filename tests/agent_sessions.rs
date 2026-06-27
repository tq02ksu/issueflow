mod common;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode, header},
};
use issueflow::session::{build_claims, sign_token};
use tower::ServiceExt;

fn auth_header() -> (header::HeaderName, String) {
    let claims = build_claims(1, "user-sub", "gitlab-access-token");
    let token = sign_token(&claims, "test-jwt-secret").unwrap();
    (header::AUTHORIZATION, format!("Bearer {token}"))
}

#[tokio::test]
async fn create_and_list_agent_sessions() {
    // seed workbench via the same pool type
    {
        let pool = common::test_pool().await;
        sqlx::query("INSERT OR IGNORE INTO users (sub, name, email) VALUES ('user-sub', 'Test', 'test@test.com')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "INSERT OR IGNORE INTO workbenches (user_id, project_id, project_name, project_path) VALUES (1, 42, 'test-project', 'group/test')",
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    let app = common::test_app(issueflow::config::Config::for_tests("secret")).await;
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
    let status = create.status();
    let body = axum::body::to_bytes(create.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8_lossy(&body);
    println!("STATUS: {status}, BODY: {body_str}");
    assert_eq!(status, StatusCode::CREATED);

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
    let app = common::test_app(issueflow::config::Config::for_tests("secret")).await;

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
