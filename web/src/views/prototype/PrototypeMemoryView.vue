<template>
  <app-shell active-key="memory" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div>
          <div class="prototype-page__eyebrow">{{ t("prototype.memory.eyebrow") }}</div>
          <h1>{{ t("prototype.memory.title") }}</h1>
          <p>{{ t("prototype.memory.description") }}</p>
        </div>
      </div>

      <div class="memory-grid">
        <n-card :bordered="false" class="memory-card">
          <template #header>
            <div class="memory-card__header">
              <span>{{ t("prototype.memory.loopMemory") }}</span>
              <span class="memory-card__count">{{ store.memoryItemsByScope.loop.length }}</span>
            </div>
          </template>
          <div class="memory-card__list">
            <article
              v-for="item in store.memoryItemsByScope.loop"
              :key="item.id"
              class="memory-item"
            >
              <div class="memory-item__meta">
                <span>{{ item.objectType }}</span>
                <span>{{ item.objectId }}</span>
              </div>
              <p>{{ item.summary }}</p>
              <div class="memory-item__tags">
                <span
                  v-for="blocker in item.knownBlockers"
                  :key="blocker"
                  class="memory-tag memory-tag--blocker"
                >{{ blocker }}</span>
                <span
                  v-for="risk in item.knownRisks"
                  :key="risk"
                  class="memory-tag memory-tag--risk"
                >{{ risk }}</span>
              </div>
              <div class="memory-item__footer">
                <span>{{ t("prototype.memory.updatedAt") }}: {{ item.lastUpdatedAt }}</span>
              </div>
            </article>
          </div>
        </n-card>

        <n-card :bordered="false" class="memory-card">
          <template #header>
            <div class="memory-card__header">
              <span>{{ t("prototype.memory.engineeringMemory") }}</span>
              <span class="memory-card__count">{{ store.memoryItemsByScope.engineering.length }}</span>
            </div>
          </template>
          <div class="memory-card__list">
            <article
              v-for="item in store.memoryItemsByScope.engineering"
              :key="item.id"
              class="memory-item"
            >
              <div class="memory-item__meta">
                <span>{{ item.objectType }}</span>
                <span>{{ item.objectId }}</span>
              </div>
              <p>{{ item.summary }}</p>
              <div class="memory-item__tags">
                <span
                  v-for="blocker in item.knownBlockers"
                  :key="blocker"
                  class="memory-tag memory-tag--blocker"
                >{{ blocker }}</span>
                <span
                  v-for="risk in item.knownRisks"
                  :key="risk"
                  class="memory-tag memory-tag--risk"
                >{{ risk }}</span>
              </div>
              <div class="memory-item__footer">
                <span>{{ t("prototype.memory.sourcesFrom") }}: {{ item.sourceRunIds.join(", ") }}</span>
              </div>
            </article>
          </div>
        </n-card>

        <n-card :bordered="false" class="memory-card memory-card--full">
          <template #header>
            <div class="memory-card__header">
              <span>{{ t("prototype.memory.governanceMemory") }}</span>
              <span class="memory-card__count">{{ store.memoryItemsByScope.governance.length }}</span>
            </div>
          </template>
          <div class="memory-card__list">
            <article
              v-for="item in store.memoryItemsByScope.governance"
              :key="item.id"
              class="memory-item"
            >
              <div class="memory-item__meta">
                <span>{{ item.objectType }}</span>
              </div>
              <p>{{ item.summary }}</p>
              <div class="memory-item__tags">
                <span
                  v-for="blocker in item.knownBlockers"
                  :key="blocker"
                  class="memory-tag memory-tag--blocker"
                >{{ blocker }}</span>
                <span
                  v-for="risk in item.knownRisks"
                  :key="risk"
                  class="memory-tag memory-tag--risk"
                >{{ risk }}</span>
              </div>
              <div class="memory-item__suggestions">
                <span class="memory-item__suggestions-label">{{ t("prototype.memory.suggestedNextSteps") }}:</span>
                <ul>
                  <li v-for="step in item.suggestedNextSteps" :key="step">{{ step }}</li>
                </ul>
              </div>
            </article>
          </div>
        </n-card>

        <n-card :bordered="false" class="memory-card memory-card--full">
          <template #header>{{ t("prototype.memory.systemStatus") }}</template>
          <div class="memory-status-grid">
            <article
              v-for="scope in store.currentMemoryScopes"
              :key="scope.scope"
              class="memory-status-item"
            >
              <div class="memory-status-item__header">
                <strong>{{ scope.scope }}</strong>
                <span
                  class="memory-status-badge"
                  :class="`memory-status-badge--${scope.status}`"
                >{{ scope.status }}</span>
              </div>
              <p>{{ scope.summary }}</p>
            </article>
          </div>
          <div class="memory-card__controls">
            <n-button secondary @click="store.clearWorkbenchMemory">
              {{ t("prototype.settings.clearMemory") }}
            </n-button>
            <n-button type="primary" @click="store.rebuildWorkbenchMemory">
              {{ t("prototype.settings.rebuildMemory") }}
            </n-button>
            <span v-if="store.lastMemoryAction !== 'idle'" class="memory-feedback">
              {{ t("prototype.settings.lastAction") }}: {{ store.lastMemoryAction }}
            </span>
          </div>
        </n-card>
      </div>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { NButton, NCard } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { usePrototypeStore } from "@/stores/prototype.store";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();
