# Configuration and OIDC Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the current environment-only, GitLab-specific OAuth configuration with a structured configuration system that loads from `config/issueflow.toml`, `.env`, and environment variables, and rename the browser login flow to single-issuer OIDC with discovery-backed `/auth/login` and `/auth/callback` routes.

**Architecture:** The Gateway loads raw configuration from three sources, merges them by precedence, validates them into a runtime `Config`, and resolves OIDC metadata from the configured issuer before serving requests. OIDC protocol handling moves into a dedicated `oidc` module, while the frontend keeps a separate SPA result route at `/auth/callback/oidc`.

**Tech Stack:** Rust, Axum, Tokio, Serde, TOML, dotenvy, reqwest, Vue 3, Vite, Vitest

---

### Task 1: Add Structured Configuration Loading With Precedence Tests

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/config.rs`
- Create: `src/config/raw.rs`
- Create: `src/config/sources.rs`
- Create: `tests/config_loading.rs`

- [ ] **Step 1: Add configuration loading dependencies and a temp-dir test helper**

Update `Cargo.toml` to:

```toml
[dependencies]
axum = "0.8"
base64 = "0.22"
dotenvy = "0.15"
hmac = "0.12"
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sha2 = "0.10"
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net"] }
toml = "0.8"

[dev-dependencies]
tempfile = "3"
tower = { version = "0.5", features = ["util"] }
```

- [ ] **Step 2: Write failing precedence tests before adding the loader**

Create `tests/config_loading.rs`:

```rust
use std::{env, fs, sync::Mutex};

use issueflow::config::Config;
use tempfile::tempdir;

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[tokio::test]
async fn config_prefers_environment_over_dotenv_and_toml() {
    let _guard = ENV_LOCK.lock().unwrap();
    let temp = tempdir().unwrap();
    let config_dir = temp.path().join("config");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(
        config_dir.join("issueflow.toml"),
        r#"
[server]
listen_addr = "127.0.0.1:9000"

[git]
webhook_secret = "toml-secret"
"#,
    )
    .unwrap();
    fs::write(
        temp.path().join(".env"),
        "LISTEN_ADDR=127.0.0.1:9100\nGIT_WEBHOOK_SECRET=dotenv-secret\n",
    )
    .unwrap();

    env::set_current_dir(temp.path()).unwrap();
    env::set_var("LISTEN_ADDR", "127.0.0.1:9200");
    env::set_var("GIT_WEBHOOK_SECRET", "env-secret");

    let config = Config::load().await.unwrap();

    assert_eq!(config.listen_addr, "127.0.0.1:9200");
    assert_eq!(config.git.webhook_secret, "env-secret");
}

