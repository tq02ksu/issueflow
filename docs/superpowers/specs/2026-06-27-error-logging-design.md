# Error & Logging Standardization Design

## Problem

The project has no unified error handling or logging. Errors are swallowed by
`map_err(|_| StatusCode::...)` across handlers. A few `eprintln!` calls are
scattered in `oidc_handler` and `gitlab/projects`, but most failure paths
produce no output at all — you see a status code but no explanation.

## Solution

Standard Rust web app pattern: `tracing` + `AppError` enum + HTTP middleware.

### Dependencies

```toml
thiserror = "2"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tower-http = { version = "0.6", features = ["trace"] }
```

### `src/error.rs` — Unified Error Type

An `AppError` enum using `thiserror`:

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("conflict")]
    Conflict,
    #[error("{0}")]
    Internal(#[from] OpaqueError),
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct OpaqueError(#[from] Box<dyn std::error::Error + Send + Sync>);
```

`IntoResponse` implementation decides:
- **Log level**: `5xx` → `tracing::error!`, `4xx` → `tracing::warn!`
- **Response body**: plain `{ "error": "..." }` JSON (no internal details exposed)

`From` conversions provided for: `sqlx::Error`, `reqwest::Error`, `serde_json::Error`,
`std::io::Error`, `String` — all mapped through `OpaqueError` into
`AppError::Internal`.

### Handler Signature Changes

All handlers that return `Result<_, StatusCode>` switch to `Result<_, AppError>`.
Handlers that return bare `StatusCode` (webhook, delete) switch to
`Result<StatusCode, AppError>`.

`map_err(|_| StatusCode::...)` is removed everywhere. Errors propagate via `?`
and are logged/responded by `IntoResponse for AppError`.

`eprintln!` calls removed entirely.

Handlers with no failure paths (`status_ping`, `session_status`, `confirm_plan`,
`app_shell`) are unchanged.

### `main.rs` — Logging Initialization

```rust
tracing_subscriber::fmt()
    .with_env_filter(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into())
    )
    .init();
```

Startup failures use `tracing::error!` instead of bare `expect`.

### Request Logging Middleware

```rust
use tower_http::trace::TraceLayer;

Router::new()
    // ... routes ...
    .layer(TraceLayer::new_for_http())
```

Records method, URI, status, latency for every request automatically.

### Workers Removed

- `eprintln!` in `oidc_handler.rs` (4 calls)
- `eprintln!` in `gitlab/projects.rs` (3 calls)

### Files Created / Modified

| File | Change |
|---|---|
| `Cargo.toml` | Add 4 dependencies |
| `src/error.rs` | New — `AppError`, `OpaqueError`, `IntoResponse`, `From` impls |
| `src/lib.rs` | Add `pub mod error` |
| `src/main.rs` | Init tracing subscriber, replace `expect` with `match` + `tracing::error!` |
| `src/http/routes.rs` | Add `TraceLayer` |
| `src/http/handlers/*.rs` | Switch `StatusCode` error to `AppError`, remove `eprintln!` |
| `src/gitlab/projects.rs` | Switch `StatusCode` to `AppError`, remove `eprintln!` |
| `src/session/mod.rs` | Switch `StatusCode` to `AppError` (session extraction) |
