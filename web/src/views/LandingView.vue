<template>
  <main v-if="isMockMode" class="landing landing--mock">
    <section class="landing__hero">
      <div class="landing__hero-toolbar">
        <LanguageSwitcher />
      </div>

      <div class="landing__hero-copy">
        <div class="landing__eyebrow">Workflow Cockpit</div>
        <h1>We don't manage work. We keep work moving.</h1>
        <p class="landing__lead">
          A loop engineering system for software delivery that keeps issues,
          MRs, and milestones moving through the next execution step.
        </p>
        <p class="landing__impact">
          Lower waiting time. Surface stalled work early. Make readiness explicit.
        </p>
        <div class="landing__actions">
          <n-button tag="a" href="/workbench" type="primary" size="large">
            Open prototype
          </n-button>
          <n-button
            tag="a"
            href="/settings"
            quaternary
            size="large"
            class="landing__settings-button"
          >
            Review settings
          </n-button>
        </div>
      </div>

      <div class="landing__hero-diagram">
        <div class="landing__diagram" data-testid="landing-diagram">
          <article class="landing__node landing__node--engine landing__node--engine-core">
            <div class="landing__node-icon">LE</div>
            <div>
              <strong>Execution Loop Engine</strong>
              <p>Clarify, advance, verify</p>
            </div>
            <button
              class="landing__hint"
              type="button"
              aria-label="Execution Loop Engine"
              data-testid="diagram-tooltip-trigger"
              :aria-expanded="isTooltipOpen('Execution Loop Engine')"
              @click="toggleTooltip('Execution Loop Engine')"
            >
              ?
            </button>
            <div
              v-show="isTooltipOpen('Execution Loop Engine')"
              class="landing__tooltip"
              :data-tooltip-open="
                isTooltipOpen('Execution Loop Engine') ? 'true' : undefined
              "
            >
              The loop engine evaluates state, writes memory, ranks the next
              action, and decides when to stop or escalate.
            </div>
          </article>

          <div class="landing__diagram-group">
            <div class="landing__group-label">Execution objects</div>
            <div class="landing__diagram-ring landing__diagram-ring--objects">
              <article
                v-for="item in diagramObjects"
                :key="item.title"
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
                  :aria-expanded="isTooltipOpen(item.title)"
                  @click="toggleTooltip(item.title)"
                >
                  ?
                </button>
                <div
                  v-show="isTooltipOpen(item.title)"
                  class="landing__tooltip"
                  :data-tooltip-open="isTooltipOpen(item.title) ? 'true' : undefined"
                >
                  {{ item.description }}
                </div>
              </article>
            </div>
          </div>

          <div class="landing__diagram-group">
            <div class="landing__group-label">Control layers</div>
            <div class="landing__diagram-ring landing__diagram-ring--controls">
              <article
                v-for="item in diagramControls"
                :key="item.title"
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
                  :aria-expanded="isTooltipOpen(item.title)"
                  @click="toggleTooltip(item.title)"
                >
                  ?
                </button>
                <div
                  v-show="isTooltipOpen(item.title)"
                  class="landing__tooltip"
                  :data-tooltip-open="isTooltipOpen(item.title) ? 'true' : undefined"
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
          <div class="landing__panel-label">Inside the system</div>
          <h2>Business, product, and engineering in one surface</h2>
        </div>
        <div class="landing__switcher" role="tablist" aria-label="Landing panels">
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
                <div class="landing__panel-label">Execution Bottlenecks We Remove</div>
                <p class="landing__summary">
                  Project work slows down when requirements stay vague, review
                  waits too long, and context lives only in people.
                </p>
              </n-card>

              <n-card
                v-for="item in overviewCards"
                :key="item.title"
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
                :key="item.title"
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
                :key="item.title"
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
import { ref } from "vue";
import { isMockMode } from "@/app-mode";
import LanguageSwitcher from "@/components/i18n/LanguageSwitcher.vue";
import { NButton, NCard } from "naive-ui";

type LandingPanelId = "overview" | "product" | "engineering";

const activePanel = ref<LandingPanelId>("overview");
const openTooltipId = ref<string | null>(null);

const panels: { id: LandingPanelId; label: string }[] = [
  { id: "overview", label: "Overview" },
  { id: "product", label: "Product" },
  { id: "engineering", label: "Engineering" },
];

