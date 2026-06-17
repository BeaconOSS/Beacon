<script setup lang="ts">
import { Download, Loader2, Package, Trash2, Upload } from "@lucide/vue";
import { VERSION_CHANNELS } from "~/scripts/pages/projects/versions";
import {
  CHANNEL_STYLES,
  channelLabel,
} from "~/scripts/pages/projects/settings/meta";
import { formatBytes, formatDate } from "~/scripts/formatters";
import type { Version } from "~/scripts/pages/projects/types";

const versionNumber = defineModel<string>("versionNumber", { required: true });
const channel = defineModel<string>("channel", { required: true });
const name = defineModel<string>("name", { required: true });
const changelog = defineModel<string>("changelog", { required: true });

defineProps<{
  file: File | null;
  uploadError: string;
  uploadPending: boolean;
  locked: boolean;
  versions: Version[];
  downloadUrl: (version: Version) => string;
}>();

defineEmits<{
  fileChange: [event: Event];
  submit: [];
  deleteVersion: [version: Version];
}>();

const confirmDeleteVersionId = ref<string | null>(null);
</script>

<template>
  <section class="space-y-6">
    <div class="card-glass space-y-5 rounded-2xl p-6">
      <div>
        <h2 class="section-title mb-1 flex items-center gap-2 text-lg">
          <Package class="text-primary size-5" />
          Upload a version
        </h2>
        <p class="text-muted-foreground text-sm leading-relaxed">
          Every release of your project is a version. Upload the downloadable
          file, give it a version number, and add a changelog so people know
          what changed.
        </p>
      </div>

      <div class="grid gap-4 sm:grid-cols-2">
        <div class="space-y-2">
          <Label for="version-number">Version number</Label>
          <Input
            id="version-number"
            v-model="versionNumber"
            placeholder="1.0.0"
          />
        </div>
        <div class="space-y-2">
          <Label for="version-channel">Release channel</Label>
          <Select id="version-channel" v-model="channel">
            <SelectTrigger class="w-full">
              <SelectValue placeholder="Select a channel" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem
                v-for="option in VERSION_CHANNELS"
                :key="option.value"
                :value="option.value"
              >
                {{ option.label }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <div class="space-y-2">
        <Label for="version-name">Display name (optional)</Label>
        <Input
          id="version-name"
          v-model="name"
          placeholder="e.g. Winter Update"
        />
      </div>

      <div class="space-y-2">
        <Label for="version-changelog">Changelog (optional)</Label>
        <Textarea
          id="version-changelog"
          v-model="changelog"
          rows="4"
          placeholder="What changed in this version?"
        />
      </div>

      <div class="space-y-2">
        <Label for="version-file">File</Label>
        <input
          id="version-file"
          type="file"
          class="border-input file:bg-muted file:text-foreground hover:file:bg-muted/70 block w-full cursor-pointer rounded-xl border bg-transparent text-sm file:mr-3 file:cursor-pointer file:rounded-lg file:border-0 file:px-3 file:py-2 file:text-sm file:font-medium"
          @change="$emit('fileChange', $event)"
        />
        <p v-if="file" class="text-muted-foreground text-xs">
          {{ file.name }} · {{ formatBytes(file.size) }}
        </p>
      </div>

      <div
        class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center"
        :class="uploadError ? 'sm:justify-between' : 'sm:justify-end'"
      >
        <p v-if="uploadError" class="text-destructive text-sm">
          {{ uploadError }}
        </p>
        <Button
          class="btn-glow shrink-0"
          :disabled="uploadPending || locked"
          @click="$emit('submit')"
        >
          <Loader2 v-if="uploadPending" class="size-4 animate-spin" />
          <Upload v-else class="size-4" />
          Publish version
        </Button>
      </div>
    </div>

    <div class="card-glass rounded-2xl p-6">
      <h2 class="section-title mb-1 flex items-center gap-2 text-lg">
        <Package class="text-primary size-5" />
        Published versions
      </h2>
      <p class="text-muted-foreground mb-5 text-sm">
        {{ versions.length }}
        {{ versions.length === 1 ? "version" : "versions" }}
        published.
      </p>

      <p
        v-if="!versions.length"
        class="text-muted-foreground border-border/60 rounded-xl border border-dashed p-8 text-center text-sm"
      >
        No versions yet. Upload your first version above.
      </p>

      <ul v-else class="space-y-3">
        <li
          v-for="version in versions"
          :key="version.id"
          class="border-border/60 bg-muted/20 rounded-xl border p-4"
        >
          <div class="flex flex-wrap items-start justify-between gap-3">
            <div class="min-w-0">
              <div class="flex flex-wrap items-center gap-2">
                <span class="text-foreground font-semibold">
                  {{ version.version_number }}
                </span>
                <span
                  class="rounded-full px-2 py-0.5 text-[11px] font-semibold"
                  :class="
                    CHANNEL_STYLES[version.channel] ??
                    'bg-muted text-muted-foreground'
                  "
                >
                  {{ channelLabel(version.channel) }}
                </span>
                <span v-if="version.name" class="text-muted-foreground text-sm">
                  {{ version.name }}
                </span>
              </div>
              <div
                class="text-muted-foreground mt-1.5 flex flex-wrap items-center gap-3 text-xs"
              >
                <span>{{ formatDate(version.created_at) }}</span>
                <span v-if="version.file">
                  {{ formatBytes(version.file.size) }}
                </span>
                <span class="inline-flex items-center gap-1">
                  <Download class="size-3.5" />
                  {{ version.download_count }}
                </span>
              </div>
            </div>

            <div class="flex shrink-0 items-center gap-2">
              <Button v-if="version.file" as-child variant="outline" size="sm">
                <a :href="downloadUrl(version)">
                  <Download class="size-4" />
                  Download
                </a>
              </Button>
              <template v-if="confirmDeleteVersionId === version.id">
                <Button
                  variant="destructive"
                  size="sm"
                  @click="
                    $emit('deleteVersion', version);
                    confirmDeleteVersionId = null;
                  "
                >
                  Confirm
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  @click="confirmDeleteVersionId = null"
                >
                  Cancel
                </Button>
              </template>
              <Button
                v-else
                variant="ghost"
                size="icon"
                aria-label="Delete version"
                :disabled="locked"
                @click="confirmDeleteVersionId = version.id"
              >
                <Trash2 class="text-destructive size-4" />
              </Button>
            </div>
          </div>

          <div
            v-if="version.changelog"
            class="text-muted-foreground mt-3 border-t pt-3 text-sm whitespace-pre-line"
          >
            {{ version.changelog }}
          </div>
        </li>
      </ul>
    </div>
  </section>
</template>
