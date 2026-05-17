# OpenAPI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a handoff-ready `openapi.yaml` at the repository root that documents both implemented and near-term planned Robot Gateway routes.

**Architecture:** Keep the first version as a single handwritten contract file. Represent current and planned endpoints together, but require every operation to carry an explicit implementation-status extension so the contract stays honest.

**Tech Stack:** OpenAPI 3.1.0, YAML

---

## File Structure

- Create: `openapi.yaml`
- Reference: `docs/superpowers/specs/2026-05-17-openapi-design.md`

### Task 1: Add Root OpenAPI Contract

**Files:**
- Create: `openapi.yaml`
- Reference: `docs/superpowers/specs/2026-05-17-openapi-design.md`

- [ ] **Step 1: Write the contract content**

```yaml
openapi: 3.1.0
info:
  title: Issueflow Robot Gateway API
  version: 0.1.0
  description: |
    Gateway contract for the Issueflow MVP.
    This file includes both implemented and planned operations.
    Each operation declares its rollout state with `x-issueflow-status`.
tags:
  - name: Status
  - name: Webhooks
  - name: Confirmations
paths:
  /status/ping:
    get:
      tags: [Status]
      summary: Ping the gateway
      description: Lightweight liveness endpoint for the Rust Gateway bootstrap.
      x-issueflow-status: implemented
      responses:
        '200':
          description: Gateway is reachable.
          content:
            text/plain:
              schema:
                type: string
                const: ok
              examples:
                default:
                  value: ok
  /webhooks/gitlab:
    post:
      tags: [Webhooks]
      summary: Receive a GitLab webhook
      description: Accepts GitLab webhook events for Gateway workflow handling.
      x-issueflow-status: planned
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/GitlabWebhook'
      responses:
        '202':
          description: Webhook accepted for processing.
        '401':
          description: Webhook token validation failed.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
  /confirm/plan/{token}:
    get:
      tags: [Confirmations]
      summary: Render the plan confirmation page
      description: Serves a lightweight HTML page for plan confirmation status.
      x-issueflow-status: planned
      parameters:
        - name: token
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: Confirmation page rendered.
          content:
            text/html:
              schema:
                type: string
  /confirm/release/{token}:
    get:
      tags: [Confirmations]
      summary: Render the release confirmation page
      description: Serves a lightweight HTML page for release confirmation status.
      x-issueflow-status: planned
      parameters:
        - name: token
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: Release confirmation page rendered.
          content:
            text/html:
              schema:
                type: string
  /status/session/{session_id}:
    get:
      tags: [Status]
      summary: Render a lightweight session status page
      description: Serves a lightweight HTML status page for a tracked workflow session.
      x-issueflow-status: planned
      parameters:
        - name: session_id
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: Session status page rendered.
          content:
            text/html:
              schema:
                type: string
components:
  schemas:
    GitlabWebhook:
      type: object
      required:
        - object_kind
      properties:
        object_kind:
          type: string
        object_attributes:
          type: object
          properties:
            note:
              type: string
            noteable_type:
              type: string
    ErrorResponse:
      type: object
      required:
        - message
      properties:
        message:
          type: string
```

- [ ] **Step 2: Create `openapi.yaml` with the approved content**

Use `apply_patch` to add `openapi.yaml` exactly as specified in Step 1.

- [ ] **Step 3: Verify the file contents**

Run: `Read /home/tq02ksu/workspace/tq02ksu/issueflow/openapi.yaml`
Expected: the file exists at repository root, uses OpenAPI `3.1.0`, and marks every operation with `x-issueflow-status`.

- [ ] **Step 4: Commit**

```bash
git add openapi.yaml docs/superpowers/plans/2026-05-17-openapi-implementation.md
git commit -m "docs: add gateway openapi contract"
```
