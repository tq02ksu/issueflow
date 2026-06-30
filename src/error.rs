use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Unauthorized,
    Forbidden,
    NotFound,
    BadRequest(String),
    Conflict,
    ServiceUnavailable(String),
    Internal(OpaqueError),
}

#[derive(Debug)]
pub struct OpaqueError(pub(super) String);

impl fmt::Display for OpaqueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for OpaqueError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unauthorized => f.write_str("unauthorized"),
            Self::Forbidden => f.write_str("forbidden"),
            Self::NotFound => f.write_str("not found"),
            Self::BadRequest(message) => write!(f, "bad request: {message}"),
            Self::Conflict => f.write_str("conflict"),
            Self::ServiceUnavailable(message) => write!(f, "service unavailable: {message}"),
            Self::Internal(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Internal(error) => Some(error),
            _ => None,
        }
    }
}

impl From<OpaqueError> for AppError {
    fn from(value: OpaqueError) -> Self {
        Self::Internal(value)
    }
}

impl From<sqlx::Error> for OpaqueError {
    fn from(e: sqlx::Error) -> Self {
        OpaqueError(e.to_string())
    }
}

impl From<reqwest::Error> for OpaqueError {
    fn from(e: reqwest::Error) -> Self {
        OpaqueError(e.to_string())
    }
}

impl From<async_openai::error::OpenAIError> for OpaqueError {
    fn from(e: async_openai::error::OpenAIError) -> Self {
        OpaqueError(e.to_string())
    }
}

impl From<agui_runtime::openai::RuntimeError> for OpaqueError {
    fn from(e: agui_runtime::openai::RuntimeError) -> Self {
        OpaqueError(e.to_string())
    }
}

impl From<serde_json::Error> for OpaqueError {
    fn from(e: serde_json::Error) -> Self {
        OpaqueError(e.to_string())
    }
}

impl From<std::io::Error> for OpaqueError {
    fn from(e: std::io::Error) -> Self {
        OpaqueError(e.to_string())
    }
}

impl From<String> for OpaqueError {
    fn from(s: String) -> Self {
        OpaqueError(s)
    }
}

impl From<&str> for OpaqueError {
    fn from(s: &str) -> Self {
        OpaqueError(s.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Internal(OpaqueError::from(e))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Internal(OpaqueError::from(e))
    }
}

impl From<async_openai::error::OpenAIError> for AppError {
    fn from(e: async_openai::error::OpenAIError) -> Self {
        AppError::Internal(OpaqueError::from(e))
    }
}

impl From<agui_runtime::openai::RuntimeError> for AppError {
    fn from(e: agui_runtime::openai::RuntimeError) -> Self {
        AppError::Internal(OpaqueError::from(e))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Internal(OpaqueError::from(e))
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Internal(OpaqueError::from(e))
    }
}

impl AppError {
    const fn status(&self) -> StatusCode {
        match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Serialize)]
struct ProblemDetail {
    #[serde(rename = "type")]
    type_: String,
    title: String,
    status: u16,
    detail: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();

        match &self {
            AppError::Internal(e) => {
                tracing::error!(error = %e, "internal server error");
            }
            AppError::ServiceUnavailable(reason) => {
                tracing::warn!(%reason, "service unavailable");
            }
            _ => {
                tracing::warn!(error = %self, "client error");
            }
        }

        let (title, detail) = match &self {
            AppError::Unauthorized => ("Unauthorized", "missing or invalid credentials"),
            AppError::Forbidden => ("Forbidden", "insufficient permissions"),
            AppError::NotFound => ("Not Found", "the requested resource does not exist"),
            AppError::BadRequest(msg) => ("Bad Request", msg.as_str()),
            AppError::Conflict => ("Conflict", "resource already exists"),
            AppError::ServiceUnavailable(msg) => ("Service Unavailable", msg.as_str()),
            AppError::Internal(_) => ("Internal Server Error", "an unexpected error occurred"),
        };

        let type_name = title.to_lowercase().replace(' ', "-");
        let body = ProblemDetail {
            type_: format!("https://issueflow.dev/errors/{type_name}"),
            title: title.to_string(),
            status: status.as_u16(),
            detail: detail.to_string(),
        };

        (
            status,
            [(header::CONTENT_TYPE, "application/problem+json")],
            Json(body),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response_for(err: AppError) -> (StatusCode, String) {
        let resp = err.into_response();
        let status = resp.status();
        // We can't easily read body from Response in unit tests without setting up
        // a full HTTP context, so just check status codes for now.
        (status, String::new())
    }

    #[test]
    fn unauthorized_returns_401() {
        let (status, _) = response_for(AppError::Unauthorized);
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn not_found_returns_404() {
        let (status, _) = response_for(AppError::NotFound);
        assert_eq!(status, StatusCode::NOT_FOUND);
    }

    #[test]
    fn bad_request_returns_400() {
        let (status, _) = response_for(AppError::BadRequest("missing field".into()));
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn conflict_returns_409() {
        let (status, _) = response_for(AppError::Conflict);
        assert_eq!(status, StatusCode::CONFLICT);
    }

    #[test]
    fn internal_returns_500() {
        let (status, _) = response_for(AppError::Internal(OpaqueError("boom".into())));
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn opaque_from_sqlx_error() {
        let sqlx_err = sqlx::Error::Protocol("test error".into());
        let opaque = OpaqueError::from(sqlx_err);
        assert!(opaque.to_string().contains("test error"));
        let app_err = AppError::from(opaque);
        assert_eq!(app_err.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn from_io_error_to_internal() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let app_err: AppError = AppError::Internal(OpaqueError::from(io_err));
        assert_eq!(app_err.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn from_string_to_internal() {
        let app_err: AppError =
            AppError::Internal(OpaqueError::from("something went wrong".to_string()));
        assert_eq!(app_err.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn opaque_error_display() {
        let e = OpaqueError("test message".into());
        assert_eq!(e.to_string(), "test message");
    }
}
