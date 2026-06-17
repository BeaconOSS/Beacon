<script setup lang="ts">
import { ChevronRight, FileDiff, Layers } from "@lucide/vue";
import { formatBytes } from "~/scripts/formatters";
import { diffStatusMeta, kindLabel } from "~/scripts/pages/moderation/meta";
import type { FilePreviewState } from "~/scripts/pages/moderation/types";
import type { PackDiff } from "~/scripts/pages/projects/types";
import ReviewFilePreview from "./ReviewFilePreview.vue";

defineProps<{
  diff: PackDiff;
  showFiles: boolean;
  activeFile: FilePreviewState | null;
  fileAnchorId: (path: string) => string;
}>();

defineEmits<{
  "update:showFiles": [value: boolean];
  toggleFile: [path: string, status: string];
}>();
</script>

<template>
  <div class="border-border/60 bg-card/40 rounded-2xl border p-5">
    <p
      class="text-muted-foreground mb-3 inline-flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
    >
      <FileDiff class="size-3.5" /> Pack changes
      <span class="text-muted-foreground/70 normal-case">
        vs previous version
      </span>
    </p>

    <p
      v-if="!diff.added && !diff.removed && !diff.modified"
      class="text-muted-foreground text-sm"
    >
      No file changes detected
      <template v-if="!diff.unchanged">
        - file index not yet available.
      </template>
      <template v-else> ({{ diff.unchanged }} files unchanged). </template>
    </p>

    <template v-else>
      <!-- Totals -->
      <div class="flex flex-wrap gap-2 text-xs">
        <span
          class="rounded-full bg-emerald-500/15 px-2 py-0.5 font-medium text-emerald-400"
        >
          +{{ diff.added }} added
        </span>
        <span
          class="rounded-full bg-red-500/15 px-2 py-0.5 font-medium text-red-400"
        >
          −{{ diff.removed }} removed
        </span>
        <span
          class="rounded-full bg-amber-500/15 px-2 py-0.5 font-medium text-amber-500"
        >
          ~{{ diff.modified }} modified
        </span>
        <span
          class="bg-muted text-muted-foreground rounded-full px-2 py-0.5 font-medium"
        >
          {{ diff.unchanged }} unchanged
        </span>
      </div>

      <!-- Per-kind rollup -->
      <ul v-if="diff.by_kind.length" class="mt-3 space-y-1 text-xs">
        <li
          v-for="row in diff.by_kind"
          :key="`kind-${row.kind}`"
          class="flex items-center justify-between gap-2"
        >
          <span class="text-muted-foreground">
            {{ kindLabel(row.kind) }}
          </span>
          <span class="flex items-center gap-1.5 font-medium">
            <span v-if="row.added" class="text-emerald-400"
              >+{{ row.added }}</span
            >
            <span v-if="row.removed" class="text-red-400"
              >−{{ row.removed }}</span
            >
            <span v-if="row.modified" class="text-amber-500"
              >~{{ row.modified }}</span
            >
          </span>
        </li>
      </ul>

      <!-- File list (collapsible) -->
      <div class="border-border/40 mt-4 border-t pt-3">
        <button
          type="button"
          class="text-muted-foreground hover:text-foreground inline-flex items-center gap-1.5 text-xs font-medium"
          @click="$emit('update:showFiles', !showFiles)"
        >
          <Layers class="size-3.5" />
          {{ showFiles ? "Hide" : "Show" }} changed files ({{
            diff.files.length
          }})
        </button>

        <ul v-if="showFiles" class="mt-2 space-y-1">
          <li
            v-for="file in diff.files"
            :id="fileAnchorId(file.path)"
            :key="`diff-${file.status}-${file.path}`"
          >
            <button
              type="button"
              class="hover:bg-muted/40 flex w-full items-center justify-between gap-2 rounded px-1 py-0.5 text-left text-xs"
              @click="$emit('toggleFile', file.path, file.status)"
            >
              <span class="flex min-w-0 items-center gap-2">
                <ChevronRight
                  class="size-3 shrink-0 transition-transform"
                  :class="{
                    'rotate-90': activeFile?.path === file.path,
                  }"
                />
                <span
                  class="rounded px-1.5 py-0.5 font-semibold"
                  :class="diffStatusMeta(file.status).class"
                >
                  {{ diffStatusMeta(file.status).label }}
                </span>
                <span
                  class="text-muted-foreground truncate font-mono"
                  :title="file.path"
                >
                  {{ file.path }}
                </span>
              </span>
              <span
                v-if="file.status === 'modified'"
                class="text-muted-foreground/70 shrink-0"
              >
                {{ formatBytes(file.old_size ?? 0) }} →
                {{ formatBytes(file.new_size ?? 0) }}
              </span>
              <span
                v-else-if="file.status === 'added'"
                class="shrink-0 text-emerald-400/80"
              >
                {{ formatBytes(file.new_size ?? 0) }}
              </span>
              <span v-else class="shrink-0 text-red-400/80">
                {{ formatBytes(file.old_size ?? 0) }}
              </span>
            </button>

            <ReviewFilePreview
              v-if="activeFile?.path === file.path"
              :preview="activeFile"
            />
          </li>
        </ul>

        <p
          v-if="showFiles && diff.files_truncated"
          class="text-muted-foreground/70 mt-2 text-xs italic"
        >
          File list truncated - too many changes to display in full.
        </p>
      </div>
    </template>
  </div>
</template>
