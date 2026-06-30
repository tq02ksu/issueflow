<template>
  <app-shell active-key="overview">
    <div v-if="!currentWorkbench">
      <n-empty description="Select or add a workbench to get started" />
    </div>

    <div v-else>
      <!-- collapsible stats -->
      <n-card size="small" style="margin-bottom: 16px">
        <template #header>
          <div
            style="
              display: flex;
              justify-content: space-between;
              align-items: center;
              cursor: pointer;
            "
            @click="statsOpen = !statsOpen"
          >
            <span style="font-size: 15px; font-weight: 600">{{
              currentWorkbench?.name || currentWorkbench?.project_path
            }}</span>
            <span style="font-size: 12px; color: var(--n-text-color-3)">{{
              statsOpen ? "▲" : "▼"
            }}</span>
          </div>
        </template>
        <div v-show="statsOpen">
          <n-spin :show="loadingStats">
            <div style="display: flex; gap: 40px; flex-wrap: wrap">
              <n-statistic label="Active Agents" :value="activeRunCount" />
              <n-statistic label="Total Issues" :value="issues.length" />
              <n-statistic label="Opened" :value="openCount" />
              <n-statistic label="Milestones" :value="milestones.length" />
            </div>
          </n-spin>
        </div>
      </n-card>

      <!-- main workspace -->
      <div class="workbench-overview">
        <div class="workbench-overview__sessions">
          <div class="workbench-overview__sessions-header">
            <span style="font-weight: 600; font-size: 14px"
              >Agent Sessions</span
            >
            <n-button size="small" @click="handleCreateSession">
              + New
            </n-button>
          </div>
          <n-scrollbar class="workbench-overview__sessions-scroll">
            <div
              v-if="agentStore.sessions.length === 0"
              style="padding: 32px 16px; text-align: center"
            >
              <n-text depth="3" style="font-size: 13px">
                No sessions yet
              </n-text>
            </div>
            <div
              v-for="s in agentStore.sessions"
              :key="s.id"
              class="workbench-session-item"
              :class="{
                'workbench-session-item--active':
                  s.id === agentStore.activeSessionId,
              }"
              @click="selectSession(s.id)"
            >
              <div
                style="
                  display: flex;
                  justify-content: space-between;
                  align-items: center;
                "
              >
                <n-ellipsis style="font-size: 13px; flex: 1">
                  {{ s.title || "New Session" }}
                </n-ellipsis>
                <n-button
                  text
                  size="tiny"
                  type="error"
                  @click.stop="handleDeleteSession(s.id)"
                >
                  ×
                </n-button>
              </div>
              <n-text depth="3" style="font-size: 11px">
                {{ s.last_message_at?.slice(0, 16).replace("T", " ") }}
              </n-text>
            </div>
          </n-scrollbar>
        </div>

        <div class="workbench-overview__chat">
          <div
            v-if="!agentStore.activeSessionId"
            style="
              flex: 1;
              display: flex;
              align-items: center;
              justify-content: center;
            "
          >
            <n-empty
              description="Select or create an agent session"
              style="font-size: 14px"
            />
          </div>
          <ChatPanel v-else :session-id="agentStore.activeSessionId" />
        </div>
      </div>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import {
  NCard,
  NEmpty,
  NScrollbar,
  NSpin,
  NStatistic,
  NButton,
  NText,
  NEllipsis,
} from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import ChatPanel from "@/components/agent/ChatPanel.vue";
import { useSessionStore } from "@/stores/session.store";
import { useAgentStore } from "@/stores/agent.store";
import { listProjectIssues, listMilestones } from "@/api/issues.api";
import {
  listSessions,
  createSession,
  deleteSession,
  getSession,
} from "@/api/agent.api";
import type { GitlabIssue, Milestone } from "@/api/issues.api";

const store = useSessionStore();
const agentStore = useAgentStore();

const issues = ref<GitlabIssue[]>([]);
const milestones = ref<Milestone[]>([]);
const loadingStats = ref(false);
const statsOpen = ref(true);

const currentWorkbench = computed(
  () =>
    store.workbenches.find((w) => w.id === store.currentWorkbenchId.value) ??
    null,
);

const openCount = computed(
  () => issues.value.filter((i) => i.state === "opened").length,
);

const activeRunCount = computed(
  () => agentStore.sessions.filter((s) => s.latest_state !== null).length,
);

async function loadStats() {
  if (!currentWorkbench.value) return;
  loadingStats.value = true;
  const [iss, ms] = await Promise.all([
    listProjectIssues(currentWorkbench.value.project_id),
    listMilestones(currentWorkbench.value.project_id),
  ]);
  issues.value = iss;
  milestones.value = ms;
  loadingStats.value = false;
}

async function loadSessions() {
  const wbId = store.currentWorkbenchId?.value ?? store.currentWorkbenchId;
  if (typeof wbId !== "number") return;
  const list = await listSessions(wbId);
  agentStore.setSessions(list);
}

async function handleCreateSession() {
  const wbId = store.currentWorkbenchId?.value ?? store.currentWorkbenchId;
  if (typeof wbId !== "number") return;
  const s = await createSession(wbId);
  if (s) {
    await loadSessions();
    selectSession(s.id);
  }
}

async function handleDeleteSession(id: string) {
  const wbId = store.currentWorkbenchId?.value ?? store.currentWorkbenchId;
  if (typeof wbId !== "number") return;
  await deleteSession(wbId, id);
  if (agentStore.activeSessionId === id) {
    agentStore.setActiveSession("");
    agentStore.clearMessages();
  }
  await loadSessions();
}

async function selectSession(id: string) {
  agentStore.setActiveSession(id);
  const wbId = store.currentWorkbenchId?.value ?? store.currentWorkbenchId;
  if (typeof wbId !== "number") return;
  const detail = await getSession(wbId, id);
  if (detail?.messages) {
    agentStore.setHistory(detail.messages);
  }
}

onMounted(async () => {
  const ok = await store.checkAuth();
  if (!ok) return;
  const list = await store.fetchWorkbenches();
  if (list.length > 0) store.setCurrentWorkbench(list[0].id);
});

watch(
  currentWorkbench,
  async (wb) => {
    if (wb) {
      loadStats();
      loadSessions();
    } else {
      issues.value = [];
      milestones.value = [];
      agentStore.setSessions([]);
    }
  },
  { immediate: true },
);
</script>

<style scoped>
.workbench-overview {
  display: flex;
  gap: 12px;
  height: calc(100vh - 260px);
  min-height: 400px;
  min-width: 0;
  min-height: 0;
}

.workbench-overview__sessions {
  width: 280px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  min-height: 0;
  border: 1px solid var(--n-border-color);
  border-radius: 6px;
}

.workbench-overview__sessions-header {
  padding: 10px 14px;
  border-bottom: 1px solid var(--n-border-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: var(--n-color-embedded);
}

.workbench-overview__sessions-scroll {
  flex: 1;
  min-height: 0;
}

.workbench-session-item {
  margin: 6px;
  padding: 10px 14px;
  cursor: pointer;
  border-radius: 10px;
  border: 1px solid transparent;
}

.workbench-session-item--active {
  background: color-mix(in srgb, var(--if-color-accent) 16%, white);
  border-color: color-mix(in srgb, var(--if-color-accent) 24%, white);
}

.workbench-overview__chat {
  flex: 1;
  min-width: 0;
  min-height: 0;
  border: 1px solid var(--n-border-color);
  border-radius: 6px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
</style>
