use uuid::Uuid;

use crate::{
    agent::models::{AgentMessageRow, AgentSessionRow},
    db::DbPool,
};

pub async fn create_session(
    pool: &DbPool,
    user_id: i64,
    workbench_id: i64,
) -> Result<AgentSessionRow, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    sqlx::query_as(
        "INSERT INTO agent_sessions (id, user_id, workbench_id, title, last_message_at, created_at, updated_at)
         VALUES (?, ?, ?, 'New Session', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
         RETURNING id, user_id, workbench_id, title, latest_state, last_message_at, created_at, updated_at",
    )
    .bind(&id)
    .bind(user_id)
    .bind(workbench_id)
    .fetch_one(pool)
    .await
}

pub async fn list_sessions(
    pool: &DbPool,
    user_id: i64,
    workbench_id: i64,
) -> Result<Vec<AgentSessionRow>, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, user_id, workbench_id, title, latest_state, last_message_at, created_at, updated_at
         FROM agent_sessions
         WHERE user_id = ? AND workbench_id = ?
         ORDER BY updated_at DESC",
    )
    .bind(user_id)
    .bind(workbench_id)
    .fetch_all(pool)
    .await
}

pub async fn get_session(
    pool: &DbPool,
    user_id: i64,
    id: &str,
) -> Result<AgentSessionRow, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, user_id, workbench_id, title, latest_state, last_message_at, created_at, updated_at
         FROM agent_sessions
         WHERE user_id = ? AND id = ?",
    )
    .bind(user_id)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn get_session_by_id(pool: &DbPool, id: &str) -> Result<AgentSessionRow, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, user_id, workbench_id, title, latest_state, last_message_at, created_at, updated_at
         FROM agent_sessions
         WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn rename_session(
    pool: &DbPool,
    user_id: i64,
    id: &str,
    title: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE agent_sessions SET title = ?, updated_at = CURRENT_TIMESTAMP WHERE user_id = ? AND id = ?",
    )
    .bind(title)
    .bind(user_id)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_session(pool: &DbPool, user_id: i64, id: &str) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "DELETE FROM agent_run_events
         WHERE run_id IN (SELECT id FROM agent_runs WHERE session_id = ?)",
    )
    .bind(id)
    .execute(&mut *tx)
    .await?;

    sqlx::query("DELETE FROM agent_messages WHERE session_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM agent_runs WHERE session_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM agent_sessions WHERE user_id = ? AND id = ?")
        .bind(user_id)
        .bind(id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn list_messages(
    pool: &DbPool,
    session_id: &str,
) -> Result<Vec<AgentMessageRow>, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, session_id, run_id, role, message_kind, content, created_at
         FROM agent_messages
         WHERE session_id = ?
         ORDER BY created_at ASC, id ASC",
    )
    .bind(session_id)
    .fetch_all(pool)
    .await
}
