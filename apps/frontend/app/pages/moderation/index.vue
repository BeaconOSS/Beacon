<script setup lang="ts">
import {
  Clock,
  Globe,
  Loader2,
  Package,
  Palette,
  ShieldCheck,
  Shirt,
} from "@lucide/vue";
import type { Component } from "vue";
import {
  useModerationQueue,
  type ModerationQueueItem,
} from "~/scripts/pages/moderation";
import { projectTypeLabel } from "~/scripts/pages/projects";
import { useAuth } from "~/scripts/auth";
import { relativeTime } from "~/scripts/formatters";

const { isModerator } = useAuth();
const { projects, error, pending, iconUrl, load } = useModerationQueue();

await load();

const TYPE_STYLES: Record<string, { icon: Component; gradient: string }> = {
  addon: {
    icon: Package,
    gradient: "from-amber-400/30 to-amber-600/10 text-amber-300",
  },
  world: {
    icon: Globe,
    gradient: "from-emerald-400/30 to-emerald-600/10 text-emerald-300",
  },
  resource_pack: {
    icon: Palette,
    gradient: "from-violet-400/30 to-violet-600/10 text-violet-300",
  },
  skin_pack: {
    icon: Shirt,
    gradient: "from-pink-400/30 to-pink-600/10 text-pink-300",
  },
};

function typeStyle(type: string) {
  return TYPE_STYLES[type] ?? TYPE_STYLES.addon!;
}

function projectPath(item: ModerationQueueItem): string {
  return `/moderation/${item.slug}`;
}
</script>

<template>
  <div class="page-canvas min-h-screen">
    <div class="mx-auto max-w-5xl px-6 py-10">
      <div class="mb-8 flex items-center gap-3">
        <div
          class="bg-primary/15 text-primary flex size-11 items-center justify-center rounded-2xl"
        >
          <ShieldCheck class="size-6" />
        </div>
        <div>
          <h1 class="display-heading text-2xl sm:text-3xl">Review queue</h1>
          <p class="text-muted-foreground text-sm">
            Projects submitted by creators awaiting moderation.
          </p>
        </div>
      </div>

      <div
        v-if="pending"
        class="text-muted-foreground flex items-center gap-2 py-20"
      >
        <Loader2 class="size-5 animate-spin" />
        Loading the review queue…
      </div>

      <div
        v-else-if="error"
        class="border-border/60 rounded-xl border p-10 text-center"
      >
        <p class="text-muted-foreground">{{ error }}</p>
        <NuxtLink
          v-if="!isModerator"
          to="/"
          class="text-primary mt-3 inline-block text-sm hover:underline"
        >
          Back to home
        </NuxtLink>
      </div>

      <div
        v-else-if="!projects.length"
        class="border-border/60 rounded-2xl border border-dashed p-12 text-center"
      >
        <ShieldCheck class="text-muted-foreground/50 mx-auto mb-3 size-10" />
        <p class="text-foreground font-medium">The queue is clear</p>
        <p class="text-muted-foreground mt-1 text-sm">
          There are no projects awaiting review right now.
        </p>
      </div>

      <ul v-else class="space-y-3">
        <li v-for="item in projects" :key="item.id">
          <NuxtLink
            :to="projectPath(item)"
            class="border-border/60 bg-card/40 hover:border-primary/40 flex items-start gap-4 rounded-2xl border p-4 backdrop-blur-sm transition-colors sm:p-5"
          >
            <div
              v-if="iconUrl(item)"
              class="size-14 shrink-0 overflow-hidden rounded-xl ring-1 ring-white/10"
            >
              <img
                :src="iconUrl(item)!"
                :alt="item.title"
                class="size-full object-cover"
              />
            </div>
            <div
              v-else
              class="flex size-14 shrink-0 items-center justify-center rounded-xl bg-gradient-to-br ring-1 ring-white/10"
              :class="typeStyle(item.project_type).gradient"
            >
              <component
                :is="typeStyle(item.project_type).icon"
                class="size-7"
              />
            </div>

            <div class="min-w-0 flex-1">
              <div class="flex flex-wrap items-center gap-2">
                <h2 class="text-foreground truncate font-semibold">
                  {{ item.title }}
                </h2>
                <span
                  class="bg-primary/15 text-primary rounded-full px-2.5 py-0.5 text-xs font-semibold"
                >
                  {{ projectTypeLabel(item.project_type) }}
                </span>
              </div>
              <p class="text-muted-foreground mt-1 line-clamp-2 text-sm">
                {{ item.summary }}
              </p>
              <div
                class="text-muted-foreground mt-2 flex flex-wrap items-center gap-4 text-xs"
              >
                <span>by {{ item.owner }}</span>
                <span class="inline-flex items-center gap-1">
                  <Clock class="size-3.5" />
                  Submitted {{ relativeTime(item.submitted_at) }}
                </span>
              </div>
            </div>
          </NuxtLink>
        </li>
      </ul>
    </div>
  </div>
</template>
