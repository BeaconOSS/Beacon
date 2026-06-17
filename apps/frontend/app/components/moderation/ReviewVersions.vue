<script setup lang="ts">
import { Download, FileArchive, Layers, Loader2 } from "@lucide/vue";
import { formatBytes, relativeTime } from "~/scripts/formatters";
import { channelMeta } from "~/scripts/pages/moderation/meta";
import type { VersionItem } from "~/scripts/pages/projects/types";

defineProps<{
  versions: VersionItem[];
  downloadingVersion: string | null;
}>();

defineEmits<{
  download: [version: string, filename: string];
}>();
</script>

<template>
  <div class="border-border/60 bg-card/40 rounded-2xl border p-5">
    <p
      class="text-muted-foreground mb-3 inline-flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
    >
      <Layers class="size-3.5" /> Versions
      <span class="text-muted-foreground/70 normal-case"
        >({{ versions.length }})</span
      >
    </p>
    <p v-if="!versions.length" class="text-muted-foreground text-sm">
      No versions uploaded.
    </p>
    <ul v-else class="space-y-3">
      <li
        v-for="version in versions"
        :key="version.version_number"
        class="border-border/40 border-l-2 pl-3"
      >
        <div class="flex flex-wrap items-center gap-2">
          <span class="text-foreground font-semibold">
            {{ version.version_number }}
          </span>
          <span
            v-if="version.name.trim()"
            class="text-muted-foreground text-sm"
          >
            {{ version.name }}
          </span>
          <span
            class="rounded-full px-2 py-0.5 text-xs font-semibold"
            :class="channelMeta(version.channel).class"
          >
            {{ channelMeta(version.channel).label }}
          </span>
          <span class="text-muted-foreground text-xs">
            {{ relativeTime(version.created_at) }}
          </span>
        </div>
        <p
          v-if="version.changelog.trim()"
          class="text-muted-foreground mt-1 text-sm whitespace-pre-wrap"
        >
          {{ version.changelog }}
        </p>
        <div
          v-if="version.file"
          class="mt-1.5 flex flex-wrap items-center gap-3"
        >
          <span
            class="text-muted-foreground/80 inline-flex items-center gap-1.5 text-xs"
          >
            <FileArchive class="size-3.5" />
            {{ version.file.filename }} ·
            {{ formatBytes(version.file.size) }}
          </span>
          <button
            type="button"
            class="text-primary hover:bg-primary/10 inline-flex items-center gap-1 rounded px-1.5 py-0.5 text-xs font-medium disabled:opacity-50"
            :disabled="downloadingVersion === version.version_number"
            @click="
              $emit('download', version.version_number, version.file.filename)
            "
          >
            <Loader2
              v-if="downloadingVersion === version.version_number"
              class="size-3.5 animate-spin"
            />
            <Download v-else class="size-3.5" />
            Download
          </button>
        </div>
      </li>
    </ul>
  </div>
</template>
