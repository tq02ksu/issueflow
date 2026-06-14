use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use issueflow::config::Config;
use tower::ServiceExt;

#[tokio::test]
async fn root_route_serves_spa_shell_html() {
    let app = issueflow::http::routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
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

    assert!(html.contains("<title>issueflow</title>"));
    assert!(html.contains("/assets/app.js"));
    assert!(html.contains("/assets/app.css"));
}

#[tokio::test]
async fn workbench_route_reuses_the_same_spa_shell() {
    let app = issueflow::http::routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/workbench")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
