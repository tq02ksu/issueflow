<template>
  <app-shell active-key="system-gateway" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div class="prototype-page__eyebrow">{{ t("prototype.gateway.eyebrow") }}</div>
        <h1>{{ t("prototype.gateway.title") }}</h1>
        <p>{{ t("prototype.gateway.description") }}</p>
      </div>

      <div class="gateway-grid">
        <n-card :bordered="false" class="gw-card">
          <template #header>{{ t("prototype.gateway.modelRouting") }}</template>
          <div class="gw-card__body">
            <article class="gw-item" v-for="tier in modelTiers" :key="tier.key">
              <div class="gw-item__header">
                <strong>{{ tier.label }}</strong>
                <span>{{ tier.model }}</span>
              </div>
              <p>{{ tier.useCase }}</p>
            </article>
          </div>
        </n-card>

        <n-card :bordered="false" class="gw-card">
          <template #header>{{ t("prototype.gateway.budget") }}</template>
          <div class="gw-card__body">
            <div class="gw-metric">
              <span class="gw-metric__label">{{ t("prototype.gateway.currentRunBudget") }}</span>
              <span class="gw-metric__value">$0.24</span>
            </div>
            <div class="gw-metric">
              <span class="gw-metric__label">{{ t("prototype.gateway.loopDailyBudget") }}</span>
              <span class="gw-metric__value">$5.00</span>
            </div>
            <div class="gw-metric">
              <span class="gw-metric__label">{{ t("prototype.gateway.monthlyBudget") }}</span>
              <span class="gw-metric__value">$120.00</span>
            </div>
          </div>
        </n-card>

        <n-card :bordered="false" class="gw-card">
          <template #header>{{ t("prototype.gateway.providers") }}</template>
          <div class="gw-card__body">
            <article class="gw-item" v-for="p in providers" :key="p.id">
              <div class="gw-item__header">
                <strong>{{ p.name }}</strong>
                <span class="gw-item__status" :class="`gw-item__status--${p.status}`">{{ p.status }}</span>
              </div>
              <p>{{ t("prototype.gateway.fallback") }}: {{ p.fallback }}</p>
            </article>
          </div>
        </n-card>

        <n-card :bordered="false" class="gw-card gw-card--full">
          <template #header>{{ t("prototype.gateway.usageLogs") }}</template>
          <div class="usage-list">
            <article class="usage-item" v-for="log in usageLogs" :key="log.id">
              <div class="usage-item__header">
                <span>{{ log.model }}</span>
                <span>{{ log.timestamp.slice(11, 19) }}</span>
              </div>
              <div class="usage-item__metrics">
                <span>{{ log.tokens.toLocaleString() }} tokens</span>
                <span>${{ log.cost.toFixed(4) }}</span>
                <span>{{ log.latencyMs }}ms</span>
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
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const modelTiers = [
  { key: "cheap-fast", label: "cheap-fast", model: "GPT-4o-mini", useCase: "Health checks, status queries, simple evaluations" },
  { key: "balanced", label: "balanced", model: "GPT-4o", useCase: "Issue analysis, MR review, recommendation generation" },
  { key: "high-reasoning", label: "high-reasoning", model: "Claude 3.5 Sonnet", useCase: "Complex verification, governance analysis" },
];

const providers = [
  { id: "p1", name: "OpenAI", status: "connected", fallback: "Anthropic" },
  { id: "p2", name: "Anthropic", status: "connected", fallback: "OpenAI" },
];

const usageLogs = [
  { id: "l1", model: "GPT-4o", timestamp: "2026-07-05T12:10:00Z", tokens: 3420, cost: 0.034, latencyMs: 1240 },
  { id: "l2", model: "GPT-4o-mini", timestamp: "2026-07-05T12:05:00Z", tokens: 980, cost: 0.0005, latencyMs: 420 },
  { id: "l3", model: "Claude 3.5 Sonnet", timestamp: "2026-07-05T11:50:00Z", tokens: 5680, cost: 0.085, latencyMs: 2100 },
  { id: "l4", model: "GPT-4o", timestamp: "2026-07-05T11:30:00Z", tokens: 2180, cost: 0.022, latencyMs: 980 },
  { id: "l5", model: "GPT-4o-mini", timestamp: "2026-07-05T11:00:00Z", tokens: 1200, cost: 0.0006, latencyMs: 380 },
];
</script>

<style scoped>
.prototype-page { display: grid; gap: 24px; }
.prototype-page__eyebrow { margin-bottom: 10px; color: var(--if-color-accent-strong); font-size: 12px; font-weight: 700; letter-spacing: 0.12em; text-transform: uppercase; }
.prototype-page__header h1 { margin: 0 0 8px; }
.prototype-page__header p { max-width: 640px; margin: 0; color: var(--if-color-muted); }
.gateway-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 20px; }
.gw-card { border-radius: var(--if-radius-lg); background: rgba(255, 250, 242, 0.92); }
.gw-card--full { grid-column: span 2; }
.gw-card__body { display: grid; gap: 12px; }
.gw-item { padding: 12px; border: 1px solid rgba(216, 204, 184, 0.8); border-radius: var(--if-radius-sm); background: rgba(255, 255, 255, 0.7); }
.gw-item__header { display: flex; justify-content: space-between; gap: 12px; margin-bottom: 4px; }
.gw-item p { margin: 4px 0 0; color: var(--if-color-muted); font-size: 13px; }
.gw-item__status { display: inline-flex; padding: 3px 8px; border-radius: 999px; font-size: 11px; font-weight: 700; }
.gw-item__status--connected { background: rgba(15, 118, 110, 0.12); color: var(--if-color-accent); }
.gw-metric { display: flex; justify-content: space-between; align-items: center; padding: 12px; border: 1px solid rgba(216, 204, 184, 0.8); border-radius: var(--if-radius-sm); background: rgba(255, 255, 255, 0.7); }
.gw-metric__label { font-size: 13px; }
.gw-metric__value { font-size: 16px; font-weight: 800; color: var(--if-color-accent-strong); }
.usage-list { display: grid; gap: 10px; }
.usage-item { padding: 12px; border: 1px solid rgba(216, 204, 184, 0.8); border-radius: var(--if-radius-sm); background: rgba(255, 255, 255, 0.7); }
.usage-item__header { display: flex; justify-content: space-between; gap: 12px; font-size: 13px; }
.usage-item__metrics { display: flex; gap: 16px; margin-top: 6px; font-size: 12px; color: var(--if-color-muted); }
@media (max-width: 900px) { .gateway-grid { grid-template-columns: 1fr; } .gw-card--full { grid-column: span 1; } }
</style>
