<script setup lang="ts">
import { ArrowRight, Compass, LayoutDashboard } from "@lucide/vue";
import { useAuth } from "~/scripts/auth";
import { Button } from "@/components/ui/button";

const { user } = useAuth();
</script>

<template>
  <section class="hero-glow relative px-6 py-24 text-center md:py-32">
    <div class="mx-auto max-w-4xl">
      <div
        class="gradient-brand mb-6 inline-block rounded-full px-4 py-1 text-sm font-medium text-white shadow-lg shadow-sky-500/20"
      >
        Open-source Bedrock content, for a good cause
      </div>

      <h1 class="display-heading mb-6 text-5xl md:text-7xl">
        The home for
        <span class="gradient-brand-text-shimmer"
          >Minecraft Bedrock content</span
        >
      </h1>

      <p
        class="text-muted-foreground mx-auto mb-10 max-w-2xl text-lg md:text-xl"
      >
        Discover Add-Ons, worlds, resource packs, and skins - or publish your
        own. Every BeaconOSS project is open source, and our profits go to
        charity.
      </p>

      <div class="flex flex-wrap justify-center gap-4">
        <Button as-child size="lg" class="btn-glow">
          <NuxtLink to="/projects">
            <Compass />
            Discover
          </NuxtLink>
        </Button>

        <Button v-if="user" as-child size="lg" variant="outline">
          <NuxtLink to="/projects">
            <LayoutDashboard />
            Dashboard
          </NuxtLink>
        </Button>
        <Button v-else as-child size="lg" variant="outline">
          <NuxtLink to="/register">
            Get started
            <ArrowRight />
          </NuxtLink>
        </Button>
      </div>
    </div>
  </section>
</template>
