<template>
  <n-dropdown trigger="click" :options="dropdownOptions" @select="handleSelect">
    <n-button quaternary block>
      <div class="selector-text">
        <span class="selector-name">{{ currentName }}</span>
        <span class="selector-path">{{ currentPath }}</span>
      </div>
    </n-button>
  </n-dropdown>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { NButton, NDropdown } from "naive-ui";
import { useSessionStore } from "@/stores/session.store";
import type { DropdownMixedOption } from "naive-ui";

const emit = defineEmits<{
  select: [id: number];
  add: [];
  rename: [];
  rebind: [];
}>();

const store = useSessionStore();

const currentWb = computed(() =>
  store.workbenches.find((w) => w.id === store.currentWorkbenchId.value),
);

const currentName = computed(() =>
  currentWb.value
    ? currentWb.value.name || currentWb.value.project_path
    : "Select...",
);

const currentPath = computed(() =>
  currentWb.value ? currentWb.value.project_path : "",
);

const dropdownOptions = computed(() => {
  const items: DropdownMixedOption[] = store.workbenches.map((wb) => ({
    label: wb.name || wb.project_path,
    key: wb.id,
  }));
  if (items.length > 0) {
    items.push({ type: "divider", key: "div1" });
  }
  if (currentWb.value) {
    items.push({ label: "Rename workbench", key: "rename" });
    items.push({ label: "Rebind GitLab project...", key: "rebind" });
    items.push({ type: "divider", key: "div2" });
  }
  items.push({ label: "+ Add workbench...", key: "add" });
  return items;
});

function handleSelect(key: string | number) {
  if (key === "add") emit("add");
  else if (key === "rename") emit("rename");
  else if (key === "rebind") emit("rebind");
  else emit("select", key as number);
}
</script>

<style scoped>
.selector-text {
  text-align: left;
  line-height: 1.3;
}

.selector-name {
  font-size: 14px;
  font-weight: 600;
  display: block;
}

.selector-path {
  font-size: 11px;
  color: var(--if-color-muted);
  display: block;
  font-weight: 400;
}
</style>
