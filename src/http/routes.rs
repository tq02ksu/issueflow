use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    config::Config,
    http::handlers::{confirm_handler, oidc_handler, spa_handler, status_handler, webhook_handler},
};

pub fn router(config: Config) -> Router {
    Router::new()
        .route("/", get(spa_handler::app_shell))
        .route("/index.html", get(spa_handler::app_shell))
        .route("/workbench", get(spa_handler::app_shell))
        .route("/auth/callback/oidc", get(spa_handler::app_shell))
        .route("/assets/{*path}", get(spa_handler::app_asset))
        .route("/auth/login", get(oidc_handler::oidc_login))
        .route("/auth/callback", get(oidc_handler::oidc_callback))
        .route("/status/ping", get(status_handler::status_ping))
        .route("/status/session/{session_id}", get(status_handler::session_status))
        .route("/confirm/plan/{token}", get(confirm_handler::confirm_plan))
        .route("/webhooks/gitlab", post(webhook_handler::handle_webhook))
        .with_state(config)
}
