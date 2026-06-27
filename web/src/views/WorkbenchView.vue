<template>
  <app-shell active-key="overview">
    <div v-if="!currentWorkbench">
      <n-empty description="Select or add a workbench to get started" />
    </div>

    <div v-else>
      <!-- collapsible stats -->
      <n-card size="small" style="margin-bottom: 16px">
        <template #header>
          <div style="display: flex; justify-content: space-between; align-items: center; cursor: pointer" @click="statsOpen = !statsOpen">
            <span>Overview</span>
            <span style="font-size: 12px; color: var(--n-text-color-3)">{{ statsOpen ? '▲' : '▼' }}</span>
          </div>
        </template>
        <div v-show="statsOpen">
          <n-spin :show="loadingStats">
            <div style="display: flex; gap: 32px; flex-wrap: wrap">
              <n-statistic label="Active Agents" :value="activeRunCount" />
              <n-statistic label="Total Issues" :value="issues.length" />
              <n-statistic label="Opened" :value="openCount" />
              <n-statistic label="Milestones" :value="milestones.length" />
            </div>
          </n-spin>
        </div>
      </n-card>

      <!-- main workspace: sessions left, a2ui right -->
      <div style="display: flex; gap: 0; min-height: 60vh">
        <!-- left: session list -->
        <div style="width: 280px; border: 1px solid var(--n-border-color); border-radius: 4px; flex-shrink: 0; display: flex; flex-direction: column">
          <div style="padding: 8px 12px; border-bottom: 1px solid var(--n-border-color); display: flex; justify-content: space-between; align-items: center">
            <span style="font-weight: 600; font-size: 14px">Agent Sessions</span>
            <n-button size="tiny" @click="handleCreateSession">+ New</n-button>
          </div>
          <n-scrollbar style="flex: 1">
            <div v-if="agentStore.sessions.length === 0" style="padding: 24px; text-align: center">
              <n-text depth="3">No sessions yet</n-text>
            </div>
            <div
              v-for="s in agentStore.sessions"
              :key="s.id"
              style="padding: 8px 12px; cursor: pointer; border-bottom: 1px solid var(--n-border-color)"
              :style="{ background: s.id === agentStore.activeSessionId ? 'var(--n-color-embedded)' : 'transparent' }"
              @click="selectSession(s.id)"
            >
              <div style="display: flex; justify-content: space-between; align-items: center">
                <n-text style="font-size: 13px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1">
                  {{ s.title || 'New Session' }}
                </n-text>
                <n-button text size="tiny" type="error" @click.stop="handleDeleteSession(s.id)">×</n-button>
              </div>
              <n-text depth="3" style="font-size: 11px">{{ s.last_message_at?.slice(0, 10) }}</n-text>
            </div>
          </n-scrollbar>
        </div>

        <!-- right: A2UI / agent content -->
        <div style="flex: 1; margin-left: 12px; border: 1px solid var(--n-border-color); border-radius: 4px; display: flex; flex-direction: column">
          <div v-if="!agentStore.activeSessionId" style="flex: 1; display: flex; align-items: center; justify-content: center">
            <n-empty description="Select an agent session or create a new one" />
          </div>
          <div v-else style="flex: 1; display: flex; flex-direction: column">
            <!-- chat area -->
            <div style="flex: 1; overflow-y: auto; padding: 12px">
              <div v-if="agentStore.messages.length === 0" style="text-align: center; padding-top: 60px">
                <n-text depth="3">Describe what you need the agent to do</n-text>
              </div>
              <div v-for="msg in agentStore.messages" :key="msg.id" style="margin-bottom: 8px">
                <div v-if="msg.role === 'user'" style="text-align: right">
                  <n-tag type="info" size="small" style="max-width: 80%; text-align: left; white-space: pre-wrap; word-break: break-word">
                    {{ msg.content }}
                  </n-tag>
                </div>
                <div v-else-if="msg.role === 'assistant'" style="text-align: left">
                  <n-text style="white-space: pre-wrap">{{ msg.content || '...' }}</n-text>
                </div>
                <div v-else-if="msg.message_kind === 'tool_call'" style="text-align: left">
                  <n-tag type="warning" size="small">🔧 {{ tryParseTool(msg.content) }}</n-tag>
                </div>
              </div>
              <div v-if="agentStore.streaming" style="text-align: center; padding: 8px">
                <n-text depth="3" style="font-size: 12px">Agent working...</n-text>
              </div>
            </div>
            <!-- input -->
            <div style="border-top: 1px solid var(--n-border-color); padding: 8px 12px">
              <div style="display: flex; gap: 8px">
                <n-input
                  v-model:value="chatInput"
                  placeholder="Describe what you need..."
                  :disabled="agentStore.streaming"
                  size="small"
                  style="flex: 1"
                  @keyup.enter="sendMessage"
                />
                <n-button size="small" type="primary" :disabled="agentStore.streaming || !chatInput.trim()" @click="sendMessage">
                  Send
                </n-button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { NCard, NEmpty, NScrollbar, NSpin, NStatistic, NTag, NButton, NInput, NText } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { useSessionStore } from "@/stores/session.store";
import { useAgentStore } from "@/stores/agent.store";
import { useAgentRun } from "@/composables/useAgentRun";
import { listProjectIssues, listMilestones } from "@/api/issues.api";
import { listSessions, createSession, deleteSession, getSession } from "@/api/agent.api";
import type { GitlabIssue, Milestone } from "@/api/issues.api";

const store = useSessionStore();
const agentStore = useAgentStore();
const { run } = useAgentRun();

const issues = ref<GitlabIssue[]>([]);
const milestones = ref<Milestone[]>([]);
const loadingStats = ref(false);
const statsOpen = ref(true);
const chatInput = ref("");

const currentWorkbench = computed(() =>
  store.workbenches.find((w) => w.id === store.currentWorkbenchId.value) ?? null,
);

const openCount = computed(() => issues.value.filter((i) => i.state === "opened").length);

const activeRunCount = computed(() =>
  agentStore.sessions.filter((s) => s.latest_state !== null).length,
);

function tryParseTool(content: string): string {
  try {
    const d = JSON.parse(content);
    return d.name || "tool";
  } catch {
    return "tool";
  }
}

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

async function sendMessage() {
  const text = chatInput.value.trim();
  if (!text) return;
  const sessionId = agentStore.activeSessionId;
  const wbId = store.currentWorkbenchId?.value ?? store.currentWorkbenchId;
  if (!sessionId || typeof wbId !== "number") return;

  agentStore.addUserMessage(text);
  chatInput.value = "";

  await run({
    threadId: sessionId,
    workbenchId: wbId,
    messages: [{ role: "user", content: text }],
  });
}

onMounted(async () => {
  const ok = await store.checkAuth();
  if (!ok) return;
  const list = await store.fetchWorkbenches();
  if (list.length > 0) store.setCurrentWorkbench(list[0].id);
});

watch(currentWorkbench, async (wb) => {
  if (wb) {
    loadStats();
    loadSessions();
  } else {
    issues.value = [];
    milestones.value = [];
    agentStore.setSessions([]);
  }
}, { immediate: true });
</script>
