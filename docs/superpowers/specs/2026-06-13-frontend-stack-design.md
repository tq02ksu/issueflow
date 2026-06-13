# Frontend Stack Design

## Goal

Add a complete frontend application skeleton to `issueflow` using `Vue 3 + Naive UI`, while keeping development ergonomics high and preserving the Rust Gateway as the security and protocol boundary.

The first implementation should establish:

- a standalone `web/` frontend workspace
- Gateway-served SPA delivery for integrated access
- a minimal OAuth-oriented user flow
- routing, state, testing, linting, and build foundations for the future Agent Workbench

## Scope

This design covers two different things on purpose:

1. the long-term Gateway surface and authentication classification that should be fixed early
2. the smaller first implementation that only builds the frontend skeleton and a minimal browser flow

The initial implementation should:

- add a `web/` application based on `Vite`
- use `Vue 3`, `TypeScript`, `Vue Router`, `Pinia`, and `Naive UI`
- serve the built SPA from the Rust Gateway at `/`, with `/index.html` as a compatible alias
- keep frontend development independent through a local Vite dev server
- preserve Gateway ownership of OAuth redirect issuance and callback validation
- implement only three browser views:
  - landing page
  - OAuth callback result page
  - minimal workbench shell
- include only one workbench menu item: `Overview`
- add basic frontend test and lint infrastructure
- add basic Rust tests for the new SPA entry behavior and OAuth redirect behavior

## Non-Goals

The first implementation should not:

- implement the full workbench information architecture
- implement complete browser business APIs under `/api/*`
- implement a full authenticated session model with durable user identity
- implement the full internal agent API surface
- replace lightweight Gateway confirmation pages with SPA flows
- introduce SSR, Nuxt, or a separate frontend deployment target
- over-design styling, data fetching, or plugin systems before they are needed

## Product and Architecture Constraints

This design must stay aligned with current repository constraints:

- Gateway pages and protocol handling remain in Rust
- OAuth validation stays in the Gateway
- the future Agent Workbench uses `Vue 3 + Naive UI`
- lightweight Gateway pages remain separate from the future workbench frontend
- the first change should be the smallest correct step toward that workbench

## Long-Term Gateway Surface Classification

The repository needs a stable route taxonomy early so frontend and Gateway work do not drift into mixed responsibilities.

### 1. Web Entrypoints

Purpose:

- serve browser HTML and static assets

Routes:

- `GET /`
- `GET /index.html`
- `GET /assets/*`

Source:

- browser

Authentication:

- static entry assets may be public
- protected data must be loaded from authenticated APIs, not embedded into the shell

### 2. Browser APIs

Purpose:

- JSON APIs used by the SPA

Routes:

- `/api/*`

Source:

- browser SPA

Authentication:

- browser session, preferably via `HttpOnly` cookie

Notes:

- this namespace is part of the design now, but full implementation is out of scope for the first frontend skeleton

### 3. Protocol Entrypoints

Purpose:

- support browser-visible protocol flows that are not general business APIs

Routes:

- `GET /auth/{provider}/login`
- `GET /auth/{provider}/callback`
- `GET /confirm/plan/{token}`

Source:

- browser
- OAuth provider redirects
- signed confirmation links

Authentication:

- OAuth uses signed `state`
- confirmation uses signed or one-time tokens
- these routes do not rely on a normal browser API session as their primary guard

### 4. External Callbacks

Purpose:

- receive third-party system callbacks

Routes:

- `/hooks/*`

Source:

- GitLab and future external platforms

Authentication:

- webhook secret, request signature, or equivalent platform verification

Notes:

- the current repository already exposes `/webhooks/gitlab`
- first implementation may keep compatibility while moving the design target toward `/hooks/gitlab`

### 5. Agent Internal APIs

Purpose:

- receive execution updates and requests from trusted-but-untrusted-by-default runtime environments

Routes:

- `/internal/agent/*`

Source:

- `GitLab CI + OpenCode` in the first execution model

Authentication:

- short-lived execution token, CI job token, request signature, and session correlation

Rules:

- agent callers can report state, artifacts, heartbeats, and action requests
- agent callers do not directly receive broad platform write authority
- high-risk external writes still pass through Gateway policy and workflow checks

Notes:

- this namespace must be documented now
- the first frontend-stack implementation only reserves the naming and architecture direction

### 6. Ops Endpoints

Purpose:

- expose liveness and readiness information

Routes:

- `GET /healthz`
- `GET /readyz`

Compatibility:

- existing `GET /status/ping` may remain as a compatibility endpoint

Source:

- load balancer
- monitoring
- orchestration systems

Authentication:

- usually none, but response shape must remain minimal

## Initial Implementation Architecture

The first implementation should use a split workflow:

- **development mode:** frontend runs as an independent Vite app with a dev proxy to the Rust Gateway
- **integrated mode:** built frontend assets are served by the Rust Gateway

This keeps frontend iteration fast without giving up the desired integrated product entry point.

## Frontend Workspace Structure

The repository should add a new `web/` application with focused responsibilities.

Expected structure:

