# Configuration and OIDC Design

## Goal

Redesign `issueflow` configuration loading and browser login configuration around a single, consistent model:

- `config/issueflow.toml`
- `.env`
- process environment variables
- OIDC-only browser login configuration

The result should simplify current GitLab-specific configuration, define a clear precedence model, and establish a durable configuration contract for future Gateway work.

## Scope

This design covers:

- configuration file locations
- configuration source precedence
- runtime configuration validation rules
- OIDC-only login configuration
- Gateway auth route naming
- documentation structure for configuration reference

This design also includes the near-term code restructuring needed to support the new model.

## Non-Goals

This change does not:

- implement multi-provider login in a single Gateway instance
- add token exchange, session persistence, or user storage
- add generic OAuth-only login support alongside OIDC
- finalize future Git webhook vendor abstraction beyond current naming cleanup discussion
- implement every future configuration section the platform may need

## Design Constraints

- Keep the change small and compatible with the current Rust Gateway foundation
- Prefer one coherent configuration system over compatibility shims
- Treat this as a cleanup opportunity: old OAuth env var names do not need compatibility support
- Support only one browser identity provider per Gateway instance
- Use OIDC terminology consistently across code, config, docs, and routes

## Configuration Sources

The Gateway should load configuration from three external sources:

1. `config/issueflow.toml`
2. `.env`
3. process environment variables

If a source file does not exist, the loader should skip it without error.

## Precedence Rules

Configuration precedence must be:

1. process environment variables
2. `.env`
3. `config/issueflow.toml`
4. built-in defaults

This precedence applies field-by-field, not source-by-source wholesale replacement.

That means:

- a value present in `config/issueflow.toml` may remain in effect unless `.env` or the process environment overrides that same field
- `.env` is a local-development override layer
- process environment variables always win

## Configuration Shape

The external configuration should be structured and minimal.

Recommended `config/issueflow.toml` example:

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

## Environment Variable Shape

Environment variable names should be redesigned to match the new configuration model.

Recommended names:

- `LISTEN_ADDR`
- `GIT_WEBHOOK_SECRET`
- `OIDC_ENABLED`
- `OIDC_ISSUER`
- `OIDC_CLIENT_ID`
- `OIDC_CLIENT_SECRET`
- `OIDC_REDIRECT_URI`
- `OIDC_SCOPES`
- `OIDC_STATE_SIGNING_SECRET`

Parsing rules:

- `OIDC_ENABLED` accepts a boolean value
- `OIDC_SCOPES` is a whitespace-delimited string in env form
- `scopes` remains an array of strings in TOML

The old variable names:

- `GITLAB_OAUTH_CLIENT_ID`
- `GITLAB_OAUTH_CLIENT_SECRET`
- `GITLAB_OAUTH_REDIRECT_URI`
- `GITLAB_OAUTH_AUTHORIZE_URL`
- `GITLAB_OAUTH_TOKEN_URL`
- `GITLAB_OAUTH_SCOPES`
- `OAUTH_STATE_SIGNING_SECRET`

should be removed from the design and not carried forward as compatibility aliases.

## Built-In Defaults

Defaults should remain intentionally small.

Recommended defaults:

- `server.listen_addr = "127.0.0.1:8080"`
- `oidc.enabled = false`
- `oidc.scopes = ["openid", "profile", "email"]`

No default should be provided for secrets or client credentials.

## OIDC-Only Identity Model

The Gateway should support only OIDC for browser login.

This means:

- configuration is named `oidc`, not `oauth`
- discovery is based on the OIDC issuer
- the login flow is described as OIDC login
- provider-specific naming such as `gitlab oauth` should be removed from the configuration model

This still allows GitLab, GitHub, or Gitee later, as long as the configured identity system behaves as an OIDC issuer for the purposes needed by the Gateway.

## Single-Issuer Runtime Model

Each Gateway instance supports exactly one configured OIDC issuer.

This avoids:

- provider maps
- per-provider route branches
- multiple simultaneous login buttons
- configuration structures like `oidc.gitlab`, `oidc.github`, or `oidc.providers.<name>`

The configured issuer determines the active identity provider for that deployment.

## OIDC Discovery

The Gateway should use issuer-based discovery.

Required input:

- `oidc.issuer`

The Gateway should derive provider metadata from the issuer discovery document, including:

- authorization endpoint
- token endpoint
- issuer identity

The implementation should target the standard OIDC discovery document path:

- `/.well-known/openid-configuration`

## Discovery Failure Policy

For this first redesign, the system should be OIDC-first and strict.

If OIDC is enabled and discovery fails, the Gateway should fail startup with a clear error.

This design intentionally does not add manual fallback fields like:

- `authorize_url`
- `token_url`

Those can be introduced later only if a real deployment requires them.

This keeps the first configuration model smaller and avoids blending OIDC-first and manual OAuth-style configuration too early.

## OIDC Validation Rules

OIDC should be considered enabled only when `oidc.enabled = true`.

When enabled, the following fields are required:

- `issuer`
- `client_id`
- `client_secret`
- `redirect_uri`
- `state_signing_secret`

