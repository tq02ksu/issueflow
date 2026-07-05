<template>
  <app-shell active-key="settings" prototype-mode>
    <div class="settings-root">
      <div class="settings-sidebar">
        <div class="settings-sidebar__title">
          {{ t("prototype.settings.loopConfig") }}
        </div>
        <n-menu
          :value="activeSection"
          :options="sectionMenuOptions"
          :root-indent="16"
          :indent="16"
          @update:value="activeSection = $event"
        />
      </div>
      <div class="settings-content">
        <section v-if="activeSection === 'profile'" class="settings-section">
          <n-card :bordered="false" class="settings-card">
            <template #header>{{ t("prototype.settings.profile") }}</template>
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.displayName") }}</label>
                <n-input
                  v-model:value="profileName"
                  :placeholder="t('prototype.settings.displayNamePlaceholder')"
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.personality") }}</label>
                <n-input
                  v-model:value="profilePersonality"
                  type="textarea"
                  :autosize="{ minRows: 3, maxRows: 5 }"
                  :placeholder="t('prototype.settings.personalityPlaceholder')"
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.waysOfWorking") }}</label>
                <n-input
                  v-model:value="profileWays"
                  type="textarea"
                  :autosize="{ minRows: 3, maxRows: 6 }"
                  :placeholder="
                    t('prototype.settings.waysOfWorkingPlaceholder')
                  "
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.defaultGoal") }}</label>
                <n-input
                  v-model:value="profileGoal"
                  type="textarea"
                  :autosize="{ minRows: 2, maxRows: 4 }"
                  :placeholder="t('prototype.settings.defaultGoalPlaceholder')"
                />
              </div>
              <n-button type="primary" @click="saveProfile">
                {{ t("common.actions.save") }}
              </n-button>
            </div>
          </n-card>
        </section>

        <section v-if="activeSection === 'soul'" class="settings-section">
          <div class="settings-section__header">
            <div class="settings-section__eyebrow">
              {{ t("prototype.settings.soulEyebrow") }}
            </div>
            <h2>{{ t("prototype.settings.soulTitle") }}</h2>
            <p>{{ t("prototype.settings.soulDescription") }}</p>
          </div>
          <n-card :bordered="false" class="settings-card">
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.mission") }}</label>
                <n-input
                  v-model:value="loopSoul.mission"
                  type="textarea"
                  :autosize="{ minRows: 3, maxRows: 5 }"
                  :placeholder="t('prototype.settings.missionPlaceholder')"
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.longTermGoals") }}</label>
                <n-input
                  v-model:value="loopSoul.goals"
                  type="textarea"
                  :autosize="{ minRows: 3, maxRows: 6 }"
                  :placeholder="
                    t('prototype.settings.longTermGoalsPlaceholder')
                  "
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.valuePriorities") }}</label>
                <div
                  v-for="(_, idx) in loopSoul.priorities"
                  :key="idx"
                  class="settings-field__row"
                >
                  <n-input
                    v-model:value="loopSoul.priorities[idx]"
                    :placeholder="
                      t('prototype.settings.valuePrioritiesPlaceholder')
                    "
                    size="small"
                  />
                  <n-button
                    size="small"
                    tertiary
                    type="error"
                    @click="loopSoul.priorities.splice(idx, 1)"
                    :disabled="loopSoul.priorities.length <= 1"
                  >
                    &times;
                  </n-button>
                </div>
                <n-button
                  size="small"
                  dashed
                  @click="loopSoul.priorities.push('')"
                >
                  + {{ t("prototype.settings.addPriority") }}
                </n-button>
              </div>
              <n-button type="primary" @click="saveSoul">
                {{ t("common.actions.save") }}
              </n-button>
            </div>
          </n-card>
        </section>

        <section v-if="activeSection === 'principle'" class="settings-section">
          <div class="settings-section__header">
            <div class="settings-section__eyebrow">
              {{ t("prototype.settings.principleEyebrow") }}
            </div>
            <h2>{{ t("prototype.settings.principleTitle") }}</h2>
            <p>{{ t("prototype.settings.principleDescription") }}</p>
          </div>
          <n-card :bordered="false" class="settings-card">
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.accuracyVsSpeed") }}</label>
                <n-slider
                  v-model:value="loopPrinciple.speedVsAccuracy"
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
                <n-switch v-model:value="loopPrinciple.allowClarification" />
                <span class="settings-field__hint">{{
                  t("prototype.settings.allowClarificationHint")
                }}</span>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.dontFabricate") }}</label>
                <n-switch v-model:value="loopPrinciple.dontFabricate" />
                <span class="settings-field__hint">{{
                  t("prototype.settings.dontFabricateHint")
                }}</span>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.stateUncertainty") }}</label>
                <n-switch v-model:value="loopPrinciple.stateUncertainty" />
                <span class="settings-field__hint">{{
                  t("prototype.settings.stateUncertaintyHint")
                }}</span>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.writeThreshold") }}</label>
                <n-radio-group v-model:value="loopPrinciple.writeThreshold">
                  <n-radio value="all">{{
                    t("prototype.settings.writeThresholdAll")
                  }}</n-radio>
                  <n-radio value="low_risk_only">{{
                    t("prototype.settings.writeThresholdLowRisk")
                  }}</n-radio>
                  <n-radio value="none">{{
                    t("prototype.settings.writeThresholdNone")
                  }}</n-radio>
                </n-radio-group>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.customPrinciples") }}</label>
                <div
                  v-for="(_, idx) in loopPrinciple.customPrinciples"
                  :key="idx"
                  class="settings-field__row"
                >
                  <n-input
                    v-model:value="loopPrinciple.customPrinciples[idx]"
                    :placeholder="
                      t('prototype.settings.customPrinciplesPlaceholder')
                    "
                    size="small"
                  />
                  <n-button
                    size="small"
                    tertiary
                    type="error"
                    @click="loopPrinciple.customPrinciples.splice(idx, 1)"
                  >
                    &times;
                  </n-button>
                </div>
                <n-button
                  size="small"
                  dashed
                  @click="loopPrinciple.customPrinciples.push('')"
                >
                  + {{ t("prototype.settings.addPrinciple") }}
                </n-button>
              </div>
              <n-button type="primary" @click="savePrinciple">
                {{ t("common.actions.save") }}
              </n-button>
            </div>
          </n-card>
        </section>

        <section v-if="activeSection === 'execution'" class="settings-section">
          <div class="settings-section__header">
            <div class="settings-section__eyebrow">
              {{ t("prototype.settings.executionEyebrow") }}
            </div>
            <h2>{{ t("prototype.settings.executionTitle") }}</h2>
            <p>{{ t("prototype.settings.executionDescription") }}</p>
          </div>

          <n-card :bordered="false" class="settings-card">
            <template #header>{{
              t("prototype.settings.shortTermGoals")
            }}</template>
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.currentGoals") }}</label>
                <n-input
                  v-model:value="loopExecution.shortTermGoals"
                  type="textarea"
                  :autosize="{ minRows: 3, maxRows: 6 }"
                  :placeholder="t('prototype.settings.currentGoalsPlaceholder')"
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.goalBreakdown") }}</label>
                <div
                  v-for="(_, idx) in loopExecution.goalSteps"
                  :key="idx"
                  class="settings-field__row"
                >
                  <n-input
                    v-model:value="loopExecution.goalSteps[idx]"
                    :placeholder="t('prototype.settings.goalStepsPlaceholder')"
                    size="small"
                  />
                  <n-button
                    size="small"
                    tertiary
                    type="error"
                    @click="loopExecution.goalSteps.splice(idx, 1)"
                  >
                    &times;
                  </n-button>
                </div>
                <n-button
                  size="small"
                  dashed
                  @click="loopExecution.goalSteps.push('')"
                >
                  + {{ t("prototype.settings.addStep") }}
                </n-button>
              </div>
            </div>
          </n-card>

          <n-card :bordered="false" class="settings-card">
            <template #header>{{
              t("prototype.settings.designTitle")
            }}</template>
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.executionFlow") }}</label>
                <n-radio-group v-model:value="loopExecution.flowStrategy">
                  <n-radio value="plan_act_observe">{{
                    t("prototype.settings.flowPlanActObserve")
                  }}</n-radio>
                  <n-radio value="single_step">{{
                    t("prototype.settings.flowSingleStep")
                  }}</n-radio>
                  <n-radio value="multi_step">{{
                    t("prototype.settings.flowMultiStep")
                  }}</n-radio>
                </n-radio-group>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.replanningStrategy") }}</label>
                <n-radio-group v-model:value="loopExecution.replanningStrategy">
                  <n-radio value="always">{{
                    t("prototype.settings.replanAlways")
                  }}</n-radio>
                  <n-radio value="on_failure">{{
                    t("prototype.settings.replanOnFailure")
                  }}</n-radio>
                  <n-radio value="on_request">{{
                    t("prototype.settings.replanOnRequest")
                  }}</n-radio>
                  <n-radio value="never">{{
                    t("prototype.settings.replanNever")
                  }}</n-radio>
                </n-radio-group>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.maxStepsPerTurn") }}</label>
                <n-input-number
                  v-model:value="loopExecution.maxStepsPerTurn"
                  :min="1"
                  :max="50"
                />
              </div>
            </div>
          </n-card>

          <n-card :bordered="false" class="settings-card">
            <template #header>{{
              t("prototype.settings.knowledgeTitle")
            }}</template>
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.memoryContextWindow") }}</label>
                <n-radio-group v-model:value="loopExecution.memoryContext">
                  <n-radio value="last_turn">{{
                    t("prototype.settings.memoryLastTurn")
                  }}</n-radio>
                  <n-radio value="last_3_turns">{{
                    t("prototype.settings.memoryLast3Turns")
                  }}</n-radio>
                  <n-radio value="full_history">{{
                    t("prototype.settings.memoryFullHistory")
                  }}</n-radio>
                  <n-radio value="loop_only">{{
                    t("prototype.settings.memoryLoopOnly")
                  }}</n-radio>
                </n-radio-group>
              </div>
              <div class="settings-field">
                <label>{{
                  t("prototype.settings.includeEngineeringMemory")
                }}</label>
                <n-switch
                  v-model:value="loopExecution.includeEngineeringMemory"
                />
              </div>
              <div class="settings-field">
                <label>{{
                  t("prototype.settings.includeGovernanceMemory")
                }}</label>
                <n-switch
                  v-model:value="loopExecution.includeGovernanceMemory"
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.toolResultsStrategy") }}</label>
                <n-radio-group
                  v-model:value="loopExecution.toolResultsStrategy"
                >
                  <n-radio value="full">{{
                    t("prototype.settings.toolResultsFull")
                  }}</n-radio>
                  <n-radio value="summary">{{
                    t("prototype.settings.toolResultsSummary")
                  }}</n-radio>
                  <n-radio value="errors_only">{{
                    t("prototype.settings.toolResultsErrors")
                  }}</n-radio>
                </n-radio-group>
              </div>
            </div>
          </n-card>

          <n-card :bordered="false" class="settings-card">
            <template #header>{{ t("prototype.settings.ruleTitle") }}</template>
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.outputFormat") }}</label>
                <n-radio-group v-model:value="loopExecution.outputFormat">
                  <n-radio value="json">{{
                    t("prototype.settings.outputJson")
                  }}</n-radio>
                  <n-radio value="markdown">{{
                    t("prototype.settings.outputMarkdown")
                  }}</n-radio>
                  <n-radio value="plain">{{
                    t("prototype.settings.outputPlain")
                  }}</n-radio>
                </n-radio-group>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.maxOutputLength") }}</label>
                <n-input-number
                  v-model:value="loopExecution.maxOutputChars"
                  :min="500"
                  :max="50000"
                  :step="500"
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.requireSchema") }}</label>
                <n-switch v-model:value="loopExecution.requireSchema" />
                <span class="settings-field__hint">{{
                  t("prototype.settings.requireSchemaHint")
                }}</span>
              </div>
            </div>
          </n-card>

          <n-button type="primary" @click="saveExecution">
            {{ t("common.actions.save") }}
          </n-button>
        </section>

        <section v-if="activeSection === 'skill'" class="settings-section">
          <div class="settings-section__header">
            <div class="settings-section__eyebrow">
              {{ t("prototype.settings.skillEyebrow") }}
            </div>
            <h2>{{ t("prototype.settings.skillTitle") }}</h2>
            <p>{{ t("prototype.settings.skillDescription") }}</p>
          </div>
          <SkillVersionPanel
            :skills="store.availableSkills"
            :active-version-id="store.currentWorkbench?.activeSkillVersionId"
            @set-active-version="store.setActiveSkillVersion"
            @toggle-version="store.toggleSkillVersion"
            @mock-upload="store.mockUploadSkill"
          />
        </section>

        <section v-if="activeSection === 'projects'" class="settings-section">
          <div class="settings-section__header">
            <div class="settings-section__eyebrow">
              {{ t("prototype.settings.projectsEyebrow") }}
            </div>
            <h2>{{ t("prototype.settings.projectsTitle") }}</h2>
            <p>{{ t("prototype.settings.projectsDescription") }}</p>
          </div>
          <n-card :bordered="false" class="settings-card">
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.projectBinding") }}</label>
                <n-input
                  v-model:value="projectConfig.projectPath"
                  :placeholder="t('prototype.settings.projectPathPlaceholder')"
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.projectId") }}</label>
                <n-input-number
                  v-model:value="projectConfig.projectId"
                  :min="1"
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.defaultBranch") }}</label>
                <n-input
                  v-model:value="projectConfig.defaultBranch"
                  placeholder="main"
                />
              </div>
              <n-button type="primary" @click="saveProjects">
                {{ t("common.actions.save") }}
              </n-button>
            </div>
          </n-card>
        </section>

        <section
          v-if="activeSection === 'integrations'"
          class="settings-section"
        >
          <div class="settings-section__header">
            <div class="settings-section__eyebrow">
              {{ t("prototype.settings.integrationsEyebrow") }}
            </div>
            <h2>{{ t("prototype.settings.integrationsTitle") }}</h2>
            <p>{{ t("prototype.settings.integrationsDescription") }}</p>
          </div>
          <n-card :bordered="false" class="settings-card">
            <template #header>{{
              t("prototype.settings.gitlabAuth")
            }}</template>
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.authMethod") }}</label>
                <n-radio-group v-model:value="integrations.gitlabAuthMethod">
                  <n-radio value="oidc">OIDC</n-radio>
                  <n-radio value="pat">{{
                    t("prototype.settings.patLabel")
                  }}</n-radio>
                </n-radio-group>
              </div>
              <div
                v-if="integrations.gitlabAuthMethod === 'pat'"
                class="settings-field"
              >
                <label>{{ t("prototype.settings.patToken") }}</label>
                <n-input
                  v-model:value="integrations.gitlabPat"
                  type="password"
                  show-password-on="click"
                  placeholder="glpat-..."
                />
              </div>
            </div>
          </n-card>
          <n-card :bordered="false" class="settings-card">
            <template #header>{{
              t("prototype.settings.agentProviders")
            }}</template>
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.providersLabel") }}</label>
                <div
                  v-for="(_, idx) in integrations.agentProviders"
                  :key="idx"
                  class="settings-field__row"
                >
                  <n-input
                    v-model:value="integrations.agentProviders[idx]"
                    :placeholder="t('prototype.settings.providerPlaceholder')"
                  />
                  <n-button
                    size="small"
                    tertiary
                    type="error"
                    @click="integrations.agentProviders.splice(idx, 1)"
                    :disabled="integrations.agentProviders.length <= 1"
                  >
                    &times;
                  </n-button>
                </div>
                <n-button
                  size="small"
                  dashed
                  @click="integrations.agentProviders.push('')"
                >
                  + {{ t("prototype.settings.addProvider") }}
                </n-button>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.memoryService") }}</label>
                <n-select
                  v-model:value="integrations.memoryService"
                  :options="[
                    { label: 'mem0', value: 'mem0' },
                    { label: 'ChromaDB', value: 'chromadb' },
                    { label: 'Qdrant', value: 'qdrant' },
                    { label: 'None', value: 'none' },
                  ]"
                />
              </div>
              <n-button type="primary" @click="saveIntegrations">
                {{ t("common.actions.save") }}
              </n-button>
            </div>
          </n-card>
        </section>

        <section
          v-if="activeSection === 'environments'"
          class="settings-section"
        >
          <div class="settings-section__header">
            <div class="settings-section__eyebrow">
              {{ t("prototype.settings.environmentsEyebrow") }}
            </div>
            <h2>{{ t("prototype.settings.environmentsTitle") }}</h2>
            <p>{{ t("prototype.settings.environmentsDescription") }}</p>
          </div>
          <n-card :bordered="false" class="settings-card">
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.defaultEnvironment") }}</label>
                <n-select
                  v-model:value="envConfig.defaultEnv"
                  :options="[
                    { label: 'production', value: 'production' },
                    { label: 'staging', value: 'staging' },
                    { label: 'development', value: 'development' },
                    { label: 'sandbox', value: 'sandbox' },
                  ]"
                />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.envVariables") }}</label>
                <div
                  v-for="(_, idx) in envConfig.variables"
                  :key="idx"
                  class="settings-field__row settings-field__row--kv"
                >
                  <n-input
                    v-model:value="envConfig.variables[idx].key"
                    :placeholder="t('prototype.settings.envKeyPlaceholder')"
                    size="small"
                  />
                  <n-input
                    v-model:value="envConfig.variables[idx].value"
                    :placeholder="t('prototype.settings.envValuePlaceholder')"
                    type="password"
                    show-password-on="click"
                    size="small"
                  />
                  <n-button
                    size="small"
                    tertiary
                    type="error"
                    @click="envConfig.variables.splice(idx, 1)"
                  >
                    &times;
                  </n-button>
                </div>
                <n-button
                  size="small"
                  dashed
                  @click="envConfig.variables.push({ key: '', value: '' })"
                >
                  + {{ t("prototype.settings.addVariable") }}
                </n-button>
              </div>
              <n-button type="primary" @click="saveEnvironments">
                {{ t("common.actions.save") }}
              </n-button>
            </div>
          </n-card>
        </section>

        <section v-if="activeSection === 'secrets'" class="settings-section">
          <div class="settings-section__header">
            <div class="settings-section__eyebrow">
              {{ t("prototype.settings.secretsEyebrow") }}
            </div>
            <h2>{{ t("prototype.settings.secretsTitle") }}</h2>
            <p>{{ t("prototype.settings.secretsDescription") }}</p>
          </div>
          <n-card :bordered="false" class="settings-card">
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.secretsList") }}</label>
                <n-table :single-line="false" size="small">
                  <thead>
                    <tr>
                      <th>{{ t("prototype.settings.secretName") }}</th>
                      <th>{{ t("prototype.settings.secretScope") }}</th>
                      <th>{{ t("prototype.settings.secretLastAudit") }}</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="secret in secretsList" :key="secret.name">
                      <td>
                        <code>{{ secret.name }}</code>
                      </td>
                      <td>{{ secret.scope }}</td>
                      <td>{{ secret.lastAudit }}</td>
                    </tr>
                  </tbody>
                </n-table>
                <n-button size="small" dashed style="margin-top: 8px">
                  + {{ t("prototype.settings.addSecret") }}
                </n-button>
              </div>
            </div>
          </n-card>
        </section>

        <section
          v-if="activeSection === 'notifications'"
          class="settings-section"
        >
          <div class="settings-section__header">
            <div class="settings-section__eyebrow">
              {{ t("prototype.settings.notificationsEyebrow") }}
            </div>
            <h2>{{ t("prototype.settings.notificationsTitle") }}</h2>
            <p>{{ t("prototype.settings.notificationsDescription") }}</p>
          </div>
          <n-card :bordered="false" class="settings-card">
            <div class="settings-card__body">
              <div class="settings-field">
                <label>{{ t("prototype.settings.dailyReport") }}</label>
                <n-switch v-model:value="notifications.dailyReport" />
                <span class="settings-field__hint">{{
                  t("prototype.settings.dailyReportHint")
                }}</span>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.weeklyReport") }}</label>
                <n-switch v-model:value="notifications.weeklyReport" />
                <span class="settings-field__hint">{{
                  t("prototype.settings.weeklyReportHint")
                }}</span>
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.approvalReminder") }}</label>
                <n-switch v-model:value="notifications.approvalReminder" />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.riskAlert") }}</label>
                <n-switch v-model:value="notifications.riskAlert" />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.turnCompleted") }}</label>
                <n-switch v-model:value="notifications.turnCompleted" />
              </div>
              <div class="settings-field">
                <label>{{ t("prototype.settings.notificationChannel") }}</label>
                <n-checkbox-group v-model:value="notifications.channels">
                  <n-checkbox value="email">{{
                    t("prototype.settings.channelEmail")
                  }}</n-checkbox>
                  <n-checkbox value="webhook">{{
                    t("prototype.settings.channelWebhook")
                  }}</n-checkbox>
                  <n-checkbox value="in_app">{{
                    t("prototype.settings.channelInApp")
                  }}</n-checkbox>
                </n-checkbox-group>
              </div>
              <n-button type="primary" @click="saveNotifications">
                {{ t("common.actions.save") }}
              </n-button>
            </div>
          </n-card>
        </section>
      </div>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { reactive, ref, watch } from "vue";
