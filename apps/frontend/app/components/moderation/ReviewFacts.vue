<script setup lang="ts">
import {
  Calendar,
  Coins,
  Download,
  Eye,
  Heart,
  Images,
  Package2,
} from "@lucide/vue";
import { formatDate } from "~/scripts/formatters";
import { visibilityLabel } from "~/scripts/pages/moderation/meta";
import type { ProjectFacts } from "~/scripts/pages/projects/types";

defineProps<{
  facts: ProjectFacts;
}>();
</script>

<template>
  <div class="border-border/60 bg-card/40 rounded-2xl border p-5">
    <p
      class="text-muted-foreground mb-3 text-xs font-semibold tracking-wide uppercase"
    >
      Project facts
    </p>
    <dl class="space-y-2 text-sm">
      <div class="flex items-center justify-between gap-2">
        <dt class="text-muted-foreground inline-flex items-center gap-1.5">
          <Eye class="size-3.5" /> Visibility
        </dt>
        <dd class="text-foreground font-medium">
          {{ visibilityLabel(facts.visibility) }}
        </dd>
      </div>
      <div class="flex items-center justify-between gap-2">
        <dt class="text-muted-foreground inline-flex items-center gap-1.5">
          <Coins class="size-3.5" /> Monetization
        </dt>
        <dd class="text-foreground font-medium">
          {{
            facts.monetization_enabled
              ? `On · ${facts.creator_share}% creator`
              : "Off"
          }}
        </dd>
      </div>
      <div class="flex items-center justify-between gap-2">
        <dt class="text-muted-foreground inline-flex items-center gap-1.5">
          <Package2 class="size-3.5" /> Versions
        </dt>
        <dd class="text-foreground font-medium">
          {{ facts.version_count }}
        </dd>
      </div>
      <div class="flex items-center justify-between gap-2">
        <dt class="text-muted-foreground inline-flex items-center gap-1.5">
          <Images class="size-3.5" /> Gallery
        </dt>
        <dd class="text-foreground font-medium">
          {{ facts.gallery_count }}
        </dd>
      </div>
      <div class="flex items-center justify-between gap-2">
        <dt class="text-muted-foreground inline-flex items-center gap-1.5">
          <Heart class="size-3.5" /> Hearts
        </dt>
        <dd class="text-foreground font-medium">
          {{ facts.heart_count }}
        </dd>
      </div>
      <div class="flex items-center justify-between gap-2">
        <dt class="text-muted-foreground inline-flex items-center gap-1.5">
          <Download class="size-3.5" /> Downloads
        </dt>
        <dd class="text-foreground font-medium">
          {{ facts.download_count }}
        </dd>
      </div>
      <div class="flex items-center justify-between gap-2">
        <dt class="text-muted-foreground inline-flex items-center gap-1.5">
          <Calendar class="size-3.5" /> Created
        </dt>
        <dd class="text-foreground font-medium">
          {{ formatDate(facts.created_at) }}
        </dd>
      </div>
    </dl>
  </div>
</template>
