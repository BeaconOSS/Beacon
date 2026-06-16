<script setup lang="ts">
import {
  ArrowLeft,
  BarChart3,
  CircleAlert,
  CircleCheck,
  Coins,
  FileText,
  Globe,
  Images,
  Info,
  Link2,
  Loader2,
  Lock,
  Package,
  Scale,
  Send,
  Settings,
  Tags,
  Trash2,
  TriangleAlert,
  Upload,
  Users,
} from "@lucide/vue";
import type { Component } from "vue";
import { toast } from "vue-sonner";
import {
  useProjectSettings,
  type ProjectVisibility,
} from "~/scripts/pages/projects";
import { useVersions } from "~/scripts/pages/projects/versions";
import { useGallery } from "~/scripts/pages/projects/gallery";

const route = useRoute();
const slug = computed(() => String(route.params.slug ?? ""));
const username = computed(() => String(route.params.username ?? ""));

const {
  project,
  error,
  pending,
  load,
  form,
  saving,
  saveError,
  iconPending,
  iconError,
  iconUrl,
  dirty,
  save,
  uploadIcon,
  removeIcon,
} = useProjectSettings(slug.value);
const { versions, load: loadVersions } = useVersions(slug.value);
const { images, load: loadGallery } = useGallery(slug.value);

await Promise.all([load(), loadVersions(), loadGallery()]);

const VISIBILITY_OPTIONS: {
  value: ProjectVisibility;
  label: string;
  description: string;
  icon: Component;
}[] = [
  {
    value: "public",
    label: "Public",
    description: "Anyone can find and view it.",
    icon: Globe,
  },
  {
    value: "unlisted",
    label: "Unlisted",
    description: "Only people with the link can view it.",
    icon: Link2,
  },
  {
    value: "private",
    label: "Private",
    description: "Only members can view it.",
    icon: Lock,
  },
];

const iconInput = ref<HTMLInputElement | null>(null);
const monetizationEnabled = ref(false);

const formSummaryLength = computed(() => form.summary.trim().length);

function pickIcon() {
  iconInput.value?.click();
}

async function onIconChange(event: Event) {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (file) {
    await uploadIcon(file);
  }
  input.value = "";
}

async function handleSave() {
  await save();
  if (!saveError.value) {
    toast.success("Project updated.");
  }
}

type SectionId =
  | "general"
  | "tags"
  | "description"
  | "versions"
  | "license"
  | "gallery"
  | "links"
  | "members"
  | "analytics";

interface NavItem {
  id: SectionId;
  label: string;
  icon: Component;
}

const NAV_ITEMS: NavItem[] = [
  { id: "general", label: "General", icon: Settings },
  { id: "tags", label: "Tags", icon: Tags },
  { id: "description", label: "Description", icon: FileText },
  { id: "versions", label: "Versions", icon: Package },
  { id: "license", label: "License", icon: Scale },
  { id: "gallery", label: "Gallery", icon: Images },
  { id: "links", label: "Links", icon: Link2 },
  { id: "members", label: "Members", icon: Users },
  { id: "analytics", label: "Analytics", icon: BarChart3 },
];

const activeSection = ref<SectionId>("general");

const SECTION_PLACEHOLDERS: Record<
  Exclude<SectionId, "general">,
  { title: string; description: string }
> = {
  tags: {
    title: "Tags",
    description:
      "Pick the categories that best describe your project so people can find it.",
  },
  description: {
    title: "Description",
    description:
      "Write a rich description that clearly explains your project's purpose and features.",
  },
  versions: {
    title: "Versions",
    description: "Upload and manage the downloadable versions of your project.",
  },
  license: {
    title: "License",
    description: "Choose the license your project is distributed under.",
  },
  gallery: {
    title: "Gallery",
    description: "Showcase screenshots and renders of your project.",
  },
  links: {
    title: "Links",
    description:
      "Add external links such as source code, an issue tracker, or a Discord invite.",
  },
  members: {
    title: "Members",
    description: "Invite collaborators and manage their roles.",
  },
  analytics: {
    title: "Analytics",
    description: "Track downloads, views, and engagement over time.",
  },
};

type ChecklistLevel = "required" | "warning" | "suggestion";

interface ChecklistItem {
  level: ChecklistLevel;
  title: string;
  description: string;
  complete: boolean;
}

const summaryLength = computed(
  () => (project.value?.summary ?? "").trim().length,
);

