<template>
  <div class="prototype-page">
    <div class="prototype-page__header">
      <div>
        <div class="prototype-page__eyebrow">{{ t("prototype.loops.eyebrow") }}</div>
        <h1>{{ t("prototype.loops.title") }}</h1>
      </div>
      <n-button type="primary" tag="a" href="/workbench/loops/create">
        {{ t("prototype.loops.create") }}
      </n-button>
    </div>

    <div class="loops-toolbar">
      <n-input
        v-model:value="searchQuery"
        :placeholder="t('prototype.loops.searchPlaceholder')"
        clearable
        class="loops-toolbar__search"
      />
      <n-select
        v-model:value="typeFilter"
        :options="typeOptions"
        :placeholder="t('prototype.loops.filterByType')"
        clearable
        class="loops-toolbar__filter"
      />
      <n-select
        v-model:value="statusFilter"
        :options="statusOptions"
        :placeholder="t('prototype.loops.filterByStatus')"
        clearable
        class="loops-toolbar__filter"
      />
    </div>

    <ObjectSplitView :density="store.activeUiProfile.density">
      <template #list>
        <div class="list-panel">
          <button
            v-for="loop in filteredLoops"
            :key="loop.id"
            class="list-item"
            :class="{ 'list-item--active': loop.id === store.selectedLoopId }"
            @click="store.selectLoop(loop.id)"
          >
            <div class="list-item__meta">
              <span class="list-item__type">#{{ loop.id }}</span>
              <WorkflowStateBadge :state="loop.status" />
            </div>
            <strong>{{ loop.name }}</strong>
            <p>{{ loop.boundObject }}</p>
            <div class="list-item__footer">
              <span class="list-item__schedule">{{ loop.schedulePolicy }}</span>
              <span v-if="loop.lastRunAt" class="list-item__lastrun">
                {{ t("prototype.loops.lastRun") }}: {{ loop.lastRunAt.slice(0, 16).replace("T", " ") }}
              </span>
            </div>
          </button>
          <div v-if="filteredLoops.length === 0" class="list-empty">
            {{ t("prototype.loops.noLoops") }}
          </div>
        </div>
      </template>

      <template #detail>
        <div v-if="store.selectedLoop" class="detail-panel">
          <div class="loop-tabs">
            <button
              v-for="tab in loopTabs"
              :key="tab.key"
              class="loop-tabs__tab"
              :class="{ 'loop-tabs__tab--active': activeLoopTab === tab.key }"
              @click="activeLoopTab = tab.key"
            >
              {{ tab.label }}
            </button>
          </div>

          <template v-if="activeLoopTab === 'overview'">
            <n-card :bordered="false" class="detail-card">
              <template #header>
                <div class="detail-card__header">
                  <div>
                    <div class="detail-card__eyebrow">{{ t("prototype.loops.loopLabel") }}</div>
                    <h2>{{ store.selectedLoop.name }}</h2>
                  </div>
                  <n-button secondary size="small" @click="$emit('toggleLoop', store.selectedLoop.id)">
                    {{ store.selectedLoop.enabled ? t("prototype.loops.disable") : t("prototype.loops.enable") }}
                  </n-button>
                </div>
              </template>
              <div class="detail-card__grid">
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.loops.type") }}</div>
                  <strong>{{ store.selectedLoop.type }}</strong>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.loops.boundObject") }}</div>
                  <strong>{{ store.selectedLoop.boundObject }}</strong>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.loops.schedulePolicy") }}</div>
                  <span>{{ store.selectedLoop.schedulePolicy }}</span>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.loops.lastRun") }}</div>
                  <span>{{ store.selectedLoop.lastRunAt?.slice(0, 16).replace("T", " ") ?? "—" }}</span>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.loops.nextRun") }}</div>
                  <span>{{ store.selectedLoop.nextRunAt?.slice(0, 16).replace("T", " ") ?? "—" }}</span>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.loops.skillRefs") }}</div>
                  <span>{{ store.selectedLoop.skillRefs.join(", ") || "—" }}</span>
                </section>
              </div>
              <div class="detail-card__desc">
                <div class="detail-section__label">{{ t("prototype.loops.goal") }}</div>
                <p>{{ store.selectedLoop.goal }}</p>
              </div>
            </n-card>
          </template>

          <template v-else-if="activeLoopTab === 'definition'">
            <n-card :bordered="false" class="detail-card">
              <template #header>{{ t("prototype.loops.definition") }}</template>
              <div class="detail-card__grid">
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.loops.stateMachine") }}</div>
                  <span>{{ store.selectedLoop.stateMachinePolicy }}</span>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.loops.verificationPolicy") }}</div>
                  <span>{{ store.selectedLoop.verificationPolicy }}</span>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.loops.budgetPolicy") }}</div>
                  <span>{{ store.selectedLoop.budgetPolicy }}</span>
                </section>
                <section class="detail-section">
                  <div class="detail-section__label">{{ t("prototype.loops.notificationPolicy") }}</div>
                  <span>{{ store.selectedLoop.notificationPolicy }}</span>
                </section>
              </div>
            </n-card>
          </template>

          <template v-else-if="activeLoopTab === 'runs'">
            <n-card :bordered="false" class="detail-card">
              <template #header>{{ t("prototype.loops.loopRuns") }}</template>
              <div class="runs-list">
                <article
                  v-for="run in loopRuns"
                  :key="run.id"
                  class="runs-list__item"
                >
                  <div class="runs-list__meta">
                    <span>{{ run.id }}</span>
                    <span class="run-status" :class="`run-status--${run.status}`">{{ run.status }}</span>
                  </div>
                  <p>{{ run.summary.slice(0, 120) }}</p>
                  <span class="runs-list__time">{{ run.startTime.slice(0, 16).replace("T", " ") }}</span>
                </article>
                <div v-if="loopRuns.length === 0" class="list-empty">
                  {{ t("prototype.loops.noRuns") }}
                </div>
              </div>
            </n-card>
          </template>

          <template v-else-if="activeLoopTab === 'memory'">
            <n-card :bordered="false" class="detail-card">
              <template #header>{{ t("prototype.loops.loopMemory") }}</template>
              <div class="memory-items">
                <article
                  v-for="mem in loopMemoryItems"
                  :key="mem.id"
                  class="memory-item"
                >
                  <p>{{ mem.summary }}</p>
                  <div class="memory-item__tags">
                    <span v-for="b in mem.knownBlockers" :key="b" class="memory-tag memory-tag--blocker">{{ b }}</span>
                    <span v-for="r in mem.knownRisks" :key="r" class="memory-tag memory-tag--risk">{{ r }}</span>
                  </div>
                </article>
                <div v-if="loopMemoryItems.length === 0" class="list-empty">
                  {{ t("prototype.loops.noMemory") }}
                </div>
              </div>
            </n-card>
          </template>

          <template v-else-if="activeLoopTab === 'actions'">
            <n-card :bordered="false" class="detail-card">
              <template #header>{{ t("prototype.loops.actions") }}</template>
              <div class="actions-list">
                <article
                  v-for="approval in loopApprovals"
                  :key="approval.id"
                  class="actions-list__item"
                >
                  <div class="actions-list__meta">
                    <span>{{ approval.actionType }}</span>
                    <span class="approval-status" :class="`approval-status--${approval.status}`">
                      {{ approval.status }}
                    </span>
                  </div>
                  <p>{{ approval.draftContent.slice(0, 120) }}</p>
                </article>
                <div v-if="loopApprovals.length === 0" class="list-empty">
                  {{ t("prototype.loops.noActions") }}
                </div>
              </div>
            </n-card>
          </template>
        </div>
      </template>
    </ObjectSplitView>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { NButton, NCard, NInput, NSelect } from "naive-ui";
