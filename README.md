# issueflow

AI coding project execution for `Codex`, `OpenCode`, and GitLab workflows.

## What it is

`issueflow` is an artifact advancement system for AI coding workflows.
It focuses on moving project artifacts forward through explicit state, structured memory, `skills`, and agent coordination.

It is built to work around GitLab artifacts such as `issues`, `milestones`, `labels`, and `merge requests`, and to help teams push work from vague request to planned execution and controlled delivery.

## What problem it solves

Many AI coding workflows do not fail at code generation. They fail because project artifacts do not move well: issues are weak, standards are unclear, context is fragmented, and execution handoff happens too early or too loosely.

`issueflow` is meant to solve that advancement problem. It gives AI coding workflows a control layer for artifact state, project memory, stakeholder context, and execution coordination.

## Core mechanism

The stable core is the artifact state machine.
Artifacts move through explicit states instead of depending on implicit chat progress.

Light agents plus `skills` handle most advancement work: understanding context, structuring project memory, coordinating roles, deciding next steps, and preparing controlled actions.

Heavy execution can then be delegated when needed.
`issueflow` is not `OpenCode` or `Codex`; it is the system that decides how and when artifacts should be advanced, and when heavier execution should be handed off.

## Why it matters

This makes AI coding workflows more controllable, reviewable, and reusable.
Instead of treating project execution as one long prompt, `issueflow` treats artifact advancement as a structured system.

## Why contribute

If you care about any of these areas, you should probably take a look:

- AI coding collaboration
- GitLab-centered engineering workflows
- issue quality and execution standards
- engineering memory and context persistence
- testing and validation support for teams without dedicated QA
- project advancement automation beyond code generation

More people working on this problem will make the project better.
If you are trying to solve the gap between "AI can write code" and "the project still does not move well", this project is for you.

## Skills and execution model

`skills` are part of how `issueflow` approaches project execution.
The goal is to make methods, standards, and operating habits more visible and versioned instead of hiding them in ad hoc prompts.

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

See [docs/local-development.md](docs/local-development.md) for a complete local development guide including OIDC setup, GitLab webhook testing, and AI-assisted GitLab workflow execution.

## Docker

```bash
docker build -t issueflow .
docker run -p 8080:8080 -e GIT_WEBHOOK_SECRET=local-dev-secret issueflow
```

## Architecture

See [docs/DESIGN.md](docs/DESIGN.md) for the system architecture, design goals, security model, and phase-based permission control.
