<script setup lang="ts">
import { ArrowLeft, Compass, Construction } from "@lucide/vue";
import { Button } from "@/components/ui/button";

defineProps<{
  title: string;
  description?: string;
}>();
</script>

<template>
  <section
    class="section-glow mx-auto flex min-h-[60vh] max-w-2xl flex-col items-center justify-center px-6 py-24 text-center"
  >
    <div
      class="bg-primary/10 text-primary mb-6 flex size-16 items-center justify-center rounded-2xl"
    >
      <Construction class="size-8" :stroke-width="1.75" />
    </div>

    <span
      class="bg-primary/10 text-primary mb-4 rounded-full px-3 py-1 text-xs font-medium"
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
</template>
