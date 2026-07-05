<template>
  <app-shell active-key="system-governance" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div class="prototype-page__eyebrow">{{ t("prototype.governance.eyebrow") }}</div>
        <h1>{{ t("prototype.governance.title") }}</h1>
        <p>{{ t("prototype.governance.description") }}</p>
      </div>

      <div class="gov-grid">
        <n-card :bordered="false" class="gov-card">
          <template #header>{{ t("prototype.governance.verificationDebt") }}</template>
          <div class="gov-card__body">
            <article class="gov-item gov-item--warn" v-for="item in verificationDebt" :key="item.id">
              <strong>{{ item.loop }}</strong>
              <p>{{ item.detail }}</p>
              <span>{{ t("prototype.governance.lastVerified") }}: {{ item.lastVerified }}</span>
            </article>
          </div>
        </n-card>

        <n-card :bordered="false" class="gov-card">
          <template #header>{{ t("prototype.governance.riskAlerts") }}</template>
          <div class="gov-card__body">
            <article class="gov-item gov-item--danger" v-for="item in riskAlerts" :key="item.id">
              <strong>{{ item.title }}</strong>
              <p>{{ item.description }}</p>
            </article>
          </div>
        </n-card>

        <n-card :bordered="false" class="gov-card">
          <template #header>{{ t("prototype.governance.comprehensionRot") }}</template>
          <div class="gov-card__body">
            <article class="gov-item" v-for="item in comprehensionRot" :key="item.id">
              <strong>{{ item.title }}</strong>
              <p>{{ item.description }}</p>
            </article>
          </div>
        </n-card>

        <n-card :bordered="false" class="gov-card">
          <template #header>{{ t("prototype.governance.improvementProposals") }}</template>
          <div class="gov-card__body">
            <article class="gov-item gov-item--info" v-for="item in proposals" :key="item.id">
              <strong>{{ item.title }}</strong>
              <p>{{ item.description }}</p>
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

const verificationDebt = [
  { id: "v1", loop: "Acceptance quality scan", detail: "Not independently verified in 14 days.", lastVerified: "2026-06-21" },
  { id: "v2", loop: "MR review scan", detail: "Output reused across 5 runs without re-verification.", lastVerified: "2026-06-28" },
];

const riskAlerts = [
  { id: "r1", title: "Critical risk: acceptance language", description: "Broad acceptance language pattern detected across 2 workbenches." },
  { id: "r2", title: "High risk: permission boundary", description: "Pending approval with execution_failed status may indicate auth issue." },
];

const comprehensionRot = [
  { id: "c1", title: "Auto-approval rate: 72%", description: "User approved 72% of actions in the last 7 days without reading details." },
  { id: "c2", title: "Summary review gap", description: "No manual summary re-review performed in 5 days." },
];

const proposals = [
  { id: "p1", title: "Tighten acceptance templates", description: "Governance engine suggests stricter acceptance criteria templates." },
  { id: "p2", title: "Skill evolution: delivery-skill v3", description: "Proposed new version with higher verification strictness defaults." },
];
</script>

<style scoped>
.prototype-page { display: grid; gap: 24px; }
.prototype-page__eyebrow { margin-bottom: 10px; color: var(--if-color-accent-strong); font-size: 12px; font-weight: 700; letter-spacing: 0.12em; text-transform: uppercase; }
.prototype-page__header h1 { margin: 0 0 8px; }
.prototype-page__header p { max-width: 640px; margin: 0; color: var(--if-color-muted); }
.gov-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 20px; }
.gov-card { border-radius: var(--if-radius-lg); background: rgba(255, 250, 242, 0.92); }
.gov-card__body { display: grid; gap: 12px; }
.gov-item { padding: 14px; border: 1px solid rgba(216, 204, 184, 0.8); border-radius: var(--if-radius-sm); background: rgba(255, 255, 255, 0.7); }
.gov-item strong { display: block; margin-bottom: 4px; }
.gov-item p { margin: 4px 0; color: var(--if-color-muted); font-size: 13px; line-height: 1.5; }
.gov-item span { font-size: 12px; color: var(--if-color-muted); }
.gov-item--warn { border-left: 3px solid var(--if-color-warning); }
.gov-item--danger { border-left: 3px solid var(--if-color-danger); }
.gov-item--info { border-left: 3px solid var(--if-color-accent-strong); }
@media (max-width: 900px) { .gov-grid { grid-template-columns: 1fr; } }
</style>
