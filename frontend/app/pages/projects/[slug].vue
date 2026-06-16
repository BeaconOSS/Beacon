<script setup lang="ts">
import type { Component } from "vue";
import { useClipboard } from "@vueuse/core";
import { toast } from "vue-sonner";
import {
  ArrowLeft,
  Bookmark,
  Calendar,
  ChevronDown,
  Clock,
  Copy,
  Download,
  ExternalLink,
  Flag,
  Globe,
  Heart,
  Link2,
  Loader2,
  MessageSquareWarning,
  MoreHorizontal,
  Package,
  Palette,
  Ban,
  CircleCheck,
  Pencil,
  Scale,
  ShieldCheck,
  Shirt,
  Users,
} from "@lucide/vue";
import { useProject, projectTypeLabel } from "~/scripts/pages/projects";
import {
  useVersions,
  formatFileSize,
  VERSION_CHANNELS,
} from "~/scripts/pages/projects/versions";
import { useGallery } from "~/scripts/pages/projects/gallery";
import { useProjectInteractions } from "~/scripts/pages/projects/interactions";
import type { Version } from "~/scripts/pages/projects/types";
import {
  useProjectReview,
  useProjectPendingReview,
} from "~/scripts/pages/moderation";
import { useAuth } from "~/scripts/auth";
import { useSettings } from "~/scripts/settings";

const route = useRoute();
const slug = computed(() => String(route.params.slug ?? ""));

const { user, isModerator } = useAuth();
const { settings } = useSettings();

const { project, error, pending, load: loadProject } = useProject(slug.value);
const { versions, load: loadVersions, downloadUrl } = useVersions(slug.value);
const { images, load: loadGallery } = useGallery(slug.value);
const { heartPending, savePending, toggleHeart, toggleSave } =
  useProjectInteractions(slug.value, project);

await Promise.all([loadProject(), loadVersions(), loadGallery()]);

const isOwner = computed(
  () =>
    !!user.value &&
    !!project.value &&
    user.value.username === project.value.owner,
);

const projectLinks = computed(() => {
  const p = project.value;
  if (!p) return [] as { label: string; url: string }[];
  return [
    { label: "Website", url: p.website_url },
    { label: "Source code", url: p.source_url },
    { label: "Issue tracker", url: p.issues_url },
    { label: "Wiki", url: p.wiki_url },
    { label: "Discord", url: p.discord_url },
  ].filter((link): link is { label: string; url: string } => Boolean(link.url));
});

interface TypeStyle {
  icon: Component;
  gradient: string;
}

