# Workbench Management — Design Spec

## Overview

Add workbench management: users bind GitLab projects to workbenches, switch between them, and manage issues within each. The binding is mutable (can be changed or removed).

## In Scope

- OIDC token exchange (auth code → access token)
- Session cookie for holding the GitLab access token
- SQLite database for workbench persistence (sqlx, PostgreSQL-compatible)
- Gateway API for workbench CRUD and GitLab project search
- Frontend dropdown + search dialog for workbench selection/management

## Out of Scope

- Issue management within a workbench (UI placeholders only, backend logic later)
- Agent session listing within a workbench (UI placeholders only)
- PostgreSQL in production (schema compatible, but only SQLite wired for now)
- Group-level workbench binding (groups are display-only in search)

## Database Schema

```sql
CREATE TABLE IF NOT EXISTS workbenches (
  id           INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id      TEXT NOT NULL,
  project_id   INTEGER NOT NULL,
  project_name TEXT NOT NULL,
  project_path TEXT NOT NULL,
  created_at   TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at   TEXT NOT NULL DEFAULT (datetime('now')),
  UNIQUE(user_id, project_id)
);
```

- `user_id` from OIDC `sub` claim
- `project_id` from GitLab project ID
- `project_path` cached full path for display (e.g. `my-group/backend`)

## OIDC Token Exchange

Current `oidc_callback` only validates state. Change to:

1. Receive auth code → POST to GitLab `/oauth/token` with client_id, client_secret, code, redirect_uri, grant_type=authorization_code
2. Parse response: `access_token`, `id_token` (JWT with `sub` claim)
3. Store access_token in an HMAC-signed session cookie
4. Parse `sub` from id_token for user identification
5. Redirect to `/auth/callback/oidc?result=success`

**Session cookie**: `session=<base64(claims.token.signature)>`, HttpOnly, SameSite=Lax, Path=/

## Gateway API

All endpoints require a valid session cookie. Gateway extracts the access token from the cookie and uses it to call GitLab API on the user's behalf.

| Method | Path | Auth | Body / Query | Response |
|---|---|---|---|---|
| `GET` | `/api/workbenches` | session | — | `[{id, project_id, project_name, project_path, created_at}]` |
| `POST` | `/api/workbenches` | session | `{project_id, project_name, project_path}` | `{id, ...}` (201) |
| `PUT` | `/api/workbenches/{id}` | session | `{project_id, project_name, project_path}` | `{id, ...}` |
| `DELETE` | `/api/workbenches/{id}` | session | — | 204 |
| `GET` | `/api/projects?search=xxx` | session | query param | GitLab API proxy: `[{id, name, path_with_namespace, namespace}]` |

## Frontend UI

### Dropdown (WorkbenchView)

Replaces the current static header. Shows current workbench name with chevron. Dropdown items:

- Existing workbenches (selected one marked)
- Separator
- "＋ Add workbench..."

### Search Dialog

Triggered by `+` button or dropdown "Add" item. Modal with:

- Search input (debounced 300ms)
- Results list: groups as grey non-clickable headers, projects as clickable rows
- Click a project → creates/updates workbench, closes dialog

### Fallback

If user has zero workbenches: show the search dialog immediately on entering `/workbench`.

## Component Tree

```
WorkbenchView
├── WorkbenchSwitcher (dropdown: current workbench + list + add)
├── WorkbenchSearchDialog (modal: search → pick project)
├── IssuePanel (placeholder: "Issues for <project_path>")
└── SessionPanel (placeholder: "Agent sessions for <project_path>")
```

## Tech Decisions

| Concern | Choice |
|---|---|
| Database | `sqlx` with `sqlite` feature, migration at startup |
| Session | HMAC-SHA256 signed cookie, no server-side session store |
| GitLab proxy | Direct `reqwest` call from gateway using user's access token |
| OIDC discovery | Already lazy-loaded, token endpoint from metadata |

## Database Migration Strategy

Gateway runs `CREATE TABLE IF NOT EXISTS` on startup. No migration framework for now (single table). PostgreSQL compatibility: use standard SQL only, avoid SQLite-specific syntax.

## Testing Strategy

- Gateway: integration tests for each new endpoint (with in-memory SQLite)
- Session middleware: unit tests for cookie sign/verify round-trip
- OIDC token exchange: integration test with a mock token endpoint
- Frontend: Vitest tests for dropdown/dialog components
