use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    gitlab::issues::{self, CreateIssueInput},
    http::routes::AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateIssueRequest {
    pub project_id: u64,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct CreateIssueResponse {
    pub id: u64,
    pub iid: u64,
    pub project_id: u64,
    pub title: String,
    pub web_url: String,
}

pub async fn create_issue(
    State(state): State<AppState>,
    Json(payload): Json<CreateIssueRequest>,
) -> Result<(StatusCode, Json<CreateIssueResponse>), AppError> {
    let title = payload.title.trim().to_string();
    if title.is_empty() {
        return Err(AppError::BadRequest("title is required".into()));
    }

    let input = CreateIssueInput {
        project_id: payload.project_id,
        title,
        description: payload.description,
    };

    let created = issues::create_issue(&state.config.git, input)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok((
        StatusCode::CREATED,
        Json(CreateIssueResponse {
            id: created.id,
            iid: created.iid,
            project_id: created.project_id,
            title: created.title,
            web_url: created.web_url,
        }),
    ))
}
