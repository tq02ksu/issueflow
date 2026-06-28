<script setup lang="ts">
import type { AgentSession } from "@/stores/agent.store";
import { NButton, NScrollbar, NText, NSpace } from "naive-ui";

defineProps<{
  sessions: AgentSession[];
  activeId: string;
}>();

const emit = defineEmits<{
  select: [id: string];
  create: [];
  delete: [id: string];
}>();
</script>

<template>
  <div
    style="padding: 12px; display: flex; flex-direction: column; height: 100%"
  >
    <NButton
      block
      type="primary"
      style="margin-bottom: 12px"
      @click="emit('create')"
    >
      + New Session
    </NButton>
    <NScrollbar style="flex: 1">
      <div
        v-for="s in sessions"
        :key="s.id"
        style="
          padding: 8px;
          margin-bottom: 4px;
          border-radius: 6px;
          cursor: pointer;
        "
        :style="{
          background:
            s.id === activeId ? 'var(--n-color-embedded)' : 'transparent',
        }"
        @click="emit('select', s.id)"
      >
        <NSpace justify="space-between" align="center">
          <NText style="font-size: 13px">
            {{ s.title || "New Session" }}
          </NText>
          <NButton
            text
            size="tiny"
            type="error"
            @click.stop="emit('delete', s.id)"
          >
            ×
          </NButton>
        </NSpace>
      </div>
    </NScrollbar>
  </div>
</template>