#[tokio::test]
async fn config_uses_defaults_when_optional_sources_are_missing() {
    let _guard = ENV_LOCK.lock().unwrap();
    let temp = tempdir().unwrap();
    env::set_current_dir(temp.path()).unwrap();
    env::remove_var("LISTEN_ADDR");
    env::set_var("GIT_WEBHOOK_SECRET", "env-secret");

    let config = Config::load().await.unwrap();

    assert_eq!(config.listen_addr, "127.0.0.1:8080");
    assert_eq!(config.git.webhook_secret, "env-secret");
}
```

- [ ] **Step 3: Run the focused config tests to verify they fail**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test config_prefers_environment_over_dotenv_and_toml -- --exact`
Expected: FAIL with missing `Config::load`, missing `git` field, or missing raw config modules

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test config_uses_defaults_when_optional_sources_are_missing -- --exact`
Expected: FAIL for the same reasons

- [ ] **Step 4: Add raw config types and source-merging helpers**

Create `src/config/raw.rs`:

```rust
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawConfig {
    pub server: Option<RawServerConfig>,
    pub git: Option<RawGitConfig>,
    pub oidc: Option<RawOidcConfig>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawServerConfig {
    pub listen_addr: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawGitConfig {
    pub webhook_secret: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawOidcConfig {
    pub enabled: Option<bool>,
    pub issuer: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub state_signing_secret: Option<String>,
}

impl RawConfig {
    pub fn merge(mut self, other: Self) -> Self {
        self.server = merge_server(self.server, other.server);
        self.git = merge_git(self.git, other.git);
        self.oidc = merge_oidc(self.oidc, other.oidc);
        self
    }
}

fn merge_server(current: Option<RawServerConfig>, incoming: Option<RawServerConfig>) -> Option<RawServerConfig> {
    match (current, incoming) {
        (Some(mut current), Some(incoming)) => {
            if incoming.listen_addr.is_some() {
                current.listen_addr = incoming.listen_addr;
            }
            Some(current)
        }
        (None, Some(incoming)) => Some(incoming),
        (current, None) => current,
    }
}

fn merge_git(current: Option<RawGitConfig>, incoming: Option<RawGitConfig>) -> Option<RawGitConfig> {
    match (current, incoming) {
        (Some(mut current), Some(incoming)) => {
            if incoming.webhook_secret.is_some() {
                current.webhook_secret = incoming.webhook_secret;
            }
            Some(current)
        }
        (None, Some(incoming)) => Some(incoming),
        (current, None) => current,
    }
}

fn merge_oidc(current: Option<RawOidcConfig>, incoming: Option<RawOidcConfig>) -> Option<RawOidcConfig> {
    match (current, incoming) {
        (Some(mut current), Some(incoming)) => {
            if incoming.enabled.is_some() {
                current.enabled = incoming.enabled;
            }
            if incoming.issuer.is_some() {
                current.issuer = incoming.issuer;
            }
            if incoming.client_id.is_some() {
                current.client_id = incoming.client_id;
            }
            if incoming.client_secret.is_some() {
                current.client_secret = incoming.client_secret;
            }
            if incoming.redirect_uri.is_some() {
                current.redirect_uri = incoming.redirect_uri;
            }
            if incoming.scopes.is_some() {
                current.scopes = incoming.scopes;
            }
            if incoming.state_signing_secret.is_some() {
                current.state_signing_secret = incoming.state_signing_secret;
            }
            Some(current)
        }
        (None, Some(incoming)) => Some(incoming),
        (current, None) => current,
    }
}
```

Create `src/config/sources.rs`:

```rust
use std::{collections::HashMap, env, fs, path::Path};

use crate::config::raw::{RawConfig, RawGitConfig, RawOidcConfig, RawServerConfig};

pub fn load_raw_config() -> Result<RawConfig, String> {
    let defaults = RawConfig {
        server: Some(RawServerConfig {
            listen_addr: Some("127.0.0.1:8080".to_string()),
        }),
        git: None,
        oidc: Some(RawOidcConfig {
            enabled: Some(false),
            scopes: Some(vec![
                "openid".to_string(),
                "profile".to_string(),
                "email".to_string(),
            ]),
            ..RawOidcConfig::default()
        }),
    };

    let toml = load_toml_file(Path::new("config/issueflow.toml"))?;
    let dotenv = raw_from_env_map(load_dotenv_map(Path::new(".env"))?);
    let env_vars = raw_from_environment();

    Ok(defaults.merge(toml).merge(dotenv).merge(env_vars))
}

fn load_toml_file(path: &Path) -> Result<RawConfig, String> {
    if !path.exists() {
        return Ok(RawConfig::default());
    }

    let contents = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    toml::from_str(&contents)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))
}

fn load_dotenv_map(path: &Path) -> Result<HashMap<String, String>, String> {
    if !path.exists() {
        return Ok(HashMap::new());
    }

    dotenvy::from_path_iter(path)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))?
        .collect::<Result<HashMap<_, _>, _>>()
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))
}

fn raw_from_environment() -> RawConfig {
    let env_map = env::vars().collect::<HashMap<_, _>>();
    raw_from_env_map(env_map)
}

fn raw_from_env_map(values: HashMap<String, String>) -> RawConfig {
    RawConfig {
        server: Some(RawServerConfig {
            listen_addr: values.get("LISTEN_ADDR").cloned(),
        }),
        git: Some(RawGitConfig {
            webhook_secret: values.get("GIT_WEBHOOK_SECRET").cloned(),
        }),
        oidc: Some(RawOidcConfig {
            enabled: values.get("OIDC_ENABLED").and_then(|value| value.parse::<bool>().ok()),
            issuer: values.get("OIDC_ISSUER").cloned(),
            client_id: values.get("OIDC_CLIENT_ID").cloned(),
            client_secret: values.get("OIDC_CLIENT_SECRET").cloned(),
            redirect_uri: values.get("OIDC_REDIRECT_URI").cloned(),
            scopes: values.get("OIDC_SCOPES").map(|value| {
                value.split_whitespace().map(str::to_string).collect::<Vec<_>>()
            }),
            state_signing_secret: values.get("OIDC_STATE_SIGNING_SECRET").cloned(),
        }),
    }
}
```

- [ ] **Step 5: Replace `Config::from_env` with async structured loading**

Update `src/config.rs` to:

```rust
pub mod raw;
pub mod sources;

