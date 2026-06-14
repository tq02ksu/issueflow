<template>
  <main class="callback">
    <n-card
      class="callback__card"
      :bordered="false"
    >
      <n-result
        :status="status"
        :title="title"
        :description="description"
      >
        <template #footer>
          <n-button
            v-if="isSuccess"
            type="primary"
            @click="goToWorkbench"
          >
            Open workbench now
          </n-button>
          <n-button
            v-else
            tag="a"
            href="/"
            quaternary
          >
            Return home
          </n-button>
        </template>
      </n-result>
    </n-card>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { NButton, NCard, NResult } from "naive-ui";
import { useSessionStore } from "@/stores/session";

const route = useRoute();
const router = useRouter();
const sessionStore = useSessionStore();

const isSuccess = computed(() => route.query.result === "success");
const title = computed(() =>
  isSuccess.value ? "GitLab connected" : "OAuth verification failed",
);
const description = computed(() =>
  isSuccess.value
    ? "Opening the workbench with the validated gateway callback."
    : `The gateway rejected the callback${route.query.reason ? `: ${route.query.reason}` : "."}`,
);
const status = computed(() => (isSuccess.value ? "success" : "error"));

function goToWorkbench() {
  router.push("/workbench");
}

onMounted(() => {
  sessionStore.captureOAuthResult(
    isSuccess.value ? "success" : "error",
    typeof route.query.reason === "string" ? route.query.reason : "",
  );

  if (isSuccess.value) {
    window.setTimeout(goToWorkbench, 1000);
  }
});
</script>

<style scoped>
.callback {
  min-height: 100vh;
  display: grid;
  place-items: center;
  padding: 24px;
}

.callback__card {
  width: min(100%, 640px);
  border-radius: var(--if-radius-lg);
  box-shadow: var(--if-shadow-panel);
}
</style>
