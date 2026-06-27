<template>
  <app-shell active-key="overview">
    <div class="header">
      <WorkbenchSwitcher @select="switchWorkbench" @add="showDialog = true" />
    </div>
    <WorkbenchSearchDialog
      :visible="showDialog"
      @close="showDialog = false"
      @select="createWorkbench"
    />
    <n-card :bordered="false" class="panel">
      <template #header>
        <span>{{ currentWorkbench ? currentWorkbench.project_path : 'Agent Workbench' }}</span>
      </template>

      <div v-if="!currentWorkbench">
        <n-empty description="Select or add a workbench to get started" />
      </div>

      <div v-else>
        <h3>Issues</h3>
        <p class="muted">Issue management for {{ currentWorkbench.project_path }}</p>

        <h3 style="margin-top: 24px">Agent Sessions</h3>
        <p class="muted">Agent sessions for {{ currentWorkbench.project_path }}</p>
      </div>
    </n-card>
  </app-shell>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { NCard, NEmpty } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import WorkbenchSwitcher from "@/components/workbench/WorkbenchSwitcher.vue";
import WorkbenchSearchDialog from "@/components/workbench/WorkbenchSearchDialog.vue";
import { useSessionStore } from "@/stores/session";
import type { GitLabProject } from "@/stores/session";

const store = useSessionStore();
const showDialog = ref(false);

const currentWorkbench = computed(() =>
  store.workbenches.find((w) => w.id === store.currentWorkbenchId.value) ?? null,
);

onMounted(async () => {
  const ok = await store.checkAuth();
  if (!ok) return;

  try {
    const resp = await store.authFetch("/api/workbenches");
    if (resp.ok) {
      const list = await resp.json();
      store.setWorkbenches(list);
      if (list.length > 0) store.setCurrentWorkbench(list[0].id);
    }
  } catch { /* API not ready */ }
});

function switchWorkbench(id: number) {
  store.setCurrentWorkbench(id);
}

async function createWorkbench(project: GitLabProject) {
  const resp = await store.authFetch("/api/workbenches", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      project_id: project.id,
      project_name: project.name,
      project_path: project.path_with_namespace,
    }),
  });
  if (resp.ok) {
    const wb = await resp.json();
    store.setWorkbenches([...store.workbenches, wb]);
    store.setCurrentWorkbench(wb.id);
    showDialog.value = false;
  }
}
</script>

<style scoped>
.header {
  padding: 12px 24px 0;
}

.panel {
  max-width: 720px;
  margin: 16px 24px;
  border-radius: var(--if-radius-md);
  box-shadow: var(--if-shadow-panel);
}

.muted {
  color: var(--if-color-muted);
}
</style>