import type { MenuOption } from "naive-ui";
import {
  NButton,
  NCard,
  NCheckbox,
  NCheckboxGroup,
  NInput,
  NInputNumber,
  NMenu,
  NRadio,
  NRadioGroup,
  NSelect,
  NSlider,
  NSwitch,
  NTable,
} from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import SkillVersionPanel from "@/components/prototype/SkillVersionPanel.vue";
import { usePrototypeStore } from "@/stores/prototype.store";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();

const activeSection = ref("profile");

const sectionMenuOptions: MenuOption[] = [
  { label: () => t("prototype.settings.profile"), key: "profile" },
  {
    label: () => t("prototype.settings.loopConfig"),
    key: "loop-config",
    children: [
      { label: () => t("prototype.settings.soul"), key: "soul" },
      { label: () => t("prototype.settings.principle"), key: "principle" },
      { label: () => t("prototype.settings.execution"), key: "execution" },
      { label: () => t("prototype.settings.skill"), key: "skill" },
    ],
  },
  { label: () => t("prototype.settings.projects"), key: "projects" },
  { label: () => t("prototype.settings.integrations"), key: "integrations" },
  { label: () => t("prototype.settings.environments"), key: "environments" },
  { label: () => t("prototype.settings.secrets"), key: "secrets" },
  { label: () => t("prototype.settings.notifications"), key: "notifications" },
];

