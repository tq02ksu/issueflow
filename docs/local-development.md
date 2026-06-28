# Local Development

This document describes the full local development environment for `issueflow`.

## Architecture Notes

- The Rust gateway is the local entrypoint.
- By default it listens on `127.0.0.1:8080`.
- The gateway serves the frontend shell and static assets from `web/dist/assets`.
- OIDC is disabled by default and only activates when explicitly configured.
- GitLab webhook requests enter through `POST /api/webhooks/gitlab` and require `x-gitlab-token` to match `git.webhook_secret`.

## Prerequisites

- Rust and Cargo
- Node.js and npm
- Optional: a local or shared OIDC provider for login testing
- Optional: a GitLab project or webhook sender for webhook testing

## Configuration Sources

Configuration loads in this order, with later sources overriding earlier ones:

1. Process environment variables
2. Project root `.env`
3. `config/issueflow.toml`
4. Built-in defaults

Key built-in defaults:

- `server.listen_addr = "127.0.0.1:8080"`
- `oidc.enabled = false`

The one value you must provide for local startup is `git.webhook_secret`, usually through `GIT_WEBHOOK_SECRET`.

## Recommended Local Config File

Create `config/issueflow.toml` for repeatable local development:

```toml
[server]
listen_addr = "127.0.0.1:8080"

[git]
webhook_secret = "local-dev-secret"

[oidc]
enabled = false
```

You can also start without a TOML file by exporting `GIT_WEBHOOK_SECRET` directly.

## Build The Frontend Assets

The gateway expects built assets at `web/dist/assets`, so build the frontend before opening the app shell:

```bash
cd web
npm install
npm run build
```

The Vite build writes `web/dist/assets/app.js` and `web/dist/assets/app.css`, which are referenced by `internal/pages/templates/app.html`.

## Minimal Gateway Startup

From the repository root:

```bash
GIT_WEBHOOK_SECRET=local-dev-secret PATH="$HOME/.cargo/bin:$PATH" cargo run
```

If you use `config/issueflow.toml` with `[git].webhook_secret` set, you can start with:

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo run
```

## Verify Local Startup

Check the health endpoint:

```bash
curl http://127.0.0.1:8080/api/status/ping
```

Expected response:

```text
ok
```

Useful local routes:

- `/`
- `/workbench`
- `/api/status/ping`
- `/api/auth/login`
- `/api/auth/callback`
- `/auth/callback/oidc`
- `/api/webhooks/gitlab`
- `/api/issues`

With OIDC disabled, `/api/auth/login` and `/api/auth/callback` return `503 Service Unavailable`.

## OIDC Local Development

Enable OIDC only when you need to test login behavior.

Example `config/issueflow.toml`:

```toml
[server]
listen_addr = "127.0.0.1:8080"

[git]
webhook_secret = "local-dev-secret"

[oidc]
enabled = true
issuer = "https://gitlab.com"
client_id = "replace-me"
client_secret = "replace-me"
redirect_uri = "http://127.0.0.1:8080/api/auth/callback"
state_signing_secret = "replace-me"
scopes = ["openid", "profile", "email"]
```

OIDC notes:

- Configure the identity provider redirect URI as `http://127.0.0.1:8080/api/auth/callback`.
- Do not configure `/auth/callback/oidc` at the provider.
- The gateway discovers metadata from `<issuer>/.well-known/openid-configuration`.

## GitLab Webhook Local Testing

The webhook endpoint is:

```text
POST /api/webhooks/gitlab
```

Required header:

```text
x-gitlab-token: <your webhook secret>
```

Minimal example with a note webhook body:

```bash
curl -X POST http://127.0.0.1:8080/api/webhooks/gitlab \
  -H "content-type: application/json" \
  -H "x-gitlab-token: local-dev-secret" \
  -d '{
    "object_kind": "note",
    "object_attributes": {
      "note": "/start-dev",
      "noteable_type": "Issue"
    }
  }'
```

Expected response status:

```text
202 Accepted
```

If the token does not match, the gateway returns `401 Unauthorized`.

## GitLab-Backed Issue Creation

The Gateway supports chat-driven GitLab issue creation. The flow is:

1. A chat session generates a structured issue draft in the workbench.
2. The user confirms the draft.
3. The Gateway creates the final issue in GitLab using the logged-in user's session access token.

### Required Configuration

GitLab issue creation requires the GitLab base URL to be configured, and the user must be logged in through OIDC so the session contains a GitLab access token.

| Variable | TOML | Purpose |
| --- | --- | --- |
| `GIT_BASE_URL` | `git.base_url` | GitLab instance URL (e.g. `https://gitlab.com`) |

Example TOML entry:

```toml
[git]
webhook_secret = "local-dev-secret"
base_url = "https://gitlab.com"
```

### Testing Issue Creation

After startup with valid GitLab base URL config and an authenticated session, test the endpoint directly:

```bash
curl -s -X POST http://127.0.0.1:8080/api/issues \
  -H "content-type: application/json" \
  -H "authorization: Bearer <issueflow-session-jwt>" \
  -d '{"project_id": 123, "title": "Test issue from issueflow", "description": "Created via dev test"}'
```

A successful response returns `201` with the created issue identity:

```json
{
  "id": 456,
  "iid": 12,
  "project_id": 123,
  "title": "Test issue from issueflow",
  "web_url": "https://gitlab.com/group/project/-/issues/12"
}
```

Invalid payloads (e.g. empty title) return `400`. Missing authentication returns `401`. Missing GitLab base URL configuration returns `500`.

## Common Development Commands

Rust gateway:

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo test
```

Focused Rust test example:

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo test status_route_returns_ok -- --exact
```

Frontend:

```bash
cd web
npm test
```

```bash
cd web
npm run lint
```

```bash
cd web
npm run typecheck
```
