#[path = "common/test_app.rs"]
mod test_app_support;
#[path = "common/test_app_with_pool.rs"]
mod test_app_with_pool_support;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use issueflow::{config::Config, session::build_claims, session::sign_token};
use tower::ServiceExt;

async fn isolated_memory_pool() -> sqlx::AnyPool {
    sqlx::any::install_default_drivers();
    let pool = sqlx::pool::PoolOptions::<sqlx::Any>::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    issueflow::db::run_migrations(&pool, "sqlite::memory:")
        .await
        .unwrap();
    pool
}

#[tokio::test]
async fn auth_me_accepts_valid_jwt() {
    let config = Config::for_tests("test-token");
    let pool = isolated_memory_pool().await;
    sqlx::query("INSERT INTO users (id, sub, name, email) VALUES (1, 'test-sub', 'Test User', 'test@example.com')")
        .execute(&pool)
        .await
        .unwrap();
    let app = test_app_with_pool_support::test_app_with_pool(config.clone(), pool).await;

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
async fn auth_me_rejects_jwt_without_local_user() {
    let config = Config::for_tests("test-token");
    let app = test_app_support::test_app(config.clone()).await;

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

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth_me_rejects_missing_header() {
    let config = Config::for_tests("test-token");
    let app = test_app_support::test_app(config).await;

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
    let app = test_app_support::test_app(config).await;

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
    let app = test_app_support::test_app(config).await;

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
