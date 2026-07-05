<template>
  <app-shell active-key="settings-loop" prototype-mode>
    <div class="page">
      <div class="page__header">
        <div class="page__eyebrow">{{ t("prototype.settings.loopEyebrow") }}</div>
        <h1>{{ t("prototype.settings.loopTitle") }}</h1>
        <p>{{ t("prototype.settings.loopDescription") }}</p>
      </div>

      <div class="section-grid">
        <n-collapse>
          <n-collapse-item :title="t('prototype.settings.soul')" name="soul">
            <div class="collapse-body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.mission") }}</label>
                <n-input
                  v-model:value="soul.mission"
                  type="textarea"
                  :autosize="{ minRows: 3, maxRows: 6 }"
                  :placeholder="t('prototype.settings.missionPlaceholder')"
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.longTermGoals") }}</label>
                <n-input
                  v-model:value="soul.goals"
                  type="textarea"
                  :autosize="{ minRows: 3, maxRows: 6 }"
                  :placeholder="t('prototype.settings.longTermGoalsPlaceholder')"
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.valuePriorities") }}</label>
                <div
                  v-for="(_, idx) in soul.priorities"
                  :key="idx"
                  class="settings-field__row"
                >
                  <n-input
                    v-model:value="soul.priorities[idx]"
                    :placeholder="t('prototype.settings.valuePrioritiesPlaceholder')"
                    size="small"
                  />
                  <n-button
                    size="small"
                    tertiary
                    type="error"
                    @click="soul.priorities.splice(idx, 1)"
                    :disabled="soul.priorities.length <= 1"
                  >
                    &times;
                  </n-button>
                </div>
                <n-button size="small" dashed @click="soul.priorities.push('')">
                  + {{ t("prototype.settings.addPriority") }}
                </n-button>
              </div>
              <n-button type="primary" size="small" style="margin-top: 8px">
                {{ t("common.actions.save") }}
              </n-button>
            </div>
          </n-collapse-item>

          <n-collapse-item :title="t('prototype.settings.principle')" name="principle">
            <div class="collapse-body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.accuracyVsSpeed") }}</label>
                <n-slider
                  v-model:value="principle.speedVsAccuracy"
                  :min="0"
                  :max="100"
                  :step="10"
                  :marks="{
                    0: t('prototype.settings.accuracyFirst'),
                    50: t('prototype.settings.balanced'),
                    100: t('prototype.settings.speedFirst'),
                  }"
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.allowClarification") }}</label>
                <n-switch v-model:value="principle.allowClarification" />
                <span class="settings-field__hint">{{ t("prototype.settings.allowClarificationHint") }}</span>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.dontFabricate") }}</label>
                <n-switch v-model:value="principle.dontFabricate" />
                <span class="settings-field__hint">{{ t("prototype.settings.dontFabricateHint") }}</span>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.stateUncertainty") }}</label>
                <n-switch v-model:value="principle.stateUncertainty" />
                <span class="settings-field__hint">{{ t("prototype.settings.stateUncertaintyHint") }}</span>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.writeThreshold") }}</label>
                <n-radio-group v-model:value="principle.writeThreshold">
                  <n-radio value="all">{{ t("prototype.settings.writeThresholdAll") }}</n-radio>
                  <n-radio value="low_risk_only">{{ t("prototype.settings.writeThresholdLowRisk") }}</n-radio>
                  <n-radio value="none">{{ t("prototype.settings.writeThresholdNone") }}</n-radio>
                </n-radio-group>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.customPrinciples") }}</label>
                <div
                  v-for="(_, idx) in principle.customPrinciples"
                  :key="idx"
                  class="settings-field__row"
                >
                  <n-input
                    v-model:value="principle.customPrinciples[idx]"
                    :placeholder="t('prototype.settings.customPrinciplesPlaceholder')"
                    size="small"
                  />
                  <n-button size="small" tertiary type="error" @click="principle.customPrinciples.splice(idx, 1)">
                    &times;
                  </n-button>
                </div>
                <n-button size="small" dashed @click="principle.customPrinciples.push('')">
                  + {{ t("prototype.settings.addPrinciple") }}
                </n-button>
              </div>
              <n-button type="primary" size="small" style="margin-top: 8px">
                {{ t("common.actions.save") }}
              </n-button>
            </div>
          </n-collapse-item>
        </n-collapse>

        <n-card :bordered="false" class="section-card">
          <template #header>{{ t("prototype.settings.execution") }}</template>
          <div class="collapse-body">
            <div class="settings-field">
              <label>{{ t("prototype.settings.currentGoals") }}</label>
              <n-input
                v-model:value="execution.shortTermGoals"
                type="textarea"
                :autosize="{ minRows: 3, maxRows: 6 }"
                :placeholder="t('prototype.settings.currentGoalsPlaceholder')"
              />
            </div>
            <div class="settings-field">
              <label>{{ t("prototype.settings.executionFlow") }}</label>
              <n-input
                v-model:value="execution.flowStrategy"
                type="textarea"
                :autosize="{ minRows: 2, maxRows: 4 }"
                :placeholder="t('prototype.settings.executionFlowPlaceholder')"
              />
            </div>
            <div class="settings-field">
              <label>{{ t("prototype.settings.outputConstraint") }}</label>
              <n-input
                v-model:value="execution.outputConstraint"
                type="textarea"
                :autosize="{ minRows: 2, maxRows: 4 }"
                :placeholder="t('prototype.settings.outputConstraintPlaceholder')"
              />
            </div>
            <n-button type="primary" size="small" style="margin-top: 8px">
              {{ t("common.actions.save") }}
            </n-button>
          </div>
        </n-card>

        <n-card :bordered="false" class="section-card">
          <template #header>{{ t("prototype.settings.skill") }}</template>
          <div class="collapse-body">
            <SkillVersionPanel
              :skills="store.availableSkills"
              :active-version-id="store.currentWorkbench?.activeSkillVersionId"
              @set-active-version="store.setActiveSkillVersion"
              @toggle-version="store.toggleSkillVersion"
              @mock-upload="store.mockUploadSkill"
            />
          </div>
        </n-card>
      </div>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { reactive } from "vue";
