# Frontend Stack Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a `Vue 3 + Naive UI` frontend skeleton under `web/`, serve the integrated SPA from the Rust Gateway root path, and redirect successful GitLab OAuth callbacks into the frontend callback route.

**Architecture:** The frontend lives in a standalone Vite workspace for local development and testing, but the Rust Gateway remains the security and protocol owner. Gateway serves a source-controlled SPA shell at `/`, exposes built static assets from `web/dist/assets`, and redirects successful OAuth callbacks to the frontend route `/auth/callback/gitlab`.

**Tech Stack:** Rust, Axum, Tokio, Vue 3, TypeScript, Vite, Vue Router, Pinia, Naive UI, Vitest, Vue Test Utils, ESLint, Prettier

---

### Task 1: Scaffold the Frontend Workspace

**Files:**
- Modify: `.gitignore`
- Create: `web/package.json`
- Create: `web/tsconfig.json`
- Create: `web/tsconfig.app.json`
- Create: `web/tsconfig.node.json`
- Create: `web/vite.config.ts`
- Create: `web/vitest.config.ts`
- Create: `web/eslint.config.js`
- Create: `web/.prettierrc.json`
- Create: `web/env.d.ts`

- [ ] **Step 1: Add ignore rules for frontend artifacts**

```gitignore
target/
.worktrees/
.idea/
web/node_modules/
web/dist/
web/.vitest/
```

- [ ] **Step 2: Write the frontend package manifest**

```json
{
  "name": "issueflow-web",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc --noEmit && vite build",
    "preview": "vite preview",
    "typecheck": "vue-tsc --noEmit",
    "lint": "eslint .",
    "test": "vitest run"
  },
  "dependencies": {
    "@vueuse/core": "^13",
    "naive-ui": "^2",
    "pinia": "^3",
    "vue": "^3",
    "vue-router": "^4"
  },
  "devDependencies": {
    "@eslint/js": "^9",
    "@types/node": "^24",
    "@vitejs/plugin-vue": "^6",
    "@vue/test-utils": "^2",
    "eslint": "^9",
    "eslint-plugin-vue": "^10",
    "globals": "^16",
    "jsdom": "^26",
    "prettier": "^3",
    "typescript": "^5",
    "typescript-eslint": "^8",
    "vite": "^7",
    "vitest": "^3",
    "vue-tsc": "^3"
  }
}
```

- [ ] **Step 3: Add TypeScript config files**

`web/tsconfig.json`

```json
{
  "files": [],
  "references": [
    { "path": "./tsconfig.app.json" },
    { "path": "./tsconfig.node.json" }
  ]
}
```

`web/tsconfig.app.json`

```json
{
  "extends": "./tsconfig.node.json",
  "compilerOptions": {
    "composite": true,
    "tsBuildInfoFile": "./node_modules/.tmp/tsconfig.app.tsbuildinfo",
    "lib": ["ES2022", "DOM", "DOM.Iterable"],
    "types": ["vite/client"],
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["env.d.ts", "src/**/*.ts", "src/**/*.vue", "src/**/*.d.ts"]
}
```

`web/tsconfig.node.json`

```json
{
  "compilerOptions": {
    "composite": true,
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "resolveJsonModule": true,
    "allowSyntheticDefaultImports": true,
    "esModuleInterop": true,
    "strict": true,
    "target": "ES2022",
    "lib": ["ES2022"],
    "types": ["node"]
  },
  "include": ["vite.config.ts", "vitest.config.ts", "eslint.config.js"]
}
```

- [ ] **Step 4: Add Vite and Vitest configuration**

`web/vite.config.ts`

```ts
import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  server: {
    port: 5173,
    proxy: {
      "/auth": "http://127.0.0.1:3000",
      "/api": "http://127.0.0.1:3000",
      "/internal": "http://127.0.0.1:3000",
    },
  },
  build: {
    outDir: "dist",
    assetsDir: "assets",
    sourcemap: true,
    rollupOptions: {
      output: {
        entryFileNames: "assets/app.js",
        chunkFileNames: "assets/[name].js",
        assetFileNames: (assetInfo) => {
          if (assetInfo.name?.endsWith(".css")) {
            return "assets/app.css";
          }

          return "assets/[name][extname]";
        },
      },
    },
  },
});
```

`web/vitest.config.ts`

