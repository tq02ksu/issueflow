#[path = "common/test_app.rs"]
mod test_app_support;
#[path = "common/test_app_with_pool.rs"]
mod test_app_with_pool_support;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use issueflow::{config::Config, session::build_claims, session::sign_token};
use serde_json::json;
use tower::ServiceExt;

fn auth_header(config: &Config) -> String {
    let claims = build_claims(1, "test-sub", "glpat-token");
    let jwt = sign_token(&claims, &config.jwt_secret).unwrap();
    format!("Bearer {jwt}")
}

async fn app_with_authenticated_user(config: Config) -> axum::Router {
    sqlx::any::install_default_drivers();
    let pool = sqlx::pool::PoolOptions::<sqlx::Any>::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    issueflow::db::run_migrations(&pool, "sqlite::memory:")
        .await
        .unwrap();
    sqlx::query("INSERT INTO users (id, sub, name, email) VALUES (1, 'test-sub', 'Test User', 'test@example.com')")
        .execute(&pool)
        .await
        .unwrap();
    test_app_with_pool_support::test_app_with_pool(config, pool).await
}

#[tokio::test]
async fn create_workbench_with_empty_name_defaults_from_path() {
    let config = Config::for_tests("test-token");
    let app = app_with_authenticated_user(config.clone()).await;
    let auth = auth_header(&config);

    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/workbenches")
                .header(header::AUTHORIZATION, &auth)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "project_id": 42,
                        "project_path": "group/subgroup/my-repo",
                        "name": ""
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn create_workbench_with_explicit_name() {
    let config = Config::for_tests("test-token");
    let app = app_with_authenticated_user(config.clone()).await;
    let auth = auth_header(&config);

    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/workbenches")
                .header(header::AUTHORIZATION, &auth)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "project_id": 99,
                        "project_path": "org/repo",
                        "name": "My Custom Name"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn create_workbench_rejects_stale_session_when_user_row_is_missing() {
    let config = Config::for_tests("test-token");
    let app = test_app_support::test_app(config.clone()).await;
    let auth = auth_header(&config);

    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/workbenches")
                .header(header::AUTHORIZATION, auth)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{"project_id":123,"project_path":"group/project","name":"My WB"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn update_workbench_name_preserves_project_binding() {
    let config = Config::for_tests("test-token");
    let app = app_with_authenticated_user(config.clone()).await;
    let auth = auth_header(&config);

    let create_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/workbenches")
                .header(header::AUTHORIZATION, &auth)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "project_id": 777,
                        "project_path": "x/y/z",
                        "name": "old"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_resp.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(create_resp.into_body(), 1024)
        .await
        .unwrap();
    let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let wb_id = created["id"].as_i64().unwrap();

    let resp = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/api/workbenches/{wb_id}"))
                .header(header::AUTHORIZATION, &auth)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "project_id": 777,
                        "project_path": "x/y/z",
                        "name": "new-name"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn get_capabilities_returns_features() {
    let config = Config::for_tests("test-token");
    let app = test_app_support::test_app(config.clone()).await;
    let auth = auth_header(&config);

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/api/workbenches/1/capabilities")
                .header(header::AUTHORIZATION, &auth)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        json["features"],
        serde_json::json!(["overview", "issues", "pending_actions"])
    );
}
