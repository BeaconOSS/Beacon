<script setup lang="ts">
import { Ban, CircleCheck, Loader2, MessageSquareWarning } from "@lucide/vue";
import { REVIEW_ACTION, type ReviewAction } from "~/scripts/constants";

defineProps<{
  notes: string;
  submitting: boolean;
}>();

defineEmits<{
  "update:notes": [value: string];
  review: [action: ReviewAction];
}>();
</script>

<template>
  <div
    class="border-primary/40 bg-primary/5 sticky bottom-4 rounded-2xl border p-5 backdrop-blur-sm"
  >
    <Textarea
      :model-value="notes"
      rows="3"
      placeholder="Notes for the creator (required to request changes or reject)…"
      class="mb-4"
      @update:model-value="$emit('update:notes', String($event))"
    />
    <div class="flex flex-col gap-2 sm:flex-row">
      <Button
        class="btn-glow gap-2"
        :disabled="submitting"
        @click="$emit('review', REVIEW_ACTION.APPROVE)"
      >
        <Loader2 v-if="submitting" class="size-4 animate-spin" />
        <CircleCheck v-else class="size-4" />
        Approve &amp; publish
      </Button>
      <Button
        variant="outline"
        class="gap-2"
        :disabled="submitting"
        @click="$emit('review', REVIEW_ACTION.REQUEST_CHANGES)"
      >
        <MessageSquareWarning class="size-4" />
        Request changes
      </Button>
      <Button
        variant="destructive"
        class="gap-2 sm:ml-auto"
        :disabled="submitting"
        @click="$emit('review', REVIEW_ACTION.REJECT)"
      >
        <Ban class="size-4" />
        Reject
      </Button>
    </div>
  </div>
</template>