```ts
import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vitest/config";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  test: {
    environment: "jsdom",
    globals: true,
    setupFiles: [],
  },
});
```

- [ ] **Step 5: Add linting, formatting, and Vue type declarations**

`web/eslint.config.js`

```js
import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import pluginVue from "eslint-plugin-vue";

export default [
  {
    ignores: ["dist/**", "coverage/**"],
  },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  ...pluginVue.configs["flat/recommended"],
  {
    files: ["**/*.{ts,vue}"],
    languageOptions: {
      globals: globals.browser,
      parserOptions: {
        parser: tseslint.parser,
        ecmaVersion: "latest",
        sourceType: "module",
        extraFileExtensions: [".vue"],
      },
    },
    rules: {
      "vue/multi-word-component-names": "off",
    },
  },
];
```

`web/.prettierrc.json`

```json
{
  "semi": true,
  "singleQuote": false
}
```

`web/env.d.ts`

```ts
/// <reference types="vite/client" />
```

- [ ] **Step 6: Install dependencies and verify the workspace is valid**

Run: `cd web && npm install`
Expected: install completes and creates `package-lock.json`

Run: `cd web && npm run typecheck`
Expected: command exits successfully before any app code exists

- [ ] **Step 7: Commit the scaffolding**

```bash
git add .gitignore web/package.json web/package-lock.json web/tsconfig.json web/tsconfig.app.json web/tsconfig.node.json web/vite.config.ts web/vitest.config.ts web/eslint.config.js web/.prettierrc.json web/env.d.ts
git commit -m "feat: scaffold frontend workspace"
```

### Task 2: Build the Minimal Vue Application and Frontend Tests

**Files:**
- Create: `web/index.html`
- Create: `web/src/main.ts`
- Create: `web/src/App.vue`
- Create: `web/src/router/index.ts`
- Create: `web/src/stores/session.ts`
- Create: `web/src/components/layout/AppShell.vue`
- Create: `web/src/views/LandingView.vue`
- Create: `web/src/views/OAuthCallbackView.vue`
- Create: `web/src/views/WorkbenchView.vue`
- Create: `web/src/styles/tokens.css`
- Create: `web/src/styles/main.css`
- Create: `web/src/tests/app.spec.ts`
- Create: `web/src/tests/oauth-callback-view.spec.ts`

- [ ] **Step 1: Write the failing frontend tests**

`web/src/tests/app.spec.ts`

```ts
import { mount } from "@vue/test-utils";
import { createPinia } from "pinia";
import { createRouter, createMemoryHistory } from "vue-router";
import App from "@/App.vue";
import { routes } from "@/router";

async function renderAt(path: string) {
  const router = createRouter({
    history: createMemoryHistory(),
    routes,
  });

  router.push(path);
  await router.isReady();

  return mount(App, {
    global: {
      plugins: [createPinia(), router],
    },
  });
}

describe("App routing", () => {
  it("renders the landing page at root", async () => {
    const wrapper = await renderAt("/");

    expect(wrapper.text()).toContain("Issueflow Gateway");
    expect(wrapper.text()).toContain("Continue with GitLab");
  });
});
```

`web/src/tests/oauth-callback-view.spec.ts`

```ts
import { mount } from "@vue/test-utils";
import { createPinia } from "pinia";
import { createRouter, createMemoryHistory } from "vue-router";
import App from "@/App.vue";
import { routes } from "@/router";

async function renderAt(path: string) {
  const router = createRouter({
    history: createMemoryHistory(),
    routes,
  });

  router.push(path);
  await router.isReady();

  return { wrapper: mount(App, { global: { plugins: [createPinia(), router] } }), router };
}

describe("OAuth callback view", () => {
  it("shows a success state from the gateway redirect", async () => {
    const { wrapper } = await renderAt("/auth/callback/gitlab?result=success");

    expect(wrapper.text()).toContain("GitLab connected");
    expect(wrapper.text()).toContain("Opening the workbench");
  });
});
```

- [ ] **Step 2: Run the frontend tests to verify they fail**

Run: `cd web && npm run test`
Expected: FAIL with missing module or component file errors for `@/App.vue` and router files

- [ ] **Step 3: Add the application entry, router, store, and styles**

`web/index.html`

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>issueflow</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

`web/src/main.ts`

