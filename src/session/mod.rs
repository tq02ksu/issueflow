use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Clone, Debug)]
pub struct SessionConfig {
    pub jwt_secret: String,
}

#[derive(Clone, Debug)]
pub struct Session {
    pub user_id: i64,
    pub sub: String,
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct SessionClaims {
    pub user_id: i64,
    pub sub: String,
    pub access_token: String,
    pub exp: usize,
    pub iat: usize,
}

impl Session {
    pub fn from_bearer(header: &str, secret: &str) -> Result<Self, AppError> {
        let token = header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        let claims = verify_token(token, secret).map_err(|_| AppError::Unauthorized)?;

        Ok(Self {
            user_id: claims.user_id,
            sub: claims.sub,
            access_token: claims.access_token,
        })
    }
}

impl<S> FromRequestParts<S> for Session
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header_value = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let config = parts
            .extensions
            .get::<SessionConfig>()
            .ok_or(AppError::Internal("missing session config".into()))?;

        Self::from_bearer(header_value, &config.jwt_secret)
    }
}

use crate::http::routes::AppState;

pub fn verify_token(token: &str, secret: &str) -> Result<SessionClaims, AppError> {
    let data = decode::<SessionClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized)?;
    Ok(data.claims)
}

pub fn sign_token(claims: &SessionClaims, secret: &str) -> Result<String, AppError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("jwt encoding error: {e}").into()))
}

use std::time::{SystemTime, UNIX_EPOCH};

fn now_epoch() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

pub fn build_claims(user_id: i64, sub: &str, access_token: &str) -> SessionClaims {
    let now = now_epoch();
    SessionClaims {
        user_id,
        sub: sub.to_string(),
        access_token: access_token.to_string(),
        iat: now,
        exp: now + 86400, // 24h
    }
}