const checklist = computed<ChecklistItem[]>(() => {
  const p = project.value;
  const summary = (p?.summary ?? "").trim();
  const title = (p?.title ?? "").trim();
  return [
    {
      level: "required",
      title: "Upload a version",
      description:
        "At least one version is required before a project can be submitted for review.",
      complete: versions.value.length > 0,
    },
    {
      level: "required",
      title: "Add a description",
      description:
        "A description that clearly explains the project's purpose and function is required.",
      complete: (p?.description ?? "").trim().length > 0,
    },
    {
      level: "required",
      title: "Select a license",
      description: "Select the license your project is distributed under.",
      complete: false,
    },
    {
      level: "required",
      title: "Make the summary unique",
      description:
        "Your summary can't be the same as your project's name. Create an informative, enticing summary.",
      complete:
        summary.length > 0 && summary.toLowerCase() !== title.toLowerCase(),
    },
    {
      level: "warning",
      title: "Expand the summary",
      description: `Your summary is ${summaryLength.value} characters. At least 30 characters is recommended.`,
      complete: summaryLength.value >= 30,
    },
    {
      level: "suggestion",
      title: "Add an icon",
      description:
        "A unique, relevant icon makes your project identifiable and helps it stand out.",
      complete: Boolean(p?.icon_url),
    },
    {
      level: "suggestion",
      title: "Feature a gallery image",
      description:
        "The featured gallery image is often how your project makes its first impression.",
      complete: images.value.length > 0,
    },
    {
      level: "suggestion",
      title: "Add external links",
      description:
        "Add relevant links outside of Beacon, such as source code, an issue tracker, or a Discord invite.",
      complete: false,
    },
  ];
});

const requiredItems = computed(() =>
  checklist.value.filter((item) => item.level === "required"),
);
const requiredComplete = computed(
  () => requiredItems.value.filter((item) => item.complete).length,
);
const canSubmit = computed(
  () => requiredComplete.value === requiredItems.value.length,
);

const LEVEL_STYLES: Record<
  ChecklistLevel,
  { icon: Component; label: string; tone: string; pill: string; accent: string }
> = {
  required: {
    icon: CircleAlert,
    label: "Required",
    tone: "text-destructive",
    pill: "bg-destructive/10 text-destructive",
    accent: "border-l-destructive",
  },
  warning: {
    icon: TriangleAlert,
    label: "Warning",
    tone: "text-amber-500",
    pill: "bg-amber-500/10 text-amber-500",
    accent: "border-l-amber-500",
  },
  suggestion: {
    icon: Info,
    label: "Suggestion",
    tone: "text-muted-foreground",
    pill: "bg-muted text-muted-foreground",
    accent: "border-l-border",
  },
};

const outstandingItems = computed(() =>
  checklist.value.filter((item) => !item.complete),
);
const completedItems = computed(() =>
  checklist.value.filter((item) => item.complete),
);

function submitForReview() {
  toast.info("Submitting for review isn't available yet - coming soon.");
}
</script>

