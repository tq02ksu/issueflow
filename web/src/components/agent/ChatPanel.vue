<script setup lang="ts">
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
  const wbId = sessionStore.currentWorkbenchId?.value ?? sessionStore.currentWorkbenchId;
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
  <div style="display: flex; flex-direction: column; height: 100%">
    <div style="flex: 1; overflow-y: auto; padding: 16px">
      <ChatMessages :messages="agentStore.messages" :streaming="agentStore.streaming" />
    </div>
    <div style="border-top: 1px solid var(--n-border-color); padding: 12px">
      <ChatInput :disabled="agentStore.streaming" />
    </div>
  </div>
</template>
