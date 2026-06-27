use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};

use crate::{
    config::Config,
    db::DbPool,
    http::handlers::{confirm_handler, issues_handler, oidc_handler, spa_handler, status_handler, webhook_handler},
    session::SessionConfig,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: DbPool,
}

async fn inject_session_config(mut req: Request, next: Next) -> Response {
    let secret = req
        .extensions()
        .get::<AppState>()
        .map(|s: &AppState| s.config.session_signing_secret.clone())
        .unwrap_or_default();
    req.extensions_mut().insert(SessionConfig {
        signing_secret: secret,
    });
    next.run(req).await
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(spa_handler::app_shell))
        .route("/workbench", get(spa_handler::app_shell))
        .route("/auth/callback/oidc", get(spa_handler::app_shell))
        .route("/assets/{*path}", get(spa_handler::app_asset))
        .route("/api/auth/login", get(oidc_handler::oidc_login))
        .route("/api/auth/callback", get(oidc_handler::oidc_callback))
        .route("/api/status/ping", get(status_handler::status_ping))
        .route("/api/status/session/{session_id}", get(status_handler::session_status))
        .route("/api/confirm/plan/{token}", get(confirm_handler::confirm_plan))
        .route("/api/webhooks/gitlab", post(webhook_handler::handle_webhook))
        .route("/api/issues", post(issues_handler::create_issue))
        .layer(middleware::from_fn(inject_session_config))
        .with_state(state)
}
