<template>
  <n-modal :show="visible" @update:show="emit('close')">
    <n-card style="width: 480px" :bordered="false">
      <template #header>
        {{ selectedProject ? "Name workbench" : "Add workbench" }}
      </template>

      <div v-if="!selectedProject">
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
              <div>
                <div>{{ item.path_with_namespace }}</div>
                <div style="font-size: 12px; color: var(--if-color-muted)">
                  {{ item.name }}
                </div>
              </div>
            </n-list-item>
          </n-list>
          <n-empty v-else-if="searched" description="No projects found" />
        </n-spin>
      </div>

      <div v-else>
        <n-input v-model:value="workbenchName" placeholder="Workbench name" />
        <div style="margin-top: 8px; font-size: 12px; color: var(--if-color-muted)">
          Repository: {{ selectedProject.path_with_namespace }}
        </div>
      </div>

      <template #footer>
        <n-button quaternary @click="onCancel">Cancel</n-button>
        <n-button v-if="selectedProject" type="primary" @click="onConfirm">Create</n-button>
      </template>
    </n-card>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import {
  NButton, NCard, NEmpty, NInput, NList, NListItem, NModal, NSpin,
} from "naive-ui";
import { search as searchProjects } from "@/api/projects.api";
import type { GitLabProject } from "@/api/projects.api";

const props = defineProps<{ visible: boolean }>();
const emit = defineEmits<{ close: []; select: [project: GitLabProject, name: string] }>();

const searchText = ref("");
const results = ref<GitLabProject[]>([]);
const loading = ref(false);
const searched = ref(false);
const selectedProject = ref<GitLabProject | null>(null);
const workbenchName = ref("");

watch(
  () => props.visible,
  (v) => {
    if (!v) {
      searchText.value = "";
      results.value = [];
      searched.value = false;
      selectedProject.value = null;
      workbenchName.value = "";
    }
  },
);

function defaultName(path: string): string {
  return path.split("/").pop() || path;
}

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
      results.value = await searchProjects(value);
    } finally {
      loading.value = false;
    }
  }, 300);
}

function selectProject(project: GitLabProject) {
  selectedProject.value = project;
  workbenchName.value = defaultName(project.path_with_namespace);
}

function onConfirm() {
  if (selectedProject.value) {
    emit("select", selectedProject.value, workbenchName.value);
  }
}

function onCancel() {
  if (selectedProject.value) {
    selectedProject.value = null;
    workbenchName.value = "";
  } else {
    emit("close");
  }
}
</script>

<style scoped>
.results {
  margin-top: 16px;
}
</style>
