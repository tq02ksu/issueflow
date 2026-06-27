# Workbench Management — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add workbench management with OIDC token exchange, session cookies, SQLite persistence, and frontend UI for binding GitLab projects to workbenches.

**Architecture:** The gateway gains a SQLite database (sqlx), OIDC token exchange (auth code → access token), HMAC-signed session cookies, and workbench CRUD endpoints. The frontend adds a dropdown switcher and search dialog for selecting/creating workbenches.

**Tech Stack:** Rust (sqlx/sqlite), Vue 3 + Naive UI, HMAC-SHA256 session cookies

## Global Constraints

- Database: `sqlx` with `sqlite` feature, `CREATE TABLE IF NOT EXISTS` at startup
- Session: HMAC-SHA256 signed cookie, HttpOnly, SameSite=Lax
- OIDC discovery: already lazy-loaded, reuse `discover_metadata` for token endpoint
- Frontend proxy: only `/api/*` routes proxy to gateway

---

### Task 1: Dependencies and Database

**Files:**
- Modify: `Cargo.toml`
- Create: `src/db/mod.rs`
- Create: `tests/db_setup.rs`

**Interfaces:**
- Produces: `DbPool` type alias, `run_migrations(pool: &DbPool) -> Result<()>`, `User` struct

- [ ] **Step 1: Add dependencies to Cargo.toml**

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "postgres", "any"] }
serde_json = "1"
```

`serde_json` is already a dependency. `any` driver enables runtime backend switching via `sqlx::AnyPool`.

- [ ] **Step 2: Write failing test for database migration**

Create `tests/db_setup.rs`:

```rust
use sqlx::any::AnyPool;

