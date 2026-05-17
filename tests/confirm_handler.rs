use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use issueflow::{config::Config, http::routes};
use tower::ServiceExt;

#[tokio::test]
async fn confirm_plan_page_returns_ok() {
    let app = routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/confirm/plan/test-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
