use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    gitlab::issues::{self, CreateIssueInput, GitlabIssue, IssueNote, Milestone},
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

pub async fn create_issue(
    State(state): State<AppState>,
    session: Session,
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

    let base_url = state
        .config
        .git
        .base_url
        .as_deref()
        .ok_or_else(|| AppError::Internal("missing git.base_url configuration".into()))?;

    let created = issues::create_issue(base_url, &session.access_token, input)
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

    let issues = issues::list_issues(base_url, &session.access_token, project_id, "all")
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(Json(issues))
}

pub async fn list_project_milestones(
    State(state): State<AppState>,
    session: Session,
    Path(project_id): Path<u64>,
) -> Result<Json<Vec<Milestone>>, AppError> {
    let base_url = state
        .config
        .git
        .base_url
        .as_deref()
        .unwrap_or("https://gitlab.com");

    let milestones = issues::list_project_milestones(base_url, &session.access_token, project_id)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(Json(milestones))
}

pub async fn list_issue_notes(
    State(state): State<AppState>,
    session: Session,
    Path((project_id, issue_iid)): Path<(u64, u64)>,
) -> Result<Json<Vec<IssueNote>>, AppError> {
    let base_url = state
        .config
        .git
        .base_url
        .as_deref()
        .unwrap_or("https://gitlab.com");

    let notes = issues::list_issue_notes(base_url, &session.access_token, project_id, issue_iid)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(Json(notes))
}
