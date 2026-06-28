<script setup lang="ts">
import { onMounted, watch } from "vue";
import { useAgentStore } from "@/stores/agent.store";
import { listSessions, createSession, deleteSession } from "@/api/agent.api";
import { useSessionStore } from "@/stores/session.store";
import SessionList from "@/components/agent/SessionList.vue";
import ChatPanel from "@/components/agent/ChatPanel.vue";
import { NSpace, NScrollbar, NButton, NText, NEmpty } from "naive-ui";

const agentStore = useAgentStore();
const sessionStore = useSessionStore();

async function loadSessions() {
  const wbId = sessionStore.currentWorkbenchId?.value ?? sessionStore.currentWorkbenchId;
  if (typeof wbId !== "number") return;
  const list = await listSessions(wbId);
  agentStore.setSessions(list);
}

async function handleCreate() {
  const wbId = sessionStore.currentWorkbenchId?.value ?? sessionStore.currentWorkbenchId;
  if (typeof wbId !== "number") return;
  const session = await createSession(wbId);
  if (session) {
    await loadSessions();
    selectSession(session.id);
  }
}

async function handleDelete(id: string) {
  const wbId = sessionStore.currentWorkbenchId?.value ?? sessionStore.currentWorkbenchId;
  if (typeof wbId !== "number") return;
  await deleteSession(wbId, id);
  if (agentStore.activeSessionId === id) {
    agentStore.setActiveSession("");
    agentStore.clearMessages();
  }
  await loadSessions();
}

function selectSession(id: string) {
  agentStore.setActiveSession(id);
}

onMounted(async () => {
  await loadSessions();
});

watch(
  () => sessionStore.currentWorkbenchId?.value ?? sessionStore.currentWorkbenchId,
  async () => {
    agentStore.setActiveSession("");
    agentStore.clearMessages();
    await loadSessions();
  },
);
</script>

<template>
  <div style="display: flex; height: calc(100vh - 100px); gap: 0">
    <div style="width: 240px; border-right: 1px solid var(--n-border-color); flex-shrink: 0; display: flex; flex-direction: column">
      <div style="padding: 12px">
        <NButton block type="primary" size="small" @click="handleCreate">+ New Session</NButton>
      </div>
      <NScrollbar style="flex: 1">
        <div
          v-for="s in agentStore.sessions"
          :key="s.id"
          style="padding: 6px 12px; cursor: pointer; display: flex; justify-content: space-between; align-items: center; border-radius: 6px; transition: background 0.15s"
          :style="{ background: s.id === agentStore.activeSessionId ? 'var(--n-color-embedded)' : 'transparent' }"
          @click="selectSession(s.id)"
        >
          <NText style="font-size: 13px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap">
            {{ s.title || 'New Session' }}
          </NText>
          <NButton text size="tiny" type="error" @click.stop="handleDelete(s.id)">×</NButton>
        </div>
      </NScrollbar>
    </div>
    <div style="flex: 1; display: flex; flex-direction: column">
      <ChatPanel
        v-if="agentStore.activeSessionId"
        :session-id="agentStore.activeSessionId"
      />
      <div v-else style="flex: 1; display: flex; align-items: center; justify-content: center">
        <NEmpty description="Select or create an agent session" />
      </div>
    </div>
  </div>
</template>