use crate::oidc::{OidcConfig, OidcEnabledConfig};

#[derive(Clone, Debug)]
pub struct Config {
    pub listen_addr: String,
    pub git: GitConfig,
    pub oidc: OidcConfig,
}

#[derive(Clone, Debug)]
pub struct GitConfig {
    pub webhook_secret: String,
}

impl Config {
    pub async fn load() -> Result<Self, String> {
        let raw = sources::load_raw_config()?;
        let listen_addr = raw
            .server
            .and_then(|server| server.listen_addr)
            .unwrap_or_else(|| "127.0.0.1:8080".to_string());
        let webhook_secret = raw
            .git
            .and_then(|git| git.webhook_secret)
            .ok_or("missing required configuration: git.webhook_secret")?;

        let oidc = OidcConfig::from_raw(raw.oidc.unwrap_or_default()).await?;

        Ok(Self {
            listen_addr,
            git: GitConfig { webhook_secret },
            oidc,
        })
    }

    pub fn for_tests(secret: &str) -> Self {
        Self {
            listen_addr: "127.0.0.1:0".to_string(),
            git: GitConfig {
                webhook_secret: secret.to_string(),
            },
            oidc: OidcConfig::disabled(),
        }
    }

    pub fn with_oidc(mut self, oidc: OidcConfig) -> Self {
        self.oidc = oidc;
        self
    }
}
```

Update `src/main.rs` to:

```rust
use issueflow::{config::Config, http::server};

#[tokio::main]
async fn main() {
    let config = Config::load()
        .await
        .expect("failed to load gateway configuration");
    server::serve(config)
        .await
        .expect("failed to run gateway server");
}
```

- [ ] **Step 6: Run the focused config tests, then one broader route test**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test config_prefers_environment_over_dotenv_and_toml -- --exact`
Expected: PASS

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test config_uses_defaults_when_optional_sources_are_missing -- --exact`
Expected: PASS

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test status_route_returns_ok -- --exact`
Expected: PASS and proves route tests still work with `Config::for_tests`

- [ ] **Step 7: Commit the structured config loading**

```bash
git add Cargo.toml src/config.rs src/config/raw.rs src/config/sources.rs src/main.rs tests/config_loading.rs
git commit -m "feat: add structured configuration loading"
```

### Task 2: Replace OAuth With OIDC and Add Issuer Discovery

**Files:**
- Modify: `src/lib.rs`
- Delete: `src/oauth/mod.rs`
- Create: `src/oidc/mod.rs`
- Create: `tests/oidc_config.rs`

- [ ] **Step 1: Write failing OIDC validation and discovery tests**

Create `tests/oidc_config.rs`:

```rust
use axum::{routing::get, Router};
use issueflow::{
    config::raw::RawOidcConfig,
    oidc::OidcConfig,
};
use tokio::net::TcpListener;

#[tokio::test]
async fn oidc_disabled_skips_required_field_validation() {
    let config = OidcConfig::from_raw(RawOidcConfig {
        enabled: Some(false),
        ..RawOidcConfig::default()
    })
    .await
    .unwrap();

    assert!(!config.is_enabled());
}

#[tokio::test]
async fn oidc_enabled_loads_metadata_from_the_issuer_document() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = Router::new().route(
        "/.well-known/openid-configuration",
        get(|| async {
            r#"{
                "issuer":"http://127.0.0.1",
                "authorization_endpoint":"http://127.0.0.1/authorize",
                "token_endpoint":"http://127.0.0.1/token"
            }"#
        }),
    );
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let config = OidcConfig::from_raw(RawOidcConfig {
        enabled: Some(true),
        issuer: Some(format!("http://{addr}")),
        client_id: Some("client-id".to_string()),
        client_secret: Some("client-secret".to_string()),
        redirect_uri: Some("http://127.0.0.1:8080/auth/callback".to_string()),
        scopes: Some(vec!["openid".to_string(), "profile".to_string()]),
        state_signing_secret: Some("state-secret".to_string()),
    })
    .await
    .unwrap();

    let enabled = config.enabled().unwrap();
    assert_eq!(enabled.metadata.authorization_endpoint, format!("http://{addr}/authorize"));
    assert_eq!(enabled.metadata.token_endpoint, format!("http://{addr}/token"));
}
```

