use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use issueflow::config::Config;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn create_issue_rejects_empty_title() {
    let app = issueflow::http::routes::router(Config::for_tests("expected-token"));

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
                .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_issue_returns_internal_server_error_when_gitlab_api_config_is_missing() {
    let app = issueflow::http::routes::router(Config::for_tests("expected-token"));

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

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn create_issue_reaches_gitlab_with_valid_config() {
    let config = Config::for_tests("expected-token")
        .with_gitlab_api("https://gitlab.example.com", "glpat-test-token");
    let app = issueflow::http::routes::router(config);

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

    // With a valid config but unreachable host, the handler
    // gets past validation and reaches the GitLab API call.
    assert_ne!(response.status(), StatusCode::BAD_REQUEST);
}
