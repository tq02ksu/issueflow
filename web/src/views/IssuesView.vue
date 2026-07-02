<template>
  <PrototypeIssuesView v-if="isMockMode" />
  <IssuesLegacyView v-else />
</template>

<script setup lang="ts">
import { isMockMode } from "@/app-mode";
import IssuesLegacyView from "@/components/default/IssuesLegacyView.vue";
import PrototypeIssuesView from "@/views/prototype/PrototypeIssuesView.vue";
</script>
