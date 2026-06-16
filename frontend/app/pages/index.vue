<script setup lang="ts">
import {
  ArrowRight,
  Boxes,
  Download,
  GitBranch,
  Heart,
  HeartHandshake,
  Search,
  Sparkles,
  Upload,
} from "@lucide/vue";
import { Button } from "@/components/ui/button";

const features = [
  {
    icon: Search,
    title: "Built for Bedrock",
    description:
      "Browse Add-Ons, worlds, resource packs, and skins. Find exactly what you need.",
  },
  {
    icon: HeartHandshake,
    title: "For a good cause",
    description: "BeaconOSS is a non-profit. Net profits donated to charity.",
  },
  {
    icon: GitBranch,
    title: "Open source by default",
    description:
      "Every BeaconOSS project is open source. Jump in and contribute yourself.",
  },
  {
    icon: Upload,
    title: "Publish your own",
    description:
      "Not just us - the whole community can upload Bedrock content.",
  },
];

const stats = [
  {
    icon: Boxes,
    label: "Open source",
    value: "Every BeaconOSS project is open source",
  },
  {
    icon: Download,
    label: "Easy downloads",
    value: "Just a simple click",
  },
  { icon: Heart, label: "Community", value: "Anyone can contribute" },
];

const posts = [
  {
    tag: "Changelog",
    title: "What's new?",
    excerpt: "The latest updates and improvements.",
  },
  {
    tag: "Guide",
    title: "Publishing your first project",
    excerpt: "A step-by-step walkthrough from create to first release.",
  },
  {
    tag: "Community",
    title: "Who are we?",
    excerpt: "Learn what BeaconOSS is all about and meet the team behind it.",
  },
];
</script>

<template>
  <div>
    <HeroSection />

    <section class="section-glow mx-auto max-w-6xl px-6 py-20">
      <div class="mb-14 text-center">
        <h2 class="display-heading mb-3 text-3xl md:text-4xl">
          Everything in one place
        </h2>
        <p class="text-muted-foreground mx-auto max-w-2xl">
          From finding the perfect Add-On to shipping your own - we got you
          covered.
        </p>
      </div>

      <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
        <div
          v-for="feature in features"
          :key="feature.title"
          class="card-glass rounded-xl p-6"
        >
          <component
            :is="feature.icon"
            class="text-primary mb-4 size-8"
            :stroke-width="1.75"
          />
          <h3 class="mb-2 text-lg font-semibold">{{ feature.title }}</h3>
          <p class="text-muted-foreground text-sm">
            {{ feature.description }}
          </p>
        </div>
      </div>
    </section>

    <section class="mx-auto max-w-6xl px-6 py-20">
      <div
        class="card-glass relative overflow-hidden rounded-2xl px-6 py-12 md:px-12"
      >
        <div class="grid items-center gap-10 md:grid-cols-2">
          <div>
            <div
              class="text-primary mb-4 inline-flex items-center gap-2 text-sm font-medium"
            >
              <Sparkles class="size-4" />
              A stepping stone for creators
            </div>
            <h2 class="display-heading mb-4 text-3xl md:text-4xl">
              Build real skills, ship real products
            </h2>
            <p class="text-muted-foreground mb-6">
              BeaconOSS is an official Minecraft Marketplace partner. Contribute
              to open-source projects, learn what it's like to ship in a real
              Marketplace setting, and help raise money for charity along the
              way.
            </p>
            <Button as-child>
              <NuxtLink to="/projects/new">
                Publish a project
                <ArrowRight />
              </NuxtLink>
            </Button>
          </div>

          <div class="grid gap-4">
            <div
              v-for="stat in stats"
              :key="stat.label"
              class="bg-background/40 flex items-center gap-4 rounded-xl border p-4"
            >
              <div
                class="bg-primary/10 text-primary flex size-11 items-center justify-center rounded-lg"
              >
                <component :is="stat.icon" class="size-5" />
              </div>
              <div>
                <p class="font-semibold">{{ stat.label }}</p>
                <p class="text-muted-foreground text-sm">{{ stat.value }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <section class="mx-auto max-w-6xl px-6 py-20">
      <div class="mb-10 flex items-end justify-between">
        <div>
          <h2 class="display-heading text-3xl md:text-4xl">Latest News</h2>
          <p class="text-muted-foreground mt-2">
            News, guides, and updates from the Beacon team.
          </p>
        </div>
        <NuxtLink
          to="/news"
          class="text-primary hidden items-center gap-1 text-sm font-medium hover:underline sm:inline-flex"
        >
          View all
          <ArrowRight class="size-4" />
        </NuxtLink>
      </div>

      <div class="grid gap-6 md:grid-cols-3">
        <article
          v-for="post in posts"
          :key="post.title"
          class="card-glass flex flex-col rounded-xl p-6"
        >
          <span
            class="bg-primary/10 text-primary mb-4 w-fit rounded-full px-2.5 py-0.5 text-xs font-medium"
          >
            {{ post.tag }}
          </span>
          <h3 class="mb-2 text-lg font-semibold">{{ post.title }}</h3>
          <p class="text-muted-foreground text-sm">{{ post.excerpt }}</p>
        </article>
      </div>
    </section>

    <section class="section-glow mx-auto max-w-4xl px-6 py-24 text-center">
      <h2 class="display-heading mb-4 text-3xl md:text-4xl">
        Ready to get started?
      </h2>
      <p class="text-muted-foreground mx-auto mb-8 max-w-lg">
        Discover Bedrock content, publish your own, or contribute to a project.
        Free to use, open by default, and for a good cause.
      </p>
      <div class="flex flex-wrap justify-center gap-4">
        <Button as-child size="lg" class="btn-glow">
          <NuxtLink to="/register">Register</NuxtLink>
        </Button>
        <Button as-child size="lg" variant="outline">
          <NuxtLink to="/projects">Browse projects</NuxtLink>
        </Button>
      </div>
    </section>
  </div>
</template>
