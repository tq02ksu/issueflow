<template>
  <n-dropdown trigger="click" :options="dropdownOptions" @select="handleSelect">
    <n-button quaternary>
      <span class="label">{{ currentLabel }}</span>
    </n-button>
  </n-dropdown>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { NButton, NDropdown } from "naive-ui";
import { useSessionStore } from "@/stores/session";

const emit = defineEmits<{
  select: [id: number];
  add: [];
}>();

const store = useSessionStore();

const currentLabel = computed(() => {
  const wb = store.workbenches.find(
    (w) => w.id === store.currentWorkbenchId.value,
  );
  return wb ? wb.project_path : "Select workbench...";
});

const dropdownOptions = computed(() => {
  const items: any[] = store.workbenches.map((wb) => ({
    label: wb.project_path,
    key: wb.id,
  }));
  if (items.length > 0) items.push({ type: "divider", key: "divider" });
  items.push({ label: "+ Add workbench...", key: "add" });
  return items;
});

function handleSelect(key: string | number) {
  if (key === "add") emit("add");
  else emit("select", key as number);
}
</script>

<style scoped>
.label {
  font-size: 14px;
  font-weight: 600;
}
</style>
