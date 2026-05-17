use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use issueflow::config::Config;
use tower::ServiceExt;

#[tokio::test]
async fn status_route_returns_ok() {
    let app = issueflow::http::routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(Request::builder().uri("/status/ping").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn session_status_page_returns_ok() {
    let app = issueflow::http::routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/status/session/demo-session")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
