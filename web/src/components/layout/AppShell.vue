<template>
  <n-layout class="shell">
    <n-layout-header bordered class="shell__header">
      <div class="shell__brand">
        <span class="shell__brand-mark">IF</span>
        <div>
          <strong>issueflow</strong>
          <div class="shell__subtitle">Agent Workbench</div>
        </div>
      </div>
    </n-layout-header>
    <n-layout has-sider position="absolute" style="top: 72px; bottom: 0">
      <n-layout-sider
        bordered
        collapse-mode="width"
        :collapsed-width="72"
        :width="220"
      >
        <div class="sider-inner">
          <WorkbenchSidebarSelector
            @select="onSelect"
            @add="showAddDialog = true"
            @rename="showRenameDialog = true"
            @rebind="onStartRebind"
          />
          <n-divider style="margin: 8px 0" />
          <n-menu :options="menuOptions" :value="activeKey" />
        </div>
      </n-layout-sider>
      <n-layout-content content-style="padding: 28px;">
        <slot />
      </n-layout-content>
    </n-layout>

    <WorkbenchSearchDialog
      :visible="showAddDialog"
      @close="showAddDialog = false"
      @select="onCreateWorkbench"
    />

    <n-modal :show="showRenameDialog" @update:show="showRenameDialog = false">
      <n-card style="width: 360px" title="Rename workbench" :bordered="false">
        <n-input v-model:value="renameValue" placeholder="Workbench name" />
        <template #footer>
          <n-button quaternary @click="showRenameDialog = false">Cancel</n-button>
          <n-button type="primary" @click="onRenameConfirm">Save</n-button>
        </template>
      </n-card>
    </n-modal>

    <WorkbenchSearchDialog
      :visible="showRebindDialog"
      @close="showRebindDialog = false"
      @select="onRebindProject"
    />
  </n-layout>
</template>

<script setup lang="ts">
import { h, ref, computed, watch } from "vue";
import { RouterLink } from "vue-router";
import {
  NLayout, NLayoutContent, NLayoutHeader, NLayoutSider, NMenu, NDivider,
  NModal, NCard, NInput, NButton,
} from "naive-ui";
import { useSessionStore } from "@/stores/session";
import type { GitLabProject, Workbench } from "@/stores/session";
import WorkbenchSidebarSelector from "./WorkbenchSidebarSelector.vue";
import WorkbenchSearchDialog from "@/components/workbench/WorkbenchSearchDialog.vue";

defineProps<{ activeKey: string }>();

const store = useSessionStore();
const showAddDialog = ref(false);
const showRenameDialog = ref(false);
const renameValue = ref("");
const showRebindDialog = ref(false);

const currentWb = computed(() =>
  store.workbenches.find((w) => w.id === store.currentWorkbenchId.value) ?? null,
);

watch(showRenameDialog, (v) => {
  if (v && currentWb.value) {
    renameValue.value = currentWb.value.name;
  }
});

const menuOptions = computed(() => {
  const features = store.capabilities.features;
  const items: any[] = [];
  if (features.includes("overview")) {
    items.push({
      key: "overview",
      label: () =>
        h(
          RouterLink,
          { to: "/workbench" },
          { default: () => "Overview" },
        ),
    });
  }
  if (features.includes("issues")) {
    items.push({
      key: "issues",
      label: "Issues",
    });
  }
  if (features.includes("agents")) {
    items.push({
      key: "agents",
      label: "Agents",
    });
  }
  if (features.includes("releases")) {
    items.push({
      key: "releases",
      label: "Releases",
    });
  }
  return items;
});

function onSelect(id: number) {
  store.setCurrentWorkbench(id);
}

async function onCreateWorkbench(project: GitLabProject, name: string) {
  const resp = await store.authFetch("/api/workbenches", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      project_id: project.id,
      project_path: project.path_with_namespace,
      name,
    }),
  });
  if (resp.ok) {
    const wb = await resp.json();
    store.setWorkbenches([...store.workbenches, wb]);
    store.setCurrentWorkbench(wb.id);
    showAddDialog.value = false;
  }
}

async function onRenameConfirm() {
  const wb = currentWb.value;
  if (!wb || !renameValue.value.trim()) return;

  const resp = await store.authFetch(`/api/workbenches/${wb.id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      project_id: wb.project_id,
      project_path: wb.project_path,
      name: renameValue.value.trim(),
    }),
  });
  if (resp.ok) {
    const updated: Workbench = await resp.json();
    store.setWorkbenches(
      store.workbenches.map((w) => (w.id === updated.id ? updated : w)),
    );
    showRenameDialog.value = false;
  }
}

function onStartRebind() {
  showRebindDialog.value = true;
}

async function onRebindProject(project: GitLabProject, _name: string) {
  const wb = currentWb.value;
  if (!wb) return;

  const resp = await store.authFetch(`/api/workbenches/${wb.id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      project_id: project.id,
      project_path: project.path_with_namespace,
      name: wb.name,
    }),
  });
  if (resp.ok) {
    const updated: Workbench = await resp.json();
    store.setWorkbenches(
      store.workbenches.map((w) => (w.id === updated.id ? updated : w)),
    );
    store.setCurrentWorkbench(updated.id);
    showRebindDialog.value = false;
  }
}
</script>

<style scoped>
.shell {
  min-height: 100vh;
}

.shell__header {
  height: 72px;
  display: flex;
  align-items: center;
  padding: 0 24px;
  background: rgba(255, 255, 255, 0.92);
  backdrop-filter: blur(12px);
}

.shell__brand {
  display: flex;
  align-items: center;
  gap: 12px;
}

.shell__brand-mark {
  width: 40px;
  height: 40px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 12px;
  background: var(--if-color-accent);
  color: #fff;
  font-weight: 700;
}

.shell__subtitle {
  color: var(--if-color-muted);
  font-size: 12px;
}

.sider-inner {
  padding: 12px;
}
</style>
