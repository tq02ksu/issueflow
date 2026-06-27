use std::sync::Arc;

use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::{get, post, put},
};
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;

use crate::{
    config::Config,
    db::DbPool,
    gitlab::projects,
    http::handlers::{
        auth_handler, confirm_handler, issues_handler, oidc_handler, spa_handler, status_handler,
        webhook_handler, workbench_handler,
    },
    oidc::OidcMetadata,
    session::SessionConfig,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: DbPool,
    pub oidc_metadata: Arc<RwLock<Option<OidcMetadata>>>,
}

async fn inject_session_config(mut req: Request, next: Next) -> Response {
    let secret = req
        .extensions()
        .get::<AppState>()
        .map(|s: &AppState| s.config.jwt_secret.clone())
        .unwrap_or_default();
    req.extensions_mut()
        .insert(SessionConfig { jwt_secret: secret });
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
        .route("/api/auth/me", get(auth_handler::me))
        .route("/api/status/ping", get(status_handler::status_ping))
        .route(
            "/api/status/session/{session_id}",
            get(status_handler::session_status),
        )
        .route(
            "/api/confirm/plan/{token}",
            get(confirm_handler::confirm_plan),
        )
        .route(
            "/api/webhooks/gitlab",
            post(webhook_handler::handle_webhook),
        )
        .route("/api/issues", post(issues_handler::create_issue))
        .route(
            "/api/workbenches",
            get(workbench_handler::list_workbenches).post(workbench_handler::create_workbench),
        )
        .route(
            "/api/workbenches/{id}",
            put(workbench_handler::update_workbench).delete(workbench_handler::delete_workbench),
        )
        .route("/api/projects", get(projects::list_projects))
        .layer(middleware::from_fn(inject_session_config))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
