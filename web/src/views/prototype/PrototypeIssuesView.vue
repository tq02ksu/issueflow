<template>
  <app-shell active-key="issues" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div>
          <div class="prototype-page__eyebrow">Issues</div>
          <h1>Drive issue quality into execution</h1>
        </div>
        <WorkflowStateBadge :state="store.selectedIssue?.state ?? 'new'" />
      </div>

      <ObjectSplitView :density="store.activeUiProfile.density">
        <template #list>
          <div class="list-panel">
            <button
              v-for="issue in store.visibleIssues"
              :key="issue.id"
              class="list-item"
              :class="{ 'list-item--active': issue.id === store.selectedIssueId }"
              @click="store.selectIssue(issue.id)"
            >
              <div class="list-item__meta">
                <span>#{{ issue.iid }}</span>
                <WorkflowStateBadge :state="issue.state" />
              </div>
              <strong>{{ issue.title }}</strong>
              <p>{{ issue.nextActionSummary }}</p>
            </button>
          </div>
        </template>

        <template #detail>
          <div v-if="store.selectedIssue" class="detail-panel">
            <n-card :bordered="false" class="detail-card">
              <template #header>
                <div class="detail-card__header">
                  <div>
                    <div class="detail-card__eyebrow">
                      Issue #{{ store.selectedIssue.iid }}
                    </div>
                    <h2>{{ store.selectedIssue.title }}</h2>
                  </div>
                  <WorkflowStateBadge :state="store.selectedIssue.state" />
                </div>
              </template>
              <p class="detail-card__lead">{{ store.selectedIssue.description }}</p>
              <div class="detail-card__grid">
                <section class="detail-section">
                  <div class="detail-section__label">Next action</div>
                  <strong>{{ store.selectedIssue.nextActionSummary }}</strong>
                  <p>{{ store.selectedIssue.blockerSummary }}</p>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">Acceptance criteria</div>
                  <ul>
                    <li
                      v-for="item in store.selectedIssue.acceptanceCriteria"
                      :key="item"
                    >
                      {{ item }}
                    </li>
                  </ul>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">Verification plan</div>
                  <ul>
                    <li
                      v-for="item in store.selectedIssue.verificationPlan"
                      :key="item"
                    >
                      {{ item }}
                    </li>
                  </ul>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">Risks</div>
                  <ul>
                    <li v-for="item in store.selectedIssue.risks" :key="item">
                      {{ item }}
                    </li>
                  </ul>
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

const store = usePrototypeStore();
</script>

<style scoped>
.prototype-page {
  display: grid;
  gap: 20px;
}

.prototype-page__header {
  display: flex;
  justify-content: space-between;
  align-items: end;
  gap: 16px;
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
.detail-card__header {
  display: flex;
  justify-content: space-between;
  align-items: start;
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

.detail-section ul {
  margin: 10px 0 0;
  padding-left: 18px;
}

@media (max-width: 720px) {
  .prototype-page__header {
    align-items: start;
    flex-direction: column;
  }

  .detail-card__grid {
    grid-template-columns: 1fr;
  }
}
</style>