import ObjectSplitView from "@/components/prototype/ObjectSplitView.vue";
import WorkflowStateBadge from "@/components/prototype/WorkflowStateBadge.vue";
import { usePrototypeStore } from "@/stores/prototype.store";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();

const searchQuery = ref("");
const typeFilter = ref<string | null>(null);
const statusFilter = ref<string | null>(null);
const activeLoopTab = ref("overview");

const typeOptions = [
  { label: "Issue", value: "issue" },
  { label: "MR", value: "mr" },
  { label: "Milestone", value: "milestone" },
];

const statusOptions = [
  { label: "Enabled", value: "enabled" },
  { label: "Disabled", value: "disabled" },
  { label: "Blocked", value: "blocked" },
  { label: "Healthy", value: "healthy" },
  { label: "Waiting Approval", value: "waiting_approval" },
];

const loopTabs = computed(() => [
  { key: "overview", label: t("prototype.loops.tabOverview") },
  { key: "definition", label: t("prototype.loops.tabDefinition") },
  { key: "runs", label: t("prototype.loops.tabRuns") },
  { key: "memory", label: t("prototype.loops.tabMemory") },
  { key: "actions", label: t("prototype.loops.tabActions") },
]);

const filteredLoops = computed(() => {
  let items = store.prototypeLoops;

  if (searchQuery.value) {
    const q = searchQuery.value.toLowerCase();
    items = items.filter(
      (l) =>
        l.name.toLowerCase().includes(q) ||
        l.boundObject.toLowerCase().includes(q),
    );
  }
  if (typeFilter.value) {
    items = items.filter((l) => l.type === typeFilter.value);
  }
  if (statusFilter.value) {
    items = items.filter((l) => l.status === statusFilter.value);
  }

  return items;
});

