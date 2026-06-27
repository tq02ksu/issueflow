<template>
  <app-shell active-key="issues">
    <div class="issues-header">
      <n-h3 style="margin: 0">Issues</n-h3>
      <n-radio-group v-model:value="viewMode" size="small">
        <n-radio-button value="list">List</n-radio-button>
        <n-radio-button value="kanban">Kanban</n-radio-button>
      </n-radio-group>
    </div>

    <n-spin :show="loading">
      <!-- List View -->
      <n-card v-if="viewMode === 'list'" :bordered="false" class="panel" style="margin-top: 16px">
        <n-list v-if="issues.length > 0">
          <n-list-item v-for="issue in issues" :key="issue.id" @click="openDetail(issue)">
            <template #prefix>
              <n-tag :type="issue.state === 'opened' ? 'success' : 'default'" size="small">
                #{{ issue.iid }}
              </n-tag>
            </template>
            <div class="issue-row">
              <span class="issue-title">{{ issue.title }}</span>
              <div class="issue-meta">
                <n-tag v-for="label in issue.labels" :key="label" size="tiny" style="margin-right: 4px">
                  {{ label }}
                </n-tag>
                <span v-if="issue.milestone" class="milestone-badge">{{ issue.milestone.title }}</span>
                <span class="comment-count">{{ commentCount(issue.iid) }} comments</span>
              </div>
            </div>
          </n-list-item>
        </n-list>
        <n-empty v-else description="No issues" />
      </n-card>

      <!-- Kanban View -->
      <div v-else class="kanban">
        <div v-for="ms in groupedMilestones" :key="ms.title" class="kanban-column">
          <n-card size="small" :title="ms.title"
            :header-style="{ padding: '8px 12px', fontSize: '13px' }"
            :content-style="{ padding: '4px 8px' }"
            style="min-width: 240px; max-width: 280px">
            <n-list>
              <n-list-item v-for="issue in ms.issues" :key="issue.id" @click="openDetail(issue)"
                style="cursor: pointer; padding: 6px 8px; margin-bottom: 4px; border-radius: 4px; border: 1px solid var(--n-border-color)">
                <div class="kanban-card">
                  <div class="issue-title">{{ issue.title }}</div>
                  <div class="issue-meta">
                    <n-tag v-for="label in issue.labels" :key="label" size="tiny" style="margin-right: 2px">
                      {{ label }}
                    </n-tag>
                  </div>
                </div>
              </n-list-item>
            </n-list>
          </n-card>
        </div>
        <!-- Unassigned column -->
        <div class="kanban-column">
          <n-card size="small" title="No Milestone"
            :header-style="{ padding: '8px 12px', fontSize: '13px' }"
            :content-style="{ padding: '4px 8px' }"
            style="min-width: 240px; max-width: 280px">
            <n-list>
              <n-list-item v-for="issue in unassignedIssues" :key="issue.id" @click="openDetail(issue)"
                style="cursor: pointer; padding: 6px 8px; margin-bottom: 4px; border-radius: 4px; border: 1px solid var(--n-border-color)">
                <div class="kanban-card">
                  <div class="issue-title">{{ issue.title }}</div>
                  <div class="issue-meta">
                    <n-tag v-for="label in issue.labels" :key="label" size="tiny" style="margin-right: 2px">
                      {{ label }}
                    </n-tag>
                  </div>
                </div>
              </n-list-item>
            </n-list>
          </n-card>
        </div>
      </div>
    </n-spin>

    <!-- Detail Drawer -->
    <n-drawer v-model:show="showDetail" :width="480" placement="right">
      <n-drawer-content v-if="detailIssue" :title="`#${detailIssue.iid} ${detailIssue.title}`">
        <n-tag :type="detailIssue.state === 'opened' ? 'success' : 'default'">
          {{ detailIssue.state }}
        </n-tag>
        <n-tag v-for="label in detailIssue.labels" :key="label" size="small" style="margin-left: 4px">
          {{ label }}
        </n-tag>
        <span v-if="detailIssue.milestone" style="margin-left: 8px; color: var(--if-color-muted)">
          {{ detailIssue.milestone.title }}
        </span>

        <n-divider />
        <div v-if="detailIssue.description" class="detail-desc">{{ detailIssue.description }}</div>

        <n-divider />
        <h4>Comments ({{ detailNotes.length }})</h4>
        <n-spin :show="loadingNotes">
          <div v-if="detailNotes.length === 0" class="muted">No comments</div>
          <div v-for="note in detailNotes" :key="note.id" class="comment">
            <div class="comment-header">
              <strong>{{ note.author_name }}</strong>
              <span class="muted">{{ note.created_at }}</span>
            </div>
            <div class="comment-body">{{ note.body }}</div>
          </div>
        </n-spin>
      </n-drawer-content>
    </n-drawer>
  </app-shell>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import {
  NButton, NCard, NDivider, NDrawer, NDrawerContent, NEmpty,
  NH3, NList, NListItem, NRadioButton, NRadioGroup, NSpin, NTag,
} from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { useSessionStore } from "@/stores/session.store";
import { listProjectIssues, listMilestones, listIssueNotes } from "@/api/issues.api";
import type { GitlabIssue, Milestone, IssueNote } from "@/api/issues.api";

