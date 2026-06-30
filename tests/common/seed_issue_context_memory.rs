use issueflow::db::DbPool;

pub async fn seed_issue_context_memory(
    pool: &DbPool,
    project_id: i64,
    workbench_id: i64,
    issue_iid: &str,
    input_text: &str,
    spec: &str,
) -> String {
    let id = format!("context-{project_id}-{workbench_id}-{issue_iid}");
    sqlx::query(
        "INSERT INTO engineering_memory (
            id, artifact_type, artifact_id, scope_type, scope_key, scope_project_id, scope_workbench_id, scope_user_id,
            memory_kind, status, revision, updated_by_user_id, input_text, input_context, source_snapshot,
            spec, validation_suggestions, risk_notes, evaluation_summary, created_at, updated_at
         ) VALUES (?, 'issue', ?, 'workbench', ?, ?, ?, NULL, 'issue_context', 'active', 1, 1, ?, ?, NULL, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
    )
    .bind(&id)
    .bind(issue_iid)
    .bind(format!("workbench:{workbench_id}"))
    .bind(project_id)
    .bind(workbench_id)
    .bind(input_text)
    .bind(r#"{"source":"test"}"#)
    .bind(spec)
    .bind(r#"{"happy_path":[],"failure_path":[],"edge_cases":[],"non_goals":[]}"#)
    .bind("[]")
    .bind(r#"{"status":"captured","summary":"Issue context seeded for tests."}"#)
    .execute(pool)
    .await
    .unwrap();

    id
}
