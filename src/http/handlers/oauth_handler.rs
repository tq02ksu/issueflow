use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, Redirect},
};
use serde::Deserialize;

use crate::{
    config::Config,
    oauth::{OAuthProviderKind, issue_state, validate_state},
};

const OAUTH_CALLBACK_HTML: &str = include_str!("../../../internal/pages/templates/oauth_callback.html");

#[derive(Deserialize)]
pub struct OAuthCallbackQuery {
    code: String,
    state: String,
}

pub async fn oauth_login(
    Path(provider): Path<String>,
    State(config): State<Config>,
) -> Result<Redirect, StatusCode> {
    let provider_kind = OAuthProviderKind::from_slug(&provider).ok_or(StatusCode::NOT_FOUND)?;
    let provider_config = config
        .oauth
        .provider(provider_kind)
        .ok_or(StatusCode::NOT_FOUND)?;
    let state = issue_state(provider_kind, &config.oauth.state_signing_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::temporary(&provider_config.authorize_url(&state)))
}

pub async fn oauth_callback(
    Path(provider): Path<String>,
    State(config): State<Config>,
    Query(query): Query<OAuthCallbackQuery>,
) -> Result<Html<String>, StatusCode> {
    let provider_kind = OAuthProviderKind::from_slug(&provider).ok_or(StatusCode::NOT_FOUND)?;

    if query.code.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    config
        .oauth
        .provider(provider_kind)
        .ok_or(StatusCode::NOT_FOUND)?;
    validate_state(&query.state, provider_kind, &config.oauth.state_signing_secret)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Html(OAUTH_CALLBACK_HTML.to_owned()))
}
