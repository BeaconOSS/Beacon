<script setup lang="ts">
import {
  ArrowLeft,
  Eye,
  Globe,
  Link2,
  Loader2,
  Lock,
  Package,
  Palette,
  Shirt,
} from "@lucide/vue";
import type { Component } from "vue";
import { useCreateProjectForm, PROJECT_TYPES } from "~/scripts/pages/projects";

const {
  title,
  projectType,
  summary,
  visibility,
  user,
  error,
  pending,
  submit,
} = useCreateProjectForm();

const TYPE_STYLES: Record<string, { icon: Component; gradient: string }> = {
  addon: {
    icon: Package,
    gradient: "linear-gradient(135deg, #f59e0b, #b45309)",
  },
  world: { icon: Globe, gradient: "linear-gradient(135deg, #10b981, #047857)" },
  resource_pack: {
    icon: Palette,
    gradient: "linear-gradient(135deg, #8b5cf6, #6d28d9)",
  },
  skin_pack: {
    icon: Shirt,
    gradient: "linear-gradient(135deg, #ec4899, #be185d)",
  },
};

function typeStyle(type: string) {
  return TYPE_STYLES[type] ?? { icon: Package, gradient: "" };
}

const VISIBILITY_OPTIONS = [
  {
    value: "public" as const,
    label: "Public",
    description: "Anyone can find and view it.",
    icon: Eye,
  },
  {
    value: "unlisted" as const,
    label: "Unlisted",
    description: "Only people with the link.",
    icon: Link2,
  },
  {
    value: "private" as const,
    label: "Private",
    description: "Only you and the team.",
    icon: Lock,
  },
];
</script>

<template>
  <div class="page-canvas">
    <div class="mx-auto max-w-2xl px-6 py-12">
      <NuxtLink
        to="/projects"
        class="text-muted-foreground hover:text-foreground mb-6 inline-flex items-center gap-1.5 text-sm transition-colors"
      >
        <ArrowLeft class="size-4" />
        Back to projects
      </NuxtLink>

      <header class="mb-8">
        <p class="text-primary eyebrow mb-2">Publish</p>
        <h1 class="display-heading mb-3 text-4xl md:text-5xl">New project</h1>
        <p class="text-muted-foreground">
          Start with the basics - you can flesh out the rest from the project
          settings afterwards.
        </p>
      </header>

      <form
        class="card-glass space-y-8 rounded-2xl p-6 md:p-8"
        @submit.prevent="submit"
      >
        <div class="space-y-3">
          <Label>Type</Label>
          <div class="grid grid-cols-2 gap-3 sm:grid-cols-4">
            <button
              v-for="type in PROJECT_TYPES"
              :key="type.value"
              type="button"
              class="flex flex-col items-center gap-2 rounded-xl border px-3 py-4 text-sm font-semibold transition-colors"
              :class="
                projectType === type.value
                  ? 'border-primary/70 bg-primary/10 text-primary'
                  : 'border-border text-muted-foreground hover:border-primary/60 hover:text-foreground'
              "
              @click="projectType = type.value"
            >
              <span
                class="icon-chip size-10 rounded-xl"
                :style="{ background: typeStyle(type.value).gradient }"
              >
                <component
                  :is="typeStyle(type.value).icon"
                  class="size-5"
                  :stroke-width="2.25"
                />
              </span>
              {{ type.label }}
            </button>
          </div>
        </div>

        <div class="space-y-2">
          <Label for="title">Name</Label>
          <Input
            id="title"
            v-model="title"
            name="title"
            placeholder="Best Project Ever"
          />
        </div>

        <div class="space-y-2">
          <Label for="owner">Owner</Label>
          <Select model-value="self">
            <SelectTrigger id="owner" class="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="self">
                {{ user ? `@${user.username}` : "Yourself" }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>

        <div class="space-y-3">
          <Label>Visibility</Label>
          <div class="grid gap-3 sm:grid-cols-3">
            <button
              v-for="option in VISIBILITY_OPTIONS"
              :key="option.value"
              type="button"
              class="flex flex-col gap-1 rounded-xl border p-4 text-left transition-colors"
              :class="
                visibility === option.value
                  ? 'border-primary/70 bg-primary/10'
                  : 'border-border hover:border-primary/60'
              "
              @click="visibility = option.value"
            >
              <span
                class="inline-flex items-center gap-2 text-sm font-semibold"
                :class="
                  visibility === option.value
                    ? 'text-primary'
                    : 'text-foreground'
                "
              >
                <component :is="option.icon" class="size-4" />
                {{ option.label }}
              </span>
              <span class="text-muted-foreground text-xs">
                {{ option.description }}
              </span>
            </button>
          </div>
        </div>

        <div class="space-y-2">
          <Label for="summary">Summary</Label>
          <Textarea
            id="summary"
            v-model="summary"
            name="summary"
            :rows="2"
            placeholder="A sentence or two describing your project."
          />
        </div>

        <p v-if="error" class="text-destructive text-sm">{{ error }}</p>

        <div class="flex items-center justify-end gap-3 border-t pt-6">
          <NuxtLink to="/projects">
            <Button type="button" variant="ghost">Cancel</Button>
          </NuxtLink>
          <Button type="submit" class="btn-glow" :disabled="pending">
            <Loader2 v-if="pending" class="size-4 animate-spin" />
            {{ pending ? "Creating..." : "Create project" }}
          </Button>
        </div>
      </form>
    </div>
  </div>
</template>
