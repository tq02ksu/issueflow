<template>
  <main class="landing">
    <n-card
      class="landing__card"
      :bordered="false"
    >
      <div class="landing__eyebrow">
        Issueflow Gateway
      </div>
      <h1>Controlled orchestration for issue-driven delivery.</h1>
      <p>
        Start with the Rust Gateway, keep OIDC and workflow control server-side,
        and grow the Agent Workbench from a stable frontend foundation.
      </p>
      <n-button
        tag="a"
        href="/api/auth/login"
        type="primary"
        size="large"
      >
        Continue to sign in
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
