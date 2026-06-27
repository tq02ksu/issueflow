# Changelog

## [0.1.0] - 2025-06-27

### Added

- Rust Gateway with axum HTTP server
- OIDC login flow (`/auth/login`, `/auth/callback`)
- GitLab webhook endpoint (`POST /webhooks/gitlab`)
- Chat-driven issue creation API (`POST /api/issues`)
- Vue 3 + Naive UI Agent Workbench frontend
- Issue/MR/Release state machines with phase-gated permissions
- Dockerfile for multi-stage container build
- GitLab CI integration template (`scripts/robot/integrations/gitlab-ci/`)
- Configuration loading from env, `.env`, `config/issueflow.toml`
