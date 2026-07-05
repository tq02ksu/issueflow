<template>
  <span class="workflow-badge" :class="badgeClass">{{ state }}</span>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  state: string;
}>();

const badgeClass = computed(() => {
  if (["blocked", "changes_requested"].includes(props.state)) {
    return "workflow-badge--danger";
  }

  if (
    ["ready_for_execution", "ready_to_merge", "in_review"].includes(props.state)
  ) {
    return "workflow-badge--accent";
  }

  if (["done", "merged"].includes(props.state)) {
    return "workflow-badge--success";
  }

  return "workflow-badge--muted";
});
</script>

<style scoped>
.workflow-badge {
  display: inline-flex;
  align-items: center;
  padding: 6px 10px;
  border-radius: 999px;
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.02em;
}

.workflow-badge--accent {
  background: rgba(21, 94, 117, 0.14);
  color: var(--if-color-accent-strong);
}

.workflow-badge--danger {
  background: rgba(180, 35, 24, 0.12);
  color: var(--if-color-danger);
}

.workflow-badge--success {
  background: rgba(15, 118, 110, 0.12);
  color: var(--if-color-accent);
}

.workflow-badge--muted {
  background: rgba(28, 34, 48, 0.08);
  color: var(--if-color-text);
}
</style>