- [ ] **Step 2: Run the OIDC-focused tests to verify they fail**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test oidc_disabled_skips_required_field_validation -- --exact`
Expected: FAIL because `src/oidc/mod.rs` and `OidcConfig::from_raw` do not exist yet

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test oidc_enabled_loads_metadata_from_the_issuer_document -- --exact`
Expected: FAIL for the same reasons

- [ ] **Step 3: Create the OIDC runtime types, metadata, discovery, and state handling**

Create `src/oidc/mod.rs`:

```rust
use std::time::{SystemTime, UNIX_EPOCH};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::config::raw::RawOidcConfig;

type HmacSha256 = Hmac<Sha256>;
const STATE_TTL_SECONDS: u64 = 600;

#[derive(Clone, Debug)]
pub enum OidcConfig {
    Disabled,
    Enabled(OidcEnabledConfig),
}

#[derive(Clone, Debug)]
pub struct OidcEnabledConfig {
    pub issuer: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub state_signing_secret: String,
    pub metadata: OidcMetadata,
}

#[derive(Clone, Debug)]
pub struct OidcMetadata {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
}

#[derive(Debug, Deserialize)]
struct OidcDiscoveryDocument {
    issuer: String,
    authorization_endpoint: String,
    token_endpoint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct OidcStateClaims {
    issuer: String,
    issued_at: u64,
}

#[derive(Debug)]
pub enum OidcStateError {
    InvalidFormat,
    InvalidSignature,
    InvalidPayload,
    Expired,
}

impl OidcConfig {
    pub fn disabled() -> Self {
        Self::Disabled
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled(_))
    }

    pub fn enabled(&self) -> Option<&OidcEnabledConfig> {
        match self {
            Self::Enabled(config) => Some(config),
            Self::Disabled => None,
        }
    }

    pub async fn from_raw(raw: RawOidcConfig) -> Result<Self, String> {
        if !raw.enabled.unwrap_or(false) {
            return Ok(Self::Disabled);
        }

        let issuer = raw.issuer.ok_or("missing required configuration: oidc.issuer")?;
        let client_id = raw.client_id.ok_or("missing required configuration: oidc.client_id")?;
        let client_secret = raw
            .client_secret
            .ok_or("missing required configuration: oidc.client_secret")?;
        let redirect_uri = raw
            .redirect_uri
            .ok_or("missing required configuration: oidc.redirect_uri")?;
        let state_signing_secret = raw
            .state_signing_secret
            .ok_or("missing required configuration: oidc.state_signing_secret")?;
        let scopes = raw.scopes.unwrap_or_else(|| {
            vec!["openid".to_string(), "profile".to_string(), "email".to_string()]
        });
        let metadata = discover_metadata(&issuer).await?;

        Ok(Self::Enabled(OidcEnabledConfig {
            issuer,
            client_id,
            client_secret,
            redirect_uri,
            scopes,
            state_signing_secret,
            metadata,
        }))
    }
}

impl OidcEnabledConfig {
    pub fn authorize_url(&self, state: &str) -> String {
        let scope = self.scopes.join(" ");
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            self.metadata.authorization_endpoint,
            encode_component(&self.client_id),
            encode_component(&self.redirect_uri),
            encode_component(&scope),
            encode_component(state),
        )
    }
}

pub async fn discover_metadata(issuer: &str) -> Result<OidcMetadata, String> {
    let issuer = issuer.trim_end_matches('/');
    let discovery_url = format!("{issuer}/.well-known/openid-configuration");
    let document = Client::new()
        .get(&discovery_url)
        .send()
        .await
        .map_err(|error| format!("failed oidc discovery request: {error}"))?
        .error_for_status()
        .map_err(|error| format!("failed oidc discovery response: {error}"))?
        .json::<OidcDiscoveryDocument>()
        .await
        .map_err(|error| format!("failed oidc discovery decode: {error}"))?;

    Ok(OidcMetadata {
        issuer: document.issuer,
        authorization_endpoint: document.authorization_endpoint,
        token_endpoint: document.token_endpoint,
    })
}

pub fn issue_state(issuer: &str, signing_secret: &str) -> Result<String, OidcStateError> {
    let claims = OidcStateClaims {
        issuer: issuer.to_string(),
        issued_at: unix_timestamp_now(),
    };
    let claims_json = serde_json::to_vec(&claims).map_err(|_| OidcStateError::InvalidPayload)?;
    let claims_token = URL_SAFE_NO_PAD.encode(claims_json);
    let signature = sign_state_component(&claims_token, signing_secret)?;
    Ok(format!("{claims_token}.{signature}"))
}

pub fn validate_state(encoded_state: &str, issuer: &str, signing_secret: &str) -> Result<(), OidcStateError> {
    let (claims_token, signature) = encoded_state
        .split_once('.')
        .ok_or(OidcStateError::InvalidFormat)?;
    let expected_signature = sign_state_component(claims_token, signing_secret)?;

    if signature != expected_signature {
        return Err(OidcStateError::InvalidSignature);
    }

    let claims_bytes = URL_SAFE_NO_PAD
        .decode(claims_token)
        .map_err(|_| OidcStateError::InvalidPayload)?;
    let claims: OidcStateClaims =
        serde_json::from_slice(&claims_bytes).map_err(|_| OidcStateError::InvalidPayload)?;

    if claims.issuer != issuer {
        return Err(OidcStateError::InvalidPayload);
    }

    if unix_timestamp_now().saturating_sub(claims.issued_at) > STATE_TTL_SECONDS {
        return Err(OidcStateError::Expired);
    }

    Ok(())
}

fn sign_state_component(claims_token: &str, signing_secret: &str) -> Result<String, OidcStateError> {
    let mut mac = HmacSha256::new_from_slice(signing_secret.as_bytes())
        .map_err(|_| OidcStateError::InvalidSignature)?;
    mac.update(claims_token.as_bytes());
    Ok(URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes()))
}

fn unix_timestamp_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_secs()
}

fn encode_component(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len());

    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(char::from(byte));
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }

    encoded
}
```

