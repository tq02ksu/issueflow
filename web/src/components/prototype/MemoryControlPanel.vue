<template>
  <n-card :bordered="false" class="settings-card">
    <template #header>Memory controls</template>
    <div class="settings-card__body">
      <article
        v-for="scope in scopes"
        :key="scope.scope"
        class="memory-panel__scope"
      >
        <div>
          <strong>{{ scope.scope }}</strong>
          <p>{{ scope.summary }}</p>
        </div>
        <span class="memory-panel__status">{{ scope.status }}</span>
      </article>
      <div class="memory-panel__actions">
        <n-button secondary @click="$emit('clearMemory')">Clear memory</n-button>
        <n-button type="primary" @click="$emit('rebuildMemory')">
          Rebuild memory
        </n-button>
      </div>
      <div class="memory-panel__feedback">Last action: {{ lastAction }}</div>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { NButton, NCard } from "naive-ui";
import type { PrototypeMemoryScope } from "@/mock/prototype.types";

defineProps<{
  scopes: PrototypeMemoryScope[];
  lastAction: "idle" | "cleared" | "rebuilt";
}>();

defineEmits<{
  clearMemory: [];
  rebuildMemory: [];
}>();
</script>

<style scoped>
.settings-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.9);
}

.settings-card__body {
  display: grid;
  gap: 12px;
}

.memory-panel__scope {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  padding: 12px 0;
  border-top: 1px solid rgba(216, 204, 184, 0.8);
}

.memory-panel__scope:first-child {
  padding-top: 0;
  border-top: 0;
}

.memory-panel__scope p {
  margin: 6px 0 0;
  color: var(--if-color-muted);
}

.memory-panel__status {
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  text-transform: uppercase;
}

.memory-panel__actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.memory-panel__feedback {
  color: var(--if-color-muted);
  font-size: 13px;
}
</style>