// Profile (user soul + workbench role)
const profileName = ref("");
const profilePersonality = ref("");
const profileWays = ref("");
const profileGoal = ref("");

watch(
  () => [store.currentUserSoul, store.currentWorkbench],
  () => {
    profileName.value = store.currentUserSoul?.name ?? "";
    profilePersonality.value = store.currentUserSoul?.personality ?? "";
    profileWays.value = (store.currentUserSoul?.waysOfWorking ?? []).join("\n");
    profileGoal.value = store.currentUserSoul?.defaultGoal ?? "";
  },
  { immediate: true },
);

function saveProfile() {
  store.updateUserSoul({
    personality: profilePersonality.value.trim(),
    waysOfWorking: profileWays.value
      .split("\n")
      .map((s) => s.trim())
      .filter(Boolean),
    defaultGoal: profileGoal.value.trim(),
  });
}

// LOOP SOUL
const loopSoul = reactive({
  mission:
    "Keep issues and MRs continuously moving without losing acceptance or test intent.",
  goals:
    "Ship a workflow-first prototype that can be reviewed end to end.\nTighten acceptance quality across all delivery objects.",
  priorities: [
    "Execution velocity without quality loss",
    "Explicit state visibility at all times",
    "Human approval before any write operation",
  ],
});

function saveSoul() {
  // mock save
}

