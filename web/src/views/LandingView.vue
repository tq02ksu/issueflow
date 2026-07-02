<template>
  <main
    v-if="isMockMode"
    class="landing landing--mock"
    :class="{ 'landing--zh': locale === 'zh-CN' }"
  >
    <section class="landing__hero">
      <div class="landing__hero-topbar">
        <LanguageSwitcher />
      </div>

      <div class="landing__hero-copy">
        <div class="landing__eyebrow">{{ t("landing.eyebrow") }}</div>
        <h1 class="landing__hero-title">
          <span>{{ t("landing.titlePrimary") }}</span>
          <span>{{ t("landing.titleSecondary") }}</span>
        </h1>
        <p class="landing__lead">{{ t("landing.lead") }}</p>
        <p class="landing__impact">{{ t("landing.impact") }}</p>
        <div class="landing__actions">
          <n-button tag="a" href="/workbench" type="primary" size="large">
            {{ t("common.actions.openPrototype") }}
          </n-button>
          <n-button
            tag="a"
            href="/settings"
            size="large"
            class="landing__settings-button"
          >
            {{ t("common.actions.reviewSettings") }}
          </n-button>
        </div>
      </div>

      <div class="landing__hero-diagram">
        <div class="landing__diagram" data-testid="landing-diagram">
          <article class="landing__node landing__node--engine landing__node--engine-core">
            <div class="landing__node-icon">LE</div>
            <div>
              <strong>{{ t("landing.diagram.engine.title") }}</strong>
              <p>{{ t("landing.diagram.engine.label") }}</p>
            </div>
            <button
              class="landing__hint"
              type="button"
              :aria-label="t('landing.diagram.engine.title')"
              data-testid="diagram-tooltip-trigger"
              :aria-expanded="isTooltipOpen('engine')"
              @click="toggleTooltip('engine')"
            >
              ?
            </button>
            <div
              v-show="isTooltipOpen('engine')"
              class="landing__tooltip"
              :data-tooltip-open="isTooltipOpen('engine') ? 'true' : undefined"
            >
              {{ t("landing.diagram.engine.description") }}
            </div>
          </article>

          <div class="landing__diagram-group">
            <div class="landing__group-label">{{ t("landing.groups.executionObjects") }}</div>
            <div class="landing__diagram-ring landing__diagram-ring--objects">
              <article
                v-for="item in diagramObjects"
                :key="item.id"
                class="landing__node landing__node--object"
              >
                <div class="landing__node-icon">{{ item.icon }}</div>
                <div>
                  <strong>{{ item.title }}</strong>
                  <p>{{ item.label }}</p>
                </div>
                <button
                  class="landing__hint"
                  type="button"
                  :aria-label="item.title"
                  data-testid="diagram-tooltip-trigger"
                  :aria-expanded="isTooltipOpen(item.id)"
                  @click="toggleTooltip(item.id)"
                >
                  ?
                </button>
                <div
                  v-show="isTooltipOpen(item.id)"
                  class="landing__tooltip"
                  :data-tooltip-open="isTooltipOpen(item.id) ? 'true' : undefined"
                >
                  {{ item.description }}
                </div>
              </article>
            </div>
          </div>

          <div class="landing__diagram-group">
            <div class="landing__group-label">{{ t("landing.groups.controlLayers") }}</div>
            <div class="landing__diagram-ring landing__diagram-ring--controls">
              <article
                v-for="item in diagramControls"
                :key="item.id"
                class="landing__node landing__node--control"
              >
                <div class="landing__node-icon">{{ item.icon }}</div>
                <div>
                  <strong>{{ item.title }}</strong>
                  <p>{{ item.label }}</p>
                </div>
                <button
                  class="landing__hint"
                  type="button"
                  :aria-label="item.title"
                  data-testid="diagram-tooltip-trigger"
                  :aria-expanded="isTooltipOpen(item.id)"
                  @click="toggleTooltip(item.id)"
                >
                  ?
                </button>
                <div
                  v-show="isTooltipOpen(item.id)"
                  class="landing__tooltip"
                  :data-tooltip-open="isTooltipOpen(item.id) ? 'true' : undefined"
                >
                  {{ item.description }}
                </div>
              </article>
            </div>
          </div>
        </div>
      </div>
    </section>

    <section class="landing__story">
      <div class="landing__story-header">
        <div>
          <div class="landing__panel-label">{{ t("landing.story.eyebrow") }}</div>
          <h2>{{ t("landing.story.title") }}</h2>
        </div>
        <div
          class="landing__switcher"
          role="tablist"
          :aria-label="t('landing.panelsLabel')"
        >
          <button
            v-for="panel in panels"
            :key="panel.id"
            class="landing__switch"
            :class="{ 'landing__switch--active': activePanel === panel.id }"
            type="button"
            :aria-selected="activePanel === panel.id"
            @click="activePanel = panel.id"
          >
            {{ panel.label }}
          </button>
        </div>
      </div>

      <transition name="landing-panel" mode="out-in">
        <section :key="activePanel" class="landing__panel-stage">
          <template v-if="activePanel === 'overview'">
            <div class="landing__overview-grid">
              <n-card class="landing__panel landing__panel--summary" :bordered="false">
                <div class="landing__panel-label">{{ t("landing.overview.title") }}</div>
                <p class="landing__summary">{{ t("landing.overview.summary") }}</p>
              </n-card>

              <n-card
                v-for="item in overviewCards"
                :key="item.id"
                class="landing__panel"
                :bordered="false"
              >
                <div class="landing__tile">
                  <div class="landing__tile-icon">{{ item.icon }}</div>
                  <div>
                    <strong>{{ item.title }}</strong>
                    <p>{{ item.description }}</p>
                  </div>
                </div>
              </n-card>
            </div>
          </template>

          <template v-else-if="activePanel === 'product'">
            <div class="landing__card-grid">
              <n-card
                v-for="item in productFeatures"
                :key="item.id"
                class="landing__panel"
                :bordered="false"
              >
                <div class="landing__tile">
                  <div class="landing__tile-icon landing__tile-icon--product">
                    {{ item.icon }}
                  </div>
                  <div>
                    <strong>{{ item.title }}</strong>
                    <p>{{ item.description }}</p>
                    <span>{{ item.note }}</span>
                  </div>
                </div>
              </n-card>
            </div>
          </template>

          <template v-else>
            <div class="landing__card-grid">
              <n-card
                v-for="item in engineeringArchitecture"
                :key="item.id"
                class="landing__panel"
                :bordered="false"
              >
                <div class="landing__tile">
                  <div class="landing__tile-icon landing__tile-icon--engineering">
                    {{ item.icon }}
                  </div>
                  <div>
                    <strong>{{ item.title }}</strong>
                    <p>{{ item.description }}</p>
                    <span>{{ item.note }}</span>
                  </div>
                </div>
              </n-card>
            </div>
          </template>
        </section>
      </transition>
    </section>
  </main>

  <main v-else class="landing">
    <n-card class="landing__card landing__card--login" :bordered="false">
      <div class="landing__eyebrow">{{ t("landing.loginEyebrow") }}</div>
      <h1>{{ t("landing.loginTitle") }}</h1>
      <p>{{ t("landing.loginBody") }}</p>
      <n-button tag="a" href="/api/auth/login" type="primary" size="large">
        {{ t("common.actions.continueToSignIn") }}
      </n-button>
    </n-card>
  </main>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { isMockMode } from "@/app-mode";
