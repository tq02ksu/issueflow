<template>
  <app-shell active-key="milestones" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div>
          <div class="prototype-page__eyebrow">
            {{ t("prototype.milestones.eyebrow") }}
          </div>
          <h1>{{ t("prototype.milestones.title") }}</h1>
        </div>
      </div>

      <ObjectSplitView :density="store.activeUiProfile.density">
        <template #list>
          <div class="list-panel">
            <button
              v-for="milestone in store.visibleMilestones"
              :key="milestone.id"
              class="list-item"
              :class="{
                'list-item--active': milestone.id === store.selectedMilestoneId,
              }"
              @click="store.selectMilestone(milestone.id)"
            >
              <div class="list-item__meta">
                <strong>{{ milestone.title }}</strong>
                <span>{{ milestone.dueDate }}</span>
              </div>
              <p>{{ milestone.nextActionSummary }}</p>
            </button>
          </div>
        </template>

        <template #detail>
          <div v-if="store.selectedMilestone" class="detail-panel">
            <n-card :bordered="false" class="detail-card">
              <template #header>
                <div class="detail-card__header">
                  <div>
                    <div class="detail-card__eyebrow">
                      {{ t("prototype.milestones.milestoneLabel") }}
                    </div>
                    <h2>{{ store.selectedMilestone.title }}</h2>
                  </div>
                  <span class="detail-card__date">
                    {{ store.selectedMilestone.dueDate }}
                  </span>
                </div>
              </template>
              <p class="detail-card__lead">
                {{ store.selectedMilestone.goal }}
              </p>
              <div class="detail-card__grid">
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.milestones.issueWorkflow") }}
                  </div>
                  <div class="summary-list">
                    <div
                      v-for="item in store.selectedMilestoneIssueSummary"
                      :key="item.state"
                      class="summary-list__item"
                    >
                      <WorkflowStateBadge :state="item.state" />
                      <strong>{{ item.count }}</strong>
                    </div>
                  </div>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.milestones.mrWorkflow") }}
                  </div>
                  <div class="summary-list">
                    <div
                      v-for="item in store.selectedMilestoneMrSummary"
                      :key="item.state"
                      class="summary-list__item"
                    >
                      <WorkflowStateBadge :state="item.state" />
                      <strong>{{ item.count }}</strong>
                    </div>
                  </div>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.milestones.riskSummary") }}
                  </div>
                  <p>{{ store.selectedMilestone.riskSummary }}</p>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.milestones.nextAction") }}
                  </div>
                  <strong>{{
                    store.selectedMilestone.nextActionSummary
                  }}</strong>
                </section>
              </div>
            </n-card>

            <RecommendedActionsCard
              :actions="store.recommendedActions"
              :tone="store.activeUiProfile.tone"
            />
          </div>
        </template>
      </ObjectSplitView>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { NCard } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import ObjectSplitView from "@/components/prototype/ObjectSplitView.vue";
import RecommendedActionsCard from "@/components/prototype/RecommendedActionsCard.vue";
import WorkflowStateBadge from "@/components/prototype/WorkflowStateBadge.vue";
import { usePrototypeStore } from "@/stores/prototype.store";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();
</script>

<style scoped>
.prototype-page {
  display: grid;
  gap: 20px;
}

.prototype-page__eyebrow,
.detail-card__eyebrow,
.detail-section__label {
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.prototype-page__header h1,
.detail-card h2 {
  margin: 6px 0 0;
}

.list-panel,
.detail-panel {
  display: grid;
  gap: 16px;
}

.list-item {
  width: 100%;
  text-align: left;
  padding: 16px;
  border: 1px solid rgba(216, 204, 184, 0.9);
  border-radius: var(--if-radius-md);
  background: rgba(255, 250, 242, 0.86);
  cursor: pointer;
}

.list-item--active {
  border-color: rgba(21, 94, 117, 0.5);
  box-shadow: 0 12px 30px rgba(21, 94, 117, 0.12);
}

.list-item__meta,
.detail-card__header,
.summary-list__item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}

.list-item p,
.detail-card__lead,
.detail-section p {
  color: var(--if-color-muted);
  line-height: 1.6;
}

.detail-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.92);
}

.detail-card__date {
  color: var(--if-color-muted);
  font-size: 13px;
  font-weight: 700;
}

.detail-card__grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 16px;
}

.detail-section {
  padding: 14px;
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
  border: 1px solid rgba(216, 204, 184, 0.8);
}

.summary-list {
  display: grid;
  gap: 10px;
}

@media (max-width: 720px) {
  .detail-card__grid {
    grid-template-columns: 1fr;
  }
}
</style>