If any required field is missing, startup must fail with a specific message naming the missing field.

If OIDC is disabled:

- missing OIDC fields should not fail startup
- OIDC login routes may return a clear unavailable response, or may remain unreachable depending on route-handling choice during implementation

## Route Naming

Since a Gateway instance supports only one configured issuer, auth routes should no longer embed provider names.

Recommended route names:

- `GET /auth/login`
- `GET /auth/callback`

Rationale:

- the active issuer is chosen by configuration, not by path
- paths remain stable if the deployment switches from GitLab to another OIDC provider
- route naming matches the single-issuer runtime model

## Frontend Redirect Model

The protocol callback remains Gateway-owned:

- `GET /auth/callback`

After validating the callback state, the Gateway may continue redirecting the browser to the frontend result route:

- `/auth/callback/oidc?result=success`

The browser-facing frontend route can remain separate from the protocol route if that simplifies SPA behavior.

The important constraint is:

- the external OIDC redirect URI is Gateway-owned
- the SPA route is a post-validation result page

## Internal Code Structure

The implementation should separate raw source loading from validated runtime configuration.

Recommended split:

- `src/config.rs`
  - public validated runtime `Config`
  - top-level load entrypoint
- `src/config/raw.rs`
  - deserializable raw config structures
- `src/config/sources.rs`
  - load TOML, `.env`, and environment values
  - apply precedence rules
- `src/oidc/mod.rs`
  - OIDC-specific logic such as state issuance, state validation, issuer metadata handling, and authorize URL generation

The current `src/oauth/mod.rs` should be renamed and refactored into OIDC terminology.

## Raw Configuration Types

The raw configuration layer should model optional values from all sources.

Suggested shape:

- `RawConfig`
- `RawServerConfig`
- `RawGitConfig`
- `RawOidcConfig`

This layer should allow partial values so precedence merging can happen before validation.

Validation belongs in the final conversion into runtime `Config`.

## Runtime Configuration Types

The validated runtime configuration should contain only final values the application can trust.

Suggested shape:

- `Config`
  - `listen_addr`
  - `git`
  - `oidc`
- `GitConfig`
  - `webhook_secret`
- `OidcConfig`
  - `enabled`
  - `issuer`
  - `client_id`
  - `client_secret`
  - `redirect_uri`
  - `scopes`
  - `state_signing_secret`
  - discovered metadata as needed

This keeps route handlers and protocol logic away from partially-populated source data.

## Loading Strategy

The loader should:

1. start from built-in defaults
2. merge `config/issueflow.toml` if present
3. merge `.env` values if present
4. merge process environment variables
5. validate and build runtime `Config`

This should happen once at startup.

## Error Handling

Configuration failures should be explicit and actionable.

Examples:

- missing required field when OIDC is enabled
- invalid boolean value in `OIDC_ENABLED`
- invalid issuer URL
- malformed TOML
- discovery request failure

Errors should name:

- which source or stage failed
- which field is invalid or missing
- whether the failure happened during loading, merging, validation, or OIDC discovery

## Testing Strategy

The implementation should add focused tests in three groups.

### 1. Source Precedence Tests

Verify:

- defaults apply when no source provides a value
- TOML populates values
- `.env` overrides TOML field-by-field
- process environment variables override both

### 2. Validation Tests

Verify:

- OIDC disabled allows missing OIDC fields
- OIDC enabled requires all mandatory fields
- invalid `OIDC_ENABLED` values fail
- env string scopes parse correctly
- TOML array scopes parse correctly

### 3. OIDC Route and Flow Tests

Verify:

- `/auth/login` redirects using the discovered authorization endpoint
- `/auth/callback` validates signed state
- successful callback redirects to the frontend result page
- current SPA routes remain unaffected

## Documentation Deliverables

Add a dedicated configuration reference document:

- `docs/configuration.md`

This document should include:

- configuration source locations
- precedence rules
- configuration field reference
- OIDC-only login model
- local development example
- `config/issueflow.toml` example
- `.env` example

README should stop carrying the full configuration reference inline.

Instead, it should add a short configuration section that links to:

- `docs/configuration.md`

## Migration Impact

This is a deliberate cleanup, not a compatibility-preserving incremental rename.

The implementation may break old environment variable names and old GitLab-specific auth route naming.

That is acceptable because:

- the current system is still early-stage
- configuration redesign is easier now than after more integrations accumulate
- the new OIDC model is simpler and more durable

## Acceptance Criteria

The design is satisfied when the implementation provides all of the following:

- configuration loading supports `config/issueflow.toml`, `.env`, and environment variables
- precedence is `env > .env > toml > defaults`
- OIDC replaces OAuth terminology in configuration and code structure
- a Gateway instance supports exactly one configured OIDC issuer
- auth routes use `/auth/login` and `/auth/callback`
- OIDC discovery uses the configured issuer
- missing required OIDC config fails startup only when OIDC is enabled
- configuration reference lives in `docs/configuration.md`
- `README.md` links to the dedicated configuration document
