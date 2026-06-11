use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    config::Config,
    http::handlers::{confirm_handler, oauth_handler, status_handler, webhook_handler},
};

pub fn router(config: Config) -> Router {
    Router::new()
        .route("/auth/{provider}/login", get(oauth_handler::oauth_login))
        .route("/auth/{provider}/callback", get(oauth_handler::oauth_callback))
        .route("/status/ping", get(status_handler::status_ping))
        .route("/status/session/{session_id}", get(status_handler::session_status))
        .route("/confirm/plan/{token}", get(confirm_handler::confirm_plan))
        .route("/webhooks/gitlab", post(webhook_handler::handle_webhook))
        .with_state(config)
}
