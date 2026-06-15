use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
};
use serde::Deserialize;

use crate::{
    config::Config,
    oidc::{issue_state, validate_state},
};

#[derive(Deserialize)]
pub struct OidcCallbackQuery {
    code: String,
    state: String,
}

pub async fn oidc_login(State(config): State<Config>) -> Result<Redirect, StatusCode> {
    let oidc = config.oidc.enabled().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;
    let state = issue_state(&oidc.issuer, &oidc.state_signing_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::temporary(&oidc.authorize_url(&state)))
}

pub async fn oidc_callback(
    State(config): State<Config>,
    Query(query): Query<OidcCallbackQuery>,
) -> Result<Redirect, StatusCode> {
    let oidc = config.oidc.enabled().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    if query.code.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    validate_state(&query.state, &oidc.issuer, &oidc.state_signing_secret)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Redirect::temporary("/auth/callback/oidc?result=success"))
}
