<template>
  <app-shell active-key="settings-access" prototype-mode>
    <div class="page">
      <div class="page__header">
        <div class="page__eyebrow">{{ t("prototype.settings.accessEyebrow") }}</div>
        <h1>{{ t("prototype.settings.accessTitle") }}</h1>
        <p>{{ t("prototype.settings.accessDescription") }}</p>
      </div>

      <n-card :bordered="false" class="section-card">
        <template #header>{{ t("prototype.settings.secretsList") }}</template>
        <div class="card-body">
          <n-table :single-line="false" size="small">
            <thead>
              <tr>
                <th>{{ t("prototype.settings.secretName") }}</th>
                <th>{{ t("prototype.settings.secretScope") }}</th>
                <th>{{ t("prototype.settings.secretLastAudit") }}</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="secret in secrets" :key="secret.name">
                <td><code>{{ secret.name }}</code></td>
                <td>{{ secret.scope }}</td>
                <td>{{ secret.lastAudit }}</td>
              </tr>
            </tbody>
          </n-table>
          <n-button size="small" dashed style="margin-top: 8px">
            + {{ t("prototype.settings.addSecret") }}
          </n-button>
        </div>
      </n-card>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { reactive } from "vue";
import { NButton, NCard, NTable } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const secrets = reactive([
  { name: "GITLAB_PAT", scope: "user", lastAudit: "2026-07-01" },
  { name: "OPENAI_API_KEY", scope: "workbench", lastAudit: "2026-06-28" },
  { name: "SLACK_WEBHOOK", scope: "project", lastAudit: "2026-06-15" },
]);
</script>

<style scoped>
.page {
  max-width: 780px;
}

.page__header {
  margin-bottom: 24px;
}

.page__eyebrow {
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: var(--if-color-accent-strong);
  margin-bottom: 8px;
}

.page__header h1 {
  margin: 0 0 6px;
  font-size: 22px;
}

.page__header p {
  margin: 0;
  color: var(--if-color-muted);
  line-height: 1.6;
}

.section-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.85);
}

.card-body {
  display: grid;
  gap: 14px;
}
</style>