Update `src/lib.rs` to:

```rust
pub mod config;
pub mod gitlab;
pub mod http;
pub mod oidc;
pub mod workflow;
```

- [ ] **Step 4: Run the OIDC tests to verify they now pass**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test oidc_disabled_skips_required_field_validation -- --exact`
Expected: PASS

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test oidc_enabled_loads_metadata_from_the_issuer_document -- --exact`
Expected: PASS

- [ ] **Step 5: Commit the OIDC module conversion**

```bash
git add Cargo.toml src/lib.rs src/oidc/mod.rs tests/oidc_config.rs
git commit -m "feat: add oidc configuration and discovery"
```

### Task 3: Rename Auth Routes and Update the Frontend Callback Flow

**Files:**
- Delete: `src/http/handlers/oauth_handler.rs`
- Create: `src/http/handlers/oidc_handler.rs`
- Modify: `src/http/handlers/mod.rs`
- Modify: `src/http/routes.rs`
- Modify: `tests/oauth_handler.rs`
- Modify: `web/src/router/index.ts`
- Modify: `web/src/tests/oauth-callback-view.spec.ts`

- [ ] **Step 1: Rewrite the route tests around `/auth/login` and `/auth/callback`**

Replace `tests/oauth_handler.rs` with:

```rust
use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use issueflow::{
    config::Config,
    oidc::{OidcConfig, OidcEnabledConfig, OidcMetadata},
};
use tower::ServiceExt;

#[tokio::test]
async fn oidc_login_redirects_to_the_discovered_authorization_endpoint() {
    let app = issueflow::http::routes::router(test_config());
    let response = app
        .oneshot(Request::builder().uri("/auth/login").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);

    let location = response
        .headers()
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .unwrap();

    assert!(location.starts_with("https://gitlab.example.com/oauth/authorize?"));
    assert!(location.contains("client_id=gitlab-test-client"));
    assert!(location.contains(
        "redirect_uri=http%3A%2F%2F127.0.0.1%3A8080%2Fauth%2Fcallback"
    ));
    assert!(location.contains("scope=openid%20profile%20email"));
    assert!(location.contains("state="));
}

#[tokio::test]
async fn oidc_login_returns_service_unavailable_when_oidc_is_disabled() {
    let app = issueflow::http::routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(Request::builder().uri("/auth/login").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn oidc_callback_rejects_invalid_state() {
    let app = issueflow::http::routes::router(test_config());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/auth/callback?code=test-code&state=invalid-state")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn oidc_callback_redirects_to_the_frontend_oidc_result_route() {
    let app = issueflow::http::routes::router(test_config());
    let login_response = app
        .clone()
        .oneshot(Request::builder().uri("/auth/login").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let state = extract_query_param(
        login_response
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok())
            .unwrap(),
        "state",
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/auth/callback?code=test-code&state={state}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/auth/callback/oidc?result=success")
    );
}

fn test_config() -> Config {
    Config::for_tests("expected-token").with_oidc(OidcConfig::Enabled(OidcEnabledConfig {
        issuer: "https://gitlab.example.com".to_string(),
        client_id: "gitlab-test-client".to_string(),
        client_secret: "gitlab-test-secret".to_string(),
        redirect_uri: "http://127.0.0.1:8080/auth/callback".to_string(),
        scopes: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
        state_signing_secret: "test-oidc-state-secret".to_string(),
        metadata: OidcMetadata {
            issuer: "https://gitlab.example.com".to_string(),
            authorization_endpoint: "https://gitlab.example.com/oauth/authorize".to_string(),
            token_endpoint: "https://gitlab.example.com/oauth/token".to_string(),
        },
    }))
}

fn extract_query_param(location: &str, key: &str) -> String {
    let query = location.split('?').nth(1).unwrap();
    let prefix = format!("{key}=");
    let value_start = query.find(&prefix).unwrap() + prefix.len();
    let value = &query[value_start..];

    value.split('&').next().unwrap().to_string()
}
```