import LanguageSwitcher from "@/components/i18n/LanguageSwitcher.vue";
import { NButton, NCard } from "naive-ui";
import { useI18n } from "vue-i18n";

type LandingPanelId = "overview" | "product" | "engineering";

type LandingCard = {
  id: string;
  icon: string;
  title: string;
  description: string;
  note?: string;
  label?: string;
};

const activePanel = ref<LandingPanelId>("overview");
const openTooltipId = ref<string | null>(null);
const { t, locale } = useI18n();

const panels = computed<{ id: LandingPanelId; label: string }[]>(() => [
  { id: "overview", label: t("landing.panels.overview") },
  { id: "product", label: t("landing.panels.product") },
  { id: "engineering", label: t("landing.panels.engineering") },
]);

const diagramObjects = computed<LandingCard[]>(() => [
  {
    id: "issue",
    icon: "IS",
    title: t("landing.diagram.issue.title"),
    label: t("landing.diagram.issue.label"),
    description: t("landing.diagram.issue.description"),
  },
  {
    id: "mr",
    icon: "MR",
    title: t("landing.diagram.mr.title"),
    label: t("landing.diagram.mr.label"),
    description: t("landing.diagram.mr.description"),
  },
  {
    id: "milestone",
    icon: "MS",
    title: t("landing.diagram.milestone.title"),
    label: t("landing.diagram.milestone.label"),
    description: t("landing.diagram.milestone.description"),
  },
]);

