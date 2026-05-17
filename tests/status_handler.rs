use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
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
async fn session_status_page_returns_html_template_without_reflecting_session_id() {
    let app = issueflow::http::routes::router(Config::for_tests("expected-token"));
    let session_id = r#"demo-session<script>alert(1)<\/script>"#;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/status/session/demo-session%3Cscript%3Ealert(1)%3C%5C%2Fscript%3E")
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

    assert!(html.contains("<title>Issueflow Session Status</title>"));
    assert!(!html.contains(session_id));
}