- [ ] **Step 2: Run one route test to verify it fails before the handler rename**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test oidc_login_redirects_to_the_discovered_authorization_endpoint -- --exact`
Expected: FAIL because `/auth/login` is not routed and the handler still imports `oauth`

- [ ] **Step 3: Replace the handler and route wiring with OIDC names**

Create `src/http/handlers/oidc_handler.rs`:

```rust
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
};
use serde::Deserialize;

use crate::{config::Config, oidc::{issue_state, validate_state}};

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
```

Update `src/http/handlers/mod.rs` to:

```rust
pub mod confirm_handler;
pub mod oidc_handler;
pub mod spa_handler;
pub mod status_handler;
pub mod webhook_handler;
```

Update `src/http/routes.rs` to:

```rust
use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    config::Config,
    http::handlers::{confirm_handler, oidc_handler, spa_handler, status_handler, webhook_handler},
};

pub fn router(config: Config) -> Router {
    Router::new()
        .route("/", get(spa_handler::app_shell))
        .route("/index.html", get(spa_handler::app_shell))
        .route("/workbench", get(spa_handler::app_shell))
        .route("/auth/callback/oidc", get(spa_handler::app_shell))
        .route("/assets/{*path}", get(spa_handler::app_asset))
        .route("/auth/login", get(oidc_handler::oidc_login))
        .route("/auth/callback", get(oidc_handler::oidc_callback))
        .route("/status/ping", get(status_handler::status_ping))
        .route("/status/session/{session_id}", get(status_handler::session_status))
        .route("/confirm/plan/{token}", get(confirm_handler::confirm_plan))
        .route("/webhooks/gitlab", post(webhook_handler::handle_webhook))
        .with_state(config)
}
```

Update `web/src/router/index.ts` to:

```ts
import { createRouter, createWebHistory } from "vue-router";
import LandingView from "@/views/LandingView.vue";
import OAuthCallbackView from "@/views/OAuthCallbackView.vue";
import WorkbenchView from "@/views/WorkbenchView.vue";

