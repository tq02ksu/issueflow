use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    gitlab::issues::{self, CreateIssueInput},
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
    State(config): State<Config>,
    Json(payload): Json<CreateIssueRequest>,
) -> Result<(StatusCode, Json<CreateIssueResponse>), StatusCode> {
    let title = payload.title.trim().to_string();
    if title.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let input = CreateIssueInput {
        project_id: payload.project_id,
        title,
        description: payload.description,
    };

    let created = issues::create_issue(&config.git, input)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
