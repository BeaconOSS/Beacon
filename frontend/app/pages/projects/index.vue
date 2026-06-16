<script setup lang="ts">
import {
  Clock,
  Download,
  Globe,
  Heart,
  LayoutGrid,
  Package,
  Palette,
  Rows3,
  Search,
  Shirt,
} from "@lucide/vue";
import { watchDebounced } from "@vueuse/core";
import type { Component } from "vue";
import {
  useProjects,
  useCategoryFilters,
  projectTypeLabel,
  PROJECT_TYPES,
} from "~/scripts/pages/projects";
import { useSettings } from "~/scripts/settings";

const route = useRoute();
const { settings } = useSettings();
const config = useRuntimeConfig();

function iconSrc(iconUrl?: string | null): string | null {
  return iconUrl ? `${config.public.apiBase}${iconUrl}` : null;
}

const { projects, error, pending, load } = useProjects();
const { categories, load: loadCategories } = useCategoryFilters();

function queryString(value: unknown): string {
  return typeof value === "string" ? value : "";
}

const selectedType = ref(queryString(route.query.type) || "addon");
const selectedCategory = ref(queryString(route.query.category));
const searchTerm = ref(queryString(route.query.q));

await Promise.all([
  load(
    selectedCategory.value || undefined,
    searchTerm.value.trim() || undefined,
  ),
  loadCategories(),
]);

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

function relativeTime(iso: string): string {
  const then = new Date(iso).getTime();
  if (Number.isNaN(then)) return "";
  const sec = Math.floor((Date.now() - then) / 1000);
  if (sec < 60) return "just now";
  const min = Math.floor(sec / 60);
  if (min < 60) return `${min}m ago`;
  const hr = Math.floor(min / 60);
  if (hr < 24) return `${hr}h ago`;
  const day = Math.floor(hr / 24);
  if (day < 30) return `${day}d ago`;
  const mon = Math.floor(day / 30);
  if (mon < 12) return `${mon}mo ago`;
  return `${Math.floor(day / 365)}y ago`;
}

const visibleProjects = computed(() =>
  selectedType.value
    ? projects.value.filter((p) => p.project_type === selectedType.value)
    : projects.value,
);

const hasFilters = computed(
  () => !!selectedCategory.value || !!searchTerm.value.trim(),
);

async function reload() {
  await load(
    selectedCategory.value || undefined,
    searchTerm.value.trim() || undefined,
  );
}

watchDebounced(searchTerm, reload, { debounce: 300 });

function selectType(value: string) {
  selectedType.value = value;
}

async function selectCategory(slug: string) {
  if (selectedCategory.value === slug) return;
  selectedCategory.value = slug;
  await reload();
}

async function clearAll() {
  selectedCategory.value = "";
  searchTerm.value = "";
  await reload();
}
</script>

