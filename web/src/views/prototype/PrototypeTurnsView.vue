<template>
  <app-shell active-key="turns" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div>
          <div class="prototype-page__eyebrow">
            {{ t("prototype.turns.eyebrow") }}
          </div>
          <h1>{{ t("prototype.turns.title") }}</h1>
          <p>{{ t("prototype.turns.description") }}</p>
        </div>
      </div>

      <ObjectSplitView :density="store.activeUiProfile.density">
        <template #list>
          <div class="list-panel">
            <button
              v-for="turn in store.visibleTurns"
              :key="turn.id"
              class="list-item"
              :class="{ 'list-item--active': turn.id === store.selectedTurnId }"
              @click="store.selectTurn(turn.id)"
            >
              <div class="list-item__meta">
                <span class="list-item__id">{{ turn.id }}</span>
                <span
                  class="turn-status"
                  :class="`turn-status--${turn.status}`"
                  >{{ turn.status }}</span
                >
              </div>
              <strong>{{ turn.loopName }}</strong>
              <p>{{ turn.summary.slice(0, 100) }}</p>
              <div class="list-item__footer">
                <span>{{ formatDuration(turn.durationSecs) }}</span>
                <span>{{ turn.startTime.slice(0, 16).replace("T", " ") }}</span>
              </div>
            </button>
          </div>
        </template>

        <template #detail>
          <div v-if="store.selectedTurn" class="detail-panel">
            <n-card :bordered="false" class="detail-card">
              <template #header>
                <div class="detail-card__header">
                  <div>
                    <div class="detail-card__eyebrow">
                      {{ t("prototype.turns.turnDetail") }}
                    </div>
                    <h2>{{ store.selectedTurn.id }}</h2>
                  </div>
                  <span
                    class="turn-status"
                    :class="`turn-status--${store.selectedTurn.status}`"
                  >
                    {{ store.selectedTurn.status }}
                  </span>
                </div>
              </template>

              <div class="detail-card__grid">
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.turns.loopName") }}
                  </div>
                  <strong>{{ store.selectedTurn.loopName }}</strong>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.turns.trigger") }}
                  </div>
                  <span>{{ store.selectedTurn.triggerSource }}</span>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.turns.duration") }}
                  </div>
                  <strong>{{
                    formatDuration(store.selectedTurn.durationSecs)
                  }}</strong>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.turns.totalTokens") }}
                  </div>
                  <strong>{{
                    store.selectedTurn.totalTokens.toLocaleString()
                  }}</strong>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.turns.totalCost") }}
                  </div>
                  <strong
                    >${{ store.selectedTurn.totalCost.toFixed(4) }}</strong
                  >
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.turns.objectsProcessed") }}
                  </div>
                  <strong>{{ store.selectedTurn.targets.length }}</strong>
                </section>
              </div>

              <div class="detail-card__summary">
                <div class="detail-section__label">
                  {{ t("prototype.turns.summary") }}
                </div>
                <p>{{ store.selectedTurn.summary }}</p>
              </div>

              <div class="detail-card__summary">
                <div class="detail-section__label">
                  {{ t("prototype.turns.conclusion") }}
                </div>
                <p>{{ store.selectedTurn.conclusion }}</p>
              </div>

              <div class="detail-card__grid">
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.turns.agents") }}
                  </div>
                  <div class="agent-list">
                    <article
                      v-for="agent in store.selectedTurn.agents"
                      :key="agent.agentId"
                      class="agent-list__item"
                    >
                      <div class="agent-list__head">
                        <strong>{{ agent.agentName }}</strong>
                        <span>{{ agent.role }}</span>
                      </div>
                      <div class="agent-list__meta">
                        <span>{{ agent.model }}</span>
                        <span
                          >{{ agent.tokensUsed.toLocaleString() }} tokens</span
                        >
                        <span>${{ agent.cost.toFixed(4) }}</span>
                      </div>
                    </article>
                  </div>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">
                    {{ t("prototype.turns.targets") }}
                  </div>
                  <div class="target-list">
                    <article
                      v-for="target in store.selectedTurn.targets"
                      :key="target.objectId"
                      class="target-list__item"
                    >
                      <span class="target-list__type">{{
                        target.objectType
                      }}</span>
                      <strong>{{ target.objectId }}</strong>
                      <p>{{ target.result }}</p>
                    </article>
                  </div>
                </section>
              </div>

              <div class="detail-card__timeline">
                <div class="detail-section__label">
                  {{ t("prototype.turns.timeline") }}
                </div>
                <ol class="timeline">
                  <li
                    v-for="event in store.selectedTurn.events"
                    :key="event.timestamp"
                    class="timeline__item"
                  >
                    <span
                      class="timeline__kind"
                      :class="`timeline__kind--${event.kind}`"
                      >{{ event.kind }}</span
                    >
                    <span class="timeline__time">{{
                      event.timestamp.slice(11, 19)
                    }}</span>
                    <p>{{ event.message }}</p>
                  </li>
                </ol>
              </div>

              <div class="detail-card__actions">
                <n-button secondary>{{
                  t("prototype.turns.stopTurn")
                }}</n-button>
                <n-button secondary type="error">{{
                  t("prototype.turns.stopLoop")
                }}</n-button>
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
import { NButton, NCard } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import ObjectSplitView from "@/components/prototype/ObjectSplitView.vue";
import RecommendedActionsCard from "@/components/prototype/RecommendedActionsCard.vue";
import { usePrototypeStore } from "@/stores/prototype.store";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();

