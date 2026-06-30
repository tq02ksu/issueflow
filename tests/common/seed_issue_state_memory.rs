use issueflow::db::DbPool;

pub async fn seed_issue_state_memory(
    pool: &DbPool,
    project_id: i64,
    issue_iid: &str,
    current_state: &str,
    next_state: &str,
) -> String {
    let id = format!("state-{project_id}-{issue_iid}");
    sqlx::query(
        "INSERT INTO engineering_memory (
            id, artifact_type, artifact_id, scope_type, scope_key, scope_project_id, scope_workbench_id, scope_user_id,
            memory_kind, status, revision, updated_by_user_id, input_text, input_context, source_snapshot,
            spec, validation_suggestions, risk_notes, evaluation_summary, created_at, updated_at
         ) VALUES (?, 'issue', ?, 'project', ?, ?, NULL, NULL, 'issue_state', 'active', 1, 1, ?, ?, NULL, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
    )
    .bind(&id)
    .bind(issue_iid)
    .bind(format!("project:{project_id}"))
    .bind(project_id)
    .bind("raw issue text")
    .bind("{}")
    .bind(r#"{"summary":"Issue state memory","acceptance_criteria":[],"boundary_conditions":[],"open_questions":[]}"#)
    .bind(r#"{"happy_path":[],"failure_path":[],"edge_cases":[],"non_goals":[]}"#)
    .bind("[]")
    .bind(format!(
        r#"{{"current_state":"{current_state}","proposed_next_state":"{next_state}","summary":"state summary","missing_context":[],"blockers":[],"role_notes":{{"product":[],"engineering":[],"delivery":[]}},"heavy_agent":{{"required":false,"reason":"","preferred_implementation":null}}}}"#
    ))
    .execute(pool)
    .await
    .unwrap();

    id
}