- `web/package.json`
- `web/index.html`
- `web/tsconfig.json`
- `web/tsconfig.app.json`
- `web/tsconfig.node.json`
- `web/vite.config.ts`
- `web/vitest.config.ts`
- `web/eslint.config.js` or equivalent flat config
- `web/src/main.ts`
- `web/src/App.vue`
- `web/src/router/index.ts`
- `web/src/stores/session.ts`
- `web/src/api/`
- `web/src/components/layout/AppShell.vue`
- `web/src/views/LandingView.vue`
- `web/src/views/OAuthCallbackView.vue`
- `web/src/views/WorkbenchView.vue`
- `web/src/styles/tokens.css`
- `web/src/styles/main.css`
- `web/src/tests/` or colocated component/view tests

Responsibilities:

- router: browser route ownership
- stores: minimal session and callback state
- api: browser-facing fetch wrapper and future `/api/*` integration point
- views: route-level screens
- layout: workbench shell
- styles: design tokens and global styling on top of Naive UI

## Frontend Technology Stack

The initial frontend stack should be:

- `Vue 3`
- `TypeScript`
- `Vite`
- `Vue Router`
- `Pinia`
- `Naive UI`
- `@vueuse/core`
- `Vitest`
- `@vue/test-utils`
- `ESLint`
- `Prettier`

The first implementation should avoid additional weight unless a concrete need appears.

Specifically defer:

- SSR
- Nuxt
- end-to-end browser automation
- i18n frameworks
- complex query/cache frameworks

## Browser Route Design

The first browser-visible routes should stay minimal.

### `GET /`

Purpose:

- product entry page and login entry

Behavior:

- render a lightweight branded landing page
- provide a primary action that sends the browser to `GET /auth/gitlab/login`
- avoid pretending that full user session data already exists

### Frontend Route `/auth/callback/gitlab`

Purpose:

- render the post-OAuth result view for users

Behavior:

- show a success or failure state based on Gateway redirect parameters
- on success, move the user to `/workbench`
- keep displayed error information minimal and safe

### Frontend Route `/workbench`

Purpose:

- provide the first Agent Workbench shell

Behavior:

- render a top-level app shell
- include a single navigation item: `Overview`
- show introductory workbench content instead of fake domain data

## OAuth Flow Design

The Gateway remains the protocol owner for OAuth.

### Login Flow

1. User visits `/`
2. User clicks the GitLab login button
3. Browser requests `GET /auth/gitlab/login`
4. Gateway issues OAuth `state` and redirects to GitLab
5. GitLab redirects the browser to `GET /auth/gitlab/callback`
6. Gateway validates provider, callback parameters, and signed `state`
7. Gateway redirects the browser to the frontend callback route

### Redirect Target

The Gateway should redirect to:

- `/auth/callback/gitlab?result=success` on success
- `/auth/callback/gitlab?result=error&reason=<safe-code>` on failure when a user-facing redirect is appropriate

Safe failure codes may include:

- `invalid_state`
- `missing_code`
- `provider_not_found`

The Gateway must not reflect sensitive OAuth inputs back into the browser page.

## Gateway SPA Integration

The Rust Gateway should add a dedicated SPA-serving path instead of relying on the old lightweight HTML templates for frontend pages.

Required behavior:

- `GET /` serves the SPA container HTML
- `GET /index.html` serves the same SPA container or compatible built file
- `GET /assets/*` serves built static assets
- browser routes intended for the SPA should fall back to the SPA container

The integration should be honest about scope:

- lightweight confirmation pages may stay server-rendered
- OAuth protocol routes stay server-owned
- the SPA is an application shell layered on top of the Gateway, not a replacement for Gateway policy logic

## Development Workflow

The development experience must stay easy.

### Frontend Development

- run the frontend with a local Vite dev server
- configure the Vite dev server to proxy Gateway-owned routes such as:
  - `/auth/*`
  - future `/api/*`
  - future `/internal/*` only when explicitly needed for local integration work

### Integrated Verification

- build frontend assets locally
- run the Rust Gateway
- verify that the built SPA is served from `/`

This gives developers both fast iteration and integrated delivery validation.

## UI Direction

The first implementation should stay simple and intentional.

Requirements:

- use Naive UI components, but not the untouched default look
- define a small set of CSS tokens for brand colors, surfaces, spacing, and typography
- keep the landing page and workbench shell visually coherent
- avoid building a large visual system before there are real product modules

## Testing Strategy

The first implementation should add focused tests only.

### Frontend Tests

At minimum:

- one router or app render test
- one OAuth callback result view test

These tests should validate basic rendering and route behavior without inventing backend data dependencies.

### Rust Tests

At minimum:

- one test verifying the SPA entry route returns success
- one test verifying successful OAuth callback handling redirects to the frontend callback route

If static assets are served through a dedicated handler, add a focused test for that handler only if needed to protect routing behavior.

## Implementation Boundaries

The first code change should only establish the skeleton required for future workbench development.

That means:

- one menu item
- minimal session store shape
- minimal callback-state handling
- no speculative domain modules
- no premature API client complexity

This is intentionally a foundation change, not a full product slice.

## Acceptance Criteria

The design is satisfied when the first implementation delivers all of the following:

- a `web/` frontend application exists in the repository
- the frontend stack uses `Vue 3 + Naive UI`
- developers can run the frontend independently during development
- the Rust Gateway can serve the built SPA from `/`
- the OAuth login route still starts in the Gateway
- the OAuth callback is validated in the Gateway and redirects to the frontend callback view
- the frontend includes a landing page, a callback page, and a minimal workbench shell
- the workbench contains exactly one initial menu item: `Overview`
- the design document clearly separates long-term API taxonomy from first-implementation scope