import {
  NButton,
  NCard,
  NCollapse,
  NCollapseItem,
  NInput,
  NRadio,
  NRadioGroup,
  NSlider,
  NSwitch,
} from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import SkillVersionPanel from "@/components/prototype/SkillVersionPanel.vue";
import { usePrototypeStore } from "@/stores/prototype.store";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();

const soul = reactive({
  mission: "Keep issues and MRs continuously moving without losing acceptance or test intent.",
  goals: "Ship a workflow-first prototype that can be reviewed end to end.\nTighten acceptance quality across all delivery objects.",
  priorities: [
    "Execution velocity without quality loss",
    "Explicit state visibility at all times",
    "Human approval before any write operation",
  ],
});

const principle = reactive({
  speedVsAccuracy: 20,
  allowClarification: true,
  dontFabricate: true,
  stateUncertainty: true,
  writeThreshold: "low_risk_only" as "all" | "low_risk_only" | "none",
  customPrinciples: [
    "No silent writes — every external action must be approved",
    "Uncertainty must be stated, not hidden",
    "Object state is primary; conversation is secondary",
  ],
});

const execution = reactive({
  shortTermGoals:
    "Complete Beta launch milestone by 2026-07-25.\nResolve review-state naming ambiguity.",
  flowStrategy: "Plan → Act → Observe. Re-plan on failure. Max 5 steps per turn.",
  outputConstraint: "Output in Markdown. Max 4000 chars. No schema required.",
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

.section-grid {
  display: grid;
  gap: 16px;
}

.section-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.85);
}

.collapse-body {
  display: grid;
  gap: 14px;
  padding: 4px 0;
}

.settings-field {
  display: grid;
  gap: 6px;
}

.settings-field label {
  font-size: 13px;
  font-weight: 600;
}

.settings-field__hint {
  font-size: 12px;
  color: var(--if-color-muted);
  margin-top: 2px;
}

.settings-field__row {
  display: flex;
  gap: 8px;
  align-items: center;
}

.settings-field__row > :first-child {
  flex: 1;
}
</style>