// LOOP PRINCIPLE
const loopPrinciple = reactive({
  speedVsAccuracy: 20,
  allowClarification: true,
  dontFabricate: true,
  stateUncertainty: true,
  writeThreshold: "low_risk_only" as "all" | "low_risk_only" | "none",
  customPrinciples: [
    "No silent writes — every external action must be approved",
    "Uncertainty must be stated, not hidden",
    "Object state is primary; conversation is secondary",
    "Memory interface must remain implementation-agnostic",
  ],
});

function savePrinciple() {
  // mock save
}

// LOOP EXECUTION
const loopExecution = reactive({
  shortTermGoals:
    "Complete Beta launch milestone by 2026-07-25.\nResolve review-state naming ambiguity across issues and MRs.",
  goalSteps: [
    "Align issue/MR state vocabulary",
    "Implement workflow state cards",
    "Add milestone summary aggregation",
    "Test acceptance language precision",
  ],
  flowStrategy: "plan_act_observe" as string,
  replanningStrategy: "on_failure" as string,
  maxStepsPerTurn: 5,
  memoryContext: "last_3_turns" as string,
  includeEngineeringMemory: true,
  includeGovernanceMemory: true,
  toolResultsStrategy: "summary" as string,
  outputFormat: "markdown" as string,
  maxOutputChars: 4000,
  requireSchema: false,
});

