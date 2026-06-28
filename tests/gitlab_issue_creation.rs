mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use issueflow::{
    config::Config,
    session::{build_claims, sign_token},
};
use serde_json::json;
use tower::ServiceExt;

fn auth_header(jwt_secret: &str, access_token: &str) -> String {
    let claims = build_claims(7, "user-sub", access_token);
    let token = sign_token(&claims, jwt_secret).unwrap();
    format!("Bearer {token}")
}

#[tokio::test]
async fn create_issue_rejects_empty_title() {
    let app = common::test_app(Config::for_tests("expected-token")).await;

    let payload = json!({
        "project_id": 123,
        "title": "  ",
        "description": "Created by issueflow agent"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/issues")
                .header(header::CONTENT_TYPE, "application/json")
                .header(
                    header::AUTHORIZATION,
                    auth_header("test-jwt-secret", "gitlab-user-access-token"),
                )
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_issue_returns_internal_server_error_when_gitlab_api_config_is_missing() {
    let app = common::test_app(Config::for_tests("expected-token")).await;

    let payload = json!({
        "project_id": 123,
        "title": "Draft onboarding issue",
        "description": "Created by issueflow agent"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/issues")
                .header(header::CONTENT_TYPE, "application/json")
                .header(
                    header::AUTHORIZATION,
                    auth_header("test-jwt-secret", "gitlab-user-access-token"),
                )
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn create_issue_requires_authenticated_session() {
    let mut config = Config::for_tests("expected-token");
    config.git.base_url = Some("https://gitlab.example.com".to_string());
    let app = common::test_app(config).await;

    let payload = json!({
        "project_id": 123,
        "title": "Draft onboarding issue",
        "description": "Created by issueflow agent"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/issues")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_issue_with_authenticated_session_reaches_gitlab_layer_without_server_token() {
    let mut config = Config::for_tests("expected-token");
    config.git.base_url = Some("https://gitlab.example.com".to_string());
    let app = common::test_app(config).await;

    let payload = json!({
        "project_id": 123,
        "title": "Draft onboarding issue",
        "description": "Created by issueflow agent"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/issues")
                .header(header::CONTENT_TYPE, "application/json")
                .header(
                    header::AUTHORIZATION,
                    auth_header("test-jwt-secret", "gitlab-user-access-token"),
                )
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
    assert_ne!(response.status(), StatusCode::BAD_REQUEST);
}
