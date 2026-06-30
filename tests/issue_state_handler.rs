#[path = "common/auth_token.rs"]
mod auth_token_support;
#[path = "common/seed_issue_context_memory.rs"]
mod seed_issue_context_memory_support;
#[path = "common/seed_issue_state_memory.rs"]
mod seed_issue_state_memory_support;
#[path = "common/seed_user_and_workbench.rs"]
mod seed_user_and_workbench_support;
#[path = "common/test_app_with_pool.rs"]
mod test_app_with_pool_support;
#[path = "common/test_pool.rs"]
mod test_pool_support;

use axum::body::to_bytes;
use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use issueflow::config::Config;
use issueflow::issue_state::{
    models::IssueStateKind,
    service::{IssueStateInput, evaluate_issue_state},
};
use tower::ServiceExt;

#[test]
fn evaluate_issue_with_description_moves_from_new_to_planned() {
    let state = evaluate_issue_state(IssueStateInput {
        current_state: IssueStateKind::New.as_str(),
        title: "Export report",
        description: Some("Users need CSV export"),
        acceptance_criteria: &[],
        verification_notes: &[],
        notes: &[],
    });

    assert_eq!(state.current_state, "new");
    assert_eq!(state.proposed_next_state, "planned");
}

#[test]
fn evaluate_issue_with_acceptance_criteria_and_test_plan_moves_to_ready_for_execution() {
    let acceptance_criteria = vec!["CSV export works for filtered results".to_string()];
    let verification_notes = vec!["Add API and UI tests for CSV export".to_string()];
    let state = evaluate_issue_state(IssueStateInput {
        current_state: IssueStateKind::Planned.as_str(),
        title: "Export report",
        description: Some("Users need CSV export from the report page."),
        acceptance_criteria: &acceptance_criteria,
        verification_notes: &verification_notes,
        notes: &[],
    });

    assert_eq!(state.current_state, "planned");
    assert_eq!(state.proposed_next_state, "ready_for_execution");
    assert!(state.missing_context.is_empty());
}

#[tokio::test]
async fn evaluate_issue_state_creates_project_state_memory_and_pending_action() {
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
    seed_issue_context_memory_support::seed_issue_context_memory(
        &pool,
        123,
        workbench_id,
        "77",
        "Export report\n\nUsers need CSV export from the report page.",
        r#"{"summary":"Export report","acceptance_criteria":["CSV export works for filtered results"],"boundary_conditions":[],"open_questions":[],"verification_notes":["Add API and UI tests for CSV export"]}"#,
    )
    .await;
    let token =
        auth_token_support::auth_token("test-jwt-secret", user_id, "user-sub", "user-token");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/workbenches/{workbench_id}/issues/77/state/evaluate"
                ))
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["workbenchContext"]["memory_kind"], "issue_context");
    assert_eq!(json["workbenchContext"]["scope_type"], "workbench");
    assert_eq!(json["projectState"]["memory_kind"], "issue_state");
    assert_eq!(json["projectState"]["scope_type"], "project");
    let summary: serde_json::Value =
        serde_json::from_str(json["projectState"]["evaluation_summary"].as_str().unwrap()).unwrap();
    assert_eq!(summary["currentState"], "new");
    assert_eq!(summary["proposedNextState"], "ready_for_execution");
    assert_eq!(
        json["pendingAction"]["action_type"],
        "apply_issue_state_transition"
    );
    let payload: serde_json::Value =
        serde_json::from_str(json["pendingAction"]["payload"].as_str().unwrap()).unwrap();
    assert_eq!(payload["currentState"], "new");
    assert_eq!(payload["proposedNextState"], "ready_for_execution");
    assert_eq!(payload["sourceMemoryId"], json["projectState"]["id"]);
}

#[tokio::test]
async fn get_issue_state_returns_project_state_memory() {
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
    seed_issue_state_memory_support::seed_issue_state_memory(
        &pool,
        123,
        "77",
        "clarifying",
        "planned",
    )
    .await;
    let token =
        auth_token_support::auth_token("test-jwt-secret", user_id, "user-sub", "user-token");

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/workbenches/{workbench_id}/issues/77/state"))
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["projectMemory"]["memory_kind"], "issue_state");
    assert_eq!(json["projectMemory"]["scope_type"], "project");
    let summary: serde_json::Value = serde_json::from_str(
        json["projectMemory"]["evaluation_summary"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(summary["current_state"], "clarifying");
    assert_eq!(summary["proposed_next_state"], "planned");
    assert!(json["personalNote"].is_null());
    assert!(json["pendingAction"].is_null());
}
