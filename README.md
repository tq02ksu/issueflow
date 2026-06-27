# issueflow

## Overview

`issueflow` is an agent for managing and advancing issues and related GitLab collaboration objects such as milestones and wiki pages. It handles them through standardized, industry-common processes and adapts to different business domains through `skills` stored in Git repositories.

## Core Process

- Triage: identify the type, priority, and current state of an issue or related GitLab object.
- Information completion: detect missing context and drive it to completion.
- Solution confirmation: align on goals, constraints, and the handling approach.
- Development kickoff: move implementation forward once entry conditions are met.
- Result write-back: record outcomes, status, and key conclusions back into the handling process.

## How Skills Guide Handling

`issueflow` does not assume every issue, milestone, or wiki task should be handled the same way. Different business domains, team conventions, and repository habits can guide the agent through Git-hosted `skills` that define how these items should be understood and advanced.

The repository that owns the issue or related GitLab object is the preferred source of handling guidance. If that repository provides relevant `skills`, the agent follows them first.

## Defaults and Overrides

The platform provides a default, general-purpose handling approach as the baseline behavior when no business-specific customization is present.

If the repository that owns the issue or related GitLab object does not provide matching `skills`, `issueflow` falls back to the default approach. When suitable repository-local `skills` exist, they override the default behavior and define the concrete handling rules for that kind of work.

This model keeps the handling skeleton consistent while allowing different businesses to preserve their own handling styles within the same system.

## Local Development

Minimal local startup requires Rust, Node.js, and a Git webhook secret.

1. Build the frontend assets:

```bash
cd web
npm install
npm run build
```

2. Start the gateway with a local webhook secret:

```bash
GIT_WEBHOOK_SECRET=local-dev-secret PATH="$HOME/.cargo/bin:$PATH" cargo run
```

By default, the service listens on `127.0.0.1:8080` and OIDC is disabled unless you explicitly configure it.

3. Verify the server is up:

```bash
curl http://127.0.0.1:8080/status/ping
```

Expected response:

```text
ok
```

For the full local development environment, including `config/issueflow.toml`, OIDC, GitLab webhook testing, chat-driven GitLab issue creation, and common test commands, see [`docs/local-development.md`](docs/local-development.md).

### Docker

```bash
docker build -t issueflow .
docker run -p 8080:8080 -e GIT_WEBHOOK_SECRET=local-dev-secret issueflow
```
