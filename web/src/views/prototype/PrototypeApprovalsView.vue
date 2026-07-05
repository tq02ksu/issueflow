<template>
  <app-shell active-key="approvals" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div>
          <div class="prototype-page__eyebrow">{{ t("prototype.approvals.eyebrow") }}</div>
          <h1>{{ t("prototype.approvals.title") }}</h1>
          <p>{{ t("prototype.approvals.description") }}</p>
        </div>
      </div>

      <div class="approvals-tabs">
        <button
          class="approvals-tabs__tab"
          :class="{ 'approvals-tabs__tab--active': activeTab === 'pending' }"
          @click="activeTab = 'pending'"
        >
          {{ t("prototype.approvals.pending") }}
          <span class="approvals-tabs__count">{{ store.pendingApprovals.length }}</span>
        </button>
        <button
          class="approvals-tabs__tab"
          :class="{ 'approvals-tabs__tab--active': activeTab === 'history' }"
          @click="activeTab = 'history'"
        >
          {{ t("prototype.approvals.history") }}
          <span class="approvals-tabs__count">{{ store.approvalHistory.length }}</span>
        </button>
      </div>

      <ObjectSplitView :density="store.activeUiProfile.density">
        <template #list>
          <div class="list-panel">
            <template v-if="activeTab === 'pending'">
              <button
                v-for="approval in store.pendingApprovals"
                :key="approval.id"
                class="list-item"
                :class="{ 'list-item--active': approval.id === store.selectedApprovalId }"
                @click="store.selectApproval(approval.id)"
              >
                <div class="list-item__meta">
                  <span class="list-item__type">{{ approval.actionType }}</span>
                  <span
                    class="risk-badge"
                    :class="`risk-badge--${approval.riskLevel}`"
                  >{{ approval.riskLevel }}</span>
                </div>
                <strong>{{ approval.targetObject }}</strong>
                <p>{{ approval.draftContent.slice(0, 80) }}...</p>
              </button>
              <div v-if="store.pendingApprovals.length === 0" class="list-empty">
                {{ t("prototype.approvals.noPending") }}
              </div>
            </template>
            <template v-else>
              <button
                v-for="approval in store.approvalHistory"
                :key="approval.id"
                class="list-item"
                :class="{ 'list-item--active': approval.id === store.selectedApprovalId }"
                @click="store.selectApproval(approval.id)"
              >
                <div class="list-item__meta">
                  <span class="list-item__type">{{ approval.actionType }}</span>
                  <span
                    class="approval-status"
                    :class="`approval-status--${approval.status}`"
                  >{{ approval.status }}</span>
                </div>
                <strong>{{ approval.targetObject }}</strong>
                <p>{{ approval.draftContent.slice(0, 80) }}...</p>
              </button>
              <div v-if="store.approvalHistory.length === 0" class="list-empty">
                {{ t("prototype.approvals.noHistory") }}
              </div>
            </template>
          </div>
        </template>

        <template #detail>
          <div v-if="store.selectedApproval" class="detail-panel">
            <n-card :bordered="false" class="detail-card">
              <template #header>
                <div class="detail-card__header">
                  <div>
                    <div class="detail-card__eyebrow">{{ t("prototype.approvals.actionDetail") }}</div>
                    <h2>{{ store.selectedApproval.actionType }}</h2>
                  </div>
                  <span
                    class="approval-status"
                    :class="`approval-status--${store.selectedApproval.status}`"
                  >{{ store.selectedApproval.status }}</span>
                </div>
              </template>

              <div class="detail-card__grid">
                <section class="detail-section detail-section--full">
                  <div class="detail-section__label">{{ t("prototype.approvals.whatWillHappen") }}</div>
                  <p class="detail-section__draft">{{ store.selectedApproval.draftContent }}</p>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.approvals.whyThisAction") }}</div>
                  <p>{{ store.selectedApproval.generationBasis }}</p>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.approvals.memoryRelation") }}</div>
                  <p>{{ store.selectedApproval.memoryRelation }}</p>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.approvals.sourceLoop") }}</div>
                  <p>{{ store.selectedApproval.sourceLoop }}</p>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.approvals.sourceRun") }}</div>
                  <p>{{ store.selectedApproval.sourceRunId }}</p>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.approvals.createdAt") }}</div>
                  <p>{{ store.selectedApproval.createdAt }}</p>
                </section>
              </div>

              <div
                v-if="store.selectedApproval.status === 'pending'"
                class="detail-card__actions"
              >
                <n-button
                  type="primary"
                  @click="store.updateApprovalStatus(store.selectedApproval.id, 'approved')"
                >
                  {{ t("prototype.approvals.approve") }}
                </n-button>
                <n-button
                  secondary
                  type="error"
                  @click="store.updateApprovalStatus(store.selectedApproval.id, 'rejected')"
                >
                  {{ t("prototype.approvals.reject") }}
                </n-button>
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
import { ref } from "vue";
import { NButton, NCard } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import ObjectSplitView from "@/components/prototype/ObjectSplitView.vue";
import RecommendedActionsCard from "@/components/prototype/RecommendedActionsCard.vue";
import { usePrototypeStore } from "@/stores/prototype.store";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();
const activeTab = ref<"pending" | "history">("pending");
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