<template>
  <div class="page-canvas">
    <div class="mx-auto max-w-6xl px-6 py-12">
      <NuxtLink
        :to="`/projects/${slug}`"
        class="text-muted-foreground hover:text-foreground mb-6 inline-flex items-center gap-1.5 text-sm transition-colors"
      >
        <ArrowLeft class="size-4" />
        Back to project
      </NuxtLink>

      <p v-if="pending" class="text-muted-foreground py-12 text-center">
        Loading project...
      </p>
      <p v-else-if="error" class="text-destructive py-12 text-center">
        {{ error }}
      </p>

      <template v-else-if="project">
        <header class="mb-8">
          <p class="text-primary eyebrow mb-2">Project settings</p>
          <h1 class="display-heading text-3xl md:text-4xl">
            {{ project.title }}
          </h1>
          <p class="text-muted-foreground mt-1 text-sm">
            @{{ username }} / {{ slug }}
          </p>
        </header>

        <div class="flex flex-col gap-8 lg:flex-row">
          <aside class="shrink-0 lg:w-56">
            <nav class="flex flex-col gap-0.5 lg:sticky lg:top-24">
              <button
                v-for="item in NAV_ITEMS"
                :key="item.id"
                type="button"
                class="flex items-center gap-2.5 rounded-lg px-3 py-2 text-left text-sm font-medium transition-colors"
                :class="
                  activeSection === item.id
                    ? 'bg-primary/10 text-primary'
                    : 'text-muted-foreground hover:bg-accent/40 hover:text-foreground'
                "
                @click="activeSection = item.id"
              >
                <component :is="item.icon" class="size-4" />
                {{ item.label }}
              </button>
            </nav>
          </aside>

          <div class="min-w-0 flex-1 space-y-8">
            <section class="card-glass rounded-2xl p-6">
              <div class="mb-5 flex items-start justify-between gap-4">
                <div>
                  <h2 class="section-title text-lg">Publish checklist</h2>
                  <p class="text-muted-foreground mt-1 text-sm">
                    Complete every required item before submitting for review.
                  </p>
                </div>
                <span
                  class="shrink-0 rounded-full px-3 py-1 text-xs font-semibold"
                  :class="
                    canSubmit
                      ? 'bg-primary/15 text-primary'
                      : 'bg-destructive/10 text-destructive'
                  "
                >
                  {{ requiredComplete }} / {{ requiredItems.length }} required
                </span>
              </div>

              <ul v-if="outstandingItems.length" class="space-y-2.5">
                <li
                  v-for="item in outstandingItems"
                  :key="item.title"
                  class="bg-muted/30 flex items-start gap-3 rounded-xl border-l-2 p-3.5"
                  :class="LEVEL_STYLES[item.level].accent"
                >
                  <component
                    :is="LEVEL_STYLES[item.level].icon"
                    class="mt-0.5 size-5 shrink-0"
                    :class="LEVEL_STYLES[item.level].tone"
                  />
                  <div class="min-w-0 flex-1">
                    <div class="flex flex-wrap items-center gap-2">
                      <span class="text-foreground text-sm font-semibold">
                        {{ item.title }}
                      </span>
                      <span
                        class="rounded-full px-1.5 py-0.5 text-[10px] font-semibold tracking-wide uppercase"
                        :class="LEVEL_STYLES[item.level].pill"
                      >
                        {{ LEVEL_STYLES[item.level].label }}
                      </span>
                    </div>
                    <p
                      class="text-muted-foreground mt-1 text-xs leading-relaxed"
                    >
                      {{ item.description }}
                    </p>
                  </div>
                </li>
              </ul>

              <div
                v-if="completedItems.length"
                class="mt-4 flex flex-wrap gap-2"
              >
                <span
                  v-for="item in completedItems"
                  :key="item.title"
                  class="bg-primary/10 text-primary inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium"
                >
                  <CircleCheck class="size-3.5" />
                  {{ item.title }}
                </span>
              </div>

              <div
                v-if="!outstandingItems.length"
                class="text-primary mt-2 flex items-center gap-2 text-sm font-medium"
              >
                <CircleCheck class="size-5" />
                Everything looks ready - submit your project for review.
              </div>

              <div
                class="mt-6 flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center sm:justify-between"
              >
                <p class="text-muted-foreground text-xs">
                  Your project is only visible to its members until it's
                  reviewed and published by moderators.
                </p>
                <Button
                  class="btn-glow shrink-0"
                  :disabled="!canSubmit"
                  @click="submitForReview"
                >
                  <Send class="size-4" />
                  Submit for review
                </Button>
              </div>
            </section>

            <section v-if="activeSection === 'general'" class="space-y-6">
              <div class="card-glass rounded-2xl p-6">
                <h2 class="section-title mb-1 text-lg">Project information</h2>
                <p class="text-muted-foreground mb-6 text-sm">
                  The core details people see across Beacon.
                </p>

                <div class="space-y-6">
                  <div class="space-y-2">
                    <Label for="project-name">Name</Label>
                    <Input
                      id="project-name"
                      v-model="form.title"
                      placeholder="My awesome project"
                    />
                  </div>

                  <div class="space-y-2">
                    <Label for="project-url">URL</Label>
                    <div
                      class="border-input bg-background focus-within:border-ring focus-within:ring-ring/50 flex items-center rounded-md border text-sm transition-[color,box-shadow] focus-within:ring-3"
                    >
                      <span
                        class="text-muted-foreground shrink-0 pl-3 select-none"
                      >
                        /{{ username }}/
                      </span>
                      <input
                        id="project-url"
                        v-model="form.urlSlug"
                        class="placeholder:text-muted-foreground min-w-0 flex-1 bg-transparent py-2 pr-3 pl-0.5 outline-none"
                        placeholder="my-awesome-project"
                      />
                    </div>
                    <p class="text-muted-foreground text-xs">
                      Changing the URL will redirect you to the new address.
                    </p>
                  </div>

                  <div class="space-y-2">
                    <Label for="project-summary">Summary</Label>
                    <Textarea
                      id="project-summary"
                      v-model="form.summary"
                      :rows="3"
                      placeholder="A short, enticing description of your project."
                    />
                    <p
                      class="text-xs"
                      :class="
                        formSummaryLength >= 30
                          ? 'text-muted-foreground'
                          : 'text-amber-500'
                      "
                    >
                      {{ formSummaryLength }} characters - at least 30 is
                      recommended.
                    </p>
                  </div>

                  <div class="space-y-2">
                    <Label>Icon</Label>
                    <div class="flex items-center gap-4">
                      <div
                        class="border-border/60 bg-muted/40 flex size-20 shrink-0 items-center justify-center overflow-hidden rounded-xl border"
                      >
                        <img
                          v-if="iconUrl"
                          :src="iconUrl"
                          alt="Project icon"
                          class="size-full object-cover"
                        />
                        <Package v-else class="text-muted-foreground size-7" />
                      </div>
                      <div class="space-y-2">
                        <div class="flex flex-wrap gap-2">
                          <Button
                            type="button"
                            variant="outline"
                            size="sm"
                            :disabled="iconPending"
                            @click="pickIcon"
                          >
                            <Loader2
                              v-if="iconPending"
                              class="size-4 animate-spin"
                            />
                            <Upload v-else class="size-4" />
                            {{ iconUrl ? "Replace" : "Upload" }}
                          </Button>
                          <Button
                            v-if="iconUrl"
                            type="button"
                            variant="ghost"
                            size="sm"
                            :disabled="iconPending"
                            @click="removeIcon"
                          >
                            <Trash2 class="size-4" />
                            Remove
                          </Button>
                        </div>
                        <p class="text-muted-foreground text-xs">
                          Optional. PNG, JPG, WEBP, or GIF.
                        </p>
                        <p v-if="iconError" class="text-destructive text-xs">
                          {{ iconError }}
                        </p>
                      </div>
                      <input
                        ref="iconInput"
                        type="file"
                        accept="image/png,image/jpeg,image/webp,image/gif"
                        class="hidden"
                        @change="onIconChange"
                      />
                    </div>
                  </div>

                  <div class="space-y-2">
                    <Label>Visibility</Label>
                    <div class="grid gap-3 sm:grid-cols-3">
                      <button
                        v-for="option in VISIBILITY_OPTIONS"
                        :key="option.value"
                        type="button"
                        class="flex flex-col gap-1.5 rounded-xl border p-3 text-left transition-colors"
                        :class="
                          form.visibility === option.value
                            ? 'border-primary bg-primary/5'
                            : 'border-border hover:border-border/80 hover:bg-accent/30'
                        "
                        @click="form.visibility = option.value"
                      >
                        <component
                          :is="option.icon"
                          class="size-4"
                          :class="
                            form.visibility === option.value
                              ? 'text-primary'
                              : 'text-muted-foreground'
                          "
                        />
                        <span class="text-foreground text-sm font-semibold">
                          {{ option.label }}
                        </span>
                        <span class="text-muted-foreground text-xs">
                          {{ option.description }}
                        </span>
                      </button>
                    </div>
                  </div>

                  <div
                    class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center"
                    :class="saveError ? 'sm:justify-between' : 'sm:justify-end'"
                  >
                    <p v-if="saveError" class="text-destructive text-sm">
                      {{ saveError }}
                    </p>
                    <Button
                      class="btn-glow shrink-0"
                      :disabled="!dirty || saving"
                      @click="handleSave"
                    >
                      <Loader2 v-if="saving" class="size-4 animate-spin" />
                      Save changes
                    </Button>
                  </div>
                </div>
              </div>

              <div class="card-glass rounded-2xl p-6">
                <div class="flex items-start justify-between gap-4">
                  <div>
                    <h2
                      class="section-title mb-1 flex items-center gap-2 text-lg"
                    >
                      <Coins class="text-primary size-5" />
                      Monetization
                    </h2>
                    <p class="text-muted-foreground text-sm">
                      Earn revenue from your project.
                    </p>
                  </div>
                  <Switch v-model="monetizationEnabled" class="mt-1 shrink-0" />
                </div>
              </div>

              <div class="border-destructive/40 rounded-2xl border p-6">
                <h2 class="text-destructive mb-1 text-lg font-semibold">
                  Delete project
                </h2>
                <p class="text-muted-foreground mb-4 text-sm">
                  Removes your project from Beacon's servers and search. This
                  action is permanent, so be extra careful.
                </p>
                <Button variant="destructive" disabled>Delete project</Button>
              </div>
            </section>

            <section v-else class="card-glass rounded-2xl p-6">
              <h2 class="section-title mb-1 text-lg">
                {{ SECTION_PLACEHOLDERS[activeSection].title }}
              </h2>
              <p class="text-muted-foreground mb-6 text-sm">
                {{ SECTION_PLACEHOLDERS[activeSection].description }}
              </p>
            </section>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>