const diagramControls = computed<LandingCard[]>(() => [
  {
    id: "role",
    icon: "RL",
    title: t("landing.diagram.role.title"),
    label: t("landing.diagram.role.label"),
    description: t("landing.diagram.role.description"),
  },
  {
    id: "memory",
    icon: "ME",
    title: t("landing.diagram.memory.title"),
    label: t("landing.diagram.memory.label"),
    description: t("landing.diagram.memory.description"),
  },
  {
    id: "skill",
    icon: "SK",
    title: t("landing.diagram.skill.title"),
    label: t("landing.diagram.skill.label"),
    description: t("landing.diagram.skill.description"),
  },
]);

const overviewCards = computed<LandingCard[]>(() => [
  {
    id: "requirement-clarity",
    icon: "RQ",
    title: t("landing.overview.requirementClarity.title"),
    description: t("landing.overview.requirementClarity.description"),
  },
  {
    id: "review-flow",
    icon: "RV",
    title: t("landing.overview.reviewFlow.title"),
    description: t("landing.overview.reviewFlow.description"),
  },
  {
    id: "context-continuity",
    icon: "CT",
    title: t("landing.overview.contextContinuity.title"),
    description: t("landing.overview.contextContinuity.description"),
  },
]);

const productFeatures = computed<LandingCard[]>(() => [
  {
    id: "loop-engine",
    icon: "LP",
    title: t("landing.product.loopEngine.title"),
    description: t("landing.product.loopEngine.description"),
    note: t("landing.product.loopEngine.note"),
  },
  {
    id: "role-workbench",
    icon: "RB",
    title: t("landing.product.roleWorkbench.title"),
    description: t("landing.product.roleWorkbench.description"),
    note: t("landing.product.roleWorkbench.note"),
  },
  {
    id: "memory-layer",
    icon: "MM",
    title: t("landing.product.memoryLayer.title"),
    description: t("landing.product.memoryLayer.description"),
    note: t("landing.product.memoryLayer.note"),
  },
  {
    id: "state-pressure",
    icon: "SP",
    title: t("landing.product.statePressure.title"),
    description: t("landing.product.statePressure.description"),
    note: t("landing.product.statePressure.note"),
  },
]);

const engineeringArchitecture = computed<LandingCard[]>(() => [
  {
    id: "state-layer",
    icon: "ST",
    title: t("landing.engineering.stateLayer.title"),
    description: t("landing.engineering.stateLayer.description"),
    note: t("landing.engineering.stateLayer.note"),
  },
  {
    id: "memory-layer",
    icon: "MX",
    title: t("landing.engineering.memoryLayer.title"),
    description: t("landing.engineering.memoryLayer.description"),
    note: t("landing.engineering.memoryLayer.note"),
  },
  {
    id: "agent-layer",
    icon: "AG",
    title: t("landing.engineering.agentLayer.title"),
    description: t("landing.engineering.agentLayer.description"),
    note: t("landing.engineering.agentLayer.note"),
  },
  {
    id: "pressure-logic",
    icon: "PR",
    title: t("landing.engineering.pressureLogic.title"),
    description: t("landing.engineering.pressureLogic.description"),
    note: t("landing.engineering.pressureLogic.note"),
  },
]);

function isTooltipOpen(id: string): boolean {
  return openTooltipId.value === id;
}

function toggleTooltip(id: string) {
  openTooltipId.value = openTooltipId.value === id ? null : id;
}
</script>

<style scoped>
.landing {
  min-height: 100vh;
  display: grid;
  place-items: center;
  padding: 24px;
}

.landing__card {
  border-radius: var(--if-radius-lg);
  box-shadow: var(--if-shadow-panel);
}

.landing__card--login {
  width: min(100%, 640px);
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
  padding: 28px 24px 40px;
}

