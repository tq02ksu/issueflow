<template>
  <main v-if="isMockMode" class="landing landing--mock">
    <section class="landing__hero">
      <div class="landing__eyebrow">Workflow cockpit</div>
      <h1>Keep issues and MRs moving with one role, one memory, one workbench.</h1>
      <p class="landing__lead">
        This prototype shows how `issueflow` turns issue quality, MR readiness,
        and milestone pressure into explicit next actions for long-running
        project execution.
      </p>
      <div class="landing__actions">
        <n-button tag="a" href="/workbench" type="primary" size="large">
          Open prototype
        </n-button>
        <n-button tag="a" href="/settings" secondary size="large">
          Review settings
        </n-button>
      </div>
    </section>

    <section class="landing__grid">
      <n-card class="landing__panel" :bordered="false">
        <div class="landing__panel-label">Problems</div>
        <ul class="landing__list">
          <li>Issue quality is inconsistent and standards are unclear.</li>
          <li>Delivery gets stuck between clarification, execution, and review.</li>
          <li>No one keeps testing intent visible once work starts moving.</li>
        </ul>
      </n-card>
      <n-card class="landing__panel" :bordered="false">
        <div class="landing__panel-label">Prototype flow</div>
        <ul class="landing__list">
          <li>Choose a workbench and let one role drive execution style.</li>
          <li>Advance issues, MRs, and milestones through explicit workflow states.</li>
          <li>Use skills to change emphasis without changing the page skeleton.</li>
        </ul>
      </n-card>
      <n-card class="landing__panel" :bordered="false">
        <div class="landing__panel-label">What you can tune</div>
        <ul class="landing__list">
          <li>User soul: personality, working style, and default goals.</li>
          <li>Workbench role: one role, one target behavior, one active skill version.</li>
          <li>Memory controls: clear or rebuild scoped memory when context drifts.</li>
        </ul>
      </n-card>
    </section>
  </main>

  <main v-else class="landing">
    <n-card class="landing__card" :bordered="false">
      <div class="landing__eyebrow">Issueflow Gateway</div>
      <h1>Controlled orchestration for issue-driven delivery.</h1>
      <p>
        Start with the Rust Gateway, keep OIDC and workflow control server-side,
        and grow the Agent Workbench from a stable frontend foundation.
      </p>
      <n-button tag="a" href="/api/auth/login" type="primary" size="large">
        Continue to sign in
      </n-button>
    </n-card>
  </main>
</template>

<script setup lang="ts">
import { isMockMode } from "@/app-mode";
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

.landing--mock {
  place-items: stretch;
  gap: 20px;
  max-width: 1240px;
  margin: 0 auto;
  padding: 48px 24px 56px;
}

.landing__hero {
  padding: 40px;
  border-radius: var(--if-radius-xl);
  border: 1px solid rgba(255, 255, 255, 0.45);
  background:
    linear-gradient(135deg, rgba(11, 25, 47, 0.96), rgba(20, 93, 88, 0.88)),
    linear-gradient(180deg, rgba(255, 255, 255, 0.06), transparent);
  color: #f8fafc;
  box-shadow: var(--if-shadow-panel);
}

.landing__lead {
  max-width: 720px;
  color: rgba(248, 250, 252, 0.84);
}

.landing__actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.landing__grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 20px;
}

.landing__panel {
  min-height: 220px;
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.86);
  backdrop-filter: blur(12px);
}

.landing__panel-label {
  margin-bottom: 12px;
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.landing__list {
  margin: 0;
  padding-left: 18px;
  color: var(--if-color-text);
  line-height: 1.7;
}

@media (max-width: 960px) {
  .landing__grid {
    grid-template-columns: 1fr;
  }

  .landing__hero {
    padding: 28px;
  }
}
</style>