export const routes = [
  {
    path: "/",
    name: "landing",
    component: LandingView,
  },
  {
    path: "/auth/callback/oidc",
    name: "oidc-callback",
    component: OAuthCallbackView,
  },
  {
    path: "/workbench",
    name: "workbench",
    component: WorkbenchView,
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});
```

Update `web/src/tests/oauth-callback-view.spec.ts` to render:

```ts
const { wrapper } = await renderAt("/auth/callback/oidc?result=success");
```

- [ ] **Step 4: Run focused route tests, then frontend tests**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test oidc_login_redirects_to_the_discovered_authorization_endpoint -- --exact`
Expected: PASS

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test oidc_callback_redirects_to_the_frontend_oidc_result_route -- --exact`
Expected: PASS

Run: `cd web && npm run test`
Expected: PASS and confirm the SPA callback route still renders correctly

- [ ] **Step 5: Commit the OIDC route rename**

```bash
git add src/http/handlers/mod.rs src/http/handlers/oidc_handler.rs src/http/routes.rs tests/oauth_handler.rs web/src/router/index.ts web/src/tests/oauth-callback-view.spec.ts
git commit -m "feat: switch auth routes to oidc naming"
```

### Task 4: Add Configuration Documentation and Run Full Verification

**Files:**
- Create: `docs/configuration.md`
- Modify: `README.md`

- [ ] **Step 1: Write the dedicated configuration reference**

Create `docs/configuration.md`:

````md
# Configuration

## Sources

`issueflow` loads configuration from these sources, in order of increasing priority:

1. `config/issueflow.toml`
2. `.env`
3. process environment variables

Higher-priority sources override lower-priority sources field by field.

## Default File Layout

```text
config/issueflow.toml
.env
```

## Example `config/issueflow.toml`

```toml
[server]
listen_addr = "127.0.0.1:8080"

[git]
webhook_secret = "replace-me"

[oidc]
enabled = true
issuer = "https://gitlab.com"
client_id = "your-client-id"
client_secret = "your-client-secret"
redirect_uri = "http://127.0.0.1:8080/auth/callback"
scopes = ["openid", "profile", "email"]
state_signing_secret = "replace-me-with-a-long-random-secret"
```

## Example `.env`

```bash
LISTEN_ADDR=127.0.0.1:8080
GIT_WEBHOOK_SECRET=replace-me
OIDC_ENABLED=true
OIDC_ISSUER=https://gitlab.com
OIDC_CLIENT_ID=your-client-id
OIDC_CLIENT_SECRET=your-client-secret
OIDC_REDIRECT_URI=http://127.0.0.1:8080/auth/callback
OIDC_SCOPES=openid profile email
OIDC_STATE_SIGNING_SECRET=replace-me-with-a-long-random-secret
```

## Fields

### `server.listen_addr`

- Default: `127.0.0.1:8080`
- Env: `LISTEN_ADDR`

### `git.webhook_secret`

- Required: yes
- Env: `GIT_WEBHOOK_SECRET`

### `oidc.enabled`

- Default: `false`
- Env: `OIDC_ENABLED`

### `oidc.issuer`

- Required when `oidc.enabled = true`
- Env: `OIDC_ISSUER`

### `oidc.client_id`

- Required when `oidc.enabled = true`
- Env: `OIDC_CLIENT_ID`

### `oidc.client_secret`

- Required when `oidc.enabled = true`
- Env: `OIDC_CLIENT_SECRET`

### `oidc.redirect_uri`

- Required when `oidc.enabled = true`
- Env: `OIDC_REDIRECT_URI`
- Should point to the Gateway-owned callback route: `/auth/callback`

### `oidc.scopes`

- Default: `["openid", "profile", "email"]`
- Env: `OIDC_SCOPES`

### `oidc.state_signing_secret`

- Required when `oidc.enabled = true`
- Env: `OIDC_STATE_SIGNING_SECRET`

## OIDC Discovery

When OIDC is enabled, `issueflow` fetches issuer metadata from:

```text
<issuer>/.well-known/openid-configuration
```

Startup fails if discovery or required field validation fails.
````

- [ ] **Step 2: Add a short README link section**

Append this section to `README.md`:

```md
## Configuration

Gateway configuration source order, field reference, and OIDC setup live in:

- `docs/configuration.md`
```

- [ ] **Step 3: Run the full verification set**

Run: `cd web && npm run typecheck && npm run lint && npm run test && npm run build`
Expected: PASS

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test`
Expected: PASS

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test oidc_ -- --nocapture`
Expected: PASS for route and OIDC-specific tests

- [ ] **Step 4: Commit the docs and final verification**

```bash
git add docs/configuration.md README.md
git commit -m "docs: add configuration and oidc reference"
```
