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

      <div class="shell__header-tools">
        <LanguageSwitcher />

        <template v-if="prototypeMode">
          <div class="shell__workbench">
            <span class="shell__section-label">Workbench</span>
            <select
              class="shell__select"
              :value="prototypeStore.currentWorkbenchId"
              @change="onPrototypeWorkbenchSelect($event)"
            >
              <option
                v-for="workbench in prototypeStore.prototypeWorkbenchesList"
                :key="workbench.id"
                :value="workbench.id"
              >
                {{ workbench.name }}
              </option>
            </select>
          </div>

          <div class="shell__status-strip">
            <span class="shell__chip">{{ prototypeStore.currentWorkbench?.role.name }}</span>
            <span class="shell__chip shell__chip--subtle">
              {{ prototypeStore.currentWorkbench?.activeSkillVersionId }}
            </span>
          </div>
          <n-button quaternary tag="a" href="/settings">Settings</n-button>
          <UserMenu
            :user-name="prototypeStore.currentUserSoul.name"
            :role-name="prototypeStore.currentWorkbench?.role.name ?? 'Role'"
          />
        </template>
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
          <template v-if="prototypeMode">
            <div class="prototype-sider__summary">
              <span class="shell__section-label">Role</span>
              <strong>{{ prototypeStore.currentWorkbench?.role.name }}</strong>
              <p>{{ prototypeStore.currentWorkbench?.role.personaSummary }}</p>
            </div>
            <n-divider style="margin: 12px 0" />
          </template>
          <template v-else>
            <WorkbenchSidebarSelector
              @select="onSelect"
              @add="showAddDialog = true"
              @rename="showRenameDialog = true"
              @rebind="onStartRebind"
            />
            <n-divider style="margin: 8px 0" />
          </template>
          <n-menu :options="menuOptions" :value="activeKey" />
        </div>
      </n-layout-sider>
      <n-layout-content class="shell__content" content-style="padding: 28px;">
        <slot />
      </n-layout-content>
    </n-layout>

    <template v-if="!prototypeMode">
      <WorkbenchSearchDialog
        :visible="showAddDialog"
        @close="showAddDialog = false"
        @select="onCreateWorkbench"
      />

      <n-modal :show="showRenameDialog" @update:show="showRenameDialog = false">
        <n-card style="width: 360px" title="Rename workbench" :bordered="false">
          <n-input v-model:value="renameValue" placeholder="Workbench name" />
          <template #footer>
            <n-button quaternary @click="showRenameDialog = false">
              Cancel
            </n-button>
            <n-button type="primary" @click="onRenameConfirm"> Save </n-button>
          </template>
        </n-card>
      </n-modal>

      <WorkbenchSearchDialog
        :visible="showRebindDialog"
        @close="showRebindDialog = false"
        @select="onRebindProject"
      />
    </template>
  </n-layout>
</template>

<script setup lang="ts">
import { h, ref, computed, watch } from "vue";
import { RouterLink } from "vue-router";
import {
  NLayout,
  NLayoutContent,
  NLayoutHeader,
  NLayoutSider,
  NMenu,
  NDivider,
  NModal,
  NCard,
  NInput,
  NButton,
} from "naive-ui";
import { useSessionStore } from "@/stores/session.store";
import { usePrototypeStore } from "@/stores/prototype.store";
import { update as updateWorkbench } from "@/api/workbench.api";
import type { GitLabProject } from "@/api/projects.api";
import WorkbenchSidebarSelector from "./WorkbenchSidebarSelector.vue";
import WorkbenchSearchDialog from "@/components/workbench/WorkbenchSearchDialog.vue";
import LanguageSwitcher from "@/components/i18n/LanguageSwitcher.vue";
import UserMenu from "@/components/prototype/UserMenu.vue";
import type { MenuOption } from "naive-ui";

const props = withDefaults(
  defineProps<{
    activeKey: string;
    prototypeMode?: boolean;
  }>(),
  {
    prototypeMode: false,
  },
);

const store = useSessionStore();
const prototypeStore = usePrototypeStore();
const showAddDialog = ref(false);
const showRenameDialog = ref(false);
const renameValue = ref("");
const showRebindDialog = ref(false);

const currentWb = computed(
  () =>
    store.workbenches.find((w) => w.id === store.currentWorkbenchId.value) ??
    null,
);

watch(showRenameDialog, (v) => {
  if (v && currentWb.value) {
    renameValue.value = currentWb.value.name;
  }
});

