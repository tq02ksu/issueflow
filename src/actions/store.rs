use crate::{
    actions::models::{CreatePendingActionInput, PendingActionRow},
    db::DbPool,
};

pub async fn insert_pending_action(
    pool: &DbPool,
    input: &CreatePendingActionInput,
) -> Result<PendingActionRow, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO pending_actions (
            id, workbench_id, project_id, artifact_type, artifact_id, action_type, status, payload,
            source_session_id, source_run_id, created_by_user_id, assigned_user_id, created_at, updated_at
         ) VALUES (?, ?, ?, ?, ?, ?, 'pending', ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
         RETURNING id, workbench_id, project_id, artifact_type, artifact_id, action_type, status, payload,
            source_session_id, source_run_id, created_by_user_id, assigned_user_id, confirmed_by_user_id,
            executed_run_id, created_at, updated_at",
    )
    .bind(&input.id)
    .bind(input.workbench_id)
    .bind(input.project_id)
    .bind(&input.artifact_type)
    .bind(&input.artifact_id)
    .bind(&input.action_type)
    .bind(&input.payload)
    .bind(&input.source_session_id)
    .bind(&input.source_run_id)
    .bind(input.created_by_user_id)
    .bind(input.assigned_user_id)
    .fetch_one(pool)
    .await
}

pub async fn list_pending_actions(
    pool: &DbPool,
    workbench_id: i64,
) -> Result<Vec<PendingActionRow>, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, workbench_id, project_id, artifact_type, artifact_id, action_type, status, payload,
            source_session_id, source_run_id, created_by_user_id, assigned_user_id, confirmed_by_user_id,
            executed_run_id, created_at, updated_at
         FROM pending_actions
         WHERE workbench_id = ?
         ORDER BY created_at DESC, id DESC",
    )
    .bind(workbench_id)
    .fetch_all(pool)
    .await
}

pub async fn get_pending_action(pool: &DbPool, id: &str) -> Result<PendingActionRow, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, workbench_id, project_id, artifact_type, artifact_id, action_type, status, payload,
            source_session_id, source_run_id, created_by_user_id, assigned_user_id, confirmed_by_user_id,
            executed_run_id, created_at, updated_at
         FROM pending_actions
         WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn find_latest_pending_action_for_issue(
    pool: &DbPool,
    workbench_id: i64,
    project_id: i64,
    artifact_id: &str,
    action_type: &str,
) -> Result<Option<PendingActionRow>, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, workbench_id, project_id, artifact_type, artifact_id, action_type, status, payload,
            source_session_id, source_run_id, created_by_user_id, assigned_user_id, confirmed_by_user_id,
            executed_run_id, created_at, updated_at
         FROM pending_actions
         WHERE workbench_id = ? AND project_id = ? AND artifact_type = 'issue' AND artifact_id = ? AND action_type = ?
         ORDER BY created_at DESC, id DESC
         LIMIT 1",
    )
    .bind(workbench_id)
    .bind(project_id)
    .bind(artifact_id)
    .bind(action_type)
    .fetch_optional(pool)
    .await
}

pub async fn confirm_pending_action(
    pool: &DbPool,
    id: &str,
    confirmed_by_user_id: i64,
) -> Result<PendingActionRow, sqlx::Error> {
    sqlx::query_as(
        "UPDATE pending_actions
         SET status = 'confirmed', confirmed_by_user_id = ?, updated_at = CURRENT_TIMESTAMP
         WHERE id = ? AND status = 'pending'
         RETURNING id, workbench_id, project_id, artifact_type, artifact_id, action_type, status, payload,
            source_session_id, source_run_id, created_by_user_id, assigned_user_id, confirmed_by_user_id,
            executed_run_id, created_at, updated_at",
    )
    .bind(confirmed_by_user_id)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn attach_executed_run(
    pool: &DbPool,
    id: &str,
    executed_run_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE pending_actions
         SET executed_run_id = ?, updated_at = CURRENT_TIMESTAMP
         WHERE id = ?",
    )
    .bind(executed_run_id)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn mark_running(pool: &DbPool, id: &str, run_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE pending_actions
         SET status = 'running', executed_run_id = ?, updated_at = CURRENT_TIMESTAMP
         WHERE id = ?",
    )
    .bind(run_id)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn mark_completed(pool: &DbPool, id: &str, run_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE pending_actions
         SET status = 'completed', executed_run_id = ?, updated_at = CURRENT_TIMESTAMP
         WHERE id = ?",
    )
    .bind(run_id)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn mark_failed(pool: &DbPool, id: &str, run_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE pending_actions
         SET status = 'failed', executed_run_id = ?, updated_at = CURRENT_TIMESTAMP
         WHERE id = ?",
    )
    .bind(run_id)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}
