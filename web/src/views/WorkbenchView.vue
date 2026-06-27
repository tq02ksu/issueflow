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
          <div class="stats">
            <n-statistic label="Total Issues" :value="issues.length" />
            <n-statistic label="Opened" :value="openCount" />
            <n-statistic label="Milestones" :value="milestones.length" />
          </div>

          <div v-if="milestones.length > 0" style="margin-top: 24px">
            <h4>By Milestone</h4>
            <n-list>
              <n-list-item v-for="m in milestones" :key="m.id">
                <template #prefix>
                  <n-tag :type="m.state === 'active' ? 'info' : 'default'" size="small">
                    {{ m.title }}
                  </n-tag>
                </template>
                {{ milestoneIssueCount(m.title) }} issues
                <span v-if="m.due_date" style="margin-left: 12px; color: var(--if-color-muted); font-size: 12px">
                  Due {{ m.due_date }}
                </span>
              </n-list-item>
            </n-list>
          </div>
        </n-spin>
      </div>
    </n-card>
  </app-shell>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { NCard, NEmpty, NList, NListItem, NSpin, NStatistic, NTag } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { useSessionStore } from "@/stores/session.store";
import { listProjectIssues, listMilestones } from "@/api/issues.api";
import type { GitlabIssue, Milestone } from "@/api/issues.api";

const store = useSessionStore();

const issues = ref<GitlabIssue[]>([]);
const milestones = ref<Milestone[]>([]);
const loadingIssues = ref(false);

const currentWorkbench = computed(() =>
  store.workbenches.find((w) => w.id === store.currentWorkbenchId.value) ?? null,
);

const openCount = computed(() => issues.value.filter((i) => i.state === "opened").length);

function milestoneIssueCount(title: string): number {
  return issues.value.filter((i) => i.milestone?.title === title).length;
}

onMounted(async () => {
  const ok = await store.checkAuth();
  if (!ok) return;

  const list = await store.fetchWorkbenches();
  if (list.length > 0) store.setCurrentWorkbench(list[0].id);
});

watch(currentWorkbench, async (wb) => {
  if (wb) {
    loadingIssues.value = true;
    const [iss, ms] = await Promise.all([
      listProjectIssues(wb.project_id),
      listMilestones(wb.project_id),
    ]);
    issues.value = iss;
    milestones.value = ms;
    loadingIssues.value = false;
  } else {
    issues.value = [];
    milestones.value = [];
  }
}, { immediate: true });
</script>

<style scoped>
.panel {
  max-width: 720px;
  border-radius: var(--if-radius-md);
  box-shadow: var(--if-shadow-panel);
}

.stats {
  display: flex;
  gap: 32px;
}
</style>
