<script setup lang="ts">
import { ref } from "vue";
import { NInput, NButton, NSpace } from "naive-ui";
import { useAgentStore } from "@/stores/agent.store";
import { useAgentRun } from "@/composables/useAgentRun";
import { useSessionStore } from "@/stores/session.store";

defineProps<{ disabled: boolean }>();

const input = ref("");
const agentStore = useAgentStore();
const sessionStore = useSessionStore();
const { run } = useAgentRun();

async function send() {
  const text = input.value.trim();
  if (!text) return;
  const sessionId = agentStore.activeSessionId;
  const wbId =
    sessionStore.currentWorkbenchId?.value ?? sessionStore.currentWorkbenchId;
  if (!sessionId || typeof wbId !== "number") return;

  agentStore.addUserMessage(text);
  input.value = "";

  await run({
    threadId: sessionId,
    workbenchId: wbId,
    messages: [{ role: "user", content: text }],
  });
}
</script>

<template>
  <NSpace style="width: 100%">
    <NInput
      v-model:value="input"
      placeholder="Describe what you need..."
      :disabled="disabled"
      style="flex: 1"
      @keyup.enter="send"
    />
    <NButton type="primary" :disabled="disabled || !input.trim()" @click="send">
      Send
    </NButton>
  </NSpace>
</template>