```ts
import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import { router } from "./router";
import "./styles/tokens.css";
import "./styles/main.css";

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.mount("#app");
```

`web/src/router/index.ts`

```ts
import { createRouter, createWebHistory } from "vue-router";
import LandingView from "@/views/LandingView.vue";
import OAuthCallbackView from "@/views/OAuthCallbackView.vue";
import WorkbenchView from "@/views/WorkbenchView.vue";

export const routes = [
  {
    path: "/",
    name: "landing",
    component: LandingView,
  },
  {
    path: "/auth/callback/:provider",
    name: "oauth-callback",
    component: OAuthCallbackView,
    props: true,
  },
  {
    path: "/workbench",
    name: "workbench",
    component: WorkbenchView,
  },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});
```

`web/src/stores/session.ts`

```ts
import { defineStore } from "pinia";

type OAuthResult = "idle" | "success" | "error";

export const useSessionStore = defineStore("session", {
  state: () => ({
    oauthResult: "idle" as OAuthResult,
    oauthReason: "",
  }),
  actions: {
    captureOAuthResult(result: OAuthResult, reason = "") {
      this.oauthResult = result;
      this.oauthReason = reason;
    },
  },
});
```

`web/src/styles/tokens.css`

```css
:root {
  --if-color-bg: #f3f5f8;
  --if-color-surface: #ffffff;
  --if-color-surface-alt: #e9eef5;
  --if-color-text: #162033;
  --if-color-muted: #5e6b82;
  --if-color-accent: #0f766e;
  --if-color-accent-strong: #115e59;
  --if-color-border: #d7dfeb;
  --if-shadow-panel: 0 24px 60px rgba(15, 23, 42, 0.08);
  --if-radius-lg: 24px;
  --if-radius-md: 16px;
}
```

`web/src/styles/main.css`

```css
html,
body,
#app {
  min-height: 100%;
}

body {
  margin: 0;
  font-family: "Segoe UI", "Helvetica Neue", sans-serif;
  background:
    radial-gradient(circle at top, rgba(15, 118, 110, 0.16), transparent 32%),
    linear-gradient(180deg, #fbfcfe 0%, var(--if-color-bg) 100%);
  color: var(--if-color-text);
}

a {
  color: inherit;
}

* {
  box-sizing: border-box;
}
```

- [ ] **Step 4: Add the views and layout components**

`web/src/App.vue`

```vue
<template>
  <n-config-provider>
    <n-global-style />
    <router-view />
  </n-config-provider>
</template>

<script setup lang="ts">
import { NConfigProvider, NGlobalStyle } from "naive-ui";
</script>
```

`web/src/components/layout/AppShell.vue`

```vue
<template>
  <n-layout class="shell">
    <n-layout-header bordered class="shell__header">
      <div class="shell__brand">
        <span class="shell__brand-mark">IF</span>
        <div>
          <strong>issueflow</strong>
          <div class="shell__subtitle">Agent Workbench</div>
        </div>
      </div>
    </n-layout-header>
    <n-layout has-sider position="absolute" style="top: 72px; bottom: 0">
      <n-layout-sider bordered collapse-mode="width" :collapsed-width="72" :width="220">
        <n-menu :options="menuOptions" :value="activeKey" />
      </n-layout-sider>
      <n-layout-content content-style="padding: 28px;">
        <slot />
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<script setup lang="ts">
import { h } from "vue";
import { RouterLink } from "vue-router";
import { NLayout, NLayoutContent, NLayoutHeader, NLayoutSider, NMenu } from "naive-ui";

defineProps<{
  activeKey: string;
}>();

const menuOptions = [
  {
    key: "overview",
    label: () =>
      h(
        RouterLink,
        {
          to: "/workbench",
        },
        { default: () => "Overview" },
      ),
  },
];
</script>

<style scoped>
.shell {
  min-height: 100vh;
}

.shell__header {
  height: 72px;
  display: flex;
  align-items: center;
  padding: 0 24px;
  background: rgba(255, 255, 255, 0.92);
  backdrop-filter: blur(12px);
}

.shell__brand {
  display: flex;
  align-items: center;
  gap: 12px;
}

.shell__brand-mark {
  width: 40px;
  height: 40px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 12px;
  background: var(--if-color-accent);
  color: #fff;
  font-weight: 700;
}

.shell__subtitle {
  color: var(--if-color-muted);
  font-size: 12px;
}
</style>
```