</script>

<style scoped>
.prototype-page {
  display: grid;
  gap: 24px;
}

.prototype-page__eyebrow {
  margin-bottom: 10px;
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.prototype-page__header h1 {
  margin: 0 0 8px;
}

.prototype-page__header p {
  max-width: 760px;
  margin: 0;
  color: var(--if-color-muted);
  line-height: 1.6;
}

.memory-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 20px;
}

.memory-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.92);
}

.memory-card--full {
  grid-column: span 2;
}

.memory-card__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  font-weight: 700;
}

.memory-card__count {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 24px;
  height: 24px;
  padding: 0 8px;
  border-radius: 999px;
  background: rgba(21, 94, 117, 0.12);
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 800;
}

.memory-card__list {
  display: grid;
  gap: 16px;
}

.memory-item {
  padding: 14px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
}

.memory-item__meta {
  display: flex;
  gap: 8px;
  margin-bottom: 8px;
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  color: var(--if-color-accent-strong);
}

.memory-item p {
  margin: 0 0 10px;
  color: var(--if-color-muted);
  line-height: 1.6;
}

.memory-item__tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
  margin-bottom: 10px;
}

.memory-tag {
  display: inline-flex;
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 11px;
  font-weight: 700;
}

.memory-tag--blocker {
  background: rgba(180, 35, 24, 0.09);
  color: var(--if-color-danger);
}

.memory-tag--risk {
  background: rgba(180, 105, 14, 0.09);
  color: var(--if-color-warning);
}

.memory-item__footer {
  color: var(--if-color-muted);
  font-size: 12px;
}

.memory-item__suggestions {
  margin-top: 8px;
}

.memory-item__suggestions-label {
  font-size: 12px;
  font-weight: 700;
}

.memory-item__suggestions ul {
  margin: 6px 0 0;
  padding-left: 18px;
  font-size: 13px;
  color: var(--if-color-muted);
}

.memory-card__controls {
  display: flex;
  gap: 12px;
  align-items: center;
  margin-top: 16px;
  flex-wrap: wrap;
}

.memory-feedback {
  color: var(--if-color-muted);
  font-size: 13px;
}

.memory-status-grid {
  display: grid;
  gap: 14px;
}

.memory-status-item {
  padding: 14px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
}

.memory-status-item__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}

.memory-status-item p {
  margin: 8px 0 0;
  color: var(--if-color-muted);
  line-height: 1.5;
}

.memory-status-badge {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
}

.memory-status-badge--healthy {
  background: rgba(15, 118, 110, 0.12);
  color: var(--if-color-accent);
}

.memory-status-badge--attention {
  background: rgba(180, 105, 14, 0.12);
  color: var(--if-color-warning);
}

@media (max-width: 900px) {
  .memory-grid {
    grid-template-columns: 1fr;
  }

  .memory-card--full {
    grid-column: span 1;
  }
}
</style>
