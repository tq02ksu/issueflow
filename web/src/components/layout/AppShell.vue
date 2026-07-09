<template>
  <n-layout class="shell">
    <n-layout-header bordered class="shell__header">
      <div class="shell__brand">
        <span class="shell__brand-mark">IF</span>
        <div>
          <strong>issueflow</strong>
          <div class="shell__subtitle">{{ t("shell.subtitle") }}</div>
        </div>
      </div>

      <div class="shell__header-tools">
        <template v-if="prototypeMode">
          <LanguageSwitcher />
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
          <n-dropdown
            trigger="click"
            :options="roleDropdownOptions"
            @select="onRoleSelect"
          >
            <span class="shell__chip shell__chip--role" style="cursor: pointer">
              {{ activeRoleName }} · {{ t("shell.roleSwitch") }}
            </span>
          </n-dropdown>
          <n-dropdown trigger="click" :options="profileDropdownOptions">
            <span class="shell__user-name" style="cursor: pointer">
              {{ prototypeStore.currentUserSoul.name }}
            </span>
          </n-dropdown>
        </template>
        <template v-else>
          <LanguageSwitcher />
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
              <span class="shell__section-label">{{ t("shell.role") }}</span>
              <strong>{{ activeRoleName }}</strong>
              <p>{{ activeRoleMission }}</p>
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
        <n-card
          style="width: 360px"
          :title="t('shell.renameWorkbench')"
          :bordered="false"
        >
          <n-input
            v-model:value="renameValue"
            :placeholder="t('shell.workbenchName')"
          />
          <template #footer>
            <n-button quaternary @click="showRenameDialog = false">
              {{ t("common.actions.cancel") }}
            </n-button>
            <n-button type="primary" @click="onRenameConfirm">
              {{ t("common.actions.save") }}
            </n-button>
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
import { useI18n } from "vue-i18n";
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
  NDropdown,
} from "naive-ui";
import { useSessionStore } from "@/stores/session.store";
import { usePrototypeStore } from "@/stores/prototype.store";
import { update as updateWorkbench } from "@/api/workbench.api";
import type { GitLabProject } from "@/api/projects.api";
import WorkbenchSidebarSelector from "./WorkbenchSidebarSelector.vue";
import WorkbenchSearchDialog from "@/components/workbench/WorkbenchSearchDialog.vue";
import LanguageSwitcher from "@/components/i18n/LanguageSwitcher.vue";
import type { MenuOption } from "naive-ui";
import type { PrototypeRoleKey } from "@/mock/prototype.types";

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
const { t } = useI18n();
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
    const link = (to: string, label: string) => () =>
      h(RouterLink, { to }, { default: () => label });

    const items: MenuOption[] = [
      {
        key: "dashboard",
        label: link("/workbench", t("shell.navigation.dashboard")),
      },
      {
        key: "turns",
        label: link("/workbench/turns", t("shell.navigation.turns")),
      },
      {
        key: "agents",
        label: link("/workbench/agents", t("shell.navigation.agents")),
      },
      {
        key: "approvals",
        label: link("/workbench/approvals", t("shell.navigation.approvals")),
      },
      {
        key: "memory",
        label: link("/workbench/memory", t("shell.navigation.memory")),
      },
      { key: "skills", label: link("/skills", t("shell.navigation.skills")) },
      {
        key: "fact-modules",
        label: t("shell.navigation.factModules"),
        children: [
          {
            key: "fact-modules-issues",
            label: link("/workbench/issues", t("shell.navigation.issues")),
          },
          {
            key: "fact-modules-mrs",
            label: link("/workbench/mrs", t("shell.navigation.mrs")),
          },
          {
            key: "fact-modules-milestones",
            label: link(
              "/workbench/milestones",
              t("shell.navigation.milestones"),
            ),
          },
        ],
      },
      { key: "divider-1", type: "divider" as const },
      {
        key: "settings",
        label: t("shell.navigation.settings"),
        children: [
          {
            key: "settings-loop",
            label: link("/settings/loop", t("shell.navigation.settingsLoop")),
          },
          {
            key: "settings-integrations",
            label: link(
              "/settings/integrations",
              t("shell.navigation.settingsIntegrations"),
            ),
          },
          {
            key: "settings-access",
            label: link(
              "/settings/access",
              t("shell.navigation.settingsAccess"),
            ),
          },
        ],
      },
      {
        key: "system",
        label: t("shell.navigation.system"),
        children: [
          {
            key: "system-gateway",
            label: link("/system/gateway", t("shell.navigation.gateway")),
          },
          {
            key: "system-governance",
            label: link("/system/governance", t("shell.navigation.governance")),
          },
        ],
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
        h(
          RouterLink,
          { to: "/workbench" },
          { default: () => t("shell.navigation.overview") },
        ),
    });
  }
  if (features.includes("issues")) {
    items.push({
      key: "issues",
      label: () =>
        h(
          RouterLink,
          { to: "/workbench/issues" },
          { default: () => t("shell.navigation.issues") },
        ),
    });
  }
  if (features.includes("pending_actions")) {
    items.push({
      key: "pending-actions",
      label: () =>
        h(
          RouterLink,
          { to: "/workbench/pending-actions" },
          { default: () => t("shell.navigation.pendingActions") },
        ),
    });
  }
  if (features.includes("releases")) {
    items.push({ key: "releases", label: t("shell.navigation.releases") });
  }
  return items;
});

const activeRoleName = computed(() => {
  const key = prototypeStore.activeRoleView?.key;
  return key ? t(`prototype.roles.${key}.name`) : "";
});

const activeRoleMission = computed(() => {
  const key = prototypeStore.activeRoleView?.key;
  return key ? t(`prototype.roles.${key}.mission`) : "";
});

const roleDropdownOptions = computed(() =>
  prototypeStore.roleViews.map((role) => ({
    key: role.key,
    label: t(`prototype.roles.${role.key}.name`),
  })),
);

function onRoleSelect(key: string) {
  prototypeStore.setActiveRole(key as PrototypeRoleKey);
}

const profileDropdownOptions = computed(() => {
  const soul = prototypeStore.currentUserSoul;
  return [
    {
      key: "header",
      type: "render" as const,
      render: () =>
        h(
          "div",
          { style: "padding: 8px 12px; font-weight: 700; font-size: 14px" },
          soul.name,
        ),
    },
    {
      key: "preferences",
      label: () =>
        h(
          RouterLink,
          { to: "/settings/preferences" },
          { default: () => t("shell.preferences") },
        ),
    },
    { key: "divider", type: "divider" as const },
    {
      key: "signout",
      label: t("shell.signOut"),
    },
  ];
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
  overflow: hidden;
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
  gap: 12px;
  flex-shrink: 0;
}

.shell__select {
  width: 180px;
  padding: 8px 12px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.8);
  color: var(--if-color-text);
  font: inherit;
  font-size: 13px;
}

.shell__chip {
  display: inline-flex;
  align-items: center;
  padding: 6px 10px;
  border-radius: 999px;
  background: rgba(15, 118, 110, 0.1);
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  white-space: nowrap;
}

.shell__user-name {
  font-weight: 700;
  font-size: 13px;
  white-space: nowrap;
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