.landing__hero {
  display: grid;
  grid-template-columns: minmax(0, 1.05fr) minmax(420px, 0.95fr);
  align-items: start;
  gap: 20px;
  padding: 24px 28px;
  border-radius: var(--if-radius-xl);
  border: 1px solid rgba(255, 255, 255, 0.45);
  background:
    linear-gradient(135deg, rgba(11, 25, 47, 0.96), rgba(20, 93, 88, 0.88)),
    linear-gradient(180deg, rgba(255, 255, 255, 0.06), transparent);
  color: #f8fafc;
  box-shadow: var(--if-shadow-panel);
}

.landing__hero-topbar {
  grid-column: 1 / -1;
  display: flex;
  justify-content: flex-end;
  margin-bottom: 2px;
}

.landing__hero-copy {
  display: grid;
  align-content: start;
  max-width: 560px;
}

.landing__hero-title {
  max-width: 540px;
  margin: 0 0 14px;
  font-size: clamp(2.15rem, 4.3vw, 4.2rem);
  line-height: 0.94;
  letter-spacing: -0.05em;
  text-wrap: balance;
}

.landing--zh .landing__hero-copy {
  max-width: 500px;
}

.landing--zh .landing__hero-title {
  max-width: 470px;
  font-size: clamp(1.82rem, 3.55vw, 3.35rem);
  line-height: 1.04;
  letter-spacing: -0.04em;
}

.landing__hero-title span {
  display: block;
}

.landing__lead {
  max-width: 540px;
  color: rgba(248, 250, 252, 0.88);
  margin-bottom: 16px;
}

.landing--zh .landing__lead {
  max-width: 500px;
}

.landing__impact {
  max-width: 500px;
  margin-top: 0;
  color: rgba(248, 250, 252, 0.72);
  font-size: 14px;
  margin-bottom: 18px;
}

.landing__actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.landing__settings-button {
  border: 1px solid rgba(248, 250, 252, 0.26);
  background: rgba(248, 250, 252, 0.08);
  color: #f8fafc;
}

.landing__hero-diagram {
  display: flex;
  align-items: stretch;
}

.landing__diagram {
  position: relative;
  width: 100%;
  min-height: 320px;
  display: grid;
  gap: 14px;
  padding: 16px;
  border-radius: 28px;
  background: rgba(255, 250, 242, 0.12);
  border: 1px solid rgba(255, 255, 255, 0.12);
}

.landing__diagram-group {
  display: grid;
  gap: 12px;
  padding: 12px;
  border-radius: 22px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.07);
}

.landing__diagram-ring {
  display: grid;
  gap: 12px;
}

