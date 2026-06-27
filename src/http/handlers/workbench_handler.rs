use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{error::AppError, http::routes::AppState, session::Session};

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct Workbench {
    pub id: i64,
    pub user_id: i64,
    pub project_id: i64,
    pub project_name: String,
    pub project_path: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(serde::Deserialize)]
pub struct CreateWorkbenchInput {
    pub project_id: i64,
    pub project_path: String,
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateWorkbenchInput {
    pub project_id: i64,
    pub project_path: String,
    pub name: String,
}

fn default_name(path: &str) -> String {
    path.rsplit('/').next().unwrap_or(path).to_string()
}

pub async fn list_workbenches(
    State(state): State<AppState>,
    session: Session,
) -> Result<Json<Vec<Workbench>>, AppError> {
    let rows: Vec<Workbench> = sqlx::query_as(
        "SELECT id, user_id, project_id, project_name, project_path, name, created_at, updated_at
         FROM workbenches WHERE user_id = ? ORDER BY created_at",
    )
    .bind(session.user_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(rows))
}

pub async fn create_workbench(
    State(state): State<AppState>,
    session: Session,
    Json(input): Json<CreateWorkbenchInput>,
) -> Result<(StatusCode, Json<Workbench>), AppError> {
    let name = if input.name.trim().is_empty() {
        default_name(&input.project_path)
    } else {
        input.name.trim().to_string()
    };

    let result = sqlx::query_as(
        "INSERT INTO workbenches (user_id, project_id, project_name, project_path, name)
         VALUES (?, ?, ?, ?, ?)
         RETURNING id, user_id, project_id, project_name, project_path, name, created_at, updated_at",
    )
    .bind(session.user_id)
    .bind(input.project_id)
    .bind(&input.project_path)
    .bind(&input.project_path)
    .bind(&name)
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok(wb) => Ok((StatusCode::CREATED, Json(wb))),
        Err(e) if e.to_string().contains("UNIQUE") => Err(AppError::Conflict),
        Err(e) => Err(e.into()),
    }
}

pub async fn update_workbench(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i64>,
    Json(input): Json<UpdateWorkbenchInput>,
) -> Result<Json<Workbench>, AppError> {
    let name = if input.name.trim().is_empty() {
        default_name(&input.project_path)
    } else {
        input.name.trim().to_string()
    };

    let result = sqlx::query_as(
        "UPDATE workbenches
         SET project_id = ?, project_name = ?, project_path = ?, name = ?, updated_at = CURRENT_TIMESTAMP
         WHERE id = ? AND user_id = ?
         RETURNING id, user_id, project_id, project_name, project_path, name, created_at, updated_at",
    )
    .bind(input.project_id)
    .bind(&input.project_path)
    .bind(&input.project_path)
    .bind(&name)
    .bind(id)
    .bind(session.user_id)
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(wb)) => Ok(Json(wb)),
        Ok(None) => Err(AppError::NotFound),
        Err(e) if e.to_string().contains("UNIQUE") => Err(AppError::Conflict),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete_workbench(
    State(state): State<AppState>,
    session: Session,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM workbenches WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(session.user_id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}

#[derive(serde::Serialize)]
pub struct Capabilities {
    pub features: Vec<&'static str>,
}

pub async fn get_capabilities(
    Path(_id): Path<i64>,
) -> Result<Json<Capabilities>, AppError> {
    Ok(Json(Capabilities {
        features: vec!["overview", "issues"],
    }))
}
