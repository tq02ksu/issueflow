# issueflow

Chat-driven issue management agent.

## How it works

1. You describe a need in chat.
2. The agent understands it, fills in missing context, and shapes it into a structured issue draft.
3. You confirm, and the agent writes it into GitLab.
4. The agent continues to advance the issue — triage, validation, development kickoff — guided by `skills`.

## Skills drive the agent

`skills` define *how* the agent handles issues. Different projects have different conventions — `skills` capture those conventions in version-controlled Git repositories so the agent adapts.

- A **project skill repo** hosts project-level `skills`, issues, docs, repo maps, and UI demos.
- The **platform skill repo** provides system-wide defaults that apply when a project does not define its own.

This means the agent's behavior is transparent, auditable, and evolves through normal Git workflows — review, diff, merge, rollback.

## Quick start

```bash
# 1. Build the frontend
cd web && npm install && npm run build && cd ..
```

```bash
# 2. Start the gateway
GIT_WEBHOOK_SECRET=local-dev-secret cargo run
```

```bash
# 3. Verify
curl http://127.0.0.1:8080/api/status/ping
# → ok
```

The gateway listens on `127.0.0.1:8080` by default. OIDC is disabled unless explicitly configured.

## Configuration

See [docs/CONFIG.md](docs/CONFIG.md) for the full configuration reference.

## Local development

See [docs/local-development.md](docs/local-development.md) for a complete local development guide including OIDC setup, GitLab webhook testing, and chat-driven issue creation.

## Docker

```bash
docker build -t issueflow .
docker run -p 8080:8080 -e GIT_WEBHOOK_SECRET=local-dev-secret issueflow
```

## How skills guide handling

`issueflow` does not assume every issue should be handled the same way. Different business domains, team conventions, and repository habits can guide the agent through Git-hosted `skills` that define how issues should be understood and advanced.

The repository that owns the issue is the preferred source of handling guidance. When suitable `skills` exist there, they override platform defaults. When they don't, the platform defaults apply.

## Architecture

See [docs/DESIGN.md](docs/DESIGN.md) for the system architecture, design goals, security model, and phase-based permission control.