.landing__diagram-ring--objects {
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.landing__diagram-ring--controls {
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.landing__node {
  position: relative;
  display: grid;
  grid-template-columns: 48px 1fr 28px;
  gap: 12px;
  align-items: start;
  padding: 16px;
  border-radius: 18px;
  border: 1px solid rgba(255, 255, 255, 0.14);
}

.landing__node strong {
  display: block;
  margin-bottom: 4px;
  color: #f8fafc;
}

.landing__node p {
  margin: 0;
  max-width: 14ch;
  color: rgba(248, 250, 252, 0.72);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.02em;
  text-transform: uppercase;
}

.landing__node--engine {
  grid-template-columns: 56px 1fr 28px;
  background: rgba(255, 255, 255, 0.12);
}

.landing__node--engine-core {
  margin-bottom: 4px;
}

.landing__node--object {
  background: rgba(15, 118, 110, 0.18);
}

.landing__node--control {
  background: rgba(180, 105, 14, 0.18);
}

.landing__node-icon,
.landing__tile-icon {
  width: 48px;
  height: 48px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 14px;
  font-size: 12px;
  font-weight: 800;
  letter-spacing: 0.06em;
}

.landing__node-icon {
  background: rgba(255, 255, 255, 0.14);
  color: #f8fafc;
}

.landing__hint {
  width: 24px;
  height: 24px;
  margin-top: 2px;
  border: 0;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.14);
  color: #f8fafc;
  font: inherit;
  font-weight: 800;
  cursor: pointer;
}

.landing__tooltip {
  position: absolute;
  top: calc(100% + 10px);
  right: 0;
  width: min(240px, 90vw);
  padding: 10px 12px;
  border-radius: 12px;
  background: rgba(14, 20, 31, 0.96);
  color: rgba(248, 250, 252, 0.92);
  font-size: 12px;
  line-height: 1.5;
  box-shadow: 0 18px 40px rgba(10, 15, 23, 0.34);
  z-index: 3;
}

.landing__story {
  display: grid;
  gap: 18px;
}

.landing__story-header {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  align-items: start;
  gap: 18px 24px;
  padding-bottom: 6px;
  border-bottom: 1px solid rgba(216, 204, 184, 0.72);
}

.landing__story-header h2 {
  margin: 0;
  font-size: clamp(1.5rem, 3vw, 2.1rem);
  max-width: 18ch;
  text-wrap: balance;
}

.landing--zh .landing__story-header h2 {
  max-width: 22ch;
  font-size: clamp(1.42rem, 2.7vw, 2rem);
}

.landing__switcher {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  padding: 6px;
  border-radius: 999px;
  background: rgba(255, 250, 242, 0.75);
  justify-self: end;
  align-self: center;
}

.landing__switch {
  border: 0;
  border-radius: 999px;
  padding: 10px 16px;
  background: transparent;
  color: var(--if-color-muted);
  font: inherit;
  font-weight: 700;
  cursor: pointer;
}

.landing__switch--active {
  background: rgba(21, 94, 117, 0.12);
  color: var(--if-color-accent-strong);
}

.landing__panel-stage {
  min-height: 220px;
}

.landing__overview-grid,
.landing__card-grid {
  display: grid;
  grid-template-columns: repeat(12, minmax(0, 1fr));
  gap: 18px;
}

.landing__panel {
  grid-column: span 4;
  min-height: 180px;
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.9);
}

.landing__panel--summary {
  grid-column: span 12;
  min-height: auto;
}

.landing__summary {
  max-width: 820px;
  margin: 0;
}

.landing__panel-label {
  margin-bottom: 12px;
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.landing__group-label {
  color: rgba(248, 250, 252, 0.72);
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.1em;
  text-transform: uppercase;
}

.landing__tile {
  display: grid;
  grid-template-columns: 48px 1fr;
  gap: 14px;
}

.landing__tile strong {
  display: block;
  margin-bottom: 6px;
}

.landing__tile p {
  margin: 0;
}

.landing__tile span {
  display: inline-block;
  margin-top: 10px;
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
}

.landing__tile-icon {
  background: rgba(21, 94, 117, 0.12);
  color: var(--if-color-accent-strong);
}

.landing__tile-icon--product {
  background: rgba(15, 118, 110, 0.12);
  color: var(--if-color-accent);
}

.landing__tile-icon--engineering {
  background: rgba(180, 105, 14, 0.12);
  color: var(--if-color-warning);
}

.landing-panel-enter-active,
.landing-panel-leave-active {
  transition: opacity 180ms ease, transform 180ms ease;
}

.landing-panel-enter-from,
.landing-panel-leave-to {
  opacity: 0;
  transform: translateX(10px);
}

@media (max-width: 1100px) {
  .landing__hero {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 960px) {
  .landing--mock {
    padding: 24px 18px 32px;
  }

  .landing__hero {
    padding: 20px;
  }

  .landing__story-header {
    grid-template-columns: 1fr;
  }

  .landing__diagram-ring--objects,
  .landing__diagram-ring--controls {
    grid-template-columns: 1fr;
  }

  .landing__panel,
  .landing__panel--summary {
    grid-column: span 12;
  }

  .landing__hero-topbar {
    margin-bottom: 8px;
  }

  .landing__hero-title {
    max-width: none;
    font-size: clamp(2rem, 10vw, 3.2rem);
  }

  .landing--zh .landing__hero-title {
    max-width: none;
    font-size: clamp(1.76rem, 8.6vw, 2.85rem);
  }

  .landing__switcher {
    justify-self: start;
  }
}

@media (max-width: 720px) {
  .landing__tile,
  .landing__node,
  .landing__node--engine {
    grid-template-columns: 1fr;
  }

  .landing__hint {
    justify-self: start;
  }

  .landing__tooltip {
    left: 0;
    right: auto;
  }
}
</style>