const menuOptions = computed(() => {
  if (props.prototypeMode) {
    const items: MenuOption[] = [
      {
        key: "overview",
        label: () =>
          h(RouterLink, { to: "/workbench" }, { default: () => "Overview" }),
      },
      {
        key: "issues",
        label: () =>
          h(RouterLink, { to: "/workbench/issues" }, { default: () => "Issues" }),
      },
      {
        key: "mrs",
        label: () =>
          h(RouterLink, { to: "/workbench/mrs" }, { default: () => "MRs" }),
      },
      {
        key: "milestones",
        label: () =>
          h(
            RouterLink,
            { to: "/workbench/milestones" },
            { default: () => "Milestones" },
          ),
      },
    ];

    return items;
  }

  const features = store.capabilities.features;
  const items: MenuOption[] = [];
  if (features.includes("overview")) {
    items.push({
      key: "overview",
      label: () =>
        h(RouterLink, { to: "/workbench" }, { default: () => "Overview" }),
    });
  }
  if (features.includes("issues")) {
    items.push({
      key: "issues",
      label: () =>
        h(RouterLink, { to: "/workbench/issues" }, { default: () => "Issues" }),
    });
  }
  if (features.includes("pending_actions")) {
    items.push({
      key: "pending-actions",
      label: () =>
        h(
          RouterLink,
          { to: "/workbench/pending-actions" },
          { default: () => "Pending Actions" },
        ),
    });
  }
  if (features.includes("releases")) {
    items.push({ key: "releases", label: "Releases" });
  }
  return items;
});

function onSelect(id: number) {
  store.setCurrentWorkbench(id);
}

function onPrototypeWorkbenchSelect(event: Event) {
  const target = event.target;
  if (!(target instanceof HTMLSelectElement)) {
    return;
  }

  prototypeStore.selectWorkbench(target.value);
}

async function onCreateWorkbench(project: GitLabProject, name: string) {
  const wb = await store.addWorkbench({
    project_id: project.id,
    project_path: project.path_with_namespace,
    name,
  });
  if (wb) {
    store.setCurrentWorkbench(wb.id);
    showAddDialog.value = false;
  }
}

async function onRenameConfirm() {
  const wb = currentWb.value;
  if (!wb || !renameValue.value.trim()) return;

  const updated = await updateWorkbench(wb.id, {
    project_id: wb.project_id,
    project_path: wb.project_path,
    name: renameValue.value.trim(),
  });
  if (updated) {
    store.setWorkbenches(
      store.workbenches.map((w) => (w.id === updated.id ? updated : w)),
    );
    showRenameDialog.value = false;
  }
}

function onStartRebind() {
  showRebindDialog.value = true;
}

async function onRebindProject(project: GitLabProject) {
  const wb = currentWb.value;
  if (!wb) return;

  const updated = await updateWorkbench(wb.id, {
    project_id: project.id,
    project_path: project.path_with_namespace,
    name: wb.name,
  });
  if (updated) {
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
  justify-content: space-between;
  padding: 0 24px;
  background: rgba(251, 248, 243, 0.9);
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

.shell__content {
  min-height: 0;
  overflow: hidden;
}

.shell__header-tools {
  display: flex;
  align-items: center;
  gap: 14px;
}

.shell__workbench {
  width: 240px;
}

.shell__select {
  width: 100%;
  padding: 9px 12px;
  border: 1px solid rgba(216, 204, 184, 0.95);
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.88);
  color: var(--if-color-text);
  font: inherit;
}

.shell__status-strip {
  display: flex;
  gap: 8px;
}

.shell__section-label {
  display: block;
  margin-bottom: 6px;
  color: var(--if-color-muted);
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.shell__chip {
  display: inline-flex;
  align-items: center;
  padding: 8px 12px;
  border-radius: 999px;
  background: rgba(15, 118, 110, 0.12);
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
}

.shell__chip--subtle {
  background: rgba(17, 24, 39, 0.06);
  color: var(--if-color-text);
}

.sider-inner {
  padding: 12px;
}

.prototype-sider__summary p {
  margin: 8px 0 0;
  color: var(--if-color-muted);
  line-height: 1.5;
}

@media (max-width: 960px) {
  .shell__header {
    height: auto;
    align-items: flex-start;
    gap: 12px;
    padding: 16px;
  }

  .shell__header-tools {
    width: 100%;
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  .shell__workbench {
    width: min(100%, 220px);
  }
}
</style>
