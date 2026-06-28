use axum::{
    Json,
    extract::{Query, State},
};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, http::routes::AppState, session::Session};

use super::client;
#[cfg(test)]
use super::codec::decode_json;

#[derive(Deserialize)]
pub struct ProjectSearchParams {
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabProject {
    pub id: i64,
    pub name: String,
    pub path_with_namespace: String,
    pub namespace: GitLabNamespace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabNamespace {
    pub id: i64,
    pub name: String,
    pub kind: String,
}

pub async fn list_projects(
    State(state): State<AppState>,
    session: Session,
    Query(params): Query<ProjectSearchParams>,
) -> Result<Json<Vec<GitLabProject>>, AppError> {
    let base_url = state
        .config
        .git
        .base_url
        .as_deref()
        .unwrap_or("https://gitlab.com");

    let projects = fetch_projects(base_url, &session.access_token, params.search.as_deref())
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    Ok(Json(projects))
}

pub async fn fetch_projects(
    base_url: &str,
    access_token: &str,
    search: Option<&str>,
) -> Result<Vec<GitLabProject>, String> {
    let client = client::build_client(base_url, access_token)?;
    let mut query = vec![
        ("membership", "true".to_string()),
        ("order_by", "updated_at".to_string()),
    ];

    if let Some(search) = search.filter(|value| !value.trim().is_empty()) {
        query.push(("search", search.to_string()));
    }

    client.get_paginated("projects", &query).await
}

#[cfg(test)]
fn parse_projects_response(body: &[u8]) -> Result<Vec<GitLabProject>, String> {
    decode_json(body, "projects")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_projects_response_reads_namespace() {
        let body = br#"[
            {
                "id": 42,
                "name": "issueflow",
                "path_with_namespace": "team/issueflow",
                "namespace": {
                    "id": 9,
                    "name": "team",
                    "kind": "group"
                }
            }
        ]"#;

        let projects = parse_projects_response(body).expect("projects should parse");

        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].id, 42);
        assert_eq!(projects[0].namespace.name, "team");
        assert_eq!(projects[0].path_with_namespace, "team/issueflow");
    }
}
