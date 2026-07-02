<template>
  <n-card :bordered="false" class="settings-card">
    <template #header>Current Workbench</template>
    <div class="settings-card__body">
      <n-input v-model:value="draftName" placeholder="Role name" />
      <n-input
        v-model:value="draftPersona"
        type="textarea"
        :autosize="{ minRows: 3, maxRows: 5 }"
        placeholder="Role summary"
      />
      <n-input
        v-model:value="draftWays"
        type="textarea"
        :autosize="{ minRows: 3, maxRows: 6 }"
        placeholder="One way of working per line"
      />
      <n-input
        v-model:value="draftGoals"
        type="textarea"
        :autosize="{ minRows: 3, maxRows: 6 }"
        placeholder="One goal per line"
      />
      <n-button type="primary" @click="emitRoleUpdate">Save role</n-button>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { NButton, NCard, NInput } from "naive-ui";
import type { PrototypeRole } from "@/mock/prototype.types";

const props = defineProps<{
  role: PrototypeRole | null;
}>();

const emit = defineEmits<{
  updateRole: [role: PrototypeRole];
}>();

const draftName = ref("");
const draftPersona = ref("");
const draftWays = ref("");
const draftGoals = ref("");

watch(
  () => props.role,
  (role) => {
    draftName.value = role?.name ?? "";
    draftPersona.value = role?.personaSummary ?? "";
    draftWays.value = role?.waysOfWorking.join("\n") ?? "";
    draftGoals.value = role?.goals.join("\n") ?? "";
  },
  { immediate: true },
);

function emitRoleUpdate() {
  emit("updateRole", {
    name: draftName.value.trim(),
    personaSummary: draftPersona.value.trim(),
    waysOfWorking: normalizeTextarea(draftWays.value),
    goals: normalizeTextarea(draftGoals.value),
  });
}

function normalizeTextarea(value: string): string[] {
  return value
    .split("\n")
    .map((item) => item.trim())
    .filter(Boolean);
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
</style>
