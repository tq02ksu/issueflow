<script setup lang="ts">
import { computed } from "vue";
import type { AgentMessage } from "@/stores/agent.store";
import { NSpace, NText, NCard } from "naive-ui";
import { extractA2UIMessages } from "@/composables/useA2UIBridge";
import ToolCallCard from "./ToolCallCard.vue";
import A2UISurfaceHost from "./A2UISurfaceHost.vue";

const props = defineProps<{
  messages: AgentMessage[];
  streaming: boolean;
}>();

function isToolCall(msg: AgentMessage) {
  return msg.message_kind === "tool_call";
}

const hasA2UIRenderMessages = computed(
  () => extractA2UIMessages(props.messages).length > 0,
);
</script>

<template>
  <NSpace vertical :size="12" style="width: 100%">
    <div v-for="msg in messages" :key="msg.id">
      <div v-if="msg.role === 'user'" style="text-align: right">
        <NCard
          size="small"
          style="
            display: inline-block;
            max-width: 80%;
            background: var(--n-color-target);
          "
        >
          {{ msg.content }}
        </NCard>
      </div>
      <div v-else-if="msg.role === 'assistant'" style="text-align: left">
        <NCard size="small" style="display: inline-block; max-width: 80%">
          <NText>{{ msg.content || "..." }}</NText>
        </NCard>
      </div>
      <div v-else-if="isToolCall(msg)" style="text-align: left">
        <ToolCallCard :message="msg" />
      </div>
    </div>
    <A2UISurfaceHost v-if="hasA2UIRenderMessages" :messages="messages" />
    <div v-if="streaming" style="text-align: center">
      <NText depth="3" style="font-size: 12px">Agent is working...</NText>
    </div>
  </NSpace>
</template>
