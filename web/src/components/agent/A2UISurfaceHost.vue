<script setup lang="ts">
import { computed, watchEffect } from "vue";
import { A2UISurface, useMessageProcessor } from "a2ui-vue";
import { extractA2UIMessages } from "@/composables/useA2UIBridge";
import type { AgentMessage } from "@/stores/agent.store";

const props = defineProps<{
  messages: AgentMessage[];
}>();

const processor = useMessageProcessor();

const a2uiMessages = computed(() => extractA2UIMessages(props.messages));

watchEffect(() => {
  processor.clearSurfaces();

  if (a2uiMessages.value.length > 0) {
    processor.processMessages(a2uiMessages.value);
  }
});

const surfaces = computed(() => {
  const version = processor.version.value;
  void version;
  return Array.from(processor.getSurfaces().entries());
});
</script>

<template>
  <div v-if="surfaces.length" class="a2ui-surface-host">
    <A2UISurface
      v-for="[surfaceId] in surfaces"
      :key="surfaceId"
      :surface-id="surfaceId"
    />
  </div>
</template>

<style scoped>
.a2ui-surface-host {
  padding: 16px;
  max-width: 100%;
  overflow-x: auto;
}
</style>
