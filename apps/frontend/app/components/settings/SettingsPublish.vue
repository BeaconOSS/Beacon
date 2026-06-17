<script setup lang="ts">
import PublishStatusBanner from "~/components/settings/PublishStatusBanner.vue";
import PublishPendingChanges from "~/components/settings/PublishPendingChanges.vue";
import PublishChecklist from "~/components/settings/PublishChecklist.vue";
import type { ProjectStatus } from "~/scripts/constants";
import type {
  ChecklistItem,
  PendingChangeRow,
  StatusBanner,
} from "~/scripts/pages/projects/settings/types";

const changelog = defineModel<string>("changelog", { required: true });

defineProps<{
  banner: StatusBanner;
  reviewNotes: string | null;
  hasPendingChanges: boolean;
  pendingChanges: PendingChangeRow[];
  iconChanged: boolean;
  publishedIconUrl: string | null;
  iconUrl: string | null;
  canSubmit: boolean;
  requiredComplete: number;
  requiredTotal: number;
  outstandingItems: ChecklistItem[];
  completedItems: ChecklistItem[];
  status: ProjectStatus;
  changelogDirty: boolean;
  savingChangelog: boolean;
  changelogError: string;
  canSubmitNow: boolean;
  submitting: boolean;
  submitLabel: string;
}>();

defineEmits<{
  saveChangelog: [];
  submit: [];
  withdraw: [];
}>();
</script>

<template>
  <section class="space-y-8">
    <PublishStatusBanner :banner="banner" :review-notes="reviewNotes" />

    <PublishPendingChanges
      v-if="hasPendingChanges"
      :pending-changes="pendingChanges"
      :icon-changed="iconChanged"
      :published-icon-url="publishedIconUrl"
      :icon-url="iconUrl"
    />

    <PublishChecklist
      v-model:changelog="changelog"
      :can-submit="canSubmit"
      :required-complete="requiredComplete"
      :required-total="requiredTotal"
      :outstanding-items="outstandingItems"
      :completed-items="completedItems"
      :status="status"
      :changelog-dirty="changelogDirty"
      :saving-changelog="savingChangelog"
      :changelog-error="changelogError"
      :can-submit-now="canSubmitNow"
      :submitting="submitting"
      :submit-label="submitLabel"
      :has-pending-changes="hasPendingChanges"
      @save-changelog="$emit('saveChangelog')"
      @submit="$emit('submit')"
      @withdraw="$emit('withdraw')"
    />
  </section>
</template>
