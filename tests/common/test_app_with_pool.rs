use std::sync::Arc;

use issueflow::{config::Config, db::DbPool, http::routes::AppState};
use tokio::sync::RwLock;

pub async fn test_app_with_pool(config: Config, pool: DbPool) -> axum::Router {
    issueflow::http::routes::router(AppState {
        config,
        pool,
        oidc_metadata: Arc::new(RwLock::new(None)),
    })
}