#[tokio::test]
async fn migration_creates_users_and_workbenches_tables() {
    let pool = AnyPool::connect("sqlite::memory:").await.unwrap();
    issueflow::db::run_migrations(&pool).await.unwrap();

    let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(user_count.0, 0);

    let wb_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM workbenches")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(wb_count.0, 0);
}
```

Run: `cargo test db_setup -- --exact`
Expected: FAIL

- [ ] **Step 3: Create migration files**

Create `migrations/20260627000001_create_users.sql`:

```sql
CREATE TABLE IF NOT EXISTS users (
    id         BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    sub        TEXT NOT NULL UNIQUE,
    name       TEXT NOT NULL DEFAULT '',
    email      TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

Create `migrations/20260627000002_create_workbenches.sql`:

```sql
CREATE TABLE IF NOT EXISTS workbenches (
    id           BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id      BIGINT NOT NULL REFERENCES users(id),
    project_id   BIGINT NOT NULL,
    project_name TEXT NOT NULL,
    project_path TEXT NOT NULL,
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, project_id)
);
```

- [ ] **Step 4: Create src/db/mod.rs**

```rust
use sqlx::any::AnyPool;
use sqlx::migrate::Migrator;

pub type DbPool = AnyPool;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub sub: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

pub async fn open(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let pool = AnyPool::connect(database_url).await?;
    run_migrations(&pool).await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::Error> {
    MIGRATOR.run(pool).await
}

pub async fn upsert_user(pool: &DbPool, sub: &str, name: &str, email: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO users (sub, name, email) VALUES (?, ?, ?)
         ON CONFLICT(sub) DO UPDATE SET name = excluded.name, email = excluded.email
         RETURNING id, sub, name, email, created_at"
    )
    .bind(sub)
    .bind(name)
    .bind(email)
    .fetch_one(pool)
    .await
}
```

Schema uses PostgreSQL-compatible SQL. `CURRENT_TIMESTAMP` and `GENERATED ALWAYS AS IDENTITY` work in both SQLite 3.35+ and PostgreSQL.

- [ ] **Step 4: Register module and verify**

In `src/lib.rs`, after `pub mod config;`:

```rust
pub mod db;
```

Run: `cargo test db_setup -- --exact`

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock src/db/mod.rs src/lib.rs tests/db_setup.rs
git commit -m "feat: add sqlx, users and workbenches tables"
```

---

### Task 2: OIDC Token Exchange + User Creation

**Files:**
- Modify: `src/oidc/mod.rs`
- Modify: `src/http/handlers/oidc_handler.rs`
- Create: `tests/oidc_token_exchange.rs`

**Interfaces:**
- Consumes: `OidcEnabledConfig`, `DbPool`, `upsert_user`
- Modifies: `oidc_callback` exchanges code for tokens, parses id_token JWT, upserts user
- Produces: `IdTokenClaims { sub: String, name: Option<String>, email: Option<String> }`

- [ ] **Step 1: Extend OidcEnabledConfig with token exchange support**

Add to `impl OidcEnabledConfig` in `src/oidc/mod.rs`:

```rust
pub fn token_url(&self) -> Option<&str> {
    self.metadata.as_ref().map(|m| m.token_endpoint.as_str())
}
```

- [ ] **Step 2: Write test for token exchange**

Create `tests/oidc_token_exchange.rs`:

```rust
use axum::{body::Body, http::{header, Request, StatusCode}};
use issueflow::config::Config;
use issueflow::oidc::{OidcConfig, OidcEnabledConfig, OidcMetadata};
use tower::ServiceExt;

fn test_config() -> Config {
    Config::for_tests("expected-token").with_oidc(OidcConfig::Enabled(OidcEnabledConfig {
        issuer: "https://gitlab.example.com".to_string(),
        client_id: "gitlab-test-client".to_string(),
        client_secret: "gitlab-test-secret".to_string(),
        redirect_uri: "http://127.0.0.1:8080/api/auth/callback".to_string(),
        scopes: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
        state_signing_secret: "test-oidc-state-secret".to_string(),
        metadata: Some(OidcMetadata {
            issuer: "https://gitlab.example.com".to_string(),
            authorization_endpoint: "https://gitlab.example.com/oauth/authorize".to_string(),
            token_endpoint: "https://gitlab.example.com/oauth/token".to_string(),
        }),
    }))
}

#[tokio::test]
async fn callback_redirects_to_success_after_valid_state() {
    let app = issueflow::http::routes::router(test_config());

    let login_response = app
        .clone()
        .oneshot(Request::builder().uri("/api/auth/login").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let location = login_response.headers().get(header::LOCATION).unwrap().to_str().unwrap();
    let state = location.split("state=").nth(1).unwrap().split('&').next().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/auth/callback?code=test-code&state={state}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response.headers().get(header::LOCATION).and_then(|v| v.to_str().ok()),
        Some("/auth/callback/oidc?result=token_exchange_failed&reason=token+endpoint+unreachable")
    );
}
```

Run: `cargo test oidc_token_exchange -- --exact`
Expected: FAIL

- [ ] **Step 3: Implement token exchange and user upsert in oidc_callback**

Modify `src/http/handlers/oidc_handler.rs` — replace the `oidc_callback` function:

```rust
use axum::{extract::State, response::Redirect};
use serde::Deserialize;
use sqlx::AnyPool;

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    id_token: Option<String>,
}

#[derive(Deserialize)]
struct IdTokenClaims {
    sub: String,
    name: Option<String>,
    email: Option<String>,
}

fn parse_id_token_sub(id_token: &str) -> Result<IdTokenClaims, String> {
    let payload = id_token
        .split('.')
        .nth(1)
        .ok_or("invalid id_token format")?;
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload)
        .map_err(|_| "invalid id_token base64")?;
    serde_json::from_slice(&decoded).map_err(|_| "invalid id_token json".to_string())
}

