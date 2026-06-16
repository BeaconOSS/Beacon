<script setup lang="ts">
import {
  ArrowRight,
  Boxes,
  Download,
  GitBranch,
  Globe,
  Heart,
  HeartHandshake,
  Package,
  Palette,
  Search,
  Shirt,
  Sparkles,
  Upload,
} from "@lucide/vue";
import { Button } from "@/components/ui/button";

const categoryTiles = [
  {
    icon: Package,
    label: "Add-Ons",
    hint: "Gameplay, mobs & mechanics",
    to: "/projects?type=addon",
    gradient: "linear-gradient(135deg, #f59e0b, #b45309)",
  },
  {
    icon: Globe,
    label: "Worlds",
    hint: "Maps & adventures",
    to: "/projects?type=world",
    gradient: "linear-gradient(135deg, #10b981, #047857)",
  },
  {
    icon: Palette,
    label: "Resource packs",
    hint: "Textures & visuals",
    to: "/projects?type=resource-pack",
    gradient: "linear-gradient(135deg, #8b5cf6, #6d28d9)",
  },
  {
    icon: Shirt,
    label: "Skins",
    hint: "Make it yours",
    to: "/projects?type=skin",
    gradient: "linear-gradient(135deg, #ec4899, #be185d)",
  },
];

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
  <div class="page-canvas linked-sections">
    <HeroSection />

    <section class="mx-auto max-w-6xl px-6 py-16">
      <div class="mb-8 flex items-end justify-between gap-4">
        <div>
          <p class="text-primary eyebrow mb-2">Browse</p>
          <h2 class="section-title text-2xl md:text-3xl">By category</h2>
        </div>
        <NuxtLink
          to="/projects"
          class="text-muted-foreground hover:text-foreground inline-flex items-center gap-1 text-sm font-semibold"
        >
          View all
          <ArrowRight class="size-4" />
        </NuxtLink>
      </div>

      <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <NuxtLink
          v-for="tile in categoryTiles"
          :key="tile.label"
          :to="tile.to"
          class="category-tile flex h-44 flex-col justify-end p-5"
          :style="{ background: tile.gradient }"
        >
          <component
            :is="tile.icon"
            class="relative z-10 mb-2 size-7 text-white drop-shadow"
            :stroke-width="2"
          />
          <span class="relative z-10 text-xl font-bold text-white">
            {{ tile.label }}
          </span>
          <span class="relative z-10 text-sm font-medium text-white/80">
            {{ tile.hint }}
          </span>
        </NuxtLink>
      </div>
    </section>

    <section class="mx-auto max-w-6xl px-6 py-16">
      <div class="mb-10">
        <p class="text-primary eyebrow mb-2">Why Beacon</p>
        <h2 class="section-title text-2xl md:text-3xl">
          Everything in one place
        </h2>
        <p class="text-muted-foreground mt-2 max-w-2xl">
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
          <span class="icon-chip mb-4 size-11">
            <component :is="feature.icon" class="size-5" :stroke-width="2" />
          </span>
          <h3 class="mb-2 text-lg font-semibold">{{ feature.title }}</h3>
          <p class="text-muted-foreground text-sm">
            {{ feature.description }}
          </p>
        </div>
      </div>
    </section>

    <section class="mx-auto max-w-6xl px-6 py-16">
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
            <h2 class="section-title mb-4 text-2xl md:text-3xl">
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
              <span class="icon-chip size-11">
                <component :is="stat.icon" class="size-5" :stroke-width="2" />
              </span>
              <div>
                <p class="font-semibold">{{ stat.label }}</p>
                <p class="text-muted-foreground text-sm">{{ stat.value }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <section class="mx-auto max-w-6xl px-6 py-16">
      <div class="mb-8 flex items-end justify-between gap-4">
        <div>
          <p class="text-primary eyebrow mb-2">Stay in the loop</p>
          <h2 class="section-title text-2xl md:text-3xl">Latest news</h2>
        </div>
        <NuxtLink
          to="/news"
          class="text-muted-foreground hover:text-foreground inline-flex items-center gap-1 text-sm font-semibold"
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
            class="bg-primary/15 text-primary mb-4 w-fit rounded-full px-2.5 py-0.5 text-xs font-semibold uppercase tracking-wide"
          >
            {{ post.tag }}
          </span>
          <h3 class="mb-2 text-lg font-semibold">{{ post.title }}</h3>
          <p class="text-muted-foreground text-sm">{{ post.excerpt }}</p>
        </article>
      </div>
    </section>

    <section class="cta-band relative mt-8 overflow-hidden px-6 py-20">
      <div class="relative z-10 mx-auto max-w-3xl text-center">
        <p class="eyebrow mb-3 text-amber-950/70">Join Beacon</p>
        <h2 class="section-title mb-4 text-3xl text-amber-950 md:text-5xl">
          Ready to get started?
        </h2>
        <p class="mx-auto mb-8 max-w-xl text-base text-amber-950/80 md:text-lg">
          Discover Bedrock content, publish your own, or contribute to a
          project. Free to use, open by default, and for a good cause.
        </p>
        <div class="flex flex-wrap justify-center gap-3">
          <Button
            as-child
            size="lg"
            class="bg-amber-950 text-amber-50 shadow-lg hover:bg-amber-900"
          >
            <NuxtLink to="/register">
              Create an account
              <ArrowRight />
            </NuxtLink>
          </Button>
          <Button
            as-child
            size="lg"
            variant="outline"
            class="border-amber-950/30 bg-white/20 text-amber-950 hover:bg-white/35"
          >
            <NuxtLink to="/projects">Browse projects</NuxtLink>
          </Button>
        </div>
      </div>
    </section>
  </div>
</template>