const loopRuns = computed(() => {
  if (!store.selectedLoop) return [];
  return store.visibleRuns.filter((r) => r.loopName === store.selectedLoop?.name);
});

const loopMemoryItems = computed(() => {
  return store.visibleMemoryItems.filter(
    (m) => m.scope === "loop" && store.selectedLoop?.boundObject.includes(m.objectId),
  );
});

const loopApprovals = computed(() => {
  return store.visibleApprovals.filter(
    (a) => a.sourceRunId && loopRuns.value.some((r) => r.id === a.sourceRunId),
  );
});
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

.loops-toolbar {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.loops-toolbar__search {
  flex: 1;
  min-width: 200px;
}

.loops-toolbar__filter {
  width: 200px;
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
  font-weight: 700;
  color: var(--if-color-accent-strong);
}

.list-item p {
  color: var(--if-color-muted);
  line-height: 1.4;
  margin: 6px 0;
}

.list-item__footer {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  margin-top: 8px;
  font-size: 12px;
  color: var(--if-color-muted);
}

.list-item__schedule {
  font-weight: 700;
}

.list-empty {
  padding: 40px 20px;
  text-align: center;
  color: var(--if-color-muted);
}

.loop-tabs {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
  padding-bottom: 8px;
}

.loop-tabs__tab {
  padding: 8px 14px;
  border: 0;
  border-bottom: 2px solid transparent;
  background: transparent;
  color: var(--if-color-muted);
  font: inherit;
  font-weight: 700;
  cursor: pointer;
}

.loop-tabs__tab--active {
  border-bottom-color: var(--if-color-accent-strong);
  color: var(--if-color-accent-strong);
}

.detail-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.92);
}

.detail-card__grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 14px;
}

.detail-section {
  padding: 14px;
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
  border: 1px solid rgba(216, 204, 184, 0.8);
}

.detail-card__desc {
  margin-top: 14px;
  padding: 14px;
  border-radius: var(--if-radius-sm);
  background: rgba(21, 94, 117, 0.06);
}

.detail-card__desc p {
  margin: 8px 0 0;
  color: var(--if-color-muted);
  line-height: 1.6;
}

.runs-list,
.actions-list,
.memory-items {
  display: grid;
  gap: 12px;
}

.runs-list__item,
.actions-list__item {
  padding: 14px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
}

.runs-list__meta,
.actions-list__meta {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 6px;
  font-size: 12px;
}

.runs-list__item p,
.actions-list__item p {
  margin: 6px 0;
  color: var(--if-color-muted);
  line-height: 1.4;
}

.runs-list__time {
  font-size: 12px;
  color: var(--if-color-muted);
}

.run-status,
.approval-status {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 11px;
  font-weight: 700;
}

.run-status--completed { background: rgba(15, 118, 110, 0.12); color: var(--if-color-accent); }
.run-status--failed { background: rgba(180, 35, 24, 0.12); color: var(--if-color-danger); }
.run-status--waiting_approval { background: rgba(180, 105, 14, 0.12); color: var(--if-color-warning); }
.approval-status--approved { background: rgba(15, 118, 110, 0.12); color: var(--if-color-accent); }
.approval-status--rejected { background: rgba(180, 35, 24, 0.12); color: var(--if-color-danger); }
.approval-status--pending { background: rgba(21, 94, 117, 0.14); color: var(--if-color-accent-strong); }

.memory-item {
  padding: 14px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
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
}

.memory-tag {
  display: inline-flex;
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 11px;
  font-weight: 700;
}

.memory-tag--blocker { background: rgba(180, 35, 24, 0.09); color: var(--if-color-danger); }
.memory-tag--risk { background: rgba(180, 105, 14, 0.09); color: var(--if-color-warning); }

@media (max-width: 720px) {
  .detail-card__grid {
    grid-template-columns: 1fr;
  }
}
</style>
