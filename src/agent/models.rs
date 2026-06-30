use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct AgentSessionRow {
    pub id: String,
    pub user_id: i64,
    pub workbench_id: i64,
    pub title: String,
    pub latest_state: Option<String>,
    pub last_message_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct AgentMessageRow {
    pub id: i64,
    pub session_id: String,
    pub run_id: Option<String>,
    pub role: String,
    pub message_kind: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RenameSessionRequest {
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct AgentSessionDetail {
    pub session: AgentSessionRow,
    pub messages: Vec<AgentMessageRow>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AgentRunRow {
    pub id: String,
    pub session_id: String,
    pub parent_run_id: Option<String>,
    pub status: String,
    pub worker_id: Option<String>,
    pub leased_until: Option<String>,
    pub attempt_count: i64,
    pub resume_cursor: Option<String>,
    pub input_payload: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub started_at: String,
    pub finished_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunAgentRequest {
    pub thread_id: String,
    pub workbench_id: i64,
    pub messages: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersistedRunInput {
    pub request: RunAgentRequest,
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExecutePendingActionInput {
    pub pending_action_id: String,
    pub access_token: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRunResponse {
    pub run_id: String,
    pub thread_id: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct RunEventsQuery {
    pub after_seq: Option<i64>,
}
