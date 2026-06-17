<script setup lang="ts">
import {
  Boxes,
  CircleX,
  FileArchive,
  FileSearch,
  Images,
  Loader2,
  Package2,
} from "@lucide/vue";
import { formatBytes } from "~/scripts/formatters";
import { decisionMeta, findingClass } from "~/scripts/pages/moderation/meta";
import type { AnalysisReport } from "~/scripts/pages/projects/types";

defineProps<{
  analysis: AnalysisReport;
  findingFile: (message: string) => string | null;
}>();

defineEmits<{
  jumpToFinding: [message: string];
}>();
</script>

<template>
  <div class="border-border/60 bg-card/40 rounded-2xl border p-5">
    <p
      class="text-muted-foreground mb-3 inline-flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
    >
      <FileSearch class="size-3.5" /> Pack analysis
      <span
        v-if="analysis.mctools_version"
        class="text-muted-foreground/70 normal-case"
        >(Creator Tools {{ analysis.mctools_version }})</span
      >
    </p>

    <p
      v-if="analysis.status === 'pending'"
      class="text-muted-foreground inline-flex items-center gap-2 text-sm"
    >
      <Loader2 class="size-4 animate-spin" /> Analysis in progress - check back
      shortly.
    </p>

    <div
      v-else-if="analysis.status === 'error'"
      class="rounded-lg border border-red-500/30 bg-red-500/10 p-3 text-sm text-red-400"
    >
      <p class="inline-flex items-center gap-1.5 font-medium">
        <CircleX class="size-4" /> Analysis failed
      </p>
      <p class="text-muted-foreground mt-1 text-xs break-words">
        {{ analysis.error || "Unknown error" }}
      </p>
    </div>

    <template v-else-if="analysis.status === 'ready' && analysis.report">
      <!-- Decision banner -->
      <div
        class="inline-flex items-center gap-2 rounded-lg border px-3 py-2 text-sm font-semibold"
        :class="decisionMeta(analysis.report.decision).class"
      >
        <component
          :is="decisionMeta(analysis.report.decision).icon"
          class="size-4"
        />
        {{ decisionMeta(analysis.report.decision).label }}
      </div>

      <!-- Counts -->
      <div class="mt-3 flex flex-wrap gap-2 text-xs">
        <span
          class="rounded-full bg-red-500/15 px-2 py-0.5 font-medium text-red-400"
        >
          {{ analysis.report.counts.errors }} errors
        </span>
        <span
          class="rounded-full bg-amber-500/15 px-2 py-0.5 font-medium text-amber-500"
        >
          {{ analysis.report.counts.warnings }} warnings
        </span>
        <span
          class="rounded-full bg-sky-500/15 px-2 py-0.5 font-medium text-sky-400"
        >
          {{ analysis.report.counts.recommendations }}
          recommendations
        </span>
        <span
          class="bg-muted text-muted-foreground rounded-full px-2 py-0.5 font-medium"
        >
          {{ analysis.report.counts.testSuccess }}/{{
            analysis.report.counts.testSuccess + analysis.report.counts.testFail
          }}
          checks passed
        </span>
      </div>

      <!-- Pack info -->
      <dl class="mt-4 grid grid-cols-2 gap-2 sm:grid-cols-4">
        <div
          class="border-border/40 bg-background/40 rounded-lg border px-3 py-2"
        >
          <dt
            class="text-muted-foreground inline-flex items-center gap-1 text-xs"
          >
            <Package2 class="size-3" /> Behavior packs
          </dt>
          <dd class="text-foreground mt-0.5 text-base font-semibold">
            {{ analysis.report.info.behaviorPackManifestCount }}
          </dd>
        </div>
        <div
          class="border-border/40 bg-background/40 rounded-lg border px-3 py-2"
        >
          <dt
            class="text-muted-foreground inline-flex items-center gap-1 text-xs"
          >
            <Images class="size-3" /> Resource packs
          </dt>
          <dd class="text-foreground mt-0.5 text-base font-semibold">
            {{ analysis.report.info.resourcePackManifestCount }}
          </dd>
        </div>
        <div
          class="border-border/40 bg-background/40 rounded-lg border px-3 py-2"
        >
          <dt
            class="text-muted-foreground inline-flex items-center gap-1 text-xs"
          >
            <FileArchive class="size-3" /> Total size
          </dt>
          <dd class="text-foreground mt-0.5 text-base font-semibold">
            {{ formatBytes(analysis.report.info.overallSize) }}
          </dd>
        </div>
        <div
          class="border-border/40 bg-background/40 rounded-lg border px-3 py-2"
        >
          <dt
            class="text-muted-foreground inline-flex items-center gap-1 text-xs"
          >
            <Boxes class="size-3" /> Textures
          </dt>
          <dd class="text-foreground mt-0.5 text-base font-semibold">
            {{ analysis.report.info.textureCount }}
          </dd>
        </div>
      </dl>

      <!-- Capabilities / APIs -->
      <div
        v-if="
          analysis.report.info.capabilities.length ||
          analysis.report.info.apisUsed.length
        "
        class="mt-3 flex flex-wrap gap-1.5"
      >
        <span
          v-for="cap in analysis.report.info.capabilities"
          :key="`cap-${cap}`"
          class="bg-primary/10 text-primary rounded-full px-2 py-0.5 text-xs font-medium"
        >
          {{ cap }}
        </span>
        <span
          v-for="api in analysis.report.info.apisUsed"
          :key="`api-${api}`"
          class="bg-muted text-muted-foreground rounded-full px-2 py-0.5 text-xs font-medium"
        >
          {{ api }}
        </span>
      </div>

      <!-- Findings -->
      <ul
        v-if="analysis.report.findings.length"
        class="border-border/40 mt-4 space-y-1.5 border-t pt-3"
      >
        <li
          v-for="(finding, i) in analysis.report.findings"
          :key="`finding-${i}`"
          class="flex items-start gap-2 text-xs"
          :class="
            findingFile(finding.message)
              ? 'hover:bg-muted/40 -mx-1 cursor-pointer rounded px-1 py-0.5'
              : ''
          "
          @click="$emit('jumpToFinding', finding.message)"
        >
          <span class="mt-px font-semibold" :class="findingClass(finding.type)">
            {{ finding.generatorId }}
          </span>
          <span class="text-muted-foreground break-words">
            {{ finding.message }}
          </span>
          <FileSearch
            v-if="findingFile(finding.message)"
            class="text-muted-foreground/60 mt-px ml-auto size-3.5 shrink-0"
          />
        </li>
      </ul>
    </template>
  </div>
</template>
