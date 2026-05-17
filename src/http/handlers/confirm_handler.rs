use axum::{extract::Path, response::Html};

pub async fn confirm_plan(Path(token): Path<String>) -> Html<String> {
    Html(format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\" /><title>Issueflow Plan Confirmation</title></head><body><main><h1>Plan Confirmation</h1><p>Token: {token}</p></main></body></html>"
    ))
}