pub async fn oidc_callback(
    State(mut config): State<Config>,
    State(pool): State<AnyPool>,
    Query(query): Query<OidcCallbackQuery>,
) -> Result<Redirect, StatusCode> {
    let oidc = config.oidc.enabled().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    if query.code.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    validate_state(&query.state, &oidc.issuer, &oidc.state_signing_secret)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let token_url = oidc.token_url().ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let client = reqwest::Client::new();
    let token_result = client
        .post(token_url)
        .form(&[
            ("client_id", oidc.client_id.as_str()),
            ("client_secret", oidc.client_secret.as_str()),
            ("code", query.code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", oidc.redirect_uri.as_str()),
        ])
        .send()
        .await;

    match token_result {
        Ok(resp) if resp.status().is_success() => {
            let tokens: TokenResponse = resp.json().await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let id_token = tokens.id_token.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
            let claims = parse_id_token_sub(&id_token)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let user = crate::db::upsert_user(
                &pool,
                &claims.sub,
                claims.name.as_deref().unwrap_or(""),
                claims.email.as_deref().unwrap_or(""),
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Sign session cookie with user_id + access_token (Task 3 will wire this)
            let _user_id = user.id;
            let _access_token = tokens.access_token;

            Ok(Redirect::temporary("/auth/callback/oidc?result=success"))
        }
        Ok(resp) => {
            let status = resp.status().as_u16();
            Ok(Redirect::temporary(&format!(
                "/auth/callback/oidc?result=token_exchange_failed&reason=http+{status}"
            )))
        }
        Err(_) => Ok(Redirect::temporary(
            "/auth/callback/oidc?result=token_exchange_failed&reason=token+endpoint+unreachable"
        )),
    }
}
```

- [ ] **Step 4: Run test, verify pass, commit**

```bash
cargo test oidc_token_exchange -- --exact
git add src/oidc/mod.rs src/http/handlers/oidc_handler.rs tests/oidc_token_exchange.rs
git commit -m "feat: add OIDC token exchange in callback handler"
```

---

### Task 3: Session Cookie + Axum Extractor

**Files:**
- Create: `src/session/mod.rs`
- Modify: `src/config.rs`
- Modify: `src/config/raw.rs`
- Modify: `src/config/sources.rs`
- Create: `tests/session_cookie.rs`

**Interfaces:**
- Produces: `SessionClaims { user_id: i64, sub: String, access_token: String }`
- Produces: `sign_session(claims: &SessionClaims, secret: &[u8]) -> String`
- Produces: `verify_session(token: &str, secret: &[u8]) -> Result<SessionClaims, String>`
- Produces: `Session { user_id: i64, sub: String, access_token: String }` (Axum FromRequestParts)

- [ ] **Step 1: Add session_signing_secret to config**

Add to `src/config/raw.rs` under `RawGitConfig` or a new struct. Since the session secret is separate from OIDC, add a `session` section:

In `src/config/raw.rs`, add after `RawGitConfig`:

```rust
#[derive(Clone, Debug, Default, Deserialize)]
pub struct RawSessionConfig {
    pub signing_secret: Option<String>,
}
```

In `src/config/sources.rs`, add session parsing to `raw_from_env_map`:

```rust
session: Some(RawSessionConfig {
    signing_secret: values.get("SESSION_SIGNING_SECRET").cloned(),
}),
```

In `src/config.rs`, add to `Config`:

```rust
pub session_signing_secret: String,
```

And in `Config::load()`:

```rust
let session_signing_secret = raw
    .session
    .as_ref()
    .and_then(|s| s.signing_secret.as_deref())
    .unwrap_or("issueflow-default-session-secret")
    .to_string();
```

In `Config::for_tests()`:

```rust
session_signing_secret: "test-session-secret".to_string(),
```

- [ ] **Step 2: Write failing test for session cookie**

Create `tests/session_cookie.rs`:

```rust
use issueflow::session::{SessionClaims, sign_session, verify_session};

#[test]
fn session_round_trips() {
    let secret = b"test-secret-key-32-bytes!!!!!";
    let claims = SessionClaims {
        user_id: 42,
        sub: "user-123".to_string(),
        access_token: "glpat-abc".to_string(),
    };

    let token = sign_session(&claims, secret);
    let verified = verify_session(&token, secret).unwrap();

    assert_eq!(verified.user_id, 42);
    assert_eq!(verified.sub, "user-123");
    assert_eq!(verified.access_token, "glpat-abc");
}

#[test]
fn session_rejects_tampered_token() {
    let secret = b"test-secret-key-32-bytes!!!!!";
    let claims = SessionClaims {
        user_id: 1,
        sub: "user-123".to_string(),
        access_token: "glpat-abc".to_string(),
    };

    let token = sign_session(&claims, secret);
    let tampered = token.replace("a", "b");
    assert!(verify_session(&tampered, secret).is_err());
}

#[test]
fn session_rejects_wrong_secret() {
    let secret = b"test-secret-key-32-bytes!!!!!";
    let wrong = b"wrong-secret-key-32-bytes!!!!";
    let claims = SessionClaims {
        user_id: 1,
        sub: "user-123".to_string(),
        access_token: "glpat-abc".to_string(),
    };

    let token = sign_session(&claims, secret);
    assert!(verify_session(&token, wrong).is_err());
}
```

Run: `cargo test session_cookie -- --exact`
Expected: FAIL (module not found)

- [ ] **Step 3: Implement session module**

Create `src/session/mod.rs`:

```rust
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionClaims {
    pub user_id: i64,
    pub sub: String,
    pub access_token: String,
}

#[derive(Clone, Debug)]
pub struct Session {
    pub user_id: i64,
    pub sub: String,
    pub access_token: String,
}

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let config = parts
            .extensions
            .get::<crate::config::Config>()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

        let cookie_header = parts
            .headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let session_cookie = cookie_header
            .split(';')
            .map(|c| c.trim())
            .find(|c| c.starts_with("session="))
            .and_then(|c| c.strip_prefix("session="))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let claims =
            verify_session(session_cookie, config.session_signing_secret.as_bytes())
                .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(Session {
            user_id: claims.user_id,
            sub: claims.sub,
            access_token: claims.access_token,
        })
    }
}

