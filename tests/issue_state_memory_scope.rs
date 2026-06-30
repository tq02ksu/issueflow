#[path = "common/test_pool.rs"]
mod test_pool_support;

use issueflow::memory::{
    models::{MemoryKind, MemoryScopeType, UpsertEngineeringMemoryInput},
    store,
};

#[tokio::test]
async fn upsert_engineering_memory_separates_project_and_personal_issue_state_scope() {
    let pool = test_pool_support::test_pool().await;

    let shared = store::upsert_engineering_memory(
        &pool,
        &UpsertEngineeringMemoryInput {
            id: "shared-1".into(),
            artifact_type: "issue".into(),
            artifact_id: "77".into(),
            scope_type: MemoryScopeType::Project,
            scope_project_id: Some(123),
            scope_workbench_id: None,
            scope_user_id: None,
            memory_kind: MemoryKind::IssueState,
            status: "active".into(),
            updated_by_user_id: Some(7),
            input_text: "raw".into(),
            input_context: "{}".into(),
            source_snapshot: Some("{}".into()),
            spec: "{}".into(),
            validation_suggestions: "{}".into(),
            risk_notes: "[]".into(),
            evaluation_summary: r#"{"status":"ready"}"#.into(),
        },
    )
    .await
    .unwrap();

    let personal = store::upsert_engineering_memory(
        &pool,
        &UpsertEngineeringMemoryInput {
            id: "personal-1".into(),
            artifact_type: "issue".into(),
            artifact_id: "77".into(),
            scope_type: MemoryScopeType::Personal,
            scope_project_id: Some(123),
            scope_workbench_id: Some(9),
            scope_user_id: Some(7),
            memory_kind: MemoryKind::IssueNote,
            status: "active".into(),
            updated_by_user_id: Some(7),
            input_text: "raw".into(),
            input_context: "{}".into(),
            source_snapshot: None,
            spec: "{}".into(),
            validation_suggestions: "{}".into(),
            risk_notes: "[]".into(),
            evaluation_summary: r#"{"status":"ready"}"#.into(),
        },
    )
    .await
    .unwrap();

    assert_ne!(shared.id, personal.id);
}
