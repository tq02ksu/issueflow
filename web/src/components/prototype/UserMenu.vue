<template>
  <div class="user-menu__trigger">
    <span class="user-menu__name">{{ userName }}</span>
    <span class="user-menu__meta">{{ roleName }}</span>
    <RouterLink class="user-menu__link" to="/settings">
      {{ t("shell.userProfile") }}
    </RouterLink>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from "vue-router";
import { useI18n } from "vue-i18n";

defineProps<{
  userName: string;
  roleName: string;
}>();

const { t } = useI18n();
</script>

<style scoped>
.user-menu__trigger {
  display: grid;
  gap: 2px;
  height: auto;
  padding: 10px 14px;
  border-radius: 999px;
  background: rgba(255, 250, 242, 0.9);
}

.user-menu__name {
  display: block;
  font-weight: 700;
}

.user-menu__meta {
  display: block;
  color: var(--if-color-muted);
  font-size: 12px;
}

.user-menu__link {
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  text-decoration: none;
}
</style>
