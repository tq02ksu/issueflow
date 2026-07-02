<template>
  <PrototypeWorkbenchOverview v-if="isMockMode" />
  <WorkbenchOverviewLegacy v-else />
</template>

<script setup lang="ts">
import { isMockMode } from "@/app-mode";
import WorkbenchOverviewLegacy from "@/components/default/WorkbenchOverviewLegacy.vue";
import PrototypeWorkbenchOverview from "@/views/prototype/PrototypeWorkbenchOverview.vue";
</script>
