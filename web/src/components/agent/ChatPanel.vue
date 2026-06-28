<script setup lang="ts">
import { NScrollbar } from "naive-ui";
import { useAgentStore } from "@/stores/agent.store";
import { getSession } from "@/api/agent.api";
import { useSessionStore } from "@/stores/session.store";
import { onMounted, watch } from "vue";
import ChatMessages from "./ChatMessages.vue";
import ChatInput from "./ChatInput.vue";

const props = defineProps<{ sessionId: string }>();
const agentStore = useAgentStore();
const sessionStore = useSessionStore();

async function loadHistory() {
  const wbId =
    sessionStore.currentWorkbenchId?.value ?? sessionStore.currentWorkbenchId;
  if (typeof wbId !== "number") return;
  const detail = await getSession(wbId, props.sessionId);
  if (detail?.messages) {
    agentStore.setHistory(detail.messages);
  }
}

onMounted(loadHistory);
watch(() => props.sessionId, loadHistory);
</script>

<template>
  <div class="agent-chat-panel">
    <NScrollbar class="agent-chat-scroll">
      <div class="agent-chat-scroll__inner">
        <ChatMessages
          :messages="agentStore.messages"
          :streaming="agentStore.streaming"
        />
      </div>
    </NScrollbar>
    <div class="agent-chat-input">
      <ChatInput :disabled="agentStore.streaming" />
    </div>
  </div>
</template>

<style scoped>
.agent-chat-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
}

.agent-chat-scroll {
  flex: 1;
  min-height: 0;
}

.agent-chat-scroll__inner {
  padding: 16px;
  min-height: 100%;
}

.agent-chat-input {
  flex-shrink: 0;
  border-top: 1px solid var(--n-border-color);
  padding: 12px;
  background: var(--if-color-surface);
}
</style>
