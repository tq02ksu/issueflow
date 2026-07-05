<template>
  <app-shell active-key="settings-loop" prototype-mode>
    <div class="page">
      <div class="page__header">
        <div class="page__eyebrow">
          {{ t("prototype.settings.loopEyebrow") }}
        </div>
        <h1>{{ t("prototype.settings.loopTitle") }}</h1>
        <p>{{ t("prototype.settings.loopDescription") }}</p>
      </div>

      <div class="section-grid">
        <n-card :bordered="false" class="section-card">
          <template #header>{{ t("prototype.settings.role") }}</template>
          <div class="card-body">
            <div class="field">
              <n-input
                v-model:value="roleName"
                :placeholder="t('prototype.settings.rolePlaceholder')"
              />
            </div>
            <n-button type="primary" size="small" @click="saveRole">
              {{ t("common.actions.save") }}
            </n-button>
          </div>
        </n-card>

        <n-card :bordered="false" class="section-card">
          <template #header>{{ t("prototype.settings.soul") }}</template>
          <div class="card-body card-body--with-hint">
            <n-input
              v-model:value="soulText"
              type="textarea"
              :autosize="{ minRows: 6, maxRows: 14 }"
              :placeholder="t('prototype.settings.soulPlaceholder')"
            />
            <aside class="hint-box">
              <div class="hint-box__title">
                {{ t("prototype.settings.soulHintTitle") }}
              </div>
              <ul>
                <li>{{ t("prototype.settings.soulHint1") }}</li>
                <li>{{ t("prototype.settings.soulHint2") }}</li>
                <li>{{ t("prototype.settings.soulHint3") }}</li>
              </ul>
            </aside>
          </div>
        </n-card>

        <n-card :bordered="false" class="section-card">
          <template #header>{{ t("prototype.settings.principle") }}</template>
          <div class="card-body card-body--with-hint">
            <n-input
              v-model:value="principleText"
              type="textarea"
              :autosize="{ minRows: 6, maxRows: 14 }"
              :placeholder="t('prototype.settings.principlePlaceholder')"
            />
            <aside class="hint-box">
              <div class="hint-box__title">
                {{ t("prototype.settings.principleHintTitle") }}
              </div>
              <ul>
                <li>{{ t("prototype.settings.principleHint1") }}</li>
                <li>{{ t("prototype.settings.principleHint2") }}</li>
                <li>{{ t("prototype.settings.principleHint3") }}</li>
              </ul>
            </aside>
          </div>
        </n-card>

        <n-card :bordered="false" class="section-card">
          <template #header>{{ t("prototype.settings.design") }}</template>
          <div class="card-body card-body--with-hint">
            <n-input
              v-model:value="designText"
              type="textarea"
              :autosize="{ minRows: 6, maxRows: 14 }"
              :placeholder="t('prototype.settings.designPlaceholder')"
            />
            <aside class="hint-box">
              <div class="hint-box__title">
                {{ t("prototype.settings.designHintTitle") }}
              </div>
              <ul>
                <li>{{ t("prototype.settings.designHint1") }}</li>
                <li>{{ t("prototype.settings.designHint2") }}</li>
                <li>{{ t("prototype.settings.designHint3") }}</li>
              </ul>
            </aside>
          </div>
        </n-card>

        <n-card :bordered="false" class="section-card">
          <template #header>{{ t("prototype.settings.resources") }}</template>
          <div class="card-body card-body--with-hint">
            <div>
              <div class="resources-list">
                <div
                  v-for="res in resources"
                  :key="res.name"
                  class="resource-item"
                >
                  <div class="resource-item__main">
                    <span class="resource-item__name">{{ res.name }}</span>
                    <span class="resource-item__badge">{{ res.type }}</span>
                  </div>
                  <span class="resource-item__desc">{{ res.description }}</span>
                </div>
              </div>
              <n-upload
                :show-file-list="false"
                accept=".csv,.md,.txt,.json,.yaml,.toml"
                style="margin-top: 8px"
              >
                <n-button size="small" dashed>
                  + {{ t("prototype.settings.uploadResource") }}
                </n-button>
              </n-upload>
            </div>
            <aside class="hint-box">
              <div class="hint-box__title">
                {{ t("prototype.settings.resourcesHintTitle") }}
              </div>
              <ul>
                <li>{{ t("prototype.settings.resourcesHint1") }}</li>
                <li>{{ t("prototype.settings.resourcesHint2") }}</li>
                <li>{{ t("prototype.settings.resourcesHint3") }}</li>
              </ul>
            </aside>
          </div>
        </n-card>

        <n-card :bordered="false" class="section-card">
          <template #header>{{ t("prototype.settings.skill") }}</template>
          <div class="card-body card-body--with-hint">
            <div>
              <p class="section-note">
                {{ t("prototype.settings.skillNote") }}
              </p>
              <div class="skill-ref-cards">
                <div
                  v-for="skill in store.availableSkills"
                  :key="skill.id"
                  class="skill-ref-card"
                >
                  <strong>{{ skill.name }}</strong>
                  <span class="skill-ref-card__version">{{
                    skill.versions.find((v) => v.enabled)?.version ?? "—"
                  }}</span>
                </div>
              </div>
            </div>
            <aside class="hint-box">
              <div class="hint-box__title">
                {{ t("prototype.settings.skillHintTitle") }}
              </div>
              <ul>
                <li>{{ t("prototype.settings.skillHint1") }}</li>
                <li>{{ t("prototype.settings.skillHint2") }}</li>
                <li>{{ t("prototype.settings.skillHint3") }}</li>
              </ul>
            </aside>
          </div>
        </n-card>
      </div>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { NButton, NCard, NInput, NUpload } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { usePrototypeStore } from "@/stores/prototype.store";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();

