use axum::{Json, extract::{Path, State}, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    gitlab::issues::{self, CreateIssueInput},
    http::routes::AppState,
    session::Session,
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

#[derive(Debug, Serialize)]
pub struct GitlabIssue {
    pub id: u64,
    pub iid: u64,
    pub project_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub web_url: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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

pub async fn list_project_issues(
    State(state): State<AppState>,
    session: Session,
    Path(project_id): Path<u64>,
) -> Result<Json<Vec<GitlabIssue>>, AppError> {
    let base_url = state
        .config
        .git
        .base_url
        .as_deref()
        .unwrap_or("https://gitlab.com");

    let url = format!("{base_url}/api/v4/projects/{project_id}/issues?per_page=50&order_by=updated_at");

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", session.access_token))
        .send()
        .await?;

    let status = resp.status();
    let body = resp.text().await?;

    if !status.is_success() {
        tracing::warn!(%url, %status, %body, "gitlab issues api returned error status");
        return Err(AppError::Internal(
            format!("gitlab api returned {status}").into(),
        ));
    }

    let issues: Vec<serde_json::Value> = serde_json::from_str(&body)?;

    let mapped: Vec<GitlabIssue> = issues
        .into_iter()
        .map(|i| GitlabIssue {
            id: i["id"].as_u64().unwrap_or(0),
            iid: i["iid"].as_u64().unwrap_or(0),
            project_id: i["project_id"].as_u64().unwrap_or(0),
            title: i["title"].as_str().unwrap_or("").to_string(),
            description: i["description"].as_str().map(|s| s.to_string()),
            state: i["state"].as_str().unwrap_or("opened").to_string(),
            web_url: i["web_url"].as_str().unwrap_or("").to_string(),
            created_at: i["created_at"].as_str().map(|s| s.to_string()),
            updated_at: i["updated_at"].as_str().map(|s| s.to_string()),
        })
        .collect();

    Ok(Json(mapped))
}
