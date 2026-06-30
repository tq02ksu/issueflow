<template>
  <n-card
    title="Work Item State"
    size="small"
    :bordered="false"
    class="state-panel"
  >
    <template #header-extra>
      <n-button
        size="small"
        secondary
        :disabled="loading"
        @click="handleEvaluate"
      >
        Evaluate State
      </n-button>
    </template>

    <n-spin :show="loading">
      <template v-if="state">
        <div class="state-panel__row">
          <span class="state-panel__label">Current</span>
          <n-tag size="small">{{ state.currentState }}</n-tag>
        </div>
        <div class="state-panel__row">
          <span class="state-panel__label">Next</span>
          <n-tag size="small" type="info">{{ state.proposedNextState }}</n-tag>
        </div>
        <p class="state-panel__summary">{{ state.summary }}</p>

        <n-alert
          v-if="state.missingContext.length > 0"
          type="warning"
          :show-icon="false"
          class="state-panel__block"
        >
          <strong>Missing context</strong>
          <ul class="state-panel__list">
            <li v-for="item in state.missingContext" :key="item">{{ item }}</li>
          </ul>
        </n-alert>

        <n-thing
          v-if="
            roleNotes.product.length > 0 ||
            roleNotes.engineering.length > 0 ||
            roleNotes.delivery.length > 0
          "
          class="state-panel__block"
        >
          <template #header>Role Notes</template>

          <div v-if="roleNotes.product.length > 0" class="state-panel__role">
            <strong>Product</strong>
            <ul class="state-panel__list">
              <li v-for="item in roleNotes.product" :key="`product-${item}`">
                {{ item }}
              </li>
            </ul>
          </div>
          <div
            v-if="roleNotes.engineering.length > 0"
            class="state-panel__role"
          >
            <strong>Engineering</strong>
            <ul class="state-panel__list">
              <li
                v-for="item in roleNotes.engineering"
                :key="`engineering-${item}`"
              >
                {{ item }}
              </li>
            </ul>
          </div>
          <div v-if="roleNotes.delivery.length > 0" class="state-panel__role">
            <strong>Delivery</strong>
            <ul class="state-panel__list">
              <li v-for="item in roleNotes.delivery" :key="`delivery-${item}`">
                {{ item }}
              </li>
            </ul>
          </div>
        </n-thing>
      </template>

      <n-empty
        v-else
        description="No shared issue state yet. Run an evaluation to capture it."
      />
    </n-spin>
  </n-card>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { NAlert, NButton, NCard, NEmpty, NSpin, NTag, NThing } from "naive-ui";
import {
  evaluateIssueState,
  getIssueState,
  type EngineeringMemoryRecord,
  type IssueStateDetail,
  type IssueStateEvaluation,
  type IssueStateEvaluationResult,
} from "@/api/workbench.api";

const props = defineProps<{
  workbenchId: number;
  issueIid: number;
}>();

interface NormalizedIssueState {
  currentState: string;
  proposedNextState: string;
  summary: string;
  missingContext: string[];
  blockers: string[];
  roleNotes: {
    product: string[];
    engineering: string[];
    delivery: string[];
  };
}

const loading = ref(false);
const detail = ref<IssueStateDetail | null>(null);

const state = computed(() => {
  const parsed = parseStateEvaluation(detail.value?.projectMemory ?? null);
  if (parsed) {
    return parsed;
  }

  const payload = detail.value?.pendingAction?.payload;
  if (!payload) {
    return null;
  }

  try {
    const pending = JSON.parse(payload) as {
      currentState?: string;
      current_state?: string;
      proposedNextState?: string;
      proposed_next_state?: string;
      transitionSummary?: string;
      transition_summary?: string;
    };

    return {
      currentState: pending.currentState ?? pending.current_state ?? "unknown",
      proposedNextState:
        pending.proposedNextState ?? pending.proposed_next_state ?? "unknown",
      summary:
        pending.transitionSummary ??
        pending.transition_summary ??
        "State transition pending confirmation.",
      missingContext: [],
      blockers: [],
      roleNotes: {
        product: [],
        engineering: [],
        delivery: [],
      },
    } satisfies NormalizedIssueState;
  } catch {
    return null;
  }
});

const roleNotes = computed(
  () =>
    state.value?.roleNotes ?? {
      product: [],
      engineering: [],
      delivery: [],
    },
);

watch(
  () => [props.workbenchId, props.issueIid],
  async () => {
    await loadState();
  },
  { immediate: true },
);

async function loadState() {
  loading.value = true;
  detail.value = await getIssueState(props.workbenchId, props.issueIid);
  loading.value = false;
}

async function handleEvaluate() {
  loading.value = true;
  const result = await evaluateIssueState(props.workbenchId, props.issueIid);
  if (result) {
    detail.value = mapEvaluationResultToDetail(result);
  }
  loading.value = false;
}

function mapEvaluationResultToDetail(
  result: IssueStateEvaluationResult,
): IssueStateDetail {
  return {
    projectMemory: result.projectState,
    personalNote: null,
    pendingAction: result.pendingAction,
  };
}

function parseStateEvaluation(
  memory: EngineeringMemoryRecord | null,
): NormalizedIssueState | null {
  if (!memory?.evaluation_summary) {
    return null;
  }

  try {
    const parsed = JSON.parse(
      memory.evaluation_summary,
    ) as IssueStateEvaluation;
    const roleNotes = parsed.roleNotes ?? parsed.role_notes ?? {};

    return {
      currentState: parsed.currentState ?? parsed.current_state ?? "unknown",
      proposedNextState:
        parsed.proposedNextState ?? parsed.proposed_next_state ?? "unknown",
      summary: parsed.summary ?? "No summary provided.",
      missingContext: parsed.missingContext ?? parsed.missing_context ?? [],
      blockers: parsed.blockers ?? [],
      roleNotes: {
        product: roleNotes.product ?? [],
        engineering: roleNotes.engineering ?? [],
        delivery: roleNotes.delivery ?? [],
      },
    };
  } catch {
    return null;
  }
}
</script>

<style scoped>
.state-panel {
  margin-top: 16px;
  background: var(--n-color-embedded);
}

.state-panel__row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 8px;
}

.state-panel__label {
  min-width: 52px;
  color: var(--if-color-muted);
  font-size: 12px;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}

.state-panel__summary {
  margin: 12px 0 0;
  line-height: 1.5;
}

.state-panel__block {
  margin-top: 12px;
}

.state-panel__role + .state-panel__role {
  margin-top: 10px;
}

.state-panel__list {
  margin: 8px 0 0;
  padding-left: 18px;
}
</style>