const roleName = ref("");

watch(
  () => store.currentWorkbench?.role.name,
  (name) => {
    roleName.value = name ?? "";
  },
  { immediate: true },
);

function saveRole() {
  store.updateWorkbenchRole({
    name: roleName.value.trim(),
    personaSummary: store.currentWorkbench?.role.personaSummary ?? "",
    waysOfWorking: store.currentWorkbench?.role.waysOfWorking ?? [],
    goals: store.currentWorkbench?.role.goals ?? [],
  });
}

const soulText = ref(
  "Mission: Keep issues and MRs continuously moving without losing acceptance or test intent.\n\n" +
    "Long-term goals:\n" +
    "- Ship a workflow-first prototype that can be reviewed end to end.\n" +
    "- Tighten acceptance quality across all delivery objects.\n\n" +
    "Value priorities:\n" +
    "- Execution velocity without quality loss\n" +
    "- Explicit state visibility at all times\n" +
    "- Human approval before any write operation",
);

const principleText = ref(
  "Accuracy first over speed.\n" +
    "Never fabricate — state uncertainty explicitly when unsure.\n" +
    "All write operations require human approval.\n" +
    "Active clarification is allowed before acting.\n\n" +
    "Behaviour rules:\n" +
    "- No silent writes — every external action must be approved\n" +
    "- Object state is primary; conversation is secondary\n" +
    "- Memory interface must remain implementation-agnostic",
);

const designText = ref(
  "Execution flow: Plan → Act → Observe. Re-plan on failure. Max 5 steps per turn.\n" +
    "Memory context: Last 3 turns + engineering memory + governance memory.\n" +
    "Tool results: Summaries only, not full output.\n\n" +
    "Short-term goals:\n" +
    "- Complete Beta launch milestone by 2026-07-25\n" +
    "- Resolve review-state naming ambiguity\n\n" +
    "Output: Markdown, max 4000 chars, no JSON schema required.",
);

const resources = [
  {
    name: "acceptance-checklist.md",
    type: "md",
    description: "Standard acceptance criteria template",
  },
  {
    name: "state-machine-rules.csv",
    type: "csv",
    description: "Issue/MR state transition rules",
  },
  {
    name: "review-guide.txt",
    type: "txt",
    description: "MR review checklist and guidelines",
  },
  {
    name: "budget-policy.toml",
    type: "toml",
    description: "Per-turn token budget constraints",
  },
];
</script>

<style scoped>
.page {
  max-width: 820px;
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

.section-grid {
  display: grid;
  gap: 16px;
}

.section-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.85);
}

.card-body {
  display: grid;
  gap: 12px;
}

.card-body--with-hint {
  grid-template-columns: 1fr 180px;
  align-items: start;
}

@media (max-width: 720px) {
  .card-body--with-hint {
    grid-template-columns: 1fr;
  }
}

.hint-box {
  padding: 12px;
  border-radius: 8px;
  background: rgba(15, 118, 110, 0.05);
  border: 1px solid rgba(15, 118, 110, 0.1);
  font-size: 12px;
  line-height: 1.5;
  color: var(--if-color-muted);
}

.hint-box__title {
  font-weight: 700;
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--if-color-accent-strong);
  margin-bottom: 6px;
}

.hint-box ul {
  margin: 0;
  padding-left: 14px;
}

.hint-box li {
  margin-bottom: 4px;
}

.field {
  display: grid;
}

.section-note {
  margin: 0;
  font-size: 13px;
  color: var(--if-color-muted);
}

.skill-ref-cards {
  display: grid;
  gap: 8px;
}

.skill-ref-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  border-radius: 8px;
  background: rgba(15, 118, 110, 0.05);
  border: 1px solid rgba(15, 118, 110, 0.1);
  font-size: 13px;
}

.skill-ref-card strong {
  font-weight: 600;
}

.skill-ref-card__version {
  font-size: 12px;
  color: var(--if-color-accent-strong);
  font-weight: 600;
}

.resources-list {
  display: grid;
  gap: 6px;
}

.resource-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border-radius: 6px;
  background: rgba(0, 0, 0, 0.02);
  border: 1px solid rgba(0, 0, 0, 0.06);
  font-size: 13px;
}

.resource-item__main {
  display: flex;
  align-items: center;
  gap: 8px;
}

.resource-item__name {
  font-weight: 600;
  font-family: monospace;
  font-size: 12px;
}

.resource-item__badge {
  display: inline-block;
  padding: 1px 6px;
  border-radius: 4px;
  background: rgba(15, 118, 110, 0.08);
  color: var(--if-color-accent-strong);
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
}

.resource-item__desc {
  font-size: 12px;
  color: var(--if-color-muted);
}
</style>
