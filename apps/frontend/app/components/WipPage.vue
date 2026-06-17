<script setup lang="ts">
import { ArrowLeft, Compass, Construction } from "@lucide/vue";
import { Button } from "@/components/ui/button";

defineProps<{
  title: string;
  description?: string;
}>();
</script>

<template>
  <div class="page-canvas">
    <section
      class="mx-auto flex min-h-[70vh] max-w-2xl flex-col items-center justify-center px-6 py-24 text-center"
    >
      <span class="icon-chip mb-6 size-16 rounded-2xl">
        <Construction class="size-8" :stroke-width="2" />
      </span>

      <span
        class="bg-primary/15 text-primary mb-4 rounded-full px-3 py-1 text-xs font-semibold uppercase tracking-wide"
      >
        Work in progress
      </span>

      <h1 class="display-heading mb-3 text-4xl md:text-5xl">{{ title }}</h1>

      <p class="text-muted-foreground mx-auto mb-8 max-w-md">
        {{
          description ??
          "This page isn't ready yet. We're still building it - check back soon."
        }}
      </p>

      <div class="flex flex-wrap justify-center gap-3">
        <Button as-child variant="outline">
          <NuxtLink to="/">
            <ArrowLeft />
            Back home
          </NuxtLink>
        </Button>
        <Button as-child class="btn-glow">
          <NuxtLink to="/projects">
            <Compass />
            Discover content
          </NuxtLink>
        </Button>
      </div>
    </section>
  </div>
</template>
