<script setup lang="ts">
import type { AgentMessage } from "@/stores/agent.store";
import { NSpace, NTag, NText, NCard } from "naive-ui";

const props = defineProps<{
  message: AgentMessage;
}>();

function toolCallData() {
  try {
    return JSON.parse(props.message.content);
  } catch {
    return {};
  }
}
</script>

<template>
  <NCard
    size="small"
    :bordered="true"
    style="display: inline-block; max-width: 80%"
  >
    <NSpace align="center">
      <NTag type="info" size="small"> 🔧 {{ toolCallData().name }} </NTag>
      <NText depth="3" style="font-size: 12px">
        {{ toolCallData().result ? "done" : "running..." }}
      </NText>
    </NSpace>
  </NCard>
</template>
