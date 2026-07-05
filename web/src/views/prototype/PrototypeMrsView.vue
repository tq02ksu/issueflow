<template>
  <app-shell active-key="mrs" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div>
          <div class="prototype-page__eyebrow">
            {{ t("prototype.mrs.eyebrow") }}
          </div>
          <h1>{{ t("prototype.mrs.title") }}</h1>
        </div>
        <WorkflowStateBadge :state="store.selectedMr?.state ?? 'draft'" />
      </div>

      <ObjectSplitView :density="store.activeUiProfile.density">
        <template #list>
          <div class="list-panel">
            <button
              v-for="mr in store.visibleMrs"
              :key="mr.id"
              class="list-item"
              :class="{ 'list-item--active': mr.id === store.selectedMrId }"
              @click="store.selectMr(mr.id)"
            >
              <div class="list-item__meta">
                <span>!{{ mr.iid }}</span>
                <WorkflowStateBadge :state="mr.state" />
              </div>
              <strong>{{ mr.title }}</strong>
              <p>{{ mr.nextActionSummary }}</p>
            </button>
          </div>
        </template>

        <template #detail>
          <div v-if="store.selectedMr" class="detail-panel">
            <n-card :bordered="false" class="detail-card">
              <template #header>
                <div class="detail-card__header">
                  <div>
                    <div class="detail-card__eyebrow">
                      MR !{{ store.selectedMr.iid }}
                    </div>
                    <h2>{{ store.selectedMr.title }}</h2>
                  </div>
                  <WorkflowStateBadge :state="store.selectedMr.state" />
                </div>
              </template>
              <div class="detail-card__grid">
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.mrs.reviewSummary") }}
                  </div>
                  <strong>{{ store.selectedMr.nextActionSummary }}</strong>
                  <p>{{ store.selectedMr.reviewSummary }}</p>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.mrs.readinessChecks") }}
                  </div>
                  <ul>
                    <li
                      v-for="item in store.selectedMr.readinessChecks"
                      :key="item"
                    >
                      {{ item }}
                    </li>
                  </ul>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.mrs.verificationNotes") }}
                  </div>
                  <ul>
                    <li
                      v-for="item in store.selectedMr.verificationNotes"
                      :key="item"
                    >
                      {{ item }}
                    </li>
                  </ul>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.mrs.risks") }}
                  </div>
                  <ul>
                    <li v-for="item in store.selectedMr.risks" :key="item">
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
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();
</script>

<style scoped>
.prototype-page {
  display: grid;
  gap: 20px;
}

.prototype-page__header,
.list-item__meta,
.detail-card__header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  gap: 12px;
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

.list-item p,
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
    flex-direction: column;
  }

  .detail-card__grid {
    grid-template-columns: 1fr;
  }
}
</style>