.prototype-page__header p {
  max-width: 640px;
  margin: 8px 0 0;
  color: var(--if-color-muted);
}

.approvals-tabs {
  display: flex;
  gap: 8px;
}

.approvals-tabs__tab {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 18px;
  border: 1px solid rgba(216, 204, 184, 0.9);
  border-radius: 999px;
  background: rgba(255, 250, 242, 0.75);
  color: var(--if-color-muted);
  font: inherit;
  font-weight: 700;
  cursor: pointer;
}

.approvals-tabs__tab--active {
  background: rgba(21, 94, 117, 0.12);
  color: var(--if-color-accent-strong);
  border-color: rgba(21, 94, 117, 0.3);
}

.approvals-tabs__count {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 22px;
  height: 22px;
  padding: 0 6px;
  border-radius: 999px;
  background: rgba(17, 24, 39, 0.08);
  font-size: 11px;
  font-weight: 800;
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
  margin-bottom: 8px;
}

.list-item__type {
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  color: var(--if-color-accent-strong);
}

.list-item p,
.detail-section p {
  color: var(--if-color-muted);
  line-height: 1.6;
}

.list-empty {
  padding: 40px 20px;
  text-align: center;
  color: var(--if-color-muted);
}

.risk-badge,
.approval-status {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 11px;
  font-weight: 700;
}

.risk-badge--low { background: rgba(15, 118, 110, 0.12); color: var(--if-color-accent); }
.risk-badge--medium { background: rgba(180, 105, 14, 0.12); color: var(--if-color-warning); }
.risk-badge--high { background: rgba(180, 35, 24, 0.09); color: var(--if-color-danger); }
.risk-badge--critical { background: rgba(180, 35, 24, 0.16); color: var(--if-color-danger); }

.approval-status--approved { background: rgba(15, 118, 110, 0.12); color: var(--if-color-accent); }
.approval-status--rejected { background: rgba(180, 35, 24, 0.12); color: var(--if-color-danger); }
.approval-status--pending { background: rgba(21, 94, 117, 0.14); color: var(--if-color-accent-strong); }
.approval-status--execution_failed { background: rgba(180, 35, 24, 0.12); color: var(--if-color-danger); }

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

.detail-section--full {
  grid-column: span 2;
}

.detail-section__draft {
  padding: 12px;
  margin-top: 8px;
  border-radius: var(--if-radius-sm);
  background: rgba(21, 94, 117, 0.06);
  line-height: 1.7;
}

.detail-card__actions {
  display: flex;
  gap: 12px;
  margin-top: 16px;
}

@media (max-width: 720px) {
  .detail-card__grid {
    grid-template-columns: 1fr;
  }

  .detail-section--full {
    grid-column: span 1;
  }
}
</style>