const TYPE_STYLES: Record<string, TypeStyle> = {
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

const typeStyle = computed<TypeStyle>(
  () =>
    TYPE_STYLES[project.value?.project_type ?? "addon"] ?? TYPE_STYLES.addon!,
);

const config = useRuntimeConfig();
const iconSrc = computed(() => {
  const path = project.value?.icon_url;
  return path ? `${config.public.apiBase}${path}` : null;
});

const downloadableVersions = computed(() =>
  versions.value.filter((v) => v.file),
);

const latestVersion = computed<Version | null>(
  () =>
    downloadableVersions.value.find((v) => v.channel === "release") ??
    downloadableVersions.value[0] ??
    null,
);

const changelogEntries = computed(() =>
  versions.value.filter((v) => v.changelog && v.changelog.trim().length > 0),
);

function relativeTime(iso: string): string {
  const then = new Date(iso).getTime();
  if (Number.isNaN(then)) return "";
  const sec = Math.floor((Date.now() - then) / 1000);
  if (sec < 60) return "just now";
  const min = Math.floor(sec / 60);
  if (min < 60) return `${min} minute${min === 1 ? "" : "s"} ago`;
  const hr = Math.floor(min / 60);
  if (hr < 24) return `${hr} hour${hr === 1 ? "" : "s"} ago`;
  const day = Math.floor(hr / 24);
  if (day < 30) return `${day} day${day === 1 ? "" : "s"} ago`;
  const mon = Math.floor(day / 30);
  if (mon < 12) return `${mon} month${mon === 1 ? "" : "s"} ago`;
  const yr = Math.floor(day / 365);
  return `${yr} year${yr === 1 ? "" : "s"} ago`;
}

const channelLabel = (value: string) =>
  VERSION_CHANNELS.find((c) => c.value === value)?.label ?? value;

const { copy } = useClipboard();

async function copyId() {
  if (!project.value) return;
  await copy(project.value.id);
  toast.success("Project ID copied to clipboard");
}

async function copyLink() {
  await copy(window.location.href);
  toast.success("Link copied to clipboard");
}

async function handleHeart() {
  if (!user.value) {
    toast.info("Sign in to heart this project.");
    await navigateTo("/login");
    return;
  }
  try {
    await toggleHeart();
  } catch (err) {
    toast.error((err as Error).message);
  }
}

async function handleSave() {
  if (!user.value) {
    toast.info("Sign in to save this project.");
    await navigateTo("/login");
    return;
  }
  try {
    await toggleSave();
    toast.success(
      project.value?.viewer_saved
        ? "Saved to your library."
        : "Removed from your saved projects.",
    );
  } catch (err) {
    toast.error((err as Error).message);
  }
}

const reportOpen = ref(false);
const reportReason = ref("");

function submitReport() {
  reportOpen.value = false;
  reportReason.value = "";
  toast.info(
    "Reporting isn't available yet - thanks, we've noted your interest.",
  );
}

const {
  submitting: reviewSubmitting,
  error: reviewError,
  review,
} = useProjectReview(slug.value);
const reviewNotes = ref("");
const showModeration = computed(
  () => isModerator.value && project.value?.status === "in_review",
);

const showOwnerPending = computed(
  () => isOwner.value && project.value?.has_pending_changes === true,
);

const {
  data: pendingReview,
  withBase: pendingWithBase,
  load: loadPendingReview,
} = useProjectPendingReview(slug.value);

watch(
  showModeration,
  (value) => {
    if (value && !pendingReview.value) {
      void loadPendingReview();
    }
  },
  { immediate: true },
);

interface FieldDiff {
  label: string;
  before: string;
  after: string;
  changed: boolean;
}

const reviewDiffs = computed<FieldDiff[]>(() => {
  const data = pendingReview.value;
  if (!data) return [];
  const before = data.published;
  const after = data.pending;
  const fields: { label: string; key: keyof typeof after }[] = [
    { label: "Title", key: "title" },
    { label: "Summary", key: "summary" },
    { label: "Description", key: "description" },
    { label: "License", key: "license" },
  ];
  const rows: FieldDiff[] = fields.map((field) => {
    const beforeValue = before ? String(before[field.key] ?? "") : "";
    const afterValue = String(after[field.key] ?? "");
    return {
      label: field.label,
      before: beforeValue,
      after: afterValue,
      changed: !data.is_first_review && beforeValue !== afterValue,
    };
  });
  const beforeCategories = before ? before.categories.join(", ") : "";
  const afterCategories = after.categories.join(", ");
  rows.push({
    label: "Categories",
    before: beforeCategories,
    after: afterCategories,
    changed: !data.is_first_review && beforeCategories !== afterCategories,
  });
  return rows;
});

async function handleReview(action: "approve" | "reject" | "request_changes") {
  if (
    (action === "reject" || action === "request_changes") &&
    !reviewNotes.value.trim()
  ) {
    toast.error("Please add notes explaining your decision.");
    return;
  }
  const ok = await review(action, reviewNotes.value.trim());
  if (ok) {
    const labels: Record<typeof action, string> = {
      approve: "Project approved and published.",
      reject: "Project rejected.",
      request_changes: "Changes requested.",
    };
    toast.success(labels[action]);
    reviewNotes.value = "";
    await loadProject();
  } else if (reviewError.value) {
    toast.error(reviewError.value);
  }
}
</script>

<template>
  <div class="page-canvas min-h-screen">
    <div class="mx-auto max-w-6xl px-6 py-10">
      <NuxtLink
        to="/projects"
        class="text-muted-foreground hover:text-foreground mb-6 inline-flex items-center gap-2 text-sm transition-colors"
      >
        <ArrowLeft class="size-4" />
        Back to Discover
      </NuxtLink>

      <div
        v-if="pending"
        class="text-muted-foreground flex items-center gap-2 py-20"
      >
        <Loader2 class="size-5 animate-spin" />
        Loading project…
      </div>

      <div
        v-else-if="error || !project"
        class="border-border/60 rounded-xl border p-10 text-center"
      >
        <p class="text-muted-foreground">{{ error || "Project not found." }}</p>
        <NuxtLink
          to="/projects"
          class="text-primary mt-3 inline-block text-sm hover:underline"
        >
          Browse all projects
        </NuxtLink>
      </div>

      <template v-else>
        <div
          v-if="showOwnerPending"
          class="border-amber-500/40 bg-amber-500/5 mb-6 flex items-start gap-3 rounded-2xl border p-5"
        >
          <Clock class="mt-0.5 size-5 shrink-0 text-amber-500" />
          <div>
            <h2 class="text-foreground font-semibold">
              Your changes are awaiting review
            </h2>
            <p class="text-muted-foreground mt-1 text-sm leading-relaxed">
              The version shown publicly is your last approved one. Your latest
              edits will go live once a moderator approves them.
            </p>
          </div>
        </div>

        <div
          v-if="showModeration"
          class="border-primary/40 bg-primary/5 mb-6 rounded-2xl border p-5 sm:p-6"
        >
          <div class="mb-4 flex items-center gap-2">
            <ShieldCheck class="text-primary size-5" />
            <h2 class="text-foreground text-lg font-semibold">
              Moderator review
            </h2>
            <span
              class="ml-auto rounded-full bg-amber-500/15 px-2.5 py-0.5 text-xs font-semibold text-amber-500"
            >
              Awaiting review
            </span>
          </div>
          <p class="text-muted-foreground mb-4 text-sm leading-relaxed">
            Review the project below as it will appear once live. Approve to
            publish it, request changes to send it back with feedback, or reject
            it. Notes are required when requesting changes or rejecting.
          </p>
          <div v-if="pendingReview" class="mb-4 space-y-4">
            <div
              class="border-border/60 bg-background/40 rounded-xl border p-4"
            >
              <p
                class="text-muted-foreground mb-1 text-xs font-semibold tracking-wide uppercase"
              >
                {{
                  pendingReview.is_first_review
                    ? "First submission"
                    : "Changes submitted for review"
                }}
              </p>
              <p class="text-foreground text-sm whitespace-pre-wrap">
                {{
                  pendingReview.changelog?.trim() ||
                  "The creator did not leave a note."
                }}
              </p>
            </div>

            <div class="overflow-hidden rounded-xl border border-border/60">
              <table class="w-full text-sm">
                <thead>
                  <tr class="bg-background/40 text-muted-foreground text-left">
                    <th class="w-28 px-3 py-2 font-medium">Field</th>
                    <th
                      v-if="!pendingReview.is_first_review"
                      class="px-3 py-2 font-medium"
                    >
                      Current (live)
                    </th>
                    <th class="px-3 py-2 font-medium">
                      {{ pendingReview.is_first_review ? "Submitted" : "New" }}
                    </th>
                  </tr>
                </thead>
                <tbody>
                  <tr
                    v-for="row in reviewDiffs"
                    :key="row.label"
                    class="border-border/40 border-t align-top"
                    :class="row.changed ? 'bg-primary/5' : ''"
                  >
                    <td
                      class="text-muted-foreground px-3 py-2 font-medium whitespace-nowrap"
                    >
                      {{ row.label }}
                      <span v-if="row.changed" class="text-primary ml-1 text-xs"
                        >•</span
                      >
                    </td>
                    <td
                      v-if="!pendingReview.is_first_review"
                      class="text-muted-foreground px-3 py-2 break-words whitespace-pre-wrap"
                    >
                      {{ row.before || "—" }}
                    </td>
                    <td
                      class="text-foreground px-3 py-2 break-words whitespace-pre-wrap"
                      :class="row.changed ? 'font-medium' : ''"
                    >
                      {{ row.after || "—" }}
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>

            <div
              v-if="pendingReview.icon_changed"
              class="flex items-center gap-4"
            >
              <span
                class="text-muted-foreground text-xs font-semibold tracking-wide uppercase"
              >
                Icon
              </span>
              <div class="flex items-center gap-3">
                <div
                  v-if="
                    !pendingReview.is_first_review &&
                    pendingReview.published?.icon_url
                  "
                  class="flex flex-col items-center gap-1"
                >
                  <img
                    :src="pendingWithBase(pendingReview.published.icon_url)!"
                    alt="Current icon"
                    class="size-14 rounded-lg object-cover ring-1 ring-white/10"
                  />
                  <span class="text-muted-foreground text-[10px]">Current</span>
                </div>
                <ArrowLeft
                  v-if="!pendingReview.is_first_review"
                  class="text-muted-foreground size-4 rotate-180"
                />
                <div
                  v-if="pendingReview.pending.icon_url"
                  class="flex flex-col items-center gap-1"
                >
                  <img
                    :src="pendingWithBase(pendingReview.pending.icon_url)!"
                    alt="New icon"
                    class="size-14 rounded-lg object-cover ring-1 ring-white/10"
                  />
                  <span class="text-primary text-[10px]">New</span>
                </div>
              </div>
            </div>
          </div>
          <Textarea
            v-model="reviewNotes"
            rows="3"
            placeholder="Notes for the creator (required to request changes or reject)…"
            class="mb-4"
          />
          <div class="flex flex-col gap-2 sm:flex-row">
            <Button
              class="btn-glow gap-2"
              :disabled="reviewSubmitting"
              @click="handleReview('approve')"
            >
              <Loader2 v-if="reviewSubmitting" class="size-4 animate-spin" />
              <CircleCheck v-else class="size-4" />
              Approve & publish
            </Button>
            <Button
              variant="outline"
              class="gap-2"
              :disabled="reviewSubmitting"
              @click="handleReview('request_changes')"
            >
              <MessageSquareWarning class="size-4" />
              Request changes
            </Button>
            <Button
              variant="destructive"
              class="gap-2 sm:ml-auto"
              :disabled="reviewSubmitting"
              @click="handleReview('reject')"
            >
              <Ban class="size-4" />
              Reject
            </Button>
          </div>
        </div>

        <section
          class="border-border/60 bg-card/40 rounded-2xl border p-6 backdrop-blur-sm sm:p-8"
        >
          <div
            class="flex flex-col gap-6 lg:flex-row lg:items-start lg:justify-between"
          >
            <div class="flex gap-5">
              <div
                class="flex size-28 shrink-0 items-center justify-center overflow-hidden rounded-2xl bg-gradient-to-br ring-1 ring-white/10"
                :class="iconSrc ? '' : typeStyle.gradient"
              >
                <img
                  v-if="iconSrc"
                  :src="iconSrc"
                  :alt="project.title"
                  class="size-full object-cover"
                />
                <component :is="typeStyle.icon" v-else class="size-12" />
              </div>
              <div class="min-w-0">
                <div class="flex flex-wrap items-center gap-3">
                  <h1 class="display-heading text-2xl sm:text-3xl">
                    {{ project.title }}
                  </h1>
                  <span
                    class="bg-primary/15 text-primary rounded-full px-3 py-1 text-xs font-semibold"
                  >
                    {{ projectTypeLabel(project.project_type) }}
                  </span>
                </div>
                <p class="text-muted-foreground mt-2 max-w-2xl break-words">
                  {{ project.summary }}
                </p>
                <div
                  class="text-muted-foreground mt-4 flex flex-wrap items-center gap-5 text-sm"
                >
                  <span class="inline-flex items-center gap-1.5">
                    <Download class="size-4" />
                    {{ project.download_count.toLocaleString() }} downloads
                  </span>
                  <span class="inline-flex items-center gap-1.5">
                    <Heart class="size-4" />
                    {{ (project.heart_count ?? 0).toLocaleString() }} hearts
                  </span>
                  <span class="inline-flex items-center gap-1.5">
                    <Clock class="size-4" />
                    {{ relativeTime(project.created_at) }}
                  </span>
                </div>
              </div>
            </div>

            <div class="flex shrink-0 items-center gap-2">
              <Button v-if="isOwner" as-child variant="outline" class="gap-2">
                <NuxtLink :to="`/${project.owner}/${slug}/settings`">
                  <Pencil class="size-4" />
                  Edit
                </NuxtLink>
              </Button>

              <DropdownMenu>
                <DropdownMenuTrigger as-child>
                  <Button class="btn-glow gap-2" :disabled="!latestVersion">
                    <Download class="size-4" />
                    {{ latestVersion ? "Download" : "No downloads" }}
                    <ChevronDown class="size-4 opacity-70" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end" class="w-72">
                  <DropdownMenuLabel>Choose a version</DropdownMenuLabel>
                  <DropdownMenuSeparator />
                  <template v-if="downloadableVersions.length">
                    <DropdownMenuItem
                      v-for="v in downloadableVersions"
                      :key="v.id"
                      as-child
                    >
                      <a
                        :href="downloadUrl(v)"
                        class="flex items-center justify-between gap-3"
                      >
                        <span class="flex flex-col">
                          <span class="font-medium">{{
                            v.version_number
                          }}</span>
                          <span class="text-muted-foreground text-xs">
                            {{ channelLabel(v.channel) }}
                            <template v-if="v.file">
                              · {{ formatFileSize(v.file.size) }}</template
                            >
                          </span>
                        </span>
                        <Download class="size-4 opacity-70" />
                      </a>
                    </DropdownMenuItem>
                  </template>
                  <DropdownMenuItem v-else disabled>
                    No downloads yet
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>

              <Button
                variant="outline"
                class="gap-2"
                :class="
                  project.viewer_hearted
                    ? 'border-rose-500/40 text-rose-500'
                    : ''
                "
                :disabled="heartPending"
                :aria-pressed="project.viewer_hearted"
                aria-label="Heart project"
                @click="handleHeart"
              >
                <Heart
                  class="size-4"
                  :class="project.viewer_hearted ? 'fill-current' : ''"
                />
                <span class="tabular-nums">{{ project.heart_count ?? 0 }}</span>
              </Button>

              <Button
                variant="outline"
                size="icon"
                :class="
                  project.viewer_saved ? 'border-primary/40 text-primary' : ''
                "
                :disabled="savePending"
                :aria-pressed="project.viewer_saved"
                aria-label="Save project"
                @click="handleSave"
              >
                <Bookmark
                  class="size-4"
                  :class="project.viewer_saved ? 'fill-current' : ''"
                />
              </Button>

              <DropdownMenu>
                <DropdownMenuTrigger as-child>
                  <Button
                    variant="outline"
                    size="icon"
                    aria-label="More actions"
                  >
                    <MoreHorizontal class="size-4" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end" class="w-48">
                  <DropdownMenuItem
                    variant="destructive"
                    @click="reportOpen = true"
                  >
                    <Flag class="size-4" />
                    Report
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem @click="copyId">
                    <Copy class="size-4" />
                    Copy ID
                  </DropdownMenuItem>
                  <DropdownMenuItem @click="copyLink">
                    <Link2 class="size-4" />
                    Copy link
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </div>
        </section>

        <div class="mt-8 flex flex-col gap-8 lg:flex-row">
          <div
            class="min-w-0 flex-1"
            :class="settings.contentSidebarLeft ? 'lg:order-2' : ''"
          >
            <Tabs default-value="description">
              <TabsList class="flex-wrap">
                <TabsTrigger value="description">Description</TabsTrigger>
                <TabsTrigger value="gallery">
                  Gallery
                  <span
                    v-if="images.length"
                    class="text-muted-foreground ml-1 text-xs"
                  >
                    {{ images.length }}
                  </span>
                </TabsTrigger>
                <TabsTrigger value="changelog">Changelog</TabsTrigger>
                <TabsTrigger value="versions">
                  Versions
                  <span
                    v-if="versions.length"
                    class="text-muted-foreground ml-1 text-xs"
                  >
                    {{ versions.length }}
                  </span>
                </TabsTrigger>
              </TabsList>

              <TabsContent value="description">
                <div class="border-border/60 bg-card/30 rounded-xl border p-6">
                  <p
                    v-if="project.description && project.description.trim()"
                    class="text-foreground/90 leading-relaxed break-words whitespace-pre-line"
                  >
                    {{ project.description }}
                  </p>
                  <p v-else class="text-muted-foreground">
                    No description provided yet.
                  </p>
                </div>
              </TabsContent>

              <TabsContent value="gallery">
                <div class="border-border/60 bg-card/30 rounded-xl border p-6">
                  <div v-if="images.length" class="grid gap-4 sm:grid-cols-2">
                    <figure
                      v-for="image in images"
                      :key="image.id"
                      class="border-border/60 group relative overflow-hidden rounded-lg border"
                    >
                      <img
                        :src="image.url"
                        :alt="image.caption"
                        class="aspect-video w-full object-cover"
                      />
                      <figcaption
                        v-if="image.caption"
                        class="text-muted-foreground bg-card/80 px-3 py-2 text-sm"
                      >
                        {{ image.caption }}
                      </figcaption>
                    </figure>
                  </div>
                  <p v-else class="text-muted-foreground">
                    No screenshots yet.
                  </p>
                </div>
              </TabsContent>

              <TabsContent value="changelog">
                <div class="border-border/60 bg-card/30 rounded-xl border p-6">
                  <ol v-if="changelogEntries.length" class="space-y-6">
                    <li
                      v-for="v in changelogEntries"
                      :key="v.id"
                      class="border-border/40 border-l-2 pl-4"
                    >
                      <div class="flex flex-wrap items-center gap-2">
                        <span class="font-semibold">{{
                          v.version_number
                        }}</span>
                        <span
                          class="bg-primary/15 text-primary rounded-full px-2 py-0.5 text-xs"
                        >
                          {{ channelLabel(v.channel) }}
                        </span>
                        <span class="text-muted-foreground text-xs">
                          {{ relativeTime(v.created_at) }}
                        </span>
                      </div>
                      <p
                        class="text-foreground/85 mt-2 break-words whitespace-pre-line"
                      >
                        {{ v.changelog }}
                      </p>
                    </li>
                  </ol>
                  <p v-else class="text-muted-foreground">
                    No changelog entries yet.
                  </p>
                </div>
              </TabsContent>

              <TabsContent value="versions">
                <div class="border-border/60 bg-card/30 rounded-xl border p-6">
                  <ul v-if="versions.length" class="divide-border/60 divide-y">
                    <li
                      v-for="v in versions"
                      :key="v.id"
                      class="flex flex-wrap items-center justify-between gap-3 py-3 first:pt-0"
                    >
                      <div class="min-w-0">
                        <div class="flex flex-wrap items-center gap-2">
                          <span class="font-medium">{{
                            v.version_number
                          }}</span>
                          <span
                            v-if="v.name"
                            class="text-muted-foreground text-sm"
                          >
                            {{ v.name }}
                          </span>
                          <span
                            class="bg-muted text-muted-foreground rounded-full px-2 py-0.5 text-xs"
                          >
                            {{ channelLabel(v.channel) }}
                          </span>
                        </div>
                        <div
                          class="text-muted-foreground mt-1 flex flex-wrap items-center gap-3 text-xs"
                        >
                          <span>{{ relativeTime(v.created_at) }}</span>
                          <span v-if="v.file">{{
                            formatFileSize(v.file.size)
                          }}</span>
                          <span class="inline-flex items-center gap-1">
                            <Download class="size-3" />
                            {{ v.download_count.toLocaleString() }}
                          </span>
                        </div>
                      </div>
                      <Button
                        v-if="v.file"
                        as-child
                        variant="outline"
                        size="sm"
                        class="gap-2"
                      >
                        <a :href="downloadUrl(v)">
                          <Download class="size-4" />
                          Download
                        </a>
                      </Button>
                    </li>
                  </ul>
                  <p v-else class="text-muted-foreground">
                    No versions published yet.
                  </p>
                </div>
              </TabsContent>
            </Tabs>
          </div>

          <aside class="space-y-4 lg:w-72 lg:shrink-0">
            <div
              v-if="project.categories.length"
              class="border-border/60 bg-card/30 rounded-xl border p-4"
            >
              <h3 class="eyebrow mb-3">Tags</h3>
              <div class="flex flex-wrap gap-2">
                <NuxtLink
                  v-for="cat in project.categories"
                  :key="cat.slug"
                  :to="`/projects?category=${cat.slug}`"
                  class="bg-muted hover:bg-primary/15 hover:text-primary rounded-full px-3 py-1 text-xs transition-colors"
                >
                  {{ cat.name }}
                </NuxtLink>
              </div>
            </div>

            <div class="border-border/60 bg-card/30 rounded-xl border p-4">
              <h3 class="eyebrow mb-3 flex items-center gap-2">
                <Link2 class="size-3.5" />
                Links
              </h3>
              <p
                v-if="!projectLinks.length"
                class="text-muted-foreground text-sm"
              >
                No links yet.
              </p>
              <ul v-else class="space-y-2">
                <li v-for="link in projectLinks" :key="link.label">
                  <a
                    :href="link.url"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-muted-foreground hover:text-primary flex items-center justify-between gap-2 text-sm transition-colors"
                  >
                    <span>{{ link.label }}</span>
                    <ExternalLink class="size-3.5 shrink-0" />
                  </a>
                </li>
              </ul>
            </div>

            <div class="border-border/60 bg-card/30 rounded-xl border p-4">
              <h3 class="eyebrow mb-3 flex items-center gap-2">
                <Users class="size-3.5" />
                Creators
              </h3>
              <div class="flex items-center gap-3">
                <div
                  class="bg-primary/15 text-primary flex size-9 items-center justify-center rounded-full text-sm font-semibold uppercase"
                >
                  {{ project.owner.charAt(0) }}
                </div>
                <div>
                  <p class="text-sm font-medium">{{ project.owner }}</p>
                  <p class="text-muted-foreground text-xs">Owner</p>
                </div>
              </div>
            </div>

            <div class="border-border/60 bg-card/30 rounded-xl border p-4">
              <h3 class="eyebrow mb-3">Details</h3>
              <dl class="space-y-2.5 text-sm">
                <div class="flex items-center justify-between gap-2">
                  <dt class="text-muted-foreground flex items-center gap-1.5">
                    <Scale class="size-3.5" />
                    License
                  </dt>
                  <dd>{{ project.license || "Not specified" }}</dd>
                </div>
                <div class="flex items-center justify-between gap-2">
                  <dt class="text-muted-foreground flex items-center gap-1.5">
                    <Calendar class="size-3.5" />
                    Published
                  </dt>
                  <dd>{{ relativeTime(project.created_at) }}</dd>
                </div>
              </dl>
            </div>
          </aside>
        </div>
      </template>
    </div>

    <Dialog v-model:open="reportOpen">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Report this project</DialogTitle>
          <DialogDescription>
            Let us know what's wrong. Reporting isn't fully wired up yet, but
            your interest helps us prioritize it.
          </DialogDescription>
        </DialogHeader>
        <Textarea
          v-model="reportReason"
          rows="4"
          placeholder="Describe the issue (optional)"
        />
        <DialogFooter>
          <Button variant="outline" @click="reportOpen = false">Cancel</Button>
          <Button @click="submitReport">Submit report</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>
