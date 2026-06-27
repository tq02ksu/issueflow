use axum::{extract::Path, response::Redirect};

pub async fn confirm_plan(Path(token): Path<String>) -> Redirect {
    Redirect::to(&format!("/workbench?confirm={token}"))
}