pub fn sign_session(claims: &SessionClaims, secret: &[u8]) -> String {
    let payload = serde_json::to_vec(claims).expect("session claims should serialize");
    let payload_b64 = URL_SAFE_NO_PAD.encode(&payload);

    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC key should be valid");
    mac.update(payload_b64.as_bytes());
    let signature = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());

    format!("{}.{}", payload_b64, signature)
}

pub fn verify_session(token: &str, secret: &[u8]) -> Result<SessionClaims, String> {
    let (payload_b64, signature) = token
        .split_once('.')
        .ok_or("invalid session token format")?;

    let mut mac = HmacSha256::new_from_slice(secret).map_err(|_| "invalid secret")?;
    mac.update(payload_b64.as_bytes());
    let expected_sig = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());

    if signature != expected_sig {
        return Err("invalid session signature".to_string());
    }

    let payload = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|_| "invalid session payload")?;

    let claims: SessionClaims =
        serde_json::from_slice(&payload).map_err(|_| "invalid session claims")?;

    Ok(claims)
}
```

Register in `src/lib.rs`:

```rust
pub mod session;
```

- [ ] **Step 4: Run tests, verify pass, commit**

```bash
cargo test session_cookie -- --exact
git add src/session/mod.rs src/lib.rs src/config.rs src/config/raw.rs src/config/sources.rs tests/session_cookie.rs
git commit -m "feat: add session cookie sign/verify module"
```

---

### Task 4: Workbench CRUD API

**Files:**
- Create: `src/http/handlers/workbench_handler.rs`
- Modify: `src/http/routes.rs`
- Modify: `src/http/handlers/mod.rs`
- Create: `tests/workbench_handler.rs`

**Interfaces:**
- Consumes: `DbPool` (from Task 1), `SessionClaims` (from Task 3)
- Produces: `Workbench { id, user_id, project_id, project_name, project_path, created_at, updated_at }`

- [ ] **Step 1: Write failing tests for workbench CRUD**

Create `tests/workbench_handler.rs`:

```rust
use axum::{body::Body, http::{header, Request, StatusCode}};
use issueflow::config::Config;
use issueflow::db::DbPool;
use sqlx::AnyPool;
use tower::ServiceExt;

async fn test_pool() -> DbPool {
    let pool = AnyPool::connect("sqlite::memory:").await.unwrap();
    issueflow::db::run_migrations(&pool).await.unwrap();
    pool
}

fn test_router(pool: DbPool) -> axum::Router {
    issueflow::http::routes::router_with_db(
        Config::for_tests("expected-token"),
        pool,
    )
}

