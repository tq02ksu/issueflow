# Frontend I18n Foundation Design

**Date:** 2026-07-02

## Goal

Add a stable internationalization foundation to the Vue frontend so the system can run in English and Simplified Chinese, starting with the landing page and mock workbench prototype.

## Scope

This design covers:

- a global frontend i18n framework
- an explicit language switcher
- browser-language-based first-run selection with English as the default fallback
- persistence of the user-selected language
- migration of the landing page, app shell, and prototype views/components to translation keys
- test coverage for locale selection and runtime switching

This design does not cover:

- route-level locale prefixes
- backend locale negotiation
- server-rendered localized content
- full migration of every legacy or non-prototype screen
- advanced date, number, or pluralization formatting beyond what current UI copy needs

## Product Behavior

The system exposes two locales:

- `en`
- `zh-CN`

Language resolution order:

1. previously saved user choice in `localStorage`
2. browser language match
3. fallback to `en`

The UI must include an explicit global switcher with two visible options:

- `EN`
- `中文`

Switcher behavior:

- available on the landing page
- available inside the authenticated or prototype shell
- changes copy immediately without a page reload
- persists the new locale in `localStorage`

This is a global application preference rather than a workbench setting or a low-frequency settings-only control.

## UX Placement

The switcher should be easy to find but visually quiet.

Placement:

- landing page hero top area
- `AppShell` header tools area

Control style:

- compact segmented toggle or equivalent two-state switch
- explicit labels `EN` and `中文`
- active state must be obvious

The switcher must not alter the page skeleton, page routing, or workbench structure.

## Technical Approach

Use `vue-i18n` as the project-standard i18n runtime.

Reasons:

- standard Vue integration
- reactive updates without custom plumbing
- clean interpolation and namespaced dictionaries
- easy extension from prototype pages into production pages
- testability consistent with Vue component patterns

Avoid a custom dictionary composable because it creates a second framework that will become technical debt once the system expands beyond the prototype.

## Architecture

### 1. I18n module

Add a dedicated `web/src/i18n/` module with:

- `index.ts` for `createI18n(...)`
- `locale.ts` for locale constants, browser detection, and persistence helpers
- `locales/en.ts`
- `locales/zh-CN.ts`

Responsibilities:

- define supported locales
- resolve startup locale
- export i18n instance
- expose helper utilities for tests if needed

### 2. Message organization

Organize messages by UI boundary instead of one flat dictionary.

Initial namespaces:

- `common`
- `landing`
- `shell`
- `prototype.overview`
- `prototype.issues`
- `prototype.mrs`
- `prototype.milestones`
- `prototype.settings`

This keeps translation ownership aligned with existing UI structure and reduces accidental coupling.

### 3. App bootstrap

Register i18n in `web/src/main.ts` before mount.

The startup sequence remains:

1. enable mocks if mock mode is active
2. create Vue app
3. install Pinia
4. install router
5. install i18n
6. install A2UI provider
7. mount app

### 4. Locale state

Do not create a separate Pinia locale store in the first pass.

Use `vue-i18n` as the source of truth and a small helper for persistence.

This keeps the foundation minimal and avoids duplicating state.

## UI Migration Plan

### Landing page

Translate:

- hero eyebrow, title, lead, impact line
- primary and secondary CTA labels
- diagram group labels
- diagram node titles, labels, and hover/click help text
- lower panel switch labels
- overview, product, and engineering card content
- non-mock landing login copy

The existing diagram structure stays intact. Only copy moves behind translation keys.

### App shell

Translate:

- header subtitle
- workbench label
- settings button
- fallback role label
- sider role label
- rename modal title
- rename input placeholder
- cancel and save buttons
- nav labels such as overview, issues, MRs, milestones, pending actions

### Prototype views and cards

Translate text in:

- prototype overview view
- prototype issues view
- prototype MRs view
- prototype milestones view
- prototype user settings view
- overview/stat cards
- role panel
- memory controls panel
- skill version panel
- recommended actions card
- issue state and related prototype presentation cards

Mock content handling:

- UI chrome, labels, descriptions, helper text, and action text must use i18n keys
- sample entity names may stay as seed data in English for the first pass if they act as user-provided content rather than product chrome
- any mock narrative intended as product explanation should move into translation dictionaries

## File-Level Design

Expected new files:

- `web/src/i18n/index.ts`
- `web/src/i18n/locale.ts`
- `web/src/i18n/locales/en.ts`
- `web/src/i18n/locales/zh-CN.ts`
- potentially a small shared `LanguageSwitcher` component if reuse reduces duplication

Expected modified files:

- `web/src/main.ts`
- `web/src/views/LandingView.vue`
- `web/src/components/layout/AppShell.vue`
- `web/src/views/prototype/*`
- `web/src/components/prototype/*`
- any directly affected legacy or support components that render visible shell/prototype copy
- existing tests asserting hard-coded English strings

## Testing

Add and update tests for:

- startup locale resolution from browser language when no stored preference exists
- stored locale taking precedence over browser language
- language switcher rendering in landing and shell contexts
- runtime switching from English to Chinese and back
- representative component text updates after locale change

Current tests that assert raw English strings should be adjusted to either:

- verify translated output under an explicit locale, or
- verify switching behavior with both locales

The first pass should focus on component and integration tests already present in `web/src/tests/`.

## Non-Goals and Guardrails

- Do not introduce locale prefixes such as `/en/...` or `/zh-CN/...`
- Do not move language selection into user settings only
- Do not fork component structure by locale
- Do not pollute prototype business data with duplicated per-locale copies unless the data is product-owned display copy
- Do not block future backend-driven localization, but do not design around it yet

## Rollout Strategy

Implement in two stages:

1. foundation
   - install and wire `vue-i18n`
   - define locale resolution and persistence
   - add shared language switcher
2. prototype migration
   - move landing, shell, and prototype UI strings to message catalogs
   - update tests

This keeps the first internationalized slice coherent and demo-ready without forcing a full frontend translation sweep.
