#[path = "common/auth_token.rs"]
mod auth_token_support;
#[path = "common/seed_user_and_workbench.rs"]
mod seed_user_and_workbench_support;
#[path = "common/test_app_with_pool.rs"]
mod test_app_with_pool_support;
#[path = "common/test_pool.rs"]
mod test_pool_support;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use issueflow::config::Config;
use tower::ServiceExt;

#[tokio::test]
async fn readiness_routes_are_not_exposed_anymore() {
    let pool = test_pool_support::test_pool().await;
    let app = test_app_with_pool_support::test_app_with_pool(
        Config::for_tests("expected-token").with_gitlab_base_url("https://gitlab.example.com"),
        pool.clone(),
    )
    .await;
    let (user_id, workbench_id) = seed_user_and_workbench_support::seed_user_and_workbench(
        &pool,
        "user-sub",
        123,
        "group/project",
    )
    .await;
    let token =
        auth_token_support::auth_token("test-jwt-secret", user_id, "user-sub", "user-token");

    let evaluate_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/workbenches/{workbench_id}/issues/77/readiness/run"
                ))
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(evaluate_response.status(), StatusCode::NOT_FOUND);

    let detail_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/api/workbenches/{workbench_id}/issues/77/readiness"
                ))
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(detail_response.status(), StatusCode::NOT_FOUND);
}
