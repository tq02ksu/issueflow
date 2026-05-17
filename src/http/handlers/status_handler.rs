use axum::{extract::Path, response::Html};

const STATUS_HTML: &str = include_str!("../../../internal/pages/templates/status.html");

pub async fn status_ping() -> &'static str {
    "ok"
}

pub async fn session_status(Path(session_id): Path<String>) -> Html<String> {
    let _ = session_id;
    Html(STATUS_HTML.to_owned())
}
