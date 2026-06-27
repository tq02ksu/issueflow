use axum::{extract::Path, response::Redirect};

pub async fn status_ping() -> &'static str {
    "ok"
}

pub async fn session_status(Path(session_id): Path<String>) -> Redirect {
    Redirect::to(&format!("/workbench?session={session_id}"))
}