`web/src/views/LandingView.vue`

```vue
<template>
  <main class="landing">
    <n-card class="landing__card" :bordered="false">
      <div class="landing__eyebrow">Issueflow Gateway</div>
      <h1>Controlled orchestration for issue-driven delivery.</h1>
      <p>
        Start with the Rust Gateway, keep OAuth and workflow control server-side,
        and grow the Agent Workbench from a stable frontend foundation.
      </p>
      <n-button tag="a" href="/auth/gitlab/login" type="primary" size="large">
        Continue with GitLab
      </n-button>
    </n-card>
  </main>
</template>

<script setup lang="ts">
import { NButton, NCard } from "naive-ui";
</script>

<style scoped>
.landing {
  min-height: 100vh;
  display: grid;
  place-items: center;
  padding: 24px;
}

.landing__card {
  width: min(100%, 640px);
  border-radius: var(--if-radius-lg);
  box-shadow: var(--if-shadow-panel);
}

.landing__eyebrow {
  margin-bottom: 12px;
  color: var(--if-color-accent);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

h1 {
  margin: 0 0 12px;
  font-size: clamp(2.25rem, 5vw, 3.5rem);
  line-height: 1.05;
}

p {
  margin: 0 0 24px;
  color: var(--if-color-muted);
  line-height: 1.6;
}
</style>
```

`web/src/views/OAuthCallbackView.vue`

```vue
<template>
  <main class="callback">
    <n-card class="callback__card" :bordered="false">
      <n-result
        :status="status"
        :title="title"
        :description="description"
      >
        <template #footer>
          <n-button v-if="isSuccess" type="primary" @click="goToWorkbench">
            Open workbench now
          </n-button>
          <n-button v-else tag="a" href="/" quaternary>
            Return home
          </n-button>
        </template>
      </n-result>
    </n-card>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { NButton, NCard, NResult } from "naive-ui";
import { useSessionStore } from "@/stores/session";

const route = useRoute();
const router = useRouter();
const sessionStore = useSessionStore();

const isSuccess = computed(() => route.query.result === "success");

const title = computed(() => (isSuccess.value ? "GitLab connected" : "OAuth verification failed"));
const description = computed(() =>
  isSuccess.value
    ? "Opening the workbench with the validated gateway callback."
    : `The gateway rejected the callback${route.query.reason ? `: ${route.query.reason}` : "."}`,
);
const status = computed(() => (isSuccess.value ? "success" : "error"));

function goToWorkbench() {
  router.push("/workbench");
}

onMounted(() => {
  sessionStore.captureOAuthResult(
    isSuccess.value ? "success" : "error",
    typeof route.query.reason === "string" ? route.query.reason : "",
  );

  if (isSuccess.value) {
    window.setTimeout(goToWorkbench, 1000);
  }
});
</script>

<style scoped>
.callback {
  min-height: 100vh;
  display: grid;
  place-items: center;
  padding: 24px;
}

.callback__card {
  width: min(100%, 640px);
  border-radius: var(--if-radius-lg);
  box-shadow: var(--if-shadow-panel);
}
</style>
```

`web/src/views/WorkbenchView.vue`

```vue
<template>
  <app-shell active-key="overview">
    <n-card :bordered="false" class="panel">
      <template #header>
        <span>Overview</span>
      </template>
      <h2>Agent Workbench skeleton</h2>
      <p>
        This initial workspace proves the frontend stack, route structure, and Gateway
        integration without inventing domain modules early.
      </p>
    </n-card>
  </app-shell>
</template>

<script setup lang="ts">
import { NCard } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
</script>

<style scoped>
.panel {
  max-width: 720px;
  border-radius: var(--if-radius-md);
  box-shadow: var(--if-shadow-panel);
}

h2 {
  margin: 0 0 12px;
}

p {
  margin: 0;
  color: var(--if-color-muted);
  line-height: 1.6;
}
</style>
```

- [ ] **Step 5: Run frontend checks to verify the app now passes**

Run: `cd web && npm run test`
Expected: PASS for `app.spec.ts` and `oauth-callback-view.spec.ts`

Run: `cd web && npm run lint`
Expected: PASS with no ESLint errors

