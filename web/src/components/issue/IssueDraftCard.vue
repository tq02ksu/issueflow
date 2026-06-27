<template>
  <n-card :bordered="true" size="small">
    <p><strong>project</strong> {{ projectId }}</p>
    <p><strong>title</strong> {{ title }}</p>
    <p v-if="description"><strong>description</strong> {{ description }}</p>
  </n-card>
</template>

<script setup lang="ts">
import { NCard } from "naive-ui";

defineProps<{
  projectId: number;
  title: string;
  description: string;
}>();
</script>
