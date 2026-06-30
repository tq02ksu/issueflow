#[path = "common/auth_token.rs"]
mod auth_token_support;
#[path = "common/seed_user_and_workbench.rs"]
mod seed_user_and_workbench_support;
#[path = "common/test_app.rs"]
mod test_app_support;
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
async fn engineering_memory_refresh_upserts_latest_snapshot() {
    let pool = test_pool_support::test_pool().await;
    let (user_id, workbench_id) = seed_user_and_workbench_support::seed_user_and_workbench(
        &pool,
        "user-sub",
        123,
        "group/project",
    )
    .await;
    let app = test_app_with_pool_support::test_app_with_pool(
        Config::for_tests("expected-token"),
        pool.clone(),
    )
    .await;
    let token =
        auth_token_support::auth_token("test-jwt-secret", user_id, "user-sub", "user-token");

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/workbenches/{workbench_id}/memory/refresh"))
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{
                        "projectId": 123,
                        "artifactType": "issue",
                        "artifactId": "77",
                        "inputText": "Need an export button",
                        "inputContext": {"source":"agent-session"},
                        "spec": {"summary":"Add export","acceptance_criteria":[],"success_conditions":[],"boundary_conditions":[],"open_questions":[]},
                        "validationSuggestions": {"happy_path":[],"failure_path":[],"edge_cases":[],"non_goals":[]},
                        "riskNotes": [],
                        "evaluationSummary": {"status":"unknown","summary":"","coverage_notes":[],"missing_cases":[]}
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(first_response.status(), StatusCode::OK);

    let second_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/workbenches/{workbench_id}/memory/refresh"))
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{
                        "projectId": 123,
                        "artifactType": "issue",
                        "artifactId": "77",
                        "inputText": "Need an export button with csv",
                        "inputContext": {"source":"agent-session"},
                        "spec": {"summary":"Add export v2","acceptance_criteria":[],"success_conditions":[],"boundary_conditions":[],"open_questions":[]},
                        "validationSuggestions": {"happy_path":[],"failure_path":[],"edge_cases":[],"non_goals":[]},
                        "riskNotes": [],
                        "evaluationSummary": {"status":"unknown","summary":"","coverage_notes":[],"missing_cases":[]}
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(second_response.status(), StatusCode::OK);

    let row: (i64, String, String) = sqlx::query_as(
        "SELECT revision, input_text, spec FROM engineering_memory WHERE scope_key = ? AND artifact_type = ? AND artifact_id = ? AND memory_kind = ?",
    )
    .bind("project:123")
    .bind("issue")
    .bind("77")
    .bind("issue_context")
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(row.0, 2);
    assert_eq!(row.1, "Need an export button with csv");
    assert!(row.2.contains("Add export v2"));
}

#[tokio::test]
async fn engineering_memory_refresh_requires_authentication() {
    let app = test_app_support::test_app(Config::for_tests("expected-token")).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/workbenches/1/memory/refresh")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
