#[path = "common/test_app.rs"]
mod test_app_support;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use issueflow::config::Config;
use tower::ServiceExt;

#[tokio::test]
async fn confirm_plan_redirects_to_workbench_with_confirm_token() {
    let app = test_app_support::test_app(Config::for_tests("expected-token")).await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/confirm/plan/test-token")
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
        Some("/workbench?confirm=test-token")
    );
}
