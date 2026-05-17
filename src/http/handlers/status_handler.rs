use axum::{extract::Path, response::Html};

pub async fn status_ping() -> &'static str {
    "ok"
}

pub async fn session_status(Path(session_id): Path<String>) -> Html<String> {
    Html(format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\" /><title>Issueflow Session Status</title></head><body><main><h1>Session Status</h1><p>Session: {session_id}</p></main></body></html>"
    ))
}
