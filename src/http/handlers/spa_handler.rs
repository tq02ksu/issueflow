use std::{
    fs,
    path::{Path as FsPath, PathBuf},
};

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
};

const APP_HTML: &str = include_str!("../../../internal/pages/templates/app.html");
const ASSET_ROOT: &str = "web/dist/assets";

pub async fn app_shell() -> Html<&'static str> {
    Html(APP_HTML)
}

pub async fn app_asset(Path(path): Path<String>) -> Result<Response, StatusCode> {
    if path.contains("..") {
        return Err(StatusCode::BAD_REQUEST);
    }

    let asset_path = PathBuf::from(ASSET_ROOT).join(&path);
    let bytes = fs::read(&asset_path).map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(([(header::CONTENT_TYPE, content_type_for(&asset_path))], bytes).into_response())
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
