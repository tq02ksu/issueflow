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
        <n-spin :show="loadingIssues">
          <h3>Issues</h3>
          <p v-if="issues.length === 0 && !loadingIssues" class="muted">No issues found</p>
          <n-list v-if="issues.length > 0">
            <n-list-item v-for="issue in issues" :key="issue.id">
              <template #prefix>
                <n-tag :type="issue.state === 'opened' ? 'success' : 'default'" size="small">
                  {{ issue.state }}
                </n-tag>
              </template>
              <a :href="issue.web_url" target="_blank" class="issue-link">{{ issue.title }}</a>
            </n-list-item>
          </n-list>
        </n-spin>
      </div>
    </n-card>
  </app-shell>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { NCard, NEmpty, NList, NListItem, NSpin, NTag } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { useSessionStore } from "@/stores/session.store";
import { listProjectIssues } from "@/api/issues.api";
import type { GitlabIssue } from "@/api/issues.api";

const store = useSessionStore();

const issues = ref<GitlabIssue[]>([]);
const loadingIssues = ref(false);

const currentWorkbench = computed(() =>
  store.workbenches.find((w) => w.id === store.currentWorkbenchId.value) ?? null,
);

onMounted(async () => {
  const ok = await store.checkAuth();
  if (!ok) return;

  const list = await store.fetchWorkbenches();
  if (list.length > 0) store.setCurrentWorkbench(list[0].id);
});

watch(currentWorkbench, async (wb) => {
  if (wb) {
    loadingIssues.value = true;
    issues.value = await listProjectIssues(wb.project_id);
    loadingIssues.value = false;
  } else {
    issues.value = [];
  }
}, { immediate: true });
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

.issue-link {
  text-decoration: none;
  color: inherit;
}

.issue-link:hover {
  color: var(--if-color-accent);
}
</style>
