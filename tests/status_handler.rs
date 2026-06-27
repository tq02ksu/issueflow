mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use issueflow::config::Config;
use tower::ServiceExt;

#[tokio::test]
async fn status_route_returns_ok() {
    let app = common::test_app(Config::for_tests("expected-token")).await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/status/ping")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn session_status_redirects_to_workbench_with_session_id() {
    let app = common::test_app(Config::for_tests("expected-token")).await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/status/session/demo-session-123")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        response
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/workbench?session=demo-session-123")
    );
}
