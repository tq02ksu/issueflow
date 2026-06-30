use crate::{
    db::DbPool,
    error::AppError,
    memory::models::{EngineeringMemoryRow, UpsertEngineeringMemoryInput},
};

pub async fn upsert_engineering_memory(
    pool: &DbPool,
    input: &UpsertEngineeringMemoryInput,
) -> Result<EngineeringMemoryRow, AppError> {
    let scope_key = input.scope_key()?;

    sqlx::query_as(
        "INSERT INTO engineering_memory (
            id, artifact_type, artifact_id, scope_type, scope_key, scope_project_id, scope_workbench_id, scope_user_id,
            memory_kind, status, revision, updated_by_user_id, input_text, input_context, source_snapshot,
            spec, validation_suggestions, risk_notes, evaluation_summary
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(scope_key, artifact_type, artifact_id, memory_kind)
         DO UPDATE SET
            status = excluded.status,
            revision = engineering_memory.revision + 1,
            updated_by_user_id = excluded.updated_by_user_id,
            input_text = excluded.input_text,
            input_context = excluded.input_context,
            source_snapshot = excluded.source_snapshot,
            spec = excluded.spec,
            validation_suggestions = excluded.validation_suggestions,
            risk_notes = excluded.risk_notes,
            evaluation_summary = excluded.evaluation_summary,
            updated_at = CURRENT_TIMESTAMP
         RETURNING id, artifact_type, artifact_id, scope_type, scope_key, scope_project_id,
            scope_workbench_id, scope_user_id, memory_kind, status, revision, updated_by_user_id,
            input_text, input_context, source_snapshot, spec, validation_suggestions, risk_notes,
            evaluation_summary, created_at, updated_at",
    )
    .bind(&input.id)
    .bind(&input.artifact_type)
    .bind(&input.artifact_id)
    .bind(input.scope_type.as_str())
    .bind(&scope_key)
    .bind(input.scope_project_id)
    .bind(input.scope_workbench_id)
    .bind(input.scope_user_id)
    .bind(input.memory_kind.as_str())
    .bind(&input.status)
    .bind(input.updated_by_user_id)
    .bind(&input.input_text)
    .bind(&input.input_context)
    .bind(&input.source_snapshot)
    .bind(&input.spec)
    .bind(&input.validation_suggestions)
    .bind(&input.risk_notes)
    .bind(&input.evaluation_summary)
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

pub async fn find_engineering_memory(
    pool: &DbPool,
    scope_key: &str,
    artifact_type: &str,
    artifact_id: &str,
    memory_kind: &str,
) -> Result<Option<EngineeringMemoryRow>, AppError> {
    sqlx::query_as(
        "SELECT id, artifact_type, artifact_id, scope_type, scope_key, scope_project_id,
            scope_workbench_id, scope_user_id, memory_kind, status, revision, updated_by_user_id,
            input_text, input_context, source_snapshot, spec, validation_suggestions, risk_notes,
            evaluation_summary, created_at, updated_at
         FROM engineering_memory
         WHERE scope_key = ? AND artifact_type = ? AND artifact_id = ? AND memory_kind = ?",
    )
    .bind(scope_key)
    .bind(artifact_type)
    .bind(artifact_id)
    .bind(memory_kind)
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)
}
