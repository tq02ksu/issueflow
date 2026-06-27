<template>
  <n-dropdown trigger="click" :options="dropdownOptions" @select="handleSelect">
    <n-button quaternary block>
      <span class="selector-label">{{ currentName }}</span>
      <span class="selector-path">{{ currentPath }}</span>
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

const currentWb = computed(() =>
  store.workbenches.find((w) => w.id === store.currentWorkbenchId.value),
);

const currentName = computed(() =>
  currentWb.value ? (currentWb.value.name || currentWb.value.project_path) : "Select...",
);

const currentPath = computed(() =>
  currentWb.value ? currentWb.value.project_path : "",
);

const dropdownOptions = computed(() => {
  const items: any[] = store.workbenches.map((wb) => ({
    label: wb.name || wb.project_path,
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
.selector-label {
  font-size: 14px;
  font-weight: 600;
  display: block;
}

.selector-path {
  font-size: 11px;
  color: var(--if-color-muted);
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