Run: `cd web && npm run build`
Expected: PASS and create `web/dist/assets/app.js` plus `web/dist/assets/app.css`

- [ ] **Step 6: Commit the frontend application skeleton**

```bash
git add web/index.html web/src/main.ts web/src/App.vue web/src/router/index.ts web/src/stores/session.ts web/src/components/layout/AppShell.vue web/src/views/LandingView.vue web/src/views/OAuthCallbackView.vue web/src/views/WorkbenchView.vue web/src/styles/tokens.css web/src/styles/main.css web/src/tests/app.spec.ts web/src/tests/oauth-callback-view.spec.ts
git commit -m "feat: add vue workbench skeleton"
```

### Task 3: Add Gateway SPA Entry Routes and OAuth Redirect Integration

**Files:**
- Modify: `src/http/handlers/mod.rs`
- Create: `src/http/handlers/spa_handler.rs`
- Modify: `src/http/handlers/oauth_handler.rs`
- Modify: `src/http/routes.rs`
- Create: `internal/pages/templates/app.html`
- Create: `tests/spa_handler.rs`
- Modify: `tests/oauth_handler.rs`

- [ ] **Step 1: Write the failing Rust integration tests**

`tests/spa_handler.rs`

```rust
use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use issueflow::config::Config;
use tower::ServiceExt;

#[tokio::test]
async fn root_route_serves_spa_shell_html() {
    let app = issueflow::http::routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.contains("text/html")));

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let html = String::from_utf8(body.to_vec()).unwrap();

    assert!(html.contains("<title>issueflow</title>"));
    assert!(html.contains("/assets/app.js"));
    assert!(html.contains("/assets/app.css"));
}

#[tokio::test]
async fn workbench_route_reuses_the_same_spa_shell() {
    let app = issueflow::http::routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(Request::builder().uri("/workbench").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

Replace the success-path test in `tests/oauth_handler.rs` with:

```rust
#[tokio::test]
async fn oauth_callback_redirects_to_the_frontend_callback_route_after_validation() {
    let app = issueflow::http::routes::router(test_config());
    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/auth/gitlab/login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let state = extract_query_param(
        login_response
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok())
            .unwrap(),
        "state",
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/auth/gitlab/callback?code=test-code&state={state}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/auth/callback/gitlab?result=success")
    );
}
```

- [ ] **Step 2: Run the focused Rust tests to verify they fail**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test root_route_serves_spa_shell_html -- --exact`
Expected: FAIL because the SPA handler route does not exist yet

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test oauth_callback_redirects_to_the_frontend_callback_route_after_validation -- --exact`
Expected: FAIL because the OAuth callback still returns HTML instead of a frontend redirect

- [ ] **Step 3: Add the SPA shell template and Gateway handler**

`internal/pages/templates/app.html`

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>issueflow</title>
    <link rel="stylesheet" href="/assets/app.css" />
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/assets/app.js"></script>
  </body>
</html>
```

`src/http/handlers/spa_handler.rs`

```rust
use std::path::{Path as FsPath, PathBuf};

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
};

const APP_HTML: &str = include_str!("../../../internal/pages/templates/app.html");
const ASSET_ROOT: &str = "web/dist/assets";

pub async fn app_shell() -> Html<&'static str> {
    Html(APP_HTML)
}

pub async fn app_asset(Path(path): Path<String>) -> Result<Response, StatusCode> {
    if path.contains("..") {
        return Err(StatusCode::BAD_REQUEST);
    }

    let asset_path = PathBuf::from(ASSET_ROOT).join(&path);
    let bytes = tokio::fs::read(&asset_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok((
        [(header::CONTENT_TYPE, content_type_for(&asset_path))],
        bytes,
    )
        .into_response())
}

fn content_type_for(path: &FsPath) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("js") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}
```

`src/http/handlers/mod.rs`

```rust
pub mod confirm_handler;
pub mod oauth_handler;
pub mod spa_handler;
pub mod status_handler;
pub mod webhook_handler;
```

`src/http/routes.rs`

