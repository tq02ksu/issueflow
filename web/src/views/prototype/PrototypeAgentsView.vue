<template>
  <app-shell active-key="agents" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div class="prototype-page__eyebrow">
          {{ t("prototype.agents.eyebrow") }}
        </div>
        <h1>{{ t("prototype.agents.title") }}</h1>
        <p>{{ t("prototype.agents.description") }}</p>
      </div>

      <div class="agents-grid">
        <n-card :bordered="false" class="agents-card">
          <template #header>{{ t("prototype.agents.loopCore") }}</template>
          <div class="agents-card__body">
            <LoopCoreChat />
          </div>
        </n-card>

        <n-card :bordered="false" class="agents-card">
          <template #header>{{ t("prototype.agents.workerAgents") }}</template>
          <div class="agents-card__body">
            <article
              class="agents-item"
              v-for="agent in workerAgents"
              :key="agent.id"
            >
              <div class="agents-item__header">
                <strong>{{ agent.name }}</strong>
                <span
                  class="agents-item__status"
                  :class="`agents-item__status--${agent.status}`"
                  >{{ agent.status }}</span
                >
              </div>
              <p>{{ agent.currentTask }}</p>
              <span class="agents-item__meta"
                >{{ t("prototype.agents.boundTo") }} {{ agent.boundRun }}</span
              >
            </article>
          </div>
        </n-card>

        <n-card :bordered="false" class="agents-card agents-card--full">
          <template #header>{{
            t("prototype.agents.externalAgents")
          }}</template>
          <div class="agents-card__body">
            <article
              class="agents-item"
              v-for="agent in externalAgents"
              :key="agent.id"
            >
              <div class="agents-item__header">
                <strong>{{ agent.name }}</strong>
                <span
                  class="agents-item__status"
                  :class="`agents-item__status--${agent.status}`"
                  >{{ agent.status }}</span
                >
              </div>
              <p>{{ agent.currentTask }}</p>
              <div class="agents-item__footer">
                <span>{{ agent.provider }}</span>
                <span>${{ agent.cost.toFixed(2) }}</span>
              </div>
            </article>
          </div>
        </n-card>
      </div>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { NCard } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import LoopCoreChat from "@/components/prototype/LoopCoreChat.vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const workerAgents = [
  {
    id: "w1",
    name: "Loop Executor v2",
    status: "running",
    currentTask: "Evaluating issue-101 readiness",
    boundRun: "run-101",
  },
  {
    id: "w2",
    name: "MR Progression Core",
    status: "idle",
    currentTask: "Waiting for next MR event",
    boundRun: "—",
  },
  {
    id: "w3",
    name: "Milestone Health Core",
    status: "running",
    currentTask: "Scanning milestone beta launch",
    boundRun: "run-103",
  },
];

const externalAgents = [
  {
    id: "e1",
    name: "OpenAI GPT-4o",
    provider: "openai",
    status: "connected",
    currentTask: "Answering execution queries",
    cost: 1.42,
  },
  {
    id: "e2",
    name: "Claude 3.5 Sonnet",
    provider: "anthropic",
    status: "connected",
    currentTask: "Review analysis helper",
    cost: 0.87,
  },
];
</script>

<style scoped>
.prototype-page {
  display: grid;
  gap: 24px;
}

.prototype-page__eyebrow {
  margin-bottom: 10px;
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.prototype-page__header h1 {
  margin: 0 0 8px;
}

.prototype-page__header p {
  max-width: 640px;
  margin: 0;
  color: var(--if-color-muted);
}

.agents-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 20px;
}

.agents-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.92);
}

.agents-card--full {
  grid-column: span 2;
}

.agents-card__body {
  display: grid;
  gap: 14px;
}

.agents-metric {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
}

.agents-metric__label {
  font-size: 13px;
}

.agents-metric__value {
  font-size: 16px;
  font-weight: 800;
  color: var(--if-color-accent-strong);
}

.agents-metric__value--ok {
  color: var(--if-color-accent);
}

.agents-item {
  padding: 14px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
}

.agents-item__header {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 6px;
}

.agents-item p {
  margin: 4px 0;
  color: var(--if-color-muted);
  font-size: 13px;
}

.agents-item__meta,
.agents-item__footer {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: var(--if-color-muted);
}

.agents-item__footer {
  justify-content: space-between;
  margin-top: 8px;
}

.agents-item__status {
  display: inline-flex;
  padding: 3px 8px;
  border-radius: 999px;
  font-size: 11px;
  font-weight: 700;
}

.agents-item__status--running {
  background: rgba(15, 118, 110, 0.12);
  color: var(--if-color-accent);
}
.agents-item__status--idle {
  background: rgba(21, 94, 117, 0.12);
  color: var(--if-color-accent-strong);
}
.agents-item__status--connected {
  background: rgba(15, 118, 110, 0.12);
  color: var(--if-color-accent);
}

@media (max-width: 900px) {
  .agents-grid {
    grid-template-columns: 1fr;
  }
  .agents-card--full {
    grid-column: span 1;
  }
}
</style>
