use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PendingActionRow {
    pub id: String,
    pub workbench_id: i64,
    pub project_id: i64,
    pub artifact_type: String,
    pub artifact_id: String,
    pub action_type: String,
    pub status: String,
    pub payload: String,
    pub source_session_id: Option<String>,
    pub source_run_id: Option<String>,
    pub created_by_user_id: Option<i64>,
    pub assigned_user_id: Option<i64>,
    pub confirmed_by_user_id: Option<i64>,
    pub executed_run_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct CreatePendingActionInput {
    pub id: String,
    pub workbench_id: i64,
    pub project_id: i64,
    pub artifact_type: String,
    pub artifact_id: String,
    pub action_type: String,
    pub payload: String,
    pub source_session_id: Option<String>,
    pub source_run_id: Option<String>,
    pub created_by_user_id: Option<i64>,
    pub assigned_user_id: Option<i64>,
}
