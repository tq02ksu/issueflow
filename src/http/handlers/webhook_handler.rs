use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
};

use crate::{
    error::AppError,
    gitlab::{commands::parse_note_command, webhook::GitlabWebhook},
    http::routes::AppState,
};

pub async fn handle_webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, AppError> {
    let token = headers
        .get("x-gitlab-token")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if token != state.config.git.webhook_secret {
        return Err(AppError::Unauthorized);
    }

    let payload: GitlabWebhook = serde_json::from_slice(&body)
        .map_err(|_| AppError::BadRequest("invalid webhook payload".into()))?;

    if payload.object_kind == "note" && payload.object_attributes.noteable_type == "Issue" {
        let _ = parse_note_command(&payload.object_attributes.note);
    }

    Ok(StatusCode::ACCEPTED)
}
