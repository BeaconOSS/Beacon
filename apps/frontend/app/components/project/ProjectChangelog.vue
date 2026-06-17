<script setup lang="ts">
import { relativeTime } from "~/scripts/formatters";
import { channelLabel } from "~/scripts/pages/projects/detail/meta";
import type { Version } from "~/scripts/pages/projects/types";

defineProps<{
  entries: Version[];
}>();
</script>

<template>
  <div class="border-border/60 bg-card/30 rounded-xl border p-6">
    <ol v-if="entries.length" class="space-y-6">
      <li
        v-for="v in entries"
        :key="v.id"
        class="border-border/40 border-l-2 pl-4"
      >
        <div class="flex flex-wrap items-center gap-2">
          <span class="font-semibold">{{ v.version_number }}</span>
          <span
            class="bg-primary/15 text-primary rounded-full px-2 py-0.5 text-xs"
          >
            {{ channelLabel(v.channel) }}
          </span>
          <span class="text-muted-foreground text-xs">
            {{ relativeTime(v.created_at) }}
          </span>
        </div>
        <p class="text-foreground/85 mt-2 break-words whitespace-pre-line">
          {{ v.changelog }}
        </p>
      </li>
    </ol>
    <p v-else class="text-muted-foreground">No changelog entries yet.</p>
  </div>
</template>