<template>
  <div class="page-canvas">
    <div class="mx-auto max-w-6xl px-6 py-12">
      <header class="mb-8 max-w-2xl">
        <p class="text-primary eyebrow mb-2">Discover</p>
        <h1 class="display-heading mb-3 text-4xl md:text-5xl">
          Browse projects
        </h1>
        <p class="text-muted-foreground">
          Find Add-Ons, worlds, resource packs, and skins for Minecraft Bedrock
          - built by the community.
        </p>
      </header>

      <div class="mb-6 flex flex-wrap gap-2">
        <button
          v-for="type in PROJECT_TYPES"
          :key="type.value"
          type="button"
          class="inline-flex items-center gap-2 rounded-xl border px-4 py-2.5 text-sm font-semibold transition-colors"
          :class="
            selectedType === type.value
              ? 'border-primary/70 bg-primary/10 text-primary'
              : 'border-border text-muted-foreground hover:border-primary/60 hover:text-foreground'
          "
          @click="selectType(type.value)"
        >
          <component
            :is="typeStyle(type.value).icon"
            class="size-4"
            :stroke-width="2.25"
          />
          {{ type.label }}
        </button>
      </div>

      <div class="relative mb-8 max-w-2xl">
        <Search
          class="text-muted-foreground pointer-events-none absolute top-1/2 left-4 size-5 -translate-y-1/2"
        />
        <input
          v-model="searchTerm"
          type="search"
          placeholder="Search projects..."
          class="border-border bg-card/70 focus:border-primary focus:ring-primary/30 h-12 w-full rounded-full border py-3 pr-4 pl-12 text-base shadow-sm backdrop-blur outline-none focus:ring-4"
        />
      </div>

      <div class="flex flex-col gap-8 lg:flex-row">
        <aside
          v-if="categories.length"
          class="shrink-0 lg:w-56"
          :class="{ 'lg:order-2': settings.filtersSidebarRight }"
        >
          <div class="lg:sticky lg:top-24">
            <p class="text-muted-foreground eyebrow mb-3">Category</p>
            <div class="flex flex-col gap-0.5">
              <button
                type="button"
                class="rounded-lg px-3 py-1.5 text-left text-sm font-medium transition-colors"
                :class="
                  selectedCategory === ''
                    ? 'bg-primary/10 text-primary'
                    : 'text-muted-foreground hover:bg-accent/40 hover:text-foreground'
                "
                @click="selectCategory('')"
              >
                All categories
              </button>
              <button
                v-for="category in categories"
                :key="category.slug"
                type="button"
                class="rounded-lg px-3 py-1.5 text-left text-sm font-medium transition-colors"
                :class="
                  selectedCategory === category.slug
                    ? 'bg-primary/10 text-primary'
                    : 'text-muted-foreground hover:bg-accent/40 hover:text-foreground'
                "
                @click="selectCategory(category.slug)"
              >
                {{ category.name }}
              </button>
            </div>
          </div>
        </aside>

        <div class="min-w-0 flex-1">
          <div class="mb-4 flex items-center justify-between gap-4">
            <div
              class="border-border flex items-center gap-1 rounded-lg border p-1"
            >
              <button
                type="button"
                aria-label="Grid layout"
                class="rounded-md p-1.5 transition-colors"
                :class="
                  settings.listLayout === 'grid'
                    ? 'bg-primary/15 text-primary'
                    : 'text-muted-foreground hover:text-foreground'
                "
                @click="settings.listLayout = 'grid'"
              >
                <LayoutGrid class="size-4" />
              </button>
              <button
                type="button"
                aria-label="Rows layout"
                class="rounded-md p-1.5 transition-colors"
                :class="
                  settings.listLayout === 'rows'
                    ? 'bg-primary/15 text-primary'
                    : 'text-muted-foreground hover:text-foreground'
                "
                @click="settings.listLayout = 'rows'"
              >
                <Rows3 class="size-4" />
              </button>
            </div>

            <p class="text-muted-foreground text-sm">
              <span class="text-foreground font-semibold">{{
                visibleProjects.length
              }}</span>
              {{ visibleProjects.length === 1 ? "project" : "projects" }}
              <button
                v-if="hasFilters"
                type="button"
                class="text-primary ml-2 hover:underline"
                @click="clearAll"
              >
                Clear filters
              </button>
            </p>
          </div>

          <p v-if="pending" class="text-muted-foreground py-12 text-center">
            Loading projects...
          </p>
          <p v-else-if="error" class="text-destructive py-12 text-center">
            {{ error }}
          </p>
          <div
            v-else-if="visibleProjects.length === 0"
            class="border-border text-muted-foreground rounded-xl border border-dashed py-16 text-center"
          >
            <p class="mb-1 font-medium">
              {{
                hasFilters
                  ? "No projects match your filters."
                  : "No projects yet."
              }}
            </p>
            <button
              v-if="hasFilters"
              type="button"
              class="text-primary text-sm hover:underline"
              @click="clearAll"
            >
              Clear filters
            </button>
          </div>

          <ul
            v-else
            :class="
              settings.listLayout === 'grid'
                ? 'grid gap-4 sm:grid-cols-2 xl:grid-cols-3'
                : 'flex flex-col gap-3'
            "
          >
            <li v-for="project in visibleProjects" :key="project.id">
              <NuxtLink
                :to="`/projects/${project.slug}`"
                class="card-glass group flex h-full gap-4 rounded-xl p-4"
              >
                <span
                  class="icon-chip size-12 shrink-0 overflow-hidden rounded-xl"
                  :style="
                    iconSrc(project.icon_url)
                      ? {}
                      : { background: typeStyle(project.project_type).gradient }
                  "
                >
                  <img
                    v-if="iconSrc(project.icon_url)"
                    :src="iconSrc(project.icon_url)!"
                    :alt="project.title"
                    class="size-full object-cover"
                  />
                  <component
                    :is="typeStyle(project.project_type).icon"
                    v-else
                    class="size-6"
                    :stroke-width="2.25"
                  />
                </span>

                <template v-if="settings.listLayout === 'rows'">
                  <div class="min-w-0 flex-1">
                    <h2 class="truncate font-semibold">
                      <span class="group-hover:text-primary transition-colors">
                        {{ project.title }}
                      </span>
                      <span class="text-muted-foreground font-normal">
                        by {{ project.owner }}
                      </span>
                    </h2>
                    <p class="text-muted-foreground line-clamp-2 text-sm">
                      {{ project.summary }}
                    </p>
                    <div class="mt-2 flex flex-wrap items-center gap-1.5">
                      <span
                        class="border-border text-muted-foreground rounded-full border px-2 py-0.5 text-xs font-medium"
                      >
                        {{ projectTypeLabel(project.project_type) }}
                      </span>
                      <span
                        v-for="cat in project.categories"
                        :key="cat.slug"
                        class="bg-muted text-muted-foreground rounded-full px-2 py-0.5 text-xs"
                      >
                        {{ cat.name }}
                      </span>
                    </div>
                  </div>
                  <div
                    class="text-muted-foreground flex shrink-0 flex-col items-end gap-2 text-sm"
                  >
                    <div class="flex items-center gap-4">
                      <span class="inline-flex items-center gap-1.5">
                        <Download class="size-4" />
                        {{ project.download_count.toLocaleString() }}
                      </span>
                      <span class="inline-flex items-center gap-1.5">
                        <Heart class="size-4" />
                        0
                      </span>
                    </div>
                    <span
                      v-if="project.updated_at"
                      class="inline-flex items-center gap-1.5 text-xs"
                    >
                      <Clock class="size-3.5" />
                      Updated {{ relativeTime(project.updated_at) }}
                    </span>
                  </div>
                </template>

                <div v-else class="min-w-0 flex-1">
                  <h2
                    class="group-hover:text-primary truncate font-semibold transition-colors"
                  >
                    {{ project.title }}
                  </h2>
                  <p class="text-muted-foreground line-clamp-2 text-sm">
                    {{ project.summary }}
                  </p>
                  <div
                    class="text-muted-foreground mt-2 flex items-center gap-3 text-xs"
                  >
                    <span
                      class="border-border rounded-full border px-2 py-0.5 font-medium"
                    >
                      {{ projectTypeLabel(project.project_type) }}
                    </span>
                    <span class="inline-flex items-center gap-1">
                      <Download class="size-3.5" />
                      {{ project.download_count }}
                    </span>
                  </div>
                </div>
              </NuxtLink>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>
