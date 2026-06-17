<script setup lang="ts">
import { CircleX } from "@lucide/vue";
import type { LightboxState } from "~/scripts/pages/moderation/types";

defineProps<{
  state: LightboxState;
}>();

defineEmits<{
  close: [];
}>();
</script>

<template>
  <Teleport to="body">
    <div
      class="fixed inset-0 z-50 flex flex-col items-center justify-center gap-3 bg-black/80 p-6 backdrop-blur-sm"
      @click="$emit('close')"
    >
      <button
        type="button"
        class="absolute top-4 right-4 rounded-full bg-white/10 p-2 text-white transition-colors hover:bg-white/20"
        aria-label="Close"
        @click.stop="$emit('close')"
      >
        <CircleX class="size-5" />
      </button>
      <img
        :src="state.url"
        :alt="state.caption || 'Preview'"
        class="max-h-[80vh] max-w-[90vw] rounded-lg object-contain shadow-2xl"
        :class="state.pixelated ? '[image-rendering:pixelated]' : ''"
        @click.stop
      />
      <p
        v-if="state.caption"
        class="max-w-[90vw] text-center text-sm text-white/80"
        @click.stop
      >
        {{ state.caption }}
      </p>
    </div>
  </Teleport>
</template>
