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
            <span style="font-size: 15px; font-weight: 600">{{ currentWorkbench?.name || currentWorkbench?.project_path }}</span>
            <span style="font-size: 12px; color: var(--n-text-color-3)">{{ statsOpen ? '▲' : '▼' }}</span>
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
      <div style="display: flex; gap: 0; height: calc(100vh - 260px); min-height: 400px">
        <!-- left: session list -->
        <div style="width: 280px; border: 1px solid var(--n-border-color); border-radius: 6px; flex-shrink: 0; display: flex; flex-direction: column">
          <div style="padding: 10px 14px; border-bottom: 1px solid var(--n-border-color); display: flex; justify-content: space-between; align-items: center; background: var(--n-color-embedded)">
            <span style="font-weight: 600; font-size: 14px">Agent Sessions</span>
            <n-button size="small" @click="handleCreateSession">+ New</n-button>
          </div>
          <n-scrollbar style="flex: 1">
            <div v-if="agentStore.sessions.length === 0" style="padding: 32px 16px; text-align: center">
              <n-text depth="3" style="font-size: 13px">No sessions yet</n-text>
            </div>
            <div
              v-for="s in agentStore.sessions"
              :key="s.id"
              style="padding: 10px 14px; cursor: pointer; border-bottom: 1px solid var(--n-divider-color)"
              :style="{ background: s.id === agentStore.activeSessionId ? 'var(--n-color-target)' : 'transparent' }"
              @click="selectSession(s.id)"
            >
              <div style="display: flex; justify-content: space-between; align-items: center">
                <n-ellipsis style="font-size: 13px; flex: 1">{{ s.title || 'New Session' }}</n-ellipsis>
                <n-button text size="tiny" type="error" @click.stop="handleDeleteSession(s.id)">×</n-button>
              </div>
              <n-text depth="3" style="font-size: 11px">{{ s.last_message_at?.slice(0, 16).replace('T', ' ') }}</n-text>
            </div>
          </n-scrollbar>
        </div>

        <!-- right: agent content -->
        <div style="flex: 1; margin-left: 12px; border: 1px solid var(--n-border-color); border-radius: 6px; display: flex; flex-direction: column; overflow: hidden">
          <div v-if="!agentStore.activeSessionId" style="flex: 1; display: flex; align-items: center; justify-content: center">
            <n-empty description="Select or create an agent session" style="font-size: 14px" />
          </div>
          <div v-else style="flex: 1; display: flex; flex-direction: column">
            <div style="flex: 1; overflow-y: auto; padding: 16px 20px">
              <div v-if="agentStore.messages.length === 0" style="text-align: center; padding-top: 80px">
                <n-text depth="3" style="font-size: 15px">Describe what you need the agent to do</n-text>
              </div>
              <div v-for="msg in agentStore.messages" :key="msg.id" style="margin-bottom: 14px">
                <!-- user message -->
                <div v-if="msg.role === 'user'" style="display: flex; justify-content: flex-end">
                  <div style="max-width: 75%; background: var(--n-color-target); border-radius: 12px 12px 0 12px; padding: 10px 16px">
                    <span style="font-size: 14px; line-height: 1.6; white-space: pre-wrap; word-break: break-word">{{ msg.content }}</span>
                  </div>
                </div>
                <!-- assistant message -->
                <div v-else-if="msg.role === 'assistant'" style="display: flex; justify-content: flex-start">
                  <div style="max-width: 75%; background: var(--n-color-embedded); border-radius: 12px 12px 12px 0; padding: 10px 16px">
                    <span style="font-size: 14px; line-height: 1.6; white-space: pre-wrap; word-break: break-word">{{ msg.content || '...' }}</span>
                  </div>
                </div>
                <!-- tool call -->
                <div v-else-if="msg.message_kind === 'tool_call'" style="display: flex; justify-content: flex-start">
                  <div style="max-width: 75%; border: 1px solid var(--n-warning-color); border-radius: 8px; padding: 8px 14px">
                    <n-text style="font-size: 13px">🔧 {{ tryParseTool(msg.content) }}</n-text>
                  </div>
                </div>
              </div>
              <div v-if="agentStore.streaming" style="text-align: center; padding: 12px">
                <n-text depth="3" style="font-size: 13px">Agent is working...</n-text>
              </div>
            </div>
            <div style="border-top: 1px solid var(--n-border-color); padding: 12px 16px">
              <div style="display: flex; gap: 10px">
                <n-input
                  v-model:value="chatInput"
                  placeholder="Describe what you need..."
                  :disabled="agentStore.streaming"
                  size="medium"
                  style="flex: 1"
                  @keyup.enter="sendMessage"
                />
                <n-button size="medium" type="primary" :disabled="agentStore.streaming || !chatInput.trim()" @click="sendMessage">
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
import { NCard, NEmpty, NScrollbar, NSpin, NStatistic, NButton, NInput, NText, NEllipsis } from "naive-ui";
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
