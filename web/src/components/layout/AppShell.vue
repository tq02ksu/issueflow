<template>
  <n-layout class="shell">
    <n-layout-header
      bordered
      class="shell__header"
    >
      <div class="shell__brand">
        <span class="shell__brand-mark">IF</span>
        <div>
          <strong>issueflow</strong>
          <div class="shell__subtitle">
            Agent Workbench
          </div>
        </div>
      </div>
    </n-layout-header>
    <n-layout
      has-sider
      position="absolute"
      style="top: 72px; bottom: 0"
    >
      <n-layout-sider
        bordered
        collapse-mode="width"
        :collapsed-width="72"
        :width="220"
      >
        <n-menu
          :options="menuOptions"
          :value="activeKey"
        />
      </n-layout-sider>
      <n-layout-content content-style="padding: 28px;">
        <slot />
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<script setup lang="ts">
import { h } from "vue";
import { RouterLink } from "vue-router";
import { NLayout, NLayoutContent, NLayoutHeader, NLayoutSider, NMenu } from "naive-ui";

defineProps<{
  activeKey: string;
}>();

const menuOptions = [
  {
    key: "overview",
    label: () =>
      h(
        RouterLink,
        {
          to: "/workbench",
        },
        { default: () => "Overview" },
      ),
  },
];
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
</style>