const diagramObjects = [
  {
    icon: "IS",
    title: "Issue",
    label: "clarify and execute",
    description:
      "Issues move from ambiguity to execution readiness with explicit acceptance, blockers, and next actions.",
  },
  {
    icon: "MR",
    title: "MR",
    label: "review and merge",
    description:
      "Merge requests stay inside an active review loop instead of becoming passive records waiting for someone to notice them.",
  },
  {
    icon: "MS",
    title: "Milestone",
    label: "apply pressure",
    description:
      "Milestones push urgency back into issue and MR state so delivery risk changes the next-best action.",
  },
];

const diagramControls = [
  {
    icon: "RL",
    title: "Role",
    label: "decision bias",
    description:
      "One workbench role defines how the loop prioritizes speed, quality, escalation, and planning style.",
  },
  {
    icon: "ME",
    title: "Memory",
    label: "durable context",
    description:
      "Workbench memory stores evolving context so standards and blockers do not disappear into chat history or human recall.",
  },
  {
    icon: "SK",
    title: "Skill",
    label: "behavior overlay",
    description:
      "Skills tune strictness, recommendation ordering, and presentation emphasis without replacing the system skeleton.",
  },
];

const overviewCards = [
  {
    icon: "RQ",
    title: "Requirement clarity",
    description:
      "Turn half-ready issues into executable work instead of bouncing questions across the team.",
  },
  {
    icon: "RV",
    title: "Review flow",
    description:
      "Keep review pressure visible so MRs move instead of aging silently in a queue.",
  },
  {
    icon: "CT",
    title: "Context continuity",
    description:
      "Persist standards, blockers, and next steps so handoffs do not reset the team every time.",
  },
];

const productFeatures = [
  {
    icon: "LP",
    title: "Loop Engine",
    description:
      "Every managed object enters a progression loop with state, verification, and stop conditions.",
    note: "This is an execution system, not a static board.",
  },
  {
    icon: "RB",
    title: "Role Workbench",
    description:
      "A workbench binds to one stable behavior model that shapes escalation, review strictness, and pace.",
    note: "Behavior system, not permission system.",
  },
  {
    icon: "MM",
    title: "Memory Layer",
    description:
      "Issue, MR, and project memory reduce drift and let the system rebuild state over time.",
    note: "Context becomes durable operational data.",
  },
  {
    icon: "SP",
    title: "State Pressure",
    description:
      "Milestone decay, MR delay, and issue stagnation feed the next-best-action ranking.",
    note: "State carries pressure, not just labels.",
  },
];

const engineeringArchitecture = [
  {
    icon: "ST",
    title: "State Layer",
    description:
      "Issue, MR, and milestone objects are modeled as explicit stateful work items.",
    note: "This is the graph the loops operate on.",
  },
  {
    icon: "MX",
    title: "Memory Layer",
    description:
      "Structured and temporal memory preserve evolving context for people and agents.",
    note: "Memory is scoped, rebuildable, and part of the product.",
  },
  {
    icon: "AG",
    title: "Agent Layer",
    description:
      "Role-driven agents evaluate state, propose the next step, and decide when to escalate.",
    note: "Agent behavior is bounded by policy and stop rules.",
  },
  {
    icon: "PR",
    title: "Pressure Logic",
    description:
      "Cross-object pressure turns stalled work into ranked execution guidance for the workbench.",
    note: "This is why the system is not Jira plus AI coding.",
  },
];

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
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1.05fr) minmax(420px, 0.95fr);
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

.landing__hero-toolbar {
  position: absolute;
  top: 20px;
  right: 20px;
  z-index: 2;
}

.landing__hero-copy {
  display: grid;
  align-content: start;
}

.landing__lead {
  max-width: 620px;
  color: rgba(248, 250, 252, 0.88);
  margin-bottom: 16px;
}

.landing__impact {
  max-width: 560px;
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
  gap: 10px;
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
  padding: 14px;
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
  color: rgba(248, 250, 252, 0.78);
  font-size: 13px;
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
  width: 28px;
  height: 28px;
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
  display: flex;
  justify-content: space-between;
  align-items: end;
  gap: 16px;
}

.landing__story-header h2 {
  margin: 0;
  font-size: clamp(1.5rem, 3vw, 2.1rem);
}

.landing__switcher {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  padding: 6px;
  border-radius: 999px;
  background: rgba(255, 250, 242, 0.75);
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
    padding-top: 72px;
  }

  .landing__story-header {
    align-items: start;
    flex-direction: column;
  }

  .landing__diagram-ring--objects,
  .landing__diagram-ring--controls {
    grid-template-columns: 1fr;
  }

  .landing__panel,
  .landing__panel--summary {
    grid-column: span 12;
  }

  .landing__hero-toolbar {
    top: 16px;
    right: 16px;
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
