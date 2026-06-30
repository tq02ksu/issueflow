#[path = "common/test_app.rs"]
mod test_app_support;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use issueflow::config::Config;
use tower::ServiceExt;

#[tokio::test]
async fn webhook_route_rejects_invalid_token() {
    let app = test_app_support::test_app(Config::for_tests("expected-token")).await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/webhooks/gitlab")
                .header("x-gitlab-token", "wrong-token")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"object_kind":"issue"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn webhook_route_rejects_invalid_token_before_json_parsing() {
    let app = test_app_support::test_app(Config::for_tests("expected-token")).await;
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/webhooks/gitlab")
                .header("x-gitlab-token", "wrong-token")
                .header("content-type", "application/json")
                .body(Body::from("not-json"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn webhook_route_accepts_valid_note_hook() {
    let app = test_app_support::test_app(Config::for_tests("expected-token")).await;
    let payload = r#"{"object_kind":"note","object_attributes":{"note":"/start-dev","noteable_type":"Issue"}}"#;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/webhooks/gitlab")
                .header("x-gitlab-token", "expected-token")
                .header("content-type", "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);
}
