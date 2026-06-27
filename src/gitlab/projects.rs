use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ProjectSearchParams {
    pub search: Option<String>,
}

#[derive(Serialize)]
pub struct GitLabProject {
    pub id: i64,
    pub name: String,
    pub path_with_namespace: String,
    pub namespace: GitLabNamespace,
}

#[derive(Serialize)]
pub struct GitLabNamespace {
    pub id: i64,
    pub name: String,
    pub kind: String,
}

pub async fn list_projects(
    Query(params): Query<ProjectSearchParams>,
) -> Json<Vec<GitLabProject>> {
    let _search = params.search.unwrap_or_default();
    Json(vec![])
}
