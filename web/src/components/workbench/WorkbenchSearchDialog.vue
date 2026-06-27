<template>
  <n-modal :show="visible" @update:show="emit('close')">
    <n-card style="width: 480px" title="Add workbench" :bordered="false">
      <n-input
        v-model:value="searchText"
        placeholder="Search GitLab projects..."
        clearable
        @update:value="onSearch"
      />
      <n-spin :show="loading" class="results">
        <n-list v-if="results.length > 0">
          <n-list-item
            v-for="item in results"
            :key="item.id"
            @click="selectProject(item)"
            style="cursor: pointer"
          >
            {{ item.path_with_namespace }}
          </n-list-item>
        </n-list>
        <n-empty
          v-else-if="searched"
          description="No projects found"
        />
      </n-spin>
      <template #footer>
        <n-button @click="emit('close')">Cancel</n-button>
      </template>
    </n-card>
  </n-modal>
</template>

<script setup lang="ts">
import { ref } from "vue";
import {
  NButton, NCard, NEmpty, NInput, NList, NListItem, NModal, NSpin,
} from "naive-ui";
import type { GitLabProject } from "@/stores/session";

const props = defineProps<{ visible: boolean }>();
const emit = defineEmits<{ close: []; select: [project: GitLabProject] }>();

const searchText = ref("");
const results = ref<GitLabProject[]>([]);
const loading = ref(false);
const searched = ref(false);

let debounceTimer: ReturnType<typeof setTimeout>;

function onSearch(value: string) {
  clearTimeout(debounceTimer);
  if (!value.trim()) {
    results.value = [];
    searched.value = false;
    return;
  }
  debounceTimer = setTimeout(async () => {
    loading.value = true;
    searched.value = true;
    try {
      const resp = await fetch(`/api/projects?search=${encodeURIComponent(value)}`);
      if (resp.ok) results.value = await resp.json();
    } finally {
      loading.value = false;
    }
  }, 300);
}

function selectProject(project: GitLabProject) {
  emit("select", project);
}
</script>

<style scoped>
.results { margin-top: 16px; }
</style>
