<template>
  <div class="role-actions">
    <n-button
      v-for="role in store.roleViews"
      :key="role.key"
      :data-role-key="role.key"
      type="primary"
      size="large"
      round
      @click="enter(role.key)"
    >
      {{ t(`prototype.roles.${role.key}.name`) }}
    </n-button>
  </div>
</template>

<script setup lang="ts">
import { NButton } from "naive-ui";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { usePrototypeStore } from "@/stores/prototype.store";
import type { PrototypeRoleKey } from "@/mock/prototype.types";

const store = usePrototypeStore();
const router = useRouter();
const { t } = useI18n();

function enter(key: PrototypeRoleKey) {
  store.setActiveRole(key);
  router.push("/workbench");
}
</script>

<style scoped>
.role-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}
</style>
