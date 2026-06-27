use axum::{Json, extract::{Query, State}, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{http::routes::AppState, session::Session};

#[derive(Deserialize)]
pub struct ProjectSearchParams {
    pub search: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GitLabProject {
    pub id: i64,
    pub name: String,
    pub path_with_namespace: String,
    pub namespace: GitLabNamespace,
}

#[derive(Serialize, Deserialize)]
pub struct GitLabNamespace {
    pub id: i64,
    pub name: String,
    pub kind: String,
}

pub async fn list_projects(
    State(state): State<AppState>,
    session: Session,
    Query(params): Query<ProjectSearchParams>,
) -> Result<Json<Vec<GitLabProject>>, StatusCode> {
    let base_url = state
        .config
        .git
        .base_url
        .as_deref()
        .unwrap_or("https://gitlab.com");

    let mut url = format!("{base_url}/api/v4/projects?membership=true&order_by=updated_at&per_page=50");
    if let Some(ref search) = params.search {
        url.push_str(&format!("&search={}", urlencoding(search)));
    }

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", session.access_token))
        .send()
        .await
        .map_err(|e| {
            eprintln!("GitLab API request failed: {e}");
            StatusCode::BAD_GATEWAY
        })?;

    let status = resp.status();
    let body = resp.text().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    eprintln!("GitLab API {url} -> {status}: {body}");

    if !status.is_success() {
        return Err(StatusCode::BAD_GATEWAY);
    }

    let projects: Vec<GitLabProject> = serde_json::from_str(&body).map_err(|e| {
        eprintln!("GitLab API parse error: {e}");
        StatusCode::BAD_GATEWAY
    })?;
    Ok(Json(projects))
}

fn urlencoding(s: &str) -> String {
    let mut encoded = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            _ => encoded.push_str(&format!("%{:02X}", b)),
        }
    }
    encoded
}