```rust
use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    config::Config,
    http::handlers::{confirm_handler, oauth_handler, spa_handler, status_handler, webhook_handler},
};

pub fn router(config: Config) -> Router {
    Router::new()
        .route("/", get(spa_handler::app_shell))
        .route("/index.html", get(spa_handler::app_shell))
        .route("/workbench", get(spa_handler::app_shell))
        .route("/auth/callback/{provider}", get(spa_handler::app_shell))
        .route("/assets/{*path}", get(spa_handler::app_asset))
        .route("/auth/{provider}/login", get(oauth_handler::oauth_login))
        .route("/auth/{provider}/callback", get(oauth_handler::oauth_callback))
        .route("/status/ping", get(status_handler::status_ping))
        .route("/status/session/{session_id}", get(status_handler::session_status))
        .route("/confirm/plan/{token}", get(confirm_handler::confirm_plan))
        .route("/webhooks/gitlab", post(webhook_handler::handle_webhook))
        .with_state(config)
}
```

- [ ] **Step 4: Change the OAuth callback success path to a frontend redirect**

Update `src/http/handlers/oauth_handler.rs` to:

```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Redirect,
};
use serde::Deserialize;

use crate::{
    config::Config,
    oauth::{OAuthProviderKind, issue_state, validate_state},
};

#[derive(Deserialize)]
pub struct OAuthCallbackQuery {
    code: String,
    state: String,
}

pub async fn oauth_login(
    Path(provider): Path<String>,
    State(config): State<Config>,
) -> Result<Redirect, StatusCode> {
    let provider_kind = OAuthProviderKind::from_slug(&provider).ok_or(StatusCode::NOT_FOUND)?;
    let provider_config = config
        .oauth
        .provider(provider_kind)
        .ok_or(StatusCode::NOT_FOUND)?;
    let state = issue_state(provider_kind, &config.oauth.state_signing_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::temporary(&provider_config.authorize_url(&state)))
}

pub async fn oauth_callback(
    Path(provider): Path<String>,
    State(config): State<Config>,
    Query(query): Query<OAuthCallbackQuery>,
) -> Result<Redirect, StatusCode> {
    let provider_kind = OAuthProviderKind::from_slug(&provider).ok_or(StatusCode::NOT_FOUND)?;

    if query.code.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    config
        .oauth
        .provider(provider_kind)
        .ok_or(StatusCode::NOT_FOUND)?;
    validate_state(&query.state, provider_kind, &config.oauth.state_signing_secret)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Redirect::temporary(&format!(
        "/auth/callback/{provider}?result=success"
    )))
}
```

- [ ] **Step 5: Run focused Rust tests, then the broader test slice**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test root_route_serves_spa_shell_html -- --exact`
Expected: PASS

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test oauth_callback_redirects_to_the_frontend_callback_route_after_validation -- --exact`
Expected: PASS

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test oauth_ -- --nocapture`
Expected: PASS for the OAuth handler tests

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test status_route_returns_ok -- --exact`
Expected: PASS and proves the existing status handler still works

- [ ] **Step 6: Commit the Gateway integration**

```bash
git add src/http/handlers/mod.rs src/http/handlers/spa_handler.rs src/http/handlers/oauth_handler.rs src/http/routes.rs internal/pages/templates/app.html tests/spa_handler.rs tests/oauth_handler.rs
git commit -m "feat: serve workbench shell from gateway"
```

### Task 4: Document the Developer Workflow and Run Integrated Verification

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Add frontend development and integration commands to the README**

Append a section like this to `README.md`:

````md
## Frontend Development

The repository now includes a Vue-based workbench skeleton under `web/`.

Install dependencies:

```bash
cd web
npm install
```

Run the frontend dev server:

```bash
cd web
npm run dev
```

The Vite dev server proxies Gateway-owned routes such as `/auth/*` to the Rust server running on `http://127.0.0.1:3000`.

Build frontend assets for Gateway integration:

```bash
cd web
npm run build
```

Run frontend checks:

```bash
cd web
npm run lint
npm run test
```
````

- [ ] **Step 2: Run the full verification set**

Run: `cd web && npm run lint && npm run test && npm run build`
Expected: PASS

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test`
Expected: PASS

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo run`
Expected: server starts successfully on the configured address

Manual verification:
1. Open `http://127.0.0.1:3000/`
2. Confirm the landing page renders
3. Open `http://127.0.0.1:3000/workbench`
4. Confirm the SPA shell loads with the single `Overview` menu

- [ ] **Step 3: Commit the documentation update**

```bash
git add README.md
git commit -m "docs: add frontend workflow commands"
```
