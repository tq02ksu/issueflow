<template>
  <app-shell active-key="settings-integrations" prototype-mode>
    <div class="page">
      <div class="page__header">
        <div class="page__eyebrow">
          {{ t("prototype.settings.integrationsEyebrow") }}
        </div>
        <h1>{{ t("prototype.settings.integrationsTitle") }}</h1>
        <p>{{ t("prototype.settings.integrationsDescription") }}</p>
      </div>

      <n-card :bordered="false" class="section-card">
        <template #header>{{ t("prototype.settings.projects") }}</template>
        <div class="card-body">
          <div class="settings-field">
            <label>{{ t("prototype.settings.projectBinding") }}</label>
            <n-input
              v-model:value="project.projectPath"
              :placeholder="t('prototype.settings.projectPathPlaceholder')"
            />
          </div>
          <div class="settings-field">
            <label>{{ t("prototype.settings.projectId") }}</label>
            <n-input-number v-model:value="project.projectId" :min="1" />
          </div>
          <div class="settings-field">
            <label>{{ t("prototype.settings.defaultBranch") }}</label>
            <n-input v-model:value="project.defaultBranch" placeholder="main" />
          </div>
          <n-button type="primary" size="small" style="margin-top: 8px">
            {{ t("common.actions.save") }}
          </n-button>
        </div>
      </n-card>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { reactive } from "vue";
import { NButton, NCard, NInput, NInputNumber } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const project = reactive({
  projectPath: "demo/alpha-delivery",
  projectId: 102,
  defaultBranch: "main",
});
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

.settings-field {
  display: grid;
  gap: 6px;
}

.settings-field label {
  font-size: 13px;
  font-weight: 600;
}
</style>
