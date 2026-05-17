use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use issueflow::{config::Config, http::routes};
use tower::ServiceExt;

#[tokio::test]
async fn webhook_route_rejects_invalid_token() {
    let app = routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/gitlab")
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
async fn webhook_route_accepts_valid_note_hook() {
    let app = routes::router(Config::for_tests("expected-token"));
    let payload = r#"{"object_kind":"note","object_attributes":{"note":"/start-dev","noteable_type":"Issue"}}"#;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/gitlab")
                .header("x-gitlab-token", "expected-token")
                .header("content-type", "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);
}
