#[path = "common/auth_token.rs"]
mod auth_token_support;
#[path = "common/seed_agent_session.rs"]
mod seed_agent_session_support;
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
use issueflow::{config::Config, http::routes::AppState};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceExt;

#[tokio::test]
async fn create_pending_action_persists_issue_update_draft() {
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

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/workbenches/{workbench_id}/pending-actions"))
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{
                        "projectId": 123,
                        "artifactType": "issue",
                        "artifactId": "77",
                        "actionType": "update_gitlab_issue",
                        "payload": {
                            "issueIid": 77,
                            "targetField": "description",
                            "updateMode": "replace_full",
                            "renderedContent": "new issue body"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let row: (String, String) = sqlx::query_as(
        "SELECT action_type, status FROM pending_actions WHERE workbench_id = ? AND project_id = ?",
    )
    .bind(workbench_id)
    .bind(123i64)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(row.0, "update_gitlab_issue");
    assert_eq!(row.1, "pending");
}

#[tokio::test]
async fn confirm_pending_action_marks_action_confirmed() {
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

    let action_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO pending_actions (
            id, workbench_id, project_id, artifact_type, artifact_id, action_type, status, payload,
            created_by_user_id, created_at, updated_at
         ) VALUES (?, ?, ?, ?, ?, ?, 'pending', ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
    )
    .bind(&action_id)
    .bind(workbench_id)
    .bind(123i64)
    .bind("issue")
    .bind("77")
    .bind("update_gitlab_issue")
    .bind(r#"{"issueIid":77,"targetField":"description","updateMode":"replace_full","renderedContent":"new body"}"#)
    .bind(user_id)
    .execute(&pool)
    .await
    .unwrap();

    let token =
        auth_token_support::auth_token("test-jwt-secret", user_id, "user-sub", "user-token");
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/pending-actions/{action_id}/confirm"))
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let row: (String, i64) =
        sqlx::query_as("SELECT status, confirmed_by_user_id FROM pending_actions WHERE id = ?")
            .bind(&action_id)
            .fetch_one(&pool)
            .await
            .unwrap();

    assert_eq!(row.0, "confirmed");
    assert_eq!(row.1, user_id);
}

#[tokio::test]
async fn create_pending_action_requires_authentication() {
    let app = test_app_support::test_app(Config::for_tests("expected-token")).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/workbenches/1/pending-actions")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn confirm_pending_action_with_source_session_enqueues_agent_run() {
    let pool = test_pool_support::test_pool().await;
    let (user_id, workbench_id) = seed_user_and_workbench_support::seed_user_and_workbench(
        &pool,
        "user-sub",
        123,
        "group/project",
    )
    .await;
    let session_id =
        seed_agent_session_support::seed_agent_session(&pool, user_id, workbench_id).await;
    let app = test_app_with_pool_support::test_app_with_pool(
        Config::for_tests("expected-token"),
        pool.clone(),
    )
    .await;

    let action_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO pending_actions (
            id, workbench_id, project_id, artifact_type, artifact_id, action_type, status, payload,
            source_session_id, created_by_user_id, created_at, updated_at
         ) VALUES (?, ?, ?, ?, ?, ?, 'pending', ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
    )
    .bind(&action_id)
    .bind(workbench_id)
    .bind(123i64)
    .bind("issue")
    .bind("77")
    .bind("refresh_memory")
    .bind(r#"{}"#)
    .bind(&session_id)
    .bind(user_id)
    .execute(&pool)
    .await
    .unwrap();

    let token =
        auth_token_support::auth_token("test-jwt-secret", user_id, "user-sub", "user-token");
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/pending-actions/{action_id}/confirm"))
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let row: (String, String) = sqlx::query_as(
        "SELECT session_id, status FROM agent_runs WHERE session_id = ? ORDER BY started_at DESC LIMIT 1",
    )
    .bind(&session_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(row.0, session_id);
    assert_eq!(row.1, "queued");
}

#[tokio::test]
async fn queued_refresh_memory_action_completes_through_worker() {
    let pool = test_pool_support::test_pool().await;
    let (user_id, workbench_id) = seed_user_and_workbench_support::seed_user_and_workbench(
        &pool,
        "user-sub",
        123,
        "group/project",
    )
    .await;
    let session_id =
        seed_agent_session_support::seed_agent_session(&pool, user_id, workbench_id).await;

    let action_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO pending_actions (
            id, workbench_id, project_id, artifact_type, artifact_id, action_type, status, payload,
            source_session_id, created_by_user_id, created_at, updated_at
         ) VALUES (?, ?, ?, ?, ?, ?, 'confirmed', ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
    )
    .bind(&action_id)
    .bind(workbench_id)
    .bind(123i64)
    .bind("issue")
    .bind("77")
    .bind("refresh_memory")
    .bind(r#"{}"#)
    .bind(&session_id)
    .bind(user_id)
    .execute(&pool)
    .await
    .unwrap();

    let input_payload =
        serde_json::to_string(&issueflow::agent::models::ExecutePendingActionInput {
            pending_action_id: action_id.clone(),
            access_token: "user-token".to_string(),
        })
        .unwrap();
    let run_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO agent_runs (
            id, session_id, status, attempt_count, input_payload, started_at
         ) VALUES (?, ?, 'queued', 0, ?, CURRENT_TIMESTAMP)",
    )
    .bind(&run_id)
    .bind(&session_id)
    .bind(&input_payload)
    .execute(&pool)
    .await
    .unwrap();

    let state = AppState {
        config: Config::for_tests("expected-token"),
        pool: pool.clone(),
        oidc_metadata: Arc::new(RwLock::new(None)),
    };

    let worked = issueflow::agent::worker::run_once(state).await.unwrap();

    assert!(worked);

    let action_status: (String,) =
        sqlx::query_as("SELECT status FROM pending_actions WHERE id = ?")
            .bind(&action_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(action_status.0, "completed");

    let run_status: (String,) = sqlx::query_as("SELECT status FROM agent_runs WHERE id = ?")
        .bind(&run_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(run_status.0, "completed");
}

#[tokio::test]
async fn get_pending_action_returns_preview_for_issue_update() {
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

    let action_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO pending_actions (
            id, workbench_id, project_id, artifact_type, artifact_id, action_type, status, payload,
            created_by_user_id, created_at, updated_at
         ) VALUES (?, ?, ?, ?, ?, ?, 'pending', ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
    )
    .bind(&action_id)
    .bind(workbench_id)
    .bind(123i64)
    .bind("issue")
    .bind("77")
    .bind("update_gitlab_issue")
    .bind(r#"{"issueIid":77,"renderedContent":"preview body"}"#)
    .bind(user_id)
    .execute(&pool)
    .await
    .unwrap();

    let token =
        auth_token_support::auth_token("test-jwt-secret", user_id, "user-sub", "user-token");
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/pending-actions/{action_id}"))
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["action"]["id"], action_id);
    assert_eq!(json["preview"]["kind"], "gitlab_issue_description");
    assert_eq!(json["preview"]["title"], "Replace issue #77 description");
    assert_eq!(json["preview"]["body"], "preview body");
}