#[tokio::test]
async fn list_workbenches_returns_empty_array() {
    let pool = test_pool().await;
    let app = test_router(pool);

    let response = app
        .oneshot(Request::builder().uri("/api/workbenches").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

Run: `cargo test workbench_handler -- --exact`
Expected: FAIL (function not found)

- [ ] **Step 2: Create workbench handler with list endpoint**

Create `src/http/handlers/workbench_handler.rs`:

```rust
use axum::{Json, extract::State};
use sqlx::AnyPool;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct Workbench {
    pub id: i64,
    pub user_id: String,
    pub project_id: i64,
    pub project_name: String,
    pub project_path: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn list_workbenches(
    State(pool): State<AnyPool>,
) -> Json<Vec<Workbench>> {
    let rows: Vec<Workbench> = sqlx::query_as(
        "SELECT id, user_id, project_id, project_name, project_path, created_at, updated_at FROM workbenches"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    Json(rows)
}
```

- [ ] **Step 3: Update routes and handlers mod**

In `src/http/handlers/mod.rs`, add:

```rust
pub mod workbench_handler;
```

In `src/http/routes.rs`, add a new constructor that accepts `DbPool`:

```rust
use sqlx::AnyPool;

pub fn router_with_db(config: Config, pool: AnyPool) -> Router {
    Router::new()
        .route("/", get(spa_handler::app_shell))
        .route("/workbench", get(spa_handler::app_shell))
        .route("/auth/callback/oidc", get(spa_handler::app_shell))
        .route("/assets/{*path}", get(spa_handler::app_asset))
        .route("/api/auth/login", get(oidc_handler::oidc_login))
        .route("/api/auth/callback", get(oidc_handler::oidc_callback))
        .route("/api/status/ping", get(status_handler::status_ping))
        .route("/api/status/session/{session_id}", get(status_handler::session_status))
        .route("/api/confirm/plan/{token}", get(confirm_handler::confirm_plan))
        .route("/api/webhooks/gitlab", post(webhook_handler::handle_webhook))
        .route("/api/issues", post(issues_handler::create_issue))
        .route("/api/workbenches", get(workbench_handler::list_workbenches))
        .with_state(config)
        .with_state(pool)
}
```

Keep the existing `router(config)` for backward compatibility (tests that don't need DB).

- [ ] **Step 4: Add POST, PUT, DELETE handlers**

Add to `workbench_handler.rs`:

```rust
use axum::{http::StatusCode, extract::Path};

#[derive(serde::Deserialize)]
pub struct CreateWorkbenchInput {
    pub project_id: i64,
    pub project_name: String,
    pub project_path: String,
}

pub async fn create_workbench(
    State(pool): State<AnyPool>,
    Json(input): Json<CreateWorkbenchInput>,
) -> Result<(StatusCode, Json<Workbench>), StatusCode> {
    let user_id = "demo-user".to_string(); // placeholder until session extractor

    let result = sqlx::query_as(
        "INSERT INTO workbenches (user_id, project_id, project_name, project_path)
         VALUES (?, ?, ?, ?)
         RETURNING id, user_id, project_id, project_name, project_path, created_at, updated_at"
    )
    .bind(&user_id)
    .bind(input.project_id)
    .bind(&input.project_name)
    .bind(&input.project_path)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(wb) => Ok((StatusCode::CREATED, Json(wb))),
        Err(_) => Err(StatusCode::CONFLICT),
    }
}

pub async fn update_workbench(
    State(pool): State<AnyPool>,
    Path(id): Path<i64>,
    Json(input): Json<CreateWorkbenchInput>,
) -> Result<Json<Workbench>, StatusCode> {
    let result = sqlx::query_as(
        "UPDATE workbenches SET project_id = ?, project_name = ?, project_path = ?, updated_at = datetime('now')
         WHERE id = ?
         RETURNING id, user_id, project_id, project_name, project_path, created_at, updated_at"
    )
    .bind(input.project_id)
    .bind(&input.project_name)
    .bind(&input.project_path)
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(wb)) => Ok(Json(wb)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::CONFLICT),
    }
}

pub async fn delete_workbench(
    State(pool): State<AnyPool>,
    Path(id): Path<i64>,
) -> StatusCode {
    let result = sqlx::query("DELETE FROM workbenches WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => StatusCode::NO_CONTENT,
        _ => StatusCode::NOT_FOUND,
    }
}
```

Add routes in `router_with_db`:

```rust
.route("/api/workbenches", post(workbench_handler::create_workbench))
.route("/api/workbenches/{id}", put(workbench_handler::update_workbench))
.route("/api/workbenches/{id}", delete(workbench_handler::delete_workbench))
```

- [ ] **Step 5: Run tests, verify pass, commit**

```bash
cargo test workbench_handler -- --exact
git add src/http/handlers/workbench_handler.rs src/http/handlers/mod.rs src/http/routes.rs tests/workbench_handler.rs
git commit -m "feat: add workbench CRUD API endpoints"
```

---

### Task 5: GitLab Projects Proxy API

**Files:**
- Create: `src/gitlab/projects.rs`
- Modify: `src/gitlab/mod.rs`
- Modify: `src/http/routes.rs`
- Modify: `src/http/handlers/mod.rs`

**Interfaces:**
- Produces: `GitLabProject { id, name, path_with_namespace, namespace }`

- [ ] **Step 1: Write failing test for project search**

Create `tests/projects_proxy.rs`:

```rust
use axum::{body::Body, http::{Request, StatusCode}};
use issueflow::config::Config;
use sqlx::AnyPool;
use tower::ServiceExt;

#[tokio::test]
async fn projects_endpoint_returns_200() {
    let pool = AnyPool::connect("sqlite::memory:").await.unwrap();
    issueflow::db::run_migrations(&pool).await.unwrap();

    let app = issueflow::http::routes::router_with_db(
        Config::for_tests("expected-token"),
        pool,
    );

    let response = app
        .oneshot(Request::builder().uri("/api/projects?search=test").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

Run: `cargo test projects_proxy -- --exact`
Expected: FAIL (route not found)

- [ ] **Step 2: Create projects proxy handler**

Create `src/gitlab/projects.rs`:

```rust
use axum::{Json, extract::Query};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ProjectSearchParams {
    pub search: Option<String>,
}

#[derive(Serialize)]
pub struct GitLabProject {
    pub id: i64,
    pub name: String,
    pub path_with_namespace: String,
    pub namespace: GitLabNamespace,
}

#[derive(Serialize)]
pub struct GitLabNamespace {
    pub id: i64,
    pub name: String,
    pub kind: String,
}

pub async fn list_projects(
    Query(params): Query<ProjectSearchParams>,
) -> Json<Vec<GitLabProject>> {
    // Placeholder until session extractor provides the access token.
    // For now, returns empty. Will be wired to GitLab API in a follow-up.
    let _search = params.search.unwrap_or_default();
    Json(vec![])
}
```

- [ ] **Step 3: Register module and route**

In `src/gitlab/mod.rs`, add:

```rust
pub mod projects;
```

In `src/http/routes.rs`, add route in `router_with_db`:

```rust
.route("/api/projects", get(gitlab::projects::list_projects))
```

In `src/http/handlers/mod.rs`, no change needed (handler is in gitlab module).

- [ ] **Step 4: Run test, verify pass, commit**

```bash
cargo test projects_proxy -- --exact
git add src/gitlab/projects.rs src/gitlab/mod.rs src/http/routes.rs tests/projects_proxy.rs
git commit -m "feat: add GitLab projects proxy API placeholder"
```

---

### Task 6: Frontend Components

**Files:**
- Create: `web/src/components/workbench/WorkbenchSwitcher.vue`
- Create: `web/src/components/workbench/WorkbenchSearchDialog.vue`
- Modify: `web/src/views/WorkbenchView.vue`
- Modify: `web/src/stores/session.ts`

- [ ] **Step 1: Add workbench types to session store**

In `web/src/stores/session.ts`, add after existing interfaces:

```typescript
export interface Workbench {
  id: number;
  project_id: number;
  project_name: string;
  project_path: string;
  created_at: string;
}

export interface GitLabProject {
  id: number;
  name: string;
  path_with_namespace: string;
  namespace: { id: number; name: string; kind: string };
}
```

Add state:

```typescript
const workbenches = reactive<{ value: Workbench[] }>({ value: [] });
const currentWorkbenchId = reactive<{ value: number | null }>({ value: null });

function setWorkbenches(list: Workbench[]) {
  workbenches.value = list;
}

function setCurrentWorkbench(id: number) {
  currentWorkbenchId.value = id;
}
```

Export them in the return block.

- [ ] **Step 2: Create WorkbenchSwitcher.vue**

Create `web/src/components/workbench/WorkbenchSwitcher.vue`:

```vue
<template>
  <div class="switcher">
    <n-dropdown trigger="click" :options="dropdownOptions" @select="handleSelect">
      <n-button quaternary>
        <template #icon>
          <span class="switcher-label">{{ currentLabel }}</span>
        </template>
      </n-button>
    </n-dropdown>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { NButton, NDropdown } from "naive-ui";
import type { Workbench } from "@/stores/session";
import { useSessionStore } from "@/stores/session";

const emit = defineEmits<{
  select: [id: number];
  add: [];
}>();

const store = useSessionStore();

const currentLabel = computed(() => {
  const wb = store.workbenches.value.find(
    (w) => w.id === store.currentWorkbenchId.value,
  );
  return wb ? wb.project_path : "Select workbench...";
});

const dropdownOptions = computed(() => {
  const items: any[] = store.workbenches.value.map((wb) => ({
    label: wb.project_path,
    key: wb.id,
  }));
  if (items.length > 0) {
    items.push({ type: "divider", key: "divider" });
  }
  items.push({ label: "+ Add workbench...", key: "add" });
  return items;
});

function handleSelect(key: string | number) {
  if (key === "add") {
    emit("add");
  } else {
    emit("select", key as number);
  }
}
</script>

<style scoped>
.switcher-label {
  font-size: 14px;
  font-weight: 600;
}
</style>
```

- [ ] **Step 3: Create WorkbenchSearchDialog.vue**

Create `web/src/components/workbench/WorkbenchSearchDialog.vue`:

```vue
<template>
  <n-modal :show="visible" @update:show="emit('close')">
    <n-card style="width: 480px" title="Add workbench" :bordered="false">
      <n-input
        v-model:value="searchText"
        placeholder="Search GitLab projects..."
        clearable
        @update:value="onSearch"
      />
      <n-spin :show="loading" class="results">
        <n-list v-if="results.length > 0">
          <n-list-item
            v-for="item in groupedResults"
            :key="item.id"
            @click="selectProject(item)"
            :style="item.kind === 'group' ? 'color: var(--n-text-color-3); cursor: default' : 'cursor: pointer'"
          >
            {{ item.kind === 'group' ? item.name : item.path_with_namespace }}
          </n-list-item>
        </n-list>
        <n-empty v-else-if="searched" description="No projects found" />
      </n-spin>
      <template #footer>
        <n-button @click="emit('close')">Cancel</n-button>
      </template>
    </n-card>
  </n-modal>
</template>

<script setup lang="ts">
import { ref } from "vue";
import {
  NButton,
  NCard,
  NEmpty,
  NInput,
  NList,
  NListItem,
  NModal,
  NSpin,
} from "naive-ui";
import type { GitLabProject } from "@/stores/session";

const props = defineProps<{ visible: boolean }>();
const emit = defineEmits<{ close: []; select: [project: GitLabProject] }>();

const searchText = ref("");
const results = ref<GitLabProject[]>([]);
const loading = ref(false);
const searched = ref(false);

let debounceTimer: ReturnType<typeof setTimeout>;

function onSearch(value: string) {
  clearTimeout(debounceTimer);
  if (!value.trim()) {
    results.value = [];
    searched.value = false;
    return;
  }
  debounceTimer = setTimeout(async () => {
    loading.value = true;
    searched.value = true;
    try {
      const resp = await fetch(
        `/api/projects?search=${encodeURIComponent(value)}`,
      );
      if (resp.ok) {
        results.value = await resp.json();
      }
    } finally {
      loading.value = false;
    }
  }, 300);
}

import { computed } from "vue";

const groupedResults = computed(() => results.value);

function selectProject(project: GitLabProject) {
  emit("select", project);
}
</script>

<style scoped>
.results {
  margin-top: 16px;
}
</style>
```

- [ ] **Step 4: Update WorkbenchView.vue**

Replace the current WorkbenchView template:

```vue
<template>
  <app-shell active-key="overview">
    <div class="workbench-header">
      <WorkbenchSwitcher @select="switchWorkbench" @add="showDialog = true" />
    </div>
    <WorkbenchSearchDialog
      :visible="showDialog"
      @close="showDialog = false"
      @select="createWorkbench"
    />
    <n-card :bordered="false" class="panel">
      <template #header>
        <span>{{ currentWorkbench ? currentWorkbench.project_path : 'Agent Workbench' }}</span>
      </template>

      <div v-if="!currentWorkbench">
        <n-empty description="Select or add a workbench to get started" />
      </div>

      <div v-else>
        <h3>Issues</h3>
        <p class="muted">Issue management for {{ currentWorkbench.project_path }}</p>

        <h3 style="margin-top: 24px">Agent Sessions</h3>
        <p class="muted">Agent sessions for {{ currentWorkbench.project_path }}</p>
      </div>
    </n-card>
  </app-shell>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { NCard, NEmpty } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import WorkbenchSwitcher from "@/components/workbench/WorkbenchSwitcher.vue";
import WorkbenchSearchDialog from "@/components/workbench/WorkbenchSearchDialog.vue";
import { useSessionStore } from "@/stores/session";
import type { GitLabProject } from "@/stores/session";

const store = useSessionStore();
const showDialog = ref(false);

const currentWorkbench = computed(() =>
  store.workbenches.value.find((w) => w.id === store.currentWorkbenchId.value) ?? null,
);

onMounted(async () => {
  try {
    const resp = await fetch("/api/workbenches");
    if (resp.ok) {
      const list = await resp.json();
      store.setWorkbenches(list);
      if (list.length > 0) {
        store.setCurrentWorkbench(list[0].id);
      }
    }
  } catch {
    // API not ready — show empty state
  }
});

function switchWorkbench(id: number) {
  store.setCurrentWorkbench(id);
}

async function createWorkbench(project: GitLabProject) {
  try {
    const resp = await fetch("/api/workbenches", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        project_id: project.id,
        project_name: project.name,
        project_path: project.path_with_namespace,
      }),
    });
    if (resp.ok) {
      const wb = await resp.json();
      store.setWorkbenches([...store.workbenches.value, wb]);
      store.setCurrentWorkbench(wb.id);
      showDialog.value = false;
    }
  } catch {
    // handle error
  }
}
</script>

