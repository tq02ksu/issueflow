<template>
  <app-shell active-key="pending-actions">
    <div v-if="!currentWorkbench">
      <n-empty
        description="Select or add a workbench to review pending actions"
      />
    </div>

    <div v-else class="pending-actions-page">
      <div class="pending-actions-page__header">
        <div>
          <n-h3 style="margin: 0">Pending Actions</n-h3>
          <n-text depth="3">
            Review AI-prepared changes before writing back to GitLab.
          </n-text>
        </div>
        <n-button size="small" @click="loadPendingActions">Refresh</n-button>
      </div>

      <n-spin :show="loading">
        <div
          v-if="pendingActions.length === 0"
          class="pending-actions-page__empty"
        >
          <n-empty description="No pending confirmations" />
        </div>

        <div v-else class="pending-actions-layout">
          <div class="pending-actions-list">
            <button
              v-for="action in pendingActions"
              :key="action.id"
              class="pending-action-item"
              :class="{
                'pending-action-item--active':
                  action.id === selectedPendingActionId,
              }"
              type="button"
              @click="selectPendingAction(action.id)"
            >
              <div class="pending-action-item__header">
                <span class="pending-action-item__title">
                  {{ action.action_type }}
                </span>
                <span class="pending-action-item__status">
                  {{ action.status }}
                </span>
              </div>
              <span class="pending-action-item__meta">
                {{ action.artifact_type }} #{{ action.artifact_id }}
              </span>
            </button>
          </div>

          <n-card
            v-if="selectedPendingActionDetail"
            class="pending-action-preview"
            :bordered="false"
          >
            <template #header>
              <div class="pending-action-preview__header">
                <div>
                  <div class="pending-action-preview__title">
                    {{
                      selectedPendingActionDetail.preview?.title ||
                      "Action Preview"
                    }}
                  </div>
                  <n-text depth="3">
                    {{
                      selectedPendingActionDetail.preview?.kind || "raw_payload"
                    }}
                  </n-text>
                </div>
                <n-button
                  size="small"
                  type="primary"
                  :disabled="
                    selectedPendingActionDetail.action.status !== 'pending'
                  "
                  @click="handleConfirmPendingAction"
                >
                  Confirm
                </n-button>
              </div>
            </template>

            <pre class="pending-action-preview__body">{{
              selectedPendingActionDetail.preview?.body ||
              selectedPendingActionDetail.action.payload
            }}</pre>
          </n-card>
        </div>
      </n-spin>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { NButton, NCard, NEmpty, NH3, NSpin, NText } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { useSessionStore } from "@/stores/session.store";
import {
  confirmPendingAction,
  getPendingAction,
  listPendingActions,
  type PendingAction,
  type PendingActionDetail,
} from "@/api/workbench.api";

const store = useSessionStore();

const pendingActions = ref<PendingAction[]>([]);
const selectedPendingActionId = ref<string | null>(null);
const selectedPendingActionDetail = ref<PendingActionDetail | null>(null);
const loading = ref(false);

const currentWorkbench = computed(
  () =>
    store.workbenches.find((w) => w.id === store.currentWorkbenchId.value) ??
    null,
);

async function loadPendingActions() {
  const wbId = store.currentWorkbenchId?.value ?? store.currentWorkbenchId;
  if (typeof wbId !== "number") return;

  loading.value = true;
  try {
    pendingActions.value = await listPendingActions(wbId);

    const activeId =
      selectedPendingActionId.value &&
      pendingActions.value.some(
        (action) => action.id === selectedPendingActionId.value,
      )
        ? selectedPendingActionId.value
        : (pendingActions.value[0]?.id ?? null);

    selectedPendingActionId.value = activeId;
    selectedPendingActionDetail.value = activeId
      ? await getPendingAction(activeId)
      : null;
  } finally {
    loading.value = false;
  }
}

async function selectPendingAction(id: string) {
  selectedPendingActionId.value = id;
  selectedPendingActionDetail.value = await getPendingAction(id);
}

async function handleConfirmPendingAction() {
  if (!selectedPendingActionId.value) return;
  const confirmed = await confirmPendingAction(selectedPendingActionId.value);
  if (!confirmed) return;
  await loadPendingActions();
}

onMounted(async () => {
  const ok = await store.checkAuth();
  if (!ok) return;

  const list = await store.fetchWorkbenches();
  if (list.length > 0 && store.currentWorkbenchId.value === null) {
    store.setCurrentWorkbench(list[0].id);
  }
});

watch(
  currentWorkbench,
  async (wb) => {
    if (wb) {
      await loadPendingActions();
    } else {
      pendingActions.value = [];
      selectedPendingActionId.value = null;
      selectedPendingActionDetail.value = null;
    }
  },
  { immediate: true },
);
</script>

<style scoped>
.pending-actions-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-height: calc(100vh - 220px);
}

.pending-actions-page__header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
}

.pending-actions-page__empty {
  padding: 48px 0;
}

.pending-actions-layout {
  display: grid;
  grid-template-columns: minmax(240px, 320px) minmax(0, 1fr);
  gap: 16px;
  min-height: 480px;
}

.pending-actions-list {
  border: 1px solid var(--n-border-color);
  border-radius: 12px;
  background: var(--n-color);
  overflow: hidden;
}

.pending-action-item {
  width: 100%;
  border: 0;
  border-bottom: 1px solid var(--n-border-color);
  background: transparent;
  padding: 14px 16px;
  display: flex;
  flex-direction: column;
  gap: 6px;
  text-align: left;
  cursor: pointer;
}

.pending-action-item:last-child {
  border-bottom: 0;
}

.pending-action-item--active {
  background: color-mix(in srgb, var(--if-color-accent) 10%, white);
}

.pending-action-item__header {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: center;
}

.pending-action-item__title {
  font-size: 13px;
  font-weight: 600;
}

.pending-action-item__status,
.pending-action-item__meta {
  font-size: 12px;
  color: var(--n-text-color-3);
}

.pending-action-item__status {
  text-transform: uppercase;
}

.pending-action-preview {
  min-width: 0;
  border-radius: 12px;
  background: linear-gradient(
    180deg,
    color-mix(in srgb, var(--if-color-accent) 5%, white),
    white 160px
  );
  border: 1px solid var(--n-border-color);
}

.pending-action-preview__header {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  align-items: flex-start;
}

.pending-action-preview__title {
  font-size: 15px;
  font-weight: 600;
}

.pending-action-preview__body {
  margin: 0;
  min-height: 360px;
  padding: 16px;
  overflow: auto;
  white-space: pre-wrap;
  word-break: break-word;
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.92);
  border: 1px solid var(--n-border-color);
  font-size: 12px;
  line-height: 1.55;
}

@media (max-width: 960px) {
  .pending-actions-layout {
    grid-template-columns: 1fr;
  }

  .pending-action-preview__header,
  .pending-actions-page__header {
    flex-direction: column;
    align-items: stretch;
  }
}
</style>
