use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use issueflow::{config::Config, http::routes};
use tower::ServiceExt;

#[tokio::test]
async fn confirm_plan_page_returns_html_template_without_reflecting_token() {
    let app = routes::router(Config::for_tests("expected-token"));
    let token = r#"test-token<script>alert(1)<\/script>"#;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/confirm/plan/test-token%3Cscript%3Ealert(1)%3C%5C%2Fscript%3E")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.contains("text/html")));

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let html = String::from_utf8(body.to_vec()).unwrap();

    assert!(html.contains("<title>Issueflow Plan Confirmation</title>"));
    assert!(!html.contains(token));
}