<style scoped>
.workbench-header {
  padding: 12px 24px 0;
}

.panel {
  max-width: 720px;
  margin: 16px 24px;
  border-radius: var(--if-radius-md);
  box-shadow: var(--if-shadow-panel);
}

.muted {
  color: var(--if-color-muted);
}
</style>
```

- [ ] **Step 5: Commit**

```bash
git add web/src/components/workbench/ web/src/views/WorkbenchView.vue web/src/stores/session.ts
git commit -m "feat: add workbench switcher and search dialog frontend"
```

---

### Task 7: Wire Up Database at Startup

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Initialize database in main.rs**

```rust
use issueflow::db;

#[tokio::main]
async fn main() {
    let config = Config::load().await.expect("failed to load gateway configuration");

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:issueflow.db?mode=rwc".to_string());
    let pool = db::open(&database_url).await
        .expect("failed to open database");

    server::serve(config, pool).await;
}
```

For production PostgreSQL: set `DATABASE_URL=postgres://user:pass@host/db`.

- [ ] **Step 2: Update server.rs to accept DbPool**

Modify `src/http/server.rs`:

```rust
use sqlx::any::AnyPool;

pub async fn serve(config: Config, pool: AnyPool) {
    let app = routes::router_with_db(config, pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

- [ ] **Step 3: Build and verify**

```bash
cargo build
git add src/main.rs src/http/server.rs
git commit -m "feat: wire database into gateway startup"
```
