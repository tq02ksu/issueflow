use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn status_route_returns_ok() {
    let app = issueflow::http::routes::router();
    let response = app
        .oneshot(Request::builder().uri("/status/ping").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
