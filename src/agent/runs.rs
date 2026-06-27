use uuid::Uuid;

use crate::{agent::models::AgentRunRow, db::DbPool};

pub async fn create_run(
    pool: &DbPool,
    session_id: &str,
    parent_run_id: Option<&str>,
    input_payload: &str,
) -> Result<AgentRunRow, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    sqlx::query_as(
        "INSERT INTO agent_runs (id, session_id, parent_run_id, status, attempt_count, input_payload, started_at)
         VALUES (?, ?, ?, 'queued', 0, ?, CURRENT_TIMESTAMP)
         RETURNING id, session_id, parent_run_id, status, worker_id, leased_until, attempt_count, resume_cursor, input_payload, error_code, error_message, started_at, finished_at",
    )
    .bind(&id)
    .bind(session_id)
    .bind(parent_run_id)
    .bind(input_payload)
    .fetch_one(pool)
    .await
}

pub async fn append_event(
    pool: &DbPool,
    run_id: &str,
    seq: i64,
    event_type: &str,
    payload: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO agent_run_events (run_id, seq, event_type, payload, created_at)
         VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)",
    )
    .bind(run_id)
    .bind(seq)
    .bind(event_type)
    .bind(payload)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_events_after(
    pool: &DbPool,
    run_id: &str,
    after_seq: i64,
) -> Result<Vec<(i64, String, String)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT seq, event_type, payload FROM agent_run_events WHERE run_id = ? AND seq > ? ORDER BY seq ASC",
    )
    .bind(run_id)
    .bind(after_seq)
    .fetch_all(pool)
    .await
}

pub async fn get_run(pool: &DbPool, run_id: &str) -> Result<AgentRunRow, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, session_id, parent_run_id, status, worker_id, leased_until, attempt_count, resume_cursor, input_payload, error_code, error_message, started_at, finished_at FROM agent_runs WHERE id = ?",
    )
    .bind(run_id)
    .fetch_one(pool)
    .await
}

pub async fn reclaim_stale(pool: &DbPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE agent_runs SET status = 'queued', worker_id = NULL, leased_until = NULL WHERE status = 'running' AND leased_until < CURRENT_TIMESTAMP",
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

pub async fn claim_run(
    pool: &DbPool,
    worker_id: &str,
    leased_until: &str,
) -> Result<Option<AgentRunRow>, sqlx::Error> {
    let row: Option<AgentRunRow> = sqlx::query_as(
        "SELECT id, session_id, parent_run_id, status, worker_id, leased_until, attempt_count, resume_cursor, input_payload, error_code, error_message, started_at, finished_at FROM agent_runs WHERE status = 'queued' LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some(ref run) = row {
        sqlx::query(
            "UPDATE agent_runs SET status = 'running', worker_id = ?, leased_until = ?, attempt_count = attempt_count + 1 WHERE id = ? AND status = 'queued'",
        )
        .bind(worker_id)
        .bind(leased_until)
        .bind(&run.id)
        .execute(pool)
        .await?;
    }

    Ok(row)
}

pub async fn mark_completed(pool: &DbPool, run_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE agent_runs SET status = 'completed', finished_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(run_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn mark_failed(pool: &DbPool, run_id: &str, error: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE agent_runs SET status = 'failed', error_message = ?, finished_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(error).bind(run_id).execute(pool).await?;
    Ok(())
}
