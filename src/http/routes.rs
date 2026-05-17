use axum::{routing::get, Router};

async fn status_ping() -> &'static str {
    "ok"
}

pub fn router() -> Router {
    Router::new().route("/status/ping", get(status_ping))
}
