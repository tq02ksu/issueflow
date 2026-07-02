<template>
  <div class="overview-grid">
    <n-card class="overview-card overview-card--hero" :bordered="false">
      <div class="overview-card__eyebrow">Current workbench</div>
      <h2>{{ workbench?.name }}</h2>
      <p>{{ workbench?.role.personaSummary }}</p>
      <div class="overview-card__chips">
        <span class="overview-chip">{{ workbench?.role.name }}</span>
        <span class="overview-chip overview-chip--subtle">
          {{ workbench?.activeSkillVersionId }}
        </span>
      </div>
    </n-card>

    <n-card class="overview-card" :bordered="false">
      <div class="overview-card__eyebrow">Issue workflow</div>
      <div class="overview-summary">
        <div
          v-for="item in issueSummary"
          :key="item.state"
          class="overview-summary__item"
        >
          <WorkflowStateBadge :state="item.state" />
          <strong>{{ item.count }}</strong>
        </div>
      </div>
    </n-card>

    <n-card class="overview-card" :bordered="false">
      <div class="overview-card__eyebrow">MR workflow</div>
      <div class="overview-summary">
        <div
          v-for="item in mrSummary"
          :key="item.state"
          class="overview-summary__item"
        >
          <WorkflowStateBadge :state="item.state" />
          <strong>{{ item.count }}</strong>
        </div>
      </div>
    </n-card>

    <RecommendedActionsCard
      class="overview-card"
      :actions="actions"
      :tone="uiProfile.tone"
    />

    <n-card class="overview-card" :bordered="false">
      <div class="overview-card__eyebrow">Recent activity</div>
      <div class="overview-activity">
        <article
          v-for="item in activity"
          :key="item.id"
          class="overview-activity__item"
        >
          <strong>{{ item.title }}</strong>
          <p>{{ item.summary }}</p>
        </article>
      </div>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { NCard } from "naive-ui";
import RecommendedActionsCard from "./RecommendedActionsCard.vue";
import WorkflowStateBadge from "./WorkflowStateBadge.vue";
import type {
  PrototypeActivityItem,
  PrototypeRecommendedAction,
  PrototypeWorkbench,
  SkillUiProfile,
  WorkflowSummaryItem,
  IssueWorkflowState,
  MrWorkflowState,
} from "@/mock/prototype.types";

defineProps<{
  workbench: PrototypeWorkbench | null;
  issueSummary: WorkflowSummaryItem<IssueWorkflowState>[];
  mrSummary: WorkflowSummaryItem<MrWorkflowState>[];
  actions: PrototypeRecommendedAction[];
  activity: PrototypeActivityItem[];
  uiProfile: SkillUiProfile;
}>();
</script>

<style scoped>
.overview-grid {
  display: grid;
  grid-template-columns: repeat(12, minmax(0, 1fr));
  gap: 20px;
}

.overview-card {
  grid-column: span 4;
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.88);
}

.overview-card--hero {
  grid-column: span 8;
  background:
    linear-gradient(140deg, rgba(17, 24, 39, 0.96), rgba(21, 94, 117, 0.86));
  color: #f8fafc;
}

.overview-card__eyebrow {
  margin-bottom: 12px;
  color: var(--if-color-muted);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.overview-card--hero .overview-card__eyebrow {
  color: rgba(248, 250, 252, 0.7);
}

.overview-card h2 {
  margin: 0 0 10px;
  font-size: 28px;
}

.overview-card p {
  margin: 0;
  line-height: 1.6;
}

.overview-card__chips {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  margin-top: 18px;
}

.overview-chip {
  display: inline-flex;
  padding: 8px 12px;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.12);
  font-size: 12px;
  font-weight: 700;
}

.overview-chip--subtle {
  background: rgba(255, 255, 255, 0.08);
}

.overview-summary {
  display: grid;
  gap: 12px;
}

.overview-summary__item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}

.overview-activity {
  display: grid;
  gap: 12px;
}

.overview-activity__item {
  padding: 12px 0;
  border-top: 1px solid rgba(216, 204, 184, 0.8);
}

.overview-activity__item:first-child {
  padding-top: 0;
  border-top: 0;
}

.overview-activity__item p {
  margin-top: 6px;
  color: var(--if-color-muted);
}

@media (max-width: 980px) {
  .overview-card,
  .overview-card--hero {
    grid-column: span 12;
  }
}
</style>
