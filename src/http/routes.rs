use std::sync::Arc;

use axum::{
    Router,
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
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: DbPool,
    pub oidc_metadata: Arc<RwLock<Option<OidcMetadata>>>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(spa_handler::app_shell))
        .route("/workbench", get(spa_handler::app_shell))
        .route("/workbench/issues", get(spa_handler::app_shell))
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
        .route(
            "/api/workbenches/{id}/capabilities",
            get(workbench_handler::get_capabilities),
        )
        .route("/api/projects", get(projects::list_projects))
        .route(
            "/api/projects/{project_id}/issues",
            get(issues_handler::list_project_issues),
        )
        .route(
            "/api/projects/{project_id}/milestones",
            get(issues_handler::list_project_milestones),
        )
        .route(
            "/api/projects/{project_id}/issues/{issue_iid}/notes",
            get(issues_handler::list_issue_notes),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