function formatDuration(seconds: number): string {
  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = seconds % 60;
  if (minutes > 0) {
    return `${minutes}m ${remainingSeconds}s`;
  }
  return `${seconds}s`;
}
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

.list-item__id {
  font-weight: 700;
  color: var(--if-color-accent-strong);
}

.list-item p {
  margin: 8px 0;
  color: var(--if-color-muted);
  line-height: 1.4;
}

.list-item__footer {
  display: flex;
  gap: 12px;
  font-size: 12px;
  color: var(--if-color-muted);
}

.turn-status {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 11px;
  font-weight: 700;
}

.turn-status--completed {
  background: rgba(15, 118, 110, 0.12);
  color: var(--if-color-accent);
}
.turn-status--failed {
  background: rgba(180, 35, 24, 0.12);
  color: var(--if-color-danger);
}
.turn-status--waiting_approval {
  background: rgba(180, 105, 14, 0.12);
  color: var(--if-color-warning);
}
.turn-status--created {
  background: rgba(21, 94, 117, 0.12);
  color: var(--if-color-accent-strong);
}
.turn-status--executing,
.turn-status--fetching,
.turn-status--evaluating {
  background: rgba(21, 94, 117, 0.14);
  color: var(--if-color-accent-strong);
}

.detail-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.92);
}

.detail-card__grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 14px;
}

.detail-section {
  padding: 14px;
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
  border: 1px solid rgba(216, 204, 184, 0.8);
}

.detail-card__summary {
  margin-top: 14px;
  padding: 14px;
  border-radius: var(--if-radius-sm);
  background: rgba(21, 94, 117, 0.06);
}

.detail-card__summary p {
  margin: 10px 0 0;
  color: var(--if-color-muted);
  line-height: 1.6;
}

.detail-card__timeline {
  margin-top: 16px;
}

.timeline {
  list-style: none;
  padding: 0;
  margin: 12px 0 0;
  display: grid;
  gap: 10px;
}

.timeline__item {
  display: grid;
  grid-template-columns: 160px 70px 1fr;
  gap: 12px;
  align-items: baseline;
  padding: 10px 14px;
  border-left: 3px solid rgba(216, 204, 184, 0.9);
  border-radius: 0 var(--if-radius-sm) var(--if-radius-sm) 0;
  background: rgba(255, 255, 255, 0.5);
}

.timeline__item p {
  margin: 0;
  font-size: 13px;
  color: var(--if-color-muted);
}

.timeline__kind {
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
}

.timeline__kind--failed {
  color: var(--if-color-danger);
}
.timeline__kind--completed {
  color: var(--if-color-accent);
}
.timeline__kind--approval_requested {
  color: var(--if-color-warning);
}

.timeline__time {
  font-size: 12px;
  color: var(--if-color-muted);
}

.detail-card__actions {
  display: flex;
  gap: 12px;
  margin-top: 16px;
}

.agent-list,
.target-list {
  display: grid;
  gap: 10px;
}
.agent-list__item,
.target-list__item {
  padding: 10px 12px;
  border: 1px solid rgba(216, 204, 184, 0.6);
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.5);
}
.agent-list__head {
  display: flex;
  justify-content: space-between;
  gap: 8px;
  margin-bottom: 4px;
}
.agent-list__meta {
  display: flex;
  gap: 10px;
  font-size: 12px;
  color: var(--if-color-muted);
}
.target-list__type {
  font-size: 10px;
  font-weight: 700;
  text-transform: uppercase;
  color: var(--if-color-accent-strong);
}
.target-list__item strong {
  display: block;
  margin: 2px 0;
  font-size: 13px;
}
.target-list__item p {
  margin: 2px 0 0;
  font-size: 12px;
  color: var(--if-color-muted);
}

@media (max-width: 900px) {
  .detail-card__grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .timeline__item {
    grid-template-columns: 1fr;
  }
}
</style>
