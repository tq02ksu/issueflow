use axum::{
    Router,
    routing::{get, post},
};

use crate::{config::Config, http::handlers::webhook_handler};

pub fn router(config: Config) -> Router {
    Router::new()
        .route("/status/ping", get(|| async { "ok" }))
        .route("/webhooks/gitlab", post(webhook_handler::handle_webhook))
        .with_state(config)
}