function saveExecution() {
  // mock save
}

// Projects
const projectConfig = reactive({
  projectPath: "demo/alpha-delivery",
  projectId: 102,
  defaultBranch: "main",
});

function saveProjects() {
  // mock save
}

// Integrations
const integrations = reactive({
  gitlabAuthMethod: "oidc" as "oidc" | "pat",
  gitlabPat: "",
  agentProviders: ["OpenCode (default)", "Codex (fallback)"],
  memoryService: "mem0" as string,
});

function saveIntegrations() {
  // mock save
}

// Environment Profiles
const envConfig = reactive({
  defaultEnv: "production" as string,
  variables: [
    { key: "GITLAB_URL", value: "https://gitlab.example.com" },
    { key: "MEMORY_STORE_URL", value: "https://mem0.internal:8080" },
  ],
});

function saveEnvironments() {
  // mock save
}

// Secrets & Access
const secretsList = reactive([
  { name: "GITLAB_PAT", scope: "user", lastAudit: "2026-07-01" },
  { name: "OPENAI_API_KEY", scope: "workbench", lastAudit: "2026-06-28" },
  { name: "SLACK_WEBHOOK", scope: "project", lastAudit: "2026-06-15" },
]);

// Notifications
const notifications = reactive({
  dailyReport: true,
  weeklyReport: true,
  approvalReminder: true,
  riskAlert: true,
  turnCompleted: false,
  channels: ["email", "in_app"] as string[],
});

