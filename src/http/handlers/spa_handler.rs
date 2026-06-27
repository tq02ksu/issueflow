use axum::{
    extract::Path,
    http::header,
    response::{Html, IntoResponse, Response},
};
use std::{
    fs,
    path::{Path as FsPath, PathBuf},
};

use crate::error::AppError;

const INDEX_HTML_PATH: &str = "web/dist/index.html";
const ASSET_ROOT: &str = "web/dist/assets";

const FALLBACK_INDEX: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>issueflow</title>
    <link rel="stylesheet" href="/assets/app.css" />
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/assets/app.js"></script>
  </body>
</html>
"#;

pub async fn app_shell() -> Html<&'static str> {
    match fs::read_to_string(INDEX_HTML_PATH) {
        Ok(html) => Html(html.leak()),
        Err(_) => Html(FALLBACK_INDEX),
    }
}

pub async fn app_asset(Path(path): Path<String>) -> Result<Response, AppError> {
    if path.contains("..") {
        return Err(AppError::BadRequest("invalid asset path".into()));
    }

    let asset_path = PathBuf::from(ASSET_ROOT).join(&path);
    let bytes = fs::read(&asset_path).map_err(|_| AppError::NotFound)?;

    Ok((
        [(header::CONTENT_TYPE, content_type_for(&asset_path))],
        bytes,
    )
        .into_response())
}

fn content_type_for(path: &FsPath) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("js") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}
