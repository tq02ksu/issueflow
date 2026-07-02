<template>
  <n-card :bordered="false" class="settings-card">
    <template #header>{{ t("prototype.settings.memoryControls") }}</template>
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
        <n-button secondary @click="$emit('clearMemory')">
          {{ t("prototype.settings.clearMemory") }}
        </n-button>
        <n-button type="primary" @click="$emit('rebuildMemory')">
          {{ t("prototype.settings.rebuildMemory") }}
        </n-button>
      </div>
      <div class="memory-panel__feedback">
        {{ t("prototype.settings.lastAction") }}: {{ localizedLastAction }}
      </div>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { NButton, NCard } from "naive-ui";
import type { PrototypeMemoryScope } from "@/mock/prototype.types";
import { useI18n } from "vue-i18n";

const props = defineProps<{
  scopes: PrototypeMemoryScope[];
  lastAction: "idle" | "cleared" | "rebuilt";
}>();

defineEmits<{
  clearMemory: [];
  rebuildMemory: [];
}>();

const { t } = useI18n();

const localizedLastAction = computed(() => {
  const suffix = capitalize(props.lastAction);
  return t(`prototype.settings.lastAction${suffix}`);
});

function capitalize(value: string): string {
  return value.slice(0, 1).toUpperCase() + value.slice(1);
}
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
