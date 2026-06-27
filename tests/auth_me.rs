mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use issueflow::{config::Config, session::build_claims, session::sign_token};
use tower::ServiceExt;

#[tokio::test]
async fn auth_me_accepts_valid_jwt() {
    let config = Config::for_tests("test-token");
    let app = common::test_app(config.clone()).await;

    let claims = build_claims(1, "test-sub", "glpat-test-token");
    let jwt = sign_token(&claims, &config.jwt_secret).unwrap();

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/me")
                .header(header::AUTHORIZATION, format!("Bearer {jwt}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn auth_me_rejects_missing_header() {
    let config = Config::for_tests("test-token");
    let app = common::test_app(config).await;

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth_me_rejects_invalid_jwt() {
    let config = Config::for_tests("test-token");
    let app = common::test_app(config).await;

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/me")
                .header(header::AUTHORIZATION, "Bearer invalid.token.here")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth_me_rejects_jwt_with_wrong_secret() {
    let config = Config::for_tests("test-token");
    let app = common::test_app(config).await;

    let claims = build_claims(1, "test-sub", "glpat-test-token");
    let jwt = sign_token(&claims, "wrong-secret").unwrap();

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/me")
                .header(header::AUTHORIZATION, format!("Bearer {jwt}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
