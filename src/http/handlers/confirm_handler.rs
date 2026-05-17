use axum::{extract::Path, response::Html};

const CONFIRM_RESULT_HTML: &str = include_str!("../../../internal/pages/templates/confirm_result.html");

pub async fn confirm_plan(Path(token): Path<String>) -> Html<String> {
    let _ = token;
    Html(CONFIRM_RESULT_HTML.to_owned())
}
