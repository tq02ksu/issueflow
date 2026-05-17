use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
};

use crate::{
    config::Config,
    gitlab::{commands::parse_note_command, webhook::GitlabWebhook},
};

pub async fn handle_webhook(
    State(config): State<Config>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    let token = headers
        .get("x-gitlab-token")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if token != config.gitlab_webhook_secret {
        return StatusCode::UNAUTHORIZED;
    }

    let payload: GitlabWebhook = match serde_json::from_slice(&body) {
        Ok(payload) => payload,
        Err(_) => return StatusCode::BAD_REQUEST,
    };

    if payload.object_kind == "note" && payload.object_attributes.noteable_type == "Issue" {
        let _ = parse_note_command(&payload.object_attributes.note);
    }

    StatusCode::ACCEPTED
}