const store = useSessionStore();

const viewMode = ref<"list" | "kanban">("list");
const issues = ref<GitlabIssue[]>([]);
const milestones = ref<Milestone[]>([]);
const loading = ref(false);

const showDetail = ref(false);
const detailIssue = ref<GitlabIssue | null>(null);
const detailNotes = ref<IssueNote[]>([]);
const loadingNotes = ref(false);

// Track comment counts (fetched on demand)
const commentCounts = ref<Map<number, number>>(new Map());

const currentWorkbench = computed(() =>
  store.workbenches.find((w) => w.id === store.currentWorkbenchId.value) ?? null,
);

const groupedMilestones = computed(() =>
  milestones.value.map((ms) => ({
    title: ms.title,
    issues: issues.value.filter((i) => i.milestone?.title === ms.title),
  })),
);

const unassignedIssues = computed(() =>
  issues.value.filter((i) => !i.milestone),
);

function commentCount(iid: number): number {
  return commentCounts.value.get(iid) ?? 0;
}

onMounted(async () => {
  const ok = await store.checkAuth();
  if (!ok) return;

  const list = await store.fetchWorkbenches();
  if (list.length > 0) store.setCurrentWorkbench(list[0].id);
});

watch(currentWorkbench, async (wb) => {
  if (wb) {
    loading.value = true;
    const [iss, ms] = await Promise.all([
      listProjectIssues(wb.project_id),
      listMilestones(wb.project_id),
    ]);
    issues.value = iss;
    milestones.value = ms;
    loading.value = false;
  } else {
    issues.value = [];
    milestones.value = [];
  }
}, { immediate: true });

async function openDetail(issue: GitlabIssue) {
  detailIssue.value = issue;
  showDetail.value = true;
  loadingNotes.value = true;
  detailNotes.value = await listIssueNotes(issue.project_id, issue.iid);
  commentCounts.value.set(issue.iid, detailNotes.value.length);
  loadingNotes.value = false;
}
</script>

<style scoped>
.issues-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.panel {
  max-width: 900px;
  border-radius: var(--if-radius-md);
  box-shadow: var(--if-shadow-panel);
}

.issue-row {
  width: 100%;
}

.issue-title {
  font-weight: 500;
}

.issue-meta {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-top: 4px;
  flex-wrap: wrap;
}

.milestone-badge {
  font-size: 12px;
  color: var(--if-color-muted);
  background: var(--n-color-embedded);
  padding: 1px 6px;
  border-radius: 4px;
}

.comment-count {
  font-size: 12px;
  color: var(--if-color-muted);
  margin-left: 8px;
}

.kanban {
  display: flex;
  gap: 16px;
  overflow-x: auto;
  padding-bottom: 16px;
}

.kanban-column {
  flex-shrink: 0;
}

.kanban-card .issue-title {
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 4px;
}

.detail-desc {
  white-space: pre-wrap;
  font-size: 14px;
  line-height: 1.6;
}

.comment {
  margin-bottom: 16px;
  padding: 8px;
  border-radius: 6px;
  background: var(--n-color-embedded);
}

.comment-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 4px;
  font-size: 13px;
}

.comment-body {
  font-size: 14px;
  line-height: 1.5;
}

.muted {
  color: var(--if-color-muted);
  font-size: 13px;
}
</style>
