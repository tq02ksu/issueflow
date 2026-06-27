<template>
  <app-shell active-key="overview">
    <n-card :bordered="false" class="panel">
      <template #header>
        <span>{{ currentWorkbench ? (currentWorkbench.name || currentWorkbench.project_path) : 'Agent Workbench' }}</span>
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
import { computed, onMounted } from "vue";
import { NCard, NEmpty } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { useSessionStore } from "@/stores/session.store";

const store = useSessionStore();

const currentWorkbench = computed(() =>
  store.workbenches.find((w) => w.id === store.currentWorkbenchId.value) ?? null,
);

onMounted(async () => {
  const ok = await store.checkAuth();
  if (!ok) return;

  const list = await store.fetchWorkbenches();
  if (list.length > 0) store.setCurrentWorkbench(list[0].id);
});
</script>

<style scoped>
.panel {
  max-width: 720px;
  border-radius: var(--if-radius-md);
  box-shadow: var(--if-shadow-panel);
}

.muted {
  color: var(--if-color-muted);
}
</style>
