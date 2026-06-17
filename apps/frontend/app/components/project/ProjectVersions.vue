<script setup lang="ts">
import { Download } from "@lucide/vue";
import { formatBytes, relativeTime } from "~/scripts/formatters";
import { channelLabel } from "~/scripts/pages/projects/detail/meta";
import type { Version } from "~/scripts/pages/projects/types";

defineProps<{
  versions: Version[];
  downloadUrl: (version: Version) => string;
}>();
</script>

<template>
  <div class="border-border/60 bg-card/30 rounded-xl border p-6">
    <ul v-if="versions.length" class="divide-border/60 divide-y">
      <li
        v-for="v in versions"
        :key="v.id"
        class="flex flex-wrap items-center justify-between gap-3 py-3 first:pt-0"
      >
        <div class="min-w-0">
          <div class="flex flex-wrap items-center gap-2">
            <span class="font-medium">{{ v.version_number }}</span>
            <span v-if="v.name" class="text-muted-foreground text-sm">
              {{ v.name }}
            </span>
            <span
              class="bg-muted text-muted-foreground rounded-full px-2 py-0.5 text-xs"
            >
              {{ channelLabel(v.channel) }}
            </span>
          </div>
          <div
            class="text-muted-foreground mt-1 flex flex-wrap items-center gap-3 text-xs"
          >
            <span>{{ relativeTime(v.created_at) }}</span>
            <span v-if="v.file">{{ formatBytes(v.file.size) }}</span>
            <span class="inline-flex items-center gap-1">
              <Download class="size-3" />
              {{ v.download_count.toLocaleString() }}
            </span>
          </div>
        </div>
        <Button
          v-if="v.file"
          as-child
          variant="outline"
          size="sm"
          class="gap-2"
        >
          <a :href="downloadUrl(v)">
            <Download class="size-4" />
            Download
          </a>
        </Button>
      </li>
    </ul>
    <p v-else class="text-muted-foreground">No versions published yet.</p>
  </div>
</template>
