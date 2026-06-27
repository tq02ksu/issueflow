<script setup lang="ts">
import type { AgentMessage } from "@/stores/agent.store";
import { NSpace, NTag, NText, NCard } from "naive-ui";

defineProps<{
  messages: AgentMessage[];
  streaming: boolean;
}>();

function isToolCall(msg: AgentMessage) {
  return msg.message_kind === "tool_call";
}

function toolCallData(msg: AgentMessage) {
  try {
    return JSON.parse(msg.content);
  } catch {
    return {};
  }
}
</script>

<template>
  <NSpace vertical :size="12" style="width: 100%">
    <div v-for="msg in messages" :key="msg.id">
      <div v-if="msg.role === 'user'" style="text-align: right">
        <NCard size="small" style="display: inline-block; max-width: 80%; background: var(--n-color-target)">
          {{ msg.content }}
        </NCard>
      </div>
      <div v-else-if="msg.role === 'assistant'" style="text-align: left">
        <NCard size="small" style="display: inline-block; max-width: 80%">
          <NText>{{ msg.content || '...' }}</NText>
        </NCard>
      </div>
      <div v-else-if="isToolCall(msg)" style="text-align: left">
        <NCard size="small" :bordered="true" style="display: inline-block; max-width: 80%">
          <NSpace align="center">
            <NTag type="info" size="small">🔧 {{ toolCallData(msg).name }}</NTag>
            <NText depth="3" style="font-size: 12px">{{ toolCallData(msg).result ? '✓ done' : 'running...' }}</NText>
          </NSpace>
        </NCard>
      </div>
    </div>
    <div v-if="streaming" style="text-align: center">
      <NText depth="3" style="font-size: 12px">Agent is working...</NText>
    </div>
  </NSpace>
</template>
