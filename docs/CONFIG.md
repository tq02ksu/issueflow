# Gateway Configuration

The `issueflow` Gateway uses a single-instance, single-OIDC-issuer login model.

## Configuration Sources

The Gateway loads configuration in the following order, with later sources overriding earlier ones:

1. Built-in defaults
2. `config/issueflow.toml`
3. Project root `.env`
4. Process environment variables

Built-in defaults:

| Field | Default |
|---|---|
| `server.listen_addr` | `127.0.0.1:8080` |
| `oidc.enabled` | `false` |
| `oidc.scopes` | `["openid", "profile", "email"]` |

## TOML Configuration

Place local development config in `config/issueflow.toml`:

```toml
[server]
listen_addr = "127.0.0.1:8080"

[git]
webhook_secret = "replace-me"

[oidc]
enabled = true
issuer = "https://gitlab.com"
client_id = "replace-me"
client_secret = "replace-me"
redirect_uri = "http://127.0.0.1:8080/api/auth/callback"
state_signing_secret = "replace-me"
scopes = ["openid", "profile", "email"]
```

When `oidc.enabled = false`, the remaining `oidc.*` fields may be omitted.

## Environment Variables

TOML field to environment variable mapping:

| TOML | Environment Variable | Required |
|---|---|---|
| `server.listen_addr` | `LISTEN_ADDR` | No |
| `git.webhook_secret` | `GIT_WEBHOOK_SECRET` | Yes |
| `git.base_url` | `GIT_BASE_URL` | For issue creation |
| `git.token` | `GIT_TOKEN` | For issue creation |
| `oidc.enabled` | `OIDC_ENABLED` | No |
| `oidc.issuer` | `OIDC_ISSUER` | When OIDC enabled |
| `oidc.client_id` | `OIDC_CLIENT_ID` | When OIDC enabled |
| `oidc.client_secret` | `OIDC_CLIENT_SECRET` | When OIDC enabled |
| `oidc.redirect_uri` | `OIDC_REDIRECT_URI` | When OIDC enabled |
| `oidc.scopes` | `OIDC_SCOPES` | No |
| `oidc.state_signing_secret` | `OIDC_STATE_SIGNING_SECRET` | When OIDC enabled |

`OIDC_SCOPES` uses space separation:

```text
OIDC_SCOPES=openid profile email
```

## OIDC Conventions

| Route | Purpose |
|---|---|
| `GET /api/auth/login` | Gateway OIDC entry point |
| `GET /api/auth/callback` | OIDC callback endpoint |
| `GET /auth/callback/oidc` | Frontend result page (do not configure at the provider) |

Configure the OIDC provider's Redirect URI as `<gateway-base-url>/api/auth/callback`.

## Discovery

The Gateway discovers OIDC metadata from the issuer's well-known endpoint:

```text
<issuer>/.well-known/openid-configuration
```

`OIDC_ISSUER` / `oidc.issuer` must point to an issuer that supports standard OIDC discovery. The current primary path is GitLab, but the configuration model is not bound to GitLab-specific fields.
