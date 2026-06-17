<script setup lang="ts">
import type { StatusBanner } from "~/scripts/pages/projects/settings/types";

defineProps<{
  banner: StatusBanner;
  reviewNotes: string | null;
}>();
</script>

<template>
  <div
    class="flex items-start gap-3 rounded-2xl border p-4"
    :class="banner.card"
  >
    <component
      :is="banner.icon"
      class="mt-0.5 size-5 shrink-0"
      :class="banner.iconTone"
    />
    <div class="min-w-0 flex-1">
      <div class="flex flex-wrap items-center gap-2">
        <span class="text-foreground text-sm font-semibold">Status</span>
        <span
          class="rounded-full px-2 py-0.5 text-[11px] font-semibold"
          :class="banner.pill"
        >
          {{ banner.label }}
        </span>
      </div>
      <p class="text-muted-foreground mt-1 text-sm leading-relaxed">
        {{ banner.description }}
      </p>
      <div
        v-if="banner.showNotes && reviewNotes"
        class="bg-background/60 text-foreground mt-3 rounded-lg border p-3 text-sm"
      >
        <p
          class="text-muted-foreground mb-1 text-xs font-semibold tracking-wide uppercase"
        >
          Moderator notes
        </p>
        {{ reviewNotes }}
      </div>
    </div>
  </div>
</template>