function saveNotifications() {
  // mock save
}
</script>

<style scoped>
.settings-root {
  display: flex;
  gap: 0;
  min-height: calc(100vh - 120px);
}

.settings-sidebar {
  width: 220px;
  flex-shrink: 0;
  border-right: 1px solid var(--n-border-color, #e8e8e8);
  padding: 16px 0;
  background: var(--n-color-embedded, rgba(255, 250, 242, 0.6));
}

.settings-sidebar__title {
  padding: 0 16px 8px;
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: var(--if-color-muted);
}

.settings-content {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
}

.settings-section {
  display: grid;
  gap: 20px;
  max-width: 780px;
}

.settings-section__header {
  margin-bottom: 4px;
}

.settings-section__eyebrow {
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: var(--if-color-accent-strong);
  margin-bottom: 8px;
}

.settings-section__header h2 {
  margin: 0 0 6px;
  font-size: 22px;
}

.settings-section__header p {
  margin: 0;
  color: var(--if-color-muted);
  line-height: 1.6;
  font-size: 14px;
}

.settings-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.85);
}

.settings-card__body {
  display: grid;
  gap: 16px;
}

.settings-field {
  display: grid;
  gap: 6px;
}

.settings-field label {
  font-size: 13px;
  font-weight: 600;
  color: var(--if-color-text);
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

.settings-field__row--kv {
  display: grid;
  grid-template-columns: 1fr 1fr auto;
}

.settings-field__row > :first-child {
  flex: 1;
}

@media (max-width: 900px) {
  .settings-root {
    flex-direction: column;
  }

  .settings-sidebar {
    width: 100%;
    border-right: none;
    border-bottom: 1px solid var(--n-border-color, #e8e8e8);
    padding: 8px 0;
  }

  .settings-content {
    padding: 16px;
  }
}
</style>
