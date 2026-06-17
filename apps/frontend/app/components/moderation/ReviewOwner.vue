<script setup lang="ts">
import { ShieldCheck, TriangleAlert, User } from "@lucide/vue";
import { formatDate } from "~/scripts/formatters";
import type { OwnerTrust } from "~/scripts/pages/moderation/types";
import type { OwnerContext } from "~/scripts/pages/projects/types";

defineProps<{
  owner: OwnerContext;
  trust: OwnerTrust | null;
}>();
</script>

<template>
  <div class="border-border/60 bg-card/40 rounded-2xl border p-5">
    <p
      class="text-muted-foreground mb-3 text-xs font-semibold tracking-wide uppercase"
    >
      Creator
    </p>
    <div class="mb-3 flex items-center gap-2">
      <div
        class="bg-primary/15 text-primary flex size-9 items-center justify-center rounded-full"
      >
        <User class="size-4" />
      </div>
      <div class="min-w-0">
        <p class="text-foreground truncate font-medium">
          {{ owner.username }}
        </p>
        <p class="text-muted-foreground text-xs">
          Joined {{ formatDate(owner.member_since) }}
        </p>
      </div>
    </div>
    <span
      v-if="trust"
      class="mb-3 inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-semibold"
      :class="trust.class"
    >
      <ShieldCheck v-if="trust.icon === 'check'" class="size-3.5" />
      <TriangleAlert v-else-if="trust.icon === 'warn'" class="size-3.5" />
      <User v-else class="size-3.5" />
      {{ trust.label }}
    </span>
    <dl class="space-y-1.5 text-sm">
      <div class="flex items-center justify-between">
        <dt class="text-muted-foreground">Projects</dt>
        <dd class="text-foreground font-medium">
          {{ owner.project_count }}
        </dd>
      </div>
      <div class="flex items-center justify-between">
        <dt class="text-muted-foreground">Approved before</dt>
        <dd class="font-medium text-emerald-400">
          {{ owner.approved_count }}
        </dd>
      </div>
      <div class="flex items-center justify-between">
        <dt class="text-muted-foreground">Rejected before</dt>
        <dd class="font-medium text-red-400">
          {{ owner.rejected_count }}
        </dd>
      </div>
    </dl>
  </div>
</template>
