<script setup lang="ts">
import {
  ArrowLeft,
  BarChart3,
  Bold,
  CircleAlert,
  CircleCheck,
  Clock,
  Code,
  Coins,
  Download,
  Eye,
  EyeOff,
  FileText,
  Globe,
  Heading,
  Image as ImageIcon,
  Images,
  Info,
  Italic,
  Link2,
  Link as LinkIcon,
  List,
  ListOrdered,
  Loader2,
  Lock,
  Package,
  Quote,
  Scale,
  Send,
  Settings,
  Strikethrough,
  Tags,
  Trash2,
  TriangleAlert,
  Upload,
  UserPlus,
  Users,
  Video,
} from "@lucide/vue";
import type { Component } from "vue";
import { toast } from "vue-sonner";
import {
  useProjectSettings,
  RECOMMENDED_DESCRIPTION_LENGTH,
  type ProjectStatus,
  type ProjectVisibility,
} from "~/scripts/pages/projects";
import {
  useVersions,
  useUploadVersionForm,
  formatFileSize,
  VERSION_CHANNELS,
} from "~/scripts/pages/projects/versions";
import {
  useGallery,
  useUploadGalleryForm,
} from "~/scripts/pages/projects/gallery";
import { useProjectMembers } from "~/scripts/pages/projects/members";
import { useProjectAnalytics } from "~/scripts/pages/projects/analytics";
import { renderMarkdown } from "~/scripts/markdown";

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
  savingDescription,
  descriptionError,
  savingMonetization,
  monetizationError,
  savingLicense,
  licenseError,
  savingTags,
  tagsError,
  availableCategories,
  selectedCategoryIds,
  tagsDirty,
  toggleCategory,
  saveTags,
  savingLinks,
  linksError,
  linksDirty,
  hasLinks,
  saveLinks,
  submitting,
  submitError,
  changelog,
  changelogDirty,
  savingChangelog,
  changelogError,
  saveChangelog,
  deleting,
  deleteError,
  deleteProject,
  iconPending,
  iconError,
  iconUrl,
  descriptionLength,
  charityShare,
  dirty,
  descriptionDirty,
  monetizationDirty,
  licenseDirty,
  save,
  saveDescription,
  saveMonetization,
  saveLicense,
  submitForReview,
  uploadIcon,
  removeIcon,
} = useProjectSettings(slug.value);
const {
  versions,
  error: versionsError,
  load: loadVersions,
  downloadUrl,
  remove: removeVersion,
} = useVersions(slug.value);
const {
  images,
  load: loadGallery,
  remove: removeGalleryImage,
} = useGallery(slug.value);
const versionForm = useUploadVersionForm(slug.value);
const galleryForm = useUploadGalleryForm(slug.value);
const {
  members,
  pending: membersPending,
  username: memberUsername,
  adding: addingMember,
  addError: memberAddError,
  load: loadMembers,
  add: addMember,
  remove: removeMember,
} = useProjectMembers(slug.value);
const {
  data: analytics,
  error: analyticsError,
  pending: analyticsPending,
  load: loadAnalytics,
} = useProjectAnalytics(slug.value);

await Promise.all([
  load(),
  loadVersions(),
  loadGallery(),
  loadMembers(),
  loadAnalytics(),
]);

async function handleUploadVersion() {
  if (await versionForm.submit()) {
    await Promise.all([loadVersions(), load()]);
    toast.success("Version published.");
  } else if (versionForm.error.value) {
    toast.error(versionForm.error.value);
  }
}

async function handleDeleteVersion(version: (typeof versions.value)[number]) {
  if (await removeVersion(version)) {
    await load();
    toast.success("Version deleted.");
  } else if (versionsError.value) {
    toast.error(versionsError.value);
  }
}

const confirmDeleteGalleryId = ref<string | null>(null);

async function handleUploadGalleryImage() {
  if (await galleryForm.submit()) {
    await loadGallery();
    toast.success("Image added to the gallery.");
  } else if (galleryForm.error.value) {
    toast.error(galleryForm.error.value);
  }
}

async function handleDeleteGalleryImage(id: string) {
  if (await removeGalleryImage(id)) {
    toast.success("Image removed.");
  } else {
    toast.error("Could not remove the image. Please try again.");
  }
}

const confirmDeleteVersionId = ref<string | null>(null);

const CHANNEL_STYLES: Record<string, string> = {
  release: "bg-primary/15 text-primary",
  beta: "bg-amber-500/15 text-amber-500",
  alpha: "bg-violet-500/15 text-violet-400",
};

function channelLabel(value: string): string {
  return (
    VERSION_CHANNELS.find((c) => c.value === value)?.label ??
    value.charAt(0).toUpperCase() + value.slice(1)
  );
}

function formatDate(iso: string): string {
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) return "";
  return date.toLocaleDateString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

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

const BEACON_SHARE = 20;

const SHARE_PRESETS: { value: number; label: string }[] = [
  { value: 80, label: "Max (80%)" },
  { value: 60, label: "60%" },
  { value: 40, label: "40%" },
  { value: 20, label: "20%" },
  { value: 0, label: "Donate all (0%)" },
];

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

async function handleSaveMonetization() {
  await saveMonetization();
  if (!monetizationError.value) {
    toast.success("Monetization settings saved.");
  }
}

function clampCreatorShare() {
  const value = Number(form.creatorShare);
  if (!Number.isFinite(value)) {
    form.creatorShare = 0;
    return;
  }
  form.creatorShare = Math.min(80, Math.max(0, Math.round(value)));
}

const descriptionInput = ref<HTMLTextAreaElement | null>(null);
const showDescriptionPreview = ref(false);
const descriptionPreview = computed(() => renderMarkdown(form.description));
const RECOMMENDED_DESCRIPTION = RECOMMENDED_DESCRIPTION_LENGTH;

interface MarkdownAction {
  icon: Component;
  label: string;
  before: string;
  after?: string;
  placeholder?: string;
  block?: boolean;
}

const MARKDOWN_ACTIONS: MarkdownAction[] = [
  {
    icon: Heading,
    label: "Heading",
    before: "## ",
    placeholder: "Heading",
    block: true,
  },
  {
    icon: Bold,
    label: "Bold",
    before: "**",
    after: "**",
    placeholder: "bold text",
  },
  {
    icon: Italic,
    label: "Italic",
    before: "_",
    after: "_",
    placeholder: "italic text",
  },
  {
    icon: Strikethrough,
    label: "Strikethrough",
    before: "~~",
    after: "~~",
    placeholder: "struck text",
  },
  { icon: Code, label: "Code", before: "`", after: "`", placeholder: "code" },
  {
    icon: LinkIcon,
    label: "Link",
    before: "[",
    after: "](https://)",
    placeholder: "link text",
  },
  {
    icon: ImageIcon,
    label: "Image",
    before: "![",
    after: "](https://)",
    placeholder: "alt text",
  },
  {
    icon: Video,
    label: "Video / embed",
    before: "[",
    after: "](https://)",
    placeholder: "video link",
  },
  {
    icon: List,
    label: "Bullet list",
    before: "- ",
    placeholder: "List item",
    block: true,
  },
  {
    icon: ListOrdered,
    label: "Numbered list",
    before: "1. ",
    placeholder: "List item",
    block: true,
  },
  {
    icon: Quote,
    label: "Quote",
    before: "> ",
    placeholder: "Quote",
    block: true,
  },
  {
    icon: EyeOff,
    label: "Spoiler",
    before: "<details><summary>Spoiler</summary>\n\n",
    after: "\n\n</details>",
    placeholder: "hidden content",
  },
];

function applyMarkdown(action: MarkdownAction) {
  const el = descriptionInput.value;
  if (!el) return;
  const start = el.selectionStart;
  const end = el.selectionEnd;
  const value = form.description;
  const selected = value.slice(start, end);
  const text = selected || action.placeholder || "";

  let inserted: string;
  let cursorStart: number;
  let cursorEnd: number;

  if (action.block) {
    const lines = text.split("\n");
    inserted = lines.map((line) => action.before + line).join("\n");
    cursorStart = start + action.before.length;
    cursorEnd = start + inserted.length;
  } else {
    const after = action.after ?? "";
    inserted = action.before + text + after;
    cursorStart = start + action.before.length;
    cursorEnd = cursorStart + text.length;
  }

  form.description = value.slice(0, start) + inserted + value.slice(end);

  nextTick(() => {
    el.focus();
    el.setSelectionRange(cursorStart, cursorEnd);
  });
}

async function handleSaveDescription() {
  await saveDescription();
  if (!descriptionError.value) {
    toast.success("Description saved.");
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
      complete: (p?.license ?? "").trim().length > 0,
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
      complete: hasLinks.value,
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

const status = computed<ProjectStatus>(() => project.value?.status ?? "draft");

const canSubmitNow = computed(
  () =>
    canSubmit.value &&
    (status.value === "draft" ||
      status.value === "changes_requested" ||
      status.value === "rejected"),
);

const submitLabel = computed(() =>
  status.value === "changes_requested" || status.value === "rejected"
    ? "Resubmit for review"
    : "Submit for review",
);

interface StatusBanner {
  label: string;
  description: string;
  icon: Component;
  pill: string;
  card: string;
  iconTone: string;
  showNotes: boolean;
}

const STATUS_BANNERS: Record<ProjectStatus, StatusBanner> = {
  draft: {
    label: "Draft",
    description:
      "Only you can see this project. Complete the checklist and submit it for review when you're ready to go live.",
    icon: FileText,
    pill: "bg-muted text-muted-foreground",
    card: "border-border bg-muted/30",
    iconTone: "text-muted-foreground",
    showNotes: false,
  },
  in_review: {
    label: "In review",
    description:
      "A moderator is reviewing your project. You'll be able to make changes again once they respond.",
    icon: Clock,
    pill: "bg-amber-500/15 text-amber-500",
    card: "border-amber-500/30 bg-amber-500/5",
    iconTone: "text-amber-500",
    showNotes: false,
  },
  changes_requested: {
    label: "Changes requested",
    description:
      "A moderator asked for changes before this project can go live. Address their notes, then resubmit.",
    icon: TriangleAlert,
    pill: "bg-amber-500/15 text-amber-500",
    card: "border-amber-500/30 bg-amber-500/5",
    iconTone: "text-amber-500",
    showNotes: true,
  },
  approved: {
    label: "Approved & live",
    description:
      "Your project is live on Beacon. Editing key details or uploading a new version will send it back for review.",
    icon: CircleCheck,
    pill: "bg-primary/15 text-primary",
    card: "border-primary/30 bg-primary/5",
    iconTone: "text-primary",
    showNotes: false,
  },
  rejected: {
    label: "Rejected",
    description:
      "A moderator rejected this project. Review their notes, make the necessary changes, and resubmit.",
    icon: CircleAlert,
    pill: "bg-destructive/15 text-destructive",
    card: "border-destructive/30 bg-destructive/5",
    iconTone: "text-destructive",
    showNotes: true,
  },
};

const statusBanner = computed(() => STATUS_BANNERS[status.value]);

const LICENSE_OPTIONS: { value: string; label: string }[] = [
  { value: "All Rights Reserved", label: "All Rights Reserved" },
  { value: "MIT", label: "MIT" },
  { value: "Apache-2.0", label: "Apache License 2.0" },
  { value: "GPL-3.0", label: "GNU GPL v3.0" },
  { value: "LGPL-3.0", label: "GNU LGPL v3.0" },
  { value: "MPL-2.0", label: "Mozilla Public License 2.0" },
  { value: "BSD-3-Clause", label: "BSD 3-Clause" },
  { value: "CC0-1.0", label: "Creative Commons Zero (Public Domain)" },
  { value: "CC-BY-4.0", label: "Creative Commons Attribution 4.0" },
  {
    value: "CC-BY-SA-4.0",
    label: "Creative Commons Attribution-ShareAlike 4.0",
  },
];

async function handleSaveLicense() {
  await saveLicense();
  if (!licenseError.value) {
    toast.success("License saved.");
  }
}

async function handleSaveTags() {
  await saveTags();
  if (!tagsError.value) {
    toast.success("Tags saved.");
  }
}

async function handleSaveLinks() {
  await saveLinks();
  if (!linksError.value) {
    toast.success("Links saved.");
  }
}

const confirmRemoveMemberId = ref<string | null>(null);

async function handleAddMember() {
  if (await addMember()) {
    toast.success("Member added.");
  } else if (memberAddError.value) {
    toast.error(memberAddError.value);
  }
}

async function handleRemoveMember(userId: string) {
  if (await removeMember(userId)) {
    toast.success("Member removed.");
  } else {
    toast.error("Could not remove that member. Please try again.");
  }
}

const analyticsMax = computed(() => {
  const series = analytics.value?.series ?? [];
  const max = Math.max(1, ...series.map((d) => Math.max(d.views, d.downloads)));
  return max;
});

async function handleSubmit() {
  const ok = await submitForReview();
  if (ok) {
    toast.success("Submitted for review.");
  } else if (submitError.value) {
    toast.error(submitError.value);
  }
}

async function handleSaveChangelog() {
  const ok = await saveChangelog();
  if (ok) {
    toast.success("Review note saved.");
  } else if (changelogError.value) {
    toast.error(changelogError.value);
  }
}

const confirmDeleteProject = ref(false);

async function handleDeleteProject() {
  const ok = await deleteProject();
  if (ok) {
    toast.success("Project deleted.");
    await navigateTo("/profile");
  } else if (deleteError.value) {
    toast.error(deleteError.value);
  }
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
            <div
              class="flex items-start gap-3 rounded-2xl border p-4"
              :class="statusBanner.card"
            >
              <component
                :is="statusBanner.icon"
                class="mt-0.5 size-5 shrink-0"
                :class="statusBanner.iconTone"
              />
              <div class="min-w-0 flex-1">
                <div class="flex flex-wrap items-center gap-2">
                  <span class="text-foreground text-sm font-semibold"
                    >Status</span
                  >
                  <span
                    class="rounded-full px-2 py-0.5 text-[11px] font-semibold"
                    :class="statusBanner.pill"
                  >
                    {{ statusBanner.label }}
                  </span>
                </div>
                <p class="text-muted-foreground mt-1 text-sm leading-relaxed">
                  {{ statusBanner.description }}
                </p>
                <div
                  v-if="statusBanner.showNotes && project.review?.notes"
                  class="bg-background/60 text-foreground mt-3 rounded-lg border p-3 text-sm"
                >
                  <p
                    class="text-muted-foreground mb-1 text-xs font-semibold tracking-wide uppercase"
                  >
                    Moderator notes
                  </p>
                  {{ project.review.notes }}
                </div>
              </div>
            </div>

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

              <div class="mt-6 space-y-2 border-t pt-5">
                <Label for="changelog-note">Note for reviewers</Label>
                <p class="text-muted-foreground text-xs">
                  Tell moderators what changed in this submission. Shown to the
                  review team alongside a diff of your edits.
                </p>
                <Textarea
                  id="changelog-note"
                  v-model="changelog"
                  rows="3"
                  placeholder="e.g. Updated the description and added two new categories."
                />
                <div
                  v-if="status === 'in_review' || status === 'approved'"
                  class="flex items-center justify-end gap-3"
                >
                  <span
                    v-if="changelogError"
                    class="text-destructive text-xs"
                    >{{ changelogError }}</span
                  >
                  <Button
                    variant="outline"
                    size="sm"
                    :disabled="!changelogDirty || savingChangelog"
                    @click="handleSaveChangelog"
                  >
                    <Loader2
                      v-if="savingChangelog"
                      class="size-4 animate-spin"
                    />
                    Save note
                  </Button>
                </div>
              </div>

              <div
                class="mt-6 flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center sm:justify-between"
              >
                <p class="text-muted-foreground text-xs">
                  Your project stays private until a moderator reviews and
                  approves it. Approved projects re-enter review when key
                  details change.
                </p>
                <div
                  v-if="status === 'in_review'"
                  class="text-muted-foreground inline-flex shrink-0 items-center gap-2 text-sm font-medium"
                >
                  <Clock class="size-4 text-amber-500" />
                  In review
                </div>
                <div
                  v-else-if="status === 'approved'"
                  class="text-primary inline-flex shrink-0 items-center gap-2 text-sm font-medium"
                >
                  <CircleCheck class="size-4" />
                  Live
                </div>
                <Button
                  v-else
                  class="btn-glow shrink-0"
                  :disabled="!canSubmitNow || submitting"
                  @click="handleSubmit"
                >
                  <Loader2 v-if="submitting" class="size-4 animate-spin" />
                  <Send v-else class="size-4" />
                  {{ submitLabel }}
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
                      Earn from your project through the Beacon Rewards Program.
                    </p>
                  </div>
                  <Switch
                    v-model="form.monetizationEnabled"
                    class="mt-1 shrink-0"
                  />
                </div>

                <div v-if="form.monetizationEnabled" class="mt-6 space-y-6">
                  <p class="text-muted-foreground text-sm leading-relaxed">
                    When monetization is on, your project earns a share of
                    revenue through the
                    <span class="text-foreground font-medium"
                      >Rewards Program</span
                    >
                    <span class="text-muted-foreground/70"> (coming soon)</span
                    >. Beacon keeps a fixed {{ BEACON_SHARE }}% to cover running
                    costs - any profit left over is donated to charity, with the
                    breakdown published on our
                    <span class="text-foreground font-medium"
                      >Beacon Finances</span
                    >
                    <span class="text-muted-foreground/70">
                      page (coming soon)</span
                    >. You can give up part of your own share to send even more
                    to charity.
                  </p>

                  <!-- Revenue split bar -->
                  <div class="space-y-3">
                    <div
                      class="bg-muted flex h-3 overflow-hidden rounded-full"
                      role="img"
                      :aria-label="`Creator ${form.creatorShare}%, charity ${charityShare}%, Beacon ${BEACON_SHARE}%`"
                    >
                      <div
                        class="bg-primary h-full transition-all"
                        :style="{ width: form.creatorShare + '%' }"
                      />
                      <div
                        class="h-full bg-emerald-500 transition-all"
                        :style="{ width: charityShare + '%' }"
                      />
                      <div
                        class="bg-muted-foreground/40 h-full transition-all"
                        :style="{ width: BEACON_SHARE + '%' }"
                      />
                    </div>
                    <div class="flex flex-wrap gap-x-6 gap-y-2 text-xs">
                      <span class="flex items-center gap-1.5">
                        <span class="bg-primary size-2.5 rounded-full" />
                        <span class="text-foreground font-medium"
                          >You {{ form.creatorShare }}%</span
                        >
                      </span>
                      <span class="flex items-center gap-1.5">
                        <span class="size-2.5 rounded-full bg-emerald-500" />
                        <span class="text-foreground font-medium"
                          >Charity {{ charityShare }}%</span
                        >
                      </span>
                      <span class="flex items-center gap-1.5">
                        <span
                          class="bg-muted-foreground/40 size-2.5 rounded-full"
                        />
                        <span class="text-muted-foreground"
                          >Beacon {{ BEACON_SHARE }}%</span
                        >
                      </span>
                    </div>
                  </div>

                  <div class="space-y-3">
                    <div class="flex items-center justify-between gap-3">
                      <Label class="text-sm">Your share</Label>
                      <div
                        class="border-input focus-within:ring-ring/50 flex items-center rounded-md border focus-within:ring-2"
                      >
                        <input
                          v-model.number="form.creatorShare"
                          type="number"
                          min="0"
                          max="80"
                          step="1"
                          class="w-14 bg-transparent py-1 pr-1 pl-2 text-right text-sm font-semibold outline-none"
                          @change="clampCreatorShare"
                        />
                        <span class="text-muted-foreground pr-2 text-sm"
                          >%</span
                        >
                      </div>
                    </div>
                    <input
                      v-model.number="form.creatorShare"
                      type="range"
                      min="0"
                      max="80"
                      step="1"
                      class="accent-primary h-2 w-full cursor-pointer"
                    />
                    <div class="flex flex-wrap gap-2">
                      <Button
                        v-for="preset in SHARE_PRESETS"
                        :key="preset.value"
                        type="button"
                        size="sm"
                        :variant="
                          form.creatorShare === preset.value
                            ? 'default'
                            : 'outline'
                        "
                        @click="form.creatorShare = preset.value"
                      >
                        {{ preset.label }}
                      </Button>
                    </div>
                  </div>

                  <div
                    v-if="charityShare > 0"
                    class="flex items-start gap-2 rounded-xl border border-emerald-500/30 bg-emerald-500/5 p-3 text-sm"
                  >
                    <CircleCheck
                      class="mt-0.5 size-4 shrink-0 text-emerald-500"
                    />
                    <p class="text-muted-foreground">
                      You're donating an extra
                      <span class="font-semibold text-emerald-500"
                        >{{ charityShare }}%</span
                      >
                      of revenue to charity on top of Beacon's contribution.
                      Thank you.
                    </p>
                  </div>
                </div>

                <p
                  v-else
                  class="text-muted-foreground mt-6 text-sm leading-relaxed"
                >
                  Monetization is off, so your project earns nothing and no
                  revenue share is collected. Turn it on if you'd like to earn
                  through the Rewards Program - or keep it off if you'd rather
                  not, or can't monetize for legal reasons.
                </p>

                <div
                  class="mt-6 flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center"
                  :class="
                    monetizationError ? 'sm:justify-between' : 'sm:justify-end'
                  "
                >
                  <p v-if="monetizationError" class="text-destructive text-sm">
                    {{ monetizationError }}
                  </p>
                  <Button
                    class="btn-glow shrink-0"
                    :disabled="!monetizationDirty || savingMonetization"
                    @click="handleSaveMonetization"
                  >
                    <Loader2
                      v-if="savingMonetization"
                      class="size-4 animate-spin"
                    />
                    Save monetization
                  </Button>
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
                <template v-if="confirmDeleteProject">
                  <p class="mb-3 text-sm font-medium">
                    This permanently deletes
                    <span class="font-semibold">{{ project?.title }}</span>
                    and all its versions, gallery images, and stats. This cannot
                    be undone.
                  </p>
                  <div class="flex flex-wrap gap-3">
                    <Button
                      variant="destructive"
                      :disabled="deleting"
                      @click="handleDeleteProject"
                    >
                      <Loader2 v-if="deleting" class="size-4 animate-spin" />
                      Yes, delete this project
                    </Button>
                    <Button
                      variant="outline"
                      :disabled="deleting"
                      @click="confirmDeleteProject = false"
                    >
                      Cancel
                    </Button>
                  </div>
                </template>
                <Button
                  v-else
                  variant="destructive"
                  @click="confirmDeleteProject = true"
                >
                  Delete project
                </Button>
              </div>
            </section>

            <section
              v-else-if="activeSection === 'tags'"
              class="card-glass space-y-5 rounded-2xl p-6"
            >
              <div>
                <h2 class="section-title mb-1 flex items-center gap-2 text-lg">
                  <Tags class="text-primary size-5" />
                  Tags
                </h2>
                <p class="text-muted-foreground text-sm leading-relaxed">
                  Pick the categories that best describe your project so people
                  can find it when browsing. Choose the ones that genuinely fit.
                </p>
              </div>

              <p
                v-if="!availableCategories.length"
                class="text-muted-foreground border-border/60 rounded-xl border border-dashed p-8 text-center text-sm"
              >
                No categories are available for this project type yet.
              </p>

              <div v-else class="flex flex-wrap gap-2">
                <button
                  v-for="category in availableCategories"
                  :key="category.id"
                  type="button"
                  class="rounded-full border px-3 py-1.5 text-sm font-medium transition-colors"
                  :class="
                    selectedCategoryIds.includes(category.id)
                      ? 'border-primary bg-primary/15 text-primary'
                      : 'border-border/60 text-muted-foreground hover:border-border hover:text-foreground'
                  "
                  @click="toggleCategory(category.id)"
                >
                  {{ category.name }}
                </button>
              </div>

              <div
                class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center"
                :class="tagsError ? 'sm:justify-between' : 'sm:justify-end'"
              >
                <p v-if="tagsError" class="text-destructive text-sm">
                  {{ tagsError }}
                </p>
                <Button
                  class="btn-glow shrink-0"
                  :disabled="!tagsDirty || savingTags"
                  @click="handleSaveTags"
                >
                  <Loader2 v-if="savingTags" class="size-4 animate-spin" />
                  Save tags
                </Button>
              </div>
            </section>

            <section
              v-else-if="activeSection === 'description'"
              class="space-y-6"
            >
              <div class="card-glass space-y-5 rounded-2xl p-6">
                <div>
                  <h2
                    class="section-title mb-1 flex items-center gap-2 text-lg"
                  >
                    <FileText class="text-primary size-5" />
                    Description
                  </h2>
                  <p class="text-muted-foreground text-sm leading-relaxed">
                    Use this space for a full, extended description of your
                    project - what it is, what it adds, how it works, and how to
                    use it. It must be honest and accurately reflect the actual
                    project: don't promise features it doesn't have or
                    misrepresent what players will get. Full Markdown formatting
                    is supported.
                  </p>
                </div>

                <div
                  class="border-input overflow-hidden rounded-xl border focus-within:ring-2 focus-within:ring-ring/50"
                >
                  <div
                    class="bg-muted/40 flex flex-wrap items-center gap-1 border-b p-1.5"
                  >
                    <button
                      v-for="action in MARKDOWN_ACTIONS"
                      :key="action.label"
                      type="button"
                      :title="action.label"
                      :aria-label="action.label"
                      :disabled="showDescriptionPreview"
                      class="text-muted-foreground hover:bg-background hover:text-foreground inline-flex size-8 items-center justify-center rounded-md transition-colors disabled:pointer-events-none disabled:opacity-40"
                      @click="applyMarkdown(action)"
                    >
                      <component :is="action.icon" class="size-4" />
                    </button>
                    <div class="ml-auto">
                      <Button
                        type="button"
                        variant="ghost"
                        size="sm"
                        @click="
                          showDescriptionPreview = !showDescriptionPreview
                        "
                      >
                        <component
                          :is="showDescriptionPreview ? EyeOff : Eye"
                          class="size-4"
                        />
                        {{ showDescriptionPreview ? "Edit" : "Preview" }}
                      </Button>
                    </div>
                  </div>

                  <div
                    v-if="showDescriptionPreview"
                    class="markdown-preview min-h-64 px-4 py-3 text-sm"
                  >
                    <div
                      v-if="form.description.trim()"
                      v-html="descriptionPreview"
                    />
                    <p v-else class="text-muted-foreground italic">
                      Nothing to preview yet.
                    </p>
                  </div>
                  <textarea
                    v-else
                    ref="descriptionInput"
                    v-model="form.description"
                    rows="14"
                    spellcheck="true"
                    placeholder="# My project&#10;&#10;Describe your project in detail using Markdown..."
                    class="placeholder:text-muted-foreground min-h-64 w-full resize-y bg-transparent px-4 py-3 font-mono text-sm outline-none"
                  />
                </div>

                <div class="flex flex-wrap items-center justify-between gap-2">
                  <p
                    class="text-xs"
                    :class="
                      descriptionLength < RECOMMENDED_DESCRIPTION
                        ? 'text-amber-500'
                        : 'text-muted-foreground'
                    "
                  >
                    {{ descriptionLength }} characters ·
                    {{ RECOMMENDED_DESCRIPTION }}+ recommended
                  </p>
                  <a
                    href="https://www.markdownguide.org/basic-syntax/"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-muted-foreground hover:text-foreground text-xs underline-offset-2 hover:underline"
                  >
                    Markdown formatting help
                  </a>
                </div>

                <div
                  class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center"
                  :class="
                    descriptionError ? 'sm:justify-between' : 'sm:justify-end'
                  "
                >
                  <p v-if="descriptionError" class="text-destructive text-sm">
                    {{ descriptionError }}
                  </p>
                  <Button
                    class="btn-glow shrink-0"
                    :disabled="!descriptionDirty || savingDescription"
                    @click="handleSaveDescription"
                  >
                    <Loader2
                      v-if="savingDescription"
                      class="size-4 animate-spin"
                    />
                    Save description
                  </Button>
                </div>
              </div>
            </section>

            <section v-else-if="activeSection === 'versions'" class="space-y-6">
              <div class="card-glass space-y-5 rounded-2xl p-6">
                <div>
                  <h2
                    class="section-title mb-1 flex items-center gap-2 text-lg"
                  >
                    <Package class="text-primary size-5" />
                    Upload a version
                  </h2>
                  <p class="text-muted-foreground text-sm leading-relaxed">
                    Every release of your project is a version. Upload the
                    downloadable file, give it a version number, and add a
                    changelog so people know what changed.
                  </p>
                </div>

                <div class="grid gap-4 sm:grid-cols-2">
                  <div class="space-y-2">
                    <Label for="version-number">Version number</Label>
                    <Input
                      id="version-number"
                      v-model="versionForm.versionNumber.value"
                      placeholder="1.0.0"
                    />
                  </div>
                  <div class="space-y-2">
                    <Label for="version-channel">Release channel</Label>
                    <Select
                      id="version-channel"
                      v-model="versionForm.channel.value"
                    >
                      <SelectTrigger class="w-full">
                        <SelectValue placeholder="Select a channel" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem
                          v-for="option in VERSION_CHANNELS"
                          :key="option.value"
                          :value="option.value"
                        >
                          {{ option.label }}
                        </SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                </div>

                <div class="space-y-2">
                  <Label for="version-name">Display name (optional)</Label>
                  <Input
                    id="version-name"
                    v-model="versionForm.name.value"
                    placeholder="e.g. Winter Update"
                  />
                </div>

                <div class="space-y-2">
                  <Label for="version-changelog">Changelog (optional)</Label>
                  <Textarea
                    id="version-changelog"
                    v-model="versionForm.changelog.value"
                    rows="4"
                    placeholder="What changed in this version?"
                  />
                </div>

                <div class="space-y-2">
                  <Label for="version-file">File</Label>
                  <input
                    id="version-file"
                    type="file"
                    class="border-input file:bg-muted file:text-foreground hover:file:bg-muted/70 block w-full cursor-pointer rounded-xl border bg-transparent text-sm file:mr-3 file:cursor-pointer file:rounded-lg file:border-0 file:px-3 file:py-2 file:text-sm file:font-medium"
                    @change="versionForm.onFileChange"
                  />
                  <p
                    v-if="versionForm.file.value"
                    class="text-muted-foreground text-xs"
                  >
                    {{ versionForm.file.value.name }} ·
                    {{ formatFileSize(versionForm.file.value.size) }}
                  </p>
                </div>

                <div
                  class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center"
                  :class="
                    versionForm.error.value
                      ? 'sm:justify-between'
                      : 'sm:justify-end'
                  "
                >
                  <p
                    v-if="versionForm.error.value"
                    class="text-destructive text-sm"
                  >
                    {{ versionForm.error.value }}
                  </p>
                  <Button
                    class="btn-glow shrink-0"
                    :disabled="versionForm.pending.value"
                    @click="handleUploadVersion"
                  >
                    <Loader2
                      v-if="versionForm.pending.value"
                      class="size-4 animate-spin"
                    />
                    <Upload v-else class="size-4" />
                    Publish version
                  </Button>
                </div>
              </div>

              <div class="card-glass rounded-2xl p-6">
                <h2 class="section-title mb-1 flex items-center gap-2 text-lg">
                  <Package class="text-primary size-5" />
                  Published versions
                </h2>
                <p class="text-muted-foreground mb-5 text-sm">
                  {{ versions.length }}
                  {{ versions.length === 1 ? "version" : "versions" }}
                  published.
                </p>

                <p
                  v-if="!versions.length"
                  class="text-muted-foreground border-border/60 rounded-xl border border-dashed p-8 text-center text-sm"
                >
                  No versions yet. Upload your first version above.
                </p>

                <ul v-else class="space-y-3">
                  <li
                    v-for="version in versions"
                    :key="version.id"
                    class="border-border/60 bg-muted/20 rounded-xl border p-4"
                  >
                    <div
                      class="flex flex-wrap items-start justify-between gap-3"
                    >
                      <div class="min-w-0">
                        <div class="flex flex-wrap items-center gap-2">
                          <span class="text-foreground font-semibold">
                            {{ version.version_number }}
                          </span>
                          <span
                            class="rounded-full px-2 py-0.5 text-[11px] font-semibold"
                            :class="
                              CHANNEL_STYLES[version.channel] ??
                              'bg-muted text-muted-foreground'
                            "
                          >
                            {{ channelLabel(version.channel) }}
                          </span>
                          <span
                            v-if="version.name"
                            class="text-muted-foreground text-sm"
                          >
                            {{ version.name }}
                          </span>
                        </div>
                        <div
                          class="text-muted-foreground mt-1.5 flex flex-wrap items-center gap-3 text-xs"
                        >
                          <span>{{ formatDate(version.created_at) }}</span>
                          <span v-if="version.file">
                            {{ formatFileSize(version.file.size) }}
                          </span>
                          <span class="inline-flex items-center gap-1">
                            <Download class="size-3.5" />
                            {{ version.download_count }}
                          </span>
                        </div>
                      </div>

                      <div class="flex shrink-0 items-center gap-2">
                        <Button
                          v-if="version.file"
                          as-child
                          variant="outline"
                          size="sm"
                        >
                          <a :href="downloadUrl(version)">
                            <Download class="size-4" />
                            Download
                          </a>
                        </Button>
                        <template v-if="confirmDeleteVersionId === version.id">
                          <Button
                            variant="destructive"
                            size="sm"
                            @click="
                              handleDeleteVersion(version);
                              confirmDeleteVersionId = null;
                            "
                          >
                            Confirm
                          </Button>
                          <Button
                            variant="ghost"
                            size="sm"
                            @click="confirmDeleteVersionId = null"
                          >
                            Cancel
                          </Button>
                        </template>
                        <Button
                          v-else
                          variant="ghost"
                          size="icon"
                          aria-label="Delete version"
                          @click="confirmDeleteVersionId = version.id"
                        >
                          <Trash2 class="text-destructive size-4" />
                        </Button>
                      </div>
                    </div>

                    <div
                      v-if="version.changelog"
                      class="text-muted-foreground mt-3 border-t pt-3 text-sm whitespace-pre-line"
                    >
                      {{ version.changelog }}
                    </div>
                  </li>
                </ul>
              </div>
            </section>

            <section
              v-else-if="activeSection === 'license'"
              class="card-glass space-y-5 rounded-2xl p-6"
            >
              <div>
                <h2 class="section-title mb-1 flex items-center gap-2 text-lg">
                  <Scale class="text-primary size-5" />
                  License
                </h2>
                <p class="text-muted-foreground text-sm leading-relaxed">
                  Choose the license your project is distributed under. This
                  tells people what they're allowed to do with your content. If
                  you're not sure, "All Rights Reserved" keeps every right with
                  you.
                </p>
              </div>

              <div class="space-y-2">
                <Label for="project-license">License</Label>
                <Select id="project-license" v-model="form.license">
                  <SelectTrigger class="w-full sm:max-w-md">
                    <SelectValue placeholder="Select a license" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem
                      v-for="option in LICENSE_OPTIONS"
                      :key="option.value"
                      :value="option.value"
                    >
                      {{ option.label }}
                    </SelectItem>
                  </SelectContent>
                </Select>
                <a
                  href="https://choosealicense.com/"
                  target="_blank"
                  rel="noopener noreferrer"
                  class="text-muted-foreground hover:text-foreground inline-block text-xs underline-offset-2 hover:underline"
                >
                  Help me choose a license
                </a>
              </div>

              <div
                class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center"
                :class="licenseError ? 'sm:justify-between' : 'sm:justify-end'"
              >
                <p v-if="licenseError" class="text-destructive text-sm">
                  {{ licenseError }}
                </p>
                <Button
                  class="btn-glow shrink-0"
                  :disabled="!licenseDirty || savingLicense"
                  @click="handleSaveLicense"
                >
                  <Loader2 v-if="savingLicense" class="size-4 animate-spin" />
                  Save license
                </Button>
              </div>
            </section>

            <section v-else-if="activeSection === 'gallery'" class="space-y-6">
              <div class="card-glass space-y-5 rounded-2xl p-6">
                <div>
                  <h2
                    class="section-title mb-1 flex items-center gap-2 text-lg"
                  >
                    <Images class="text-primary size-5" />
                    Add a gallery image
                  </h2>
                  <p class="text-muted-foreground text-sm leading-relaxed">
                    Showcase screenshots and renders of your project. Good
                    imagery is often the first thing people notice.
                  </p>
                </div>

                <div class="space-y-2">
                  <Label for="gallery-caption">Caption (optional)</Label>
                  <Input
                    id="gallery-caption"
                    v-model="galleryForm.caption.value"
                    placeholder="Describe what this image shows"
                  />
                </div>

                <div class="space-y-2">
                  <Label for="gallery-file">Image</Label>
                  <input
                    id="gallery-file"
                    type="file"
                    accept="image/png,image/jpeg,image/webp,image/gif"
                    class="border-input file:bg-muted file:text-foreground hover:file:bg-muted/70 block w-full cursor-pointer rounded-xl border bg-transparent text-sm file:mr-3 file:cursor-pointer file:rounded-lg file:border-0 file:px-3 file:py-2 file:text-sm file:font-medium"
                    @change="galleryForm.onFileChange"
                  />
                  <p
                    v-if="galleryForm.image.value"
                    class="text-muted-foreground text-xs"
                  >
                    {{ galleryForm.image.value.name }}
                  </p>
                </div>

                <div
                  class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center"
                  :class="
                    galleryForm.error.value
                      ? 'sm:justify-between'
                      : 'sm:justify-end'
                  "
                >
                  <p
                    v-if="galleryForm.error.value"
                    class="text-destructive text-sm"
                  >
                    {{ galleryForm.error.value }}
                  </p>
                  <Button
                    class="btn-glow shrink-0"
                    :disabled="galleryForm.pending.value"
                    @click="handleUploadGalleryImage"
                  >
                    <Loader2
                      v-if="galleryForm.pending.value"
                      class="size-4 animate-spin"
                    />
                    <Upload v-else class="size-4" />
                    Add image
                  </Button>
                </div>
              </div>

              <div class="card-glass rounded-2xl p-6">
                <h2 class="section-title mb-1 flex items-center gap-2 text-lg">
                  <Images class="text-primary size-5" />
                  Gallery images
                </h2>
                <p class="text-muted-foreground mb-5 text-sm">
                  {{ images.length }}
                  {{ images.length === 1 ? "image" : "images" }}.
                </p>

                <p
                  v-if="!images.length"
                  class="text-muted-foreground border-border/60 rounded-xl border border-dashed p-8 text-center text-sm"
                >
                  No images yet. Upload your first image above.
                </p>

                <div
                  v-else
                  class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3"
                >
                  <div
                    v-for="image in images"
                    :key="image.id"
                    class="border-border/60 bg-muted/20 group relative overflow-hidden rounded-xl border"
                  >
                    <img
                      :src="image.url"
                      :alt="image.caption || 'Gallery image'"
                      class="aspect-video w-full object-cover"
                    />
                    <div
                      v-if="image.caption"
                      class="text-muted-foreground p-3 text-xs"
                    >
                      {{ image.caption }}
                    </div>
                    <div class="absolute top-2 right-2 flex items-center gap-2">
                      <template v-if="confirmDeleteGalleryId === image.id">
                        <Button
                          variant="destructive"
                          size="sm"
                          @click="
                            handleDeleteGalleryImage(image.id);
                            confirmDeleteGalleryId = null;
                          "
                        >
                          Confirm
                        </Button>
                        <Button
                          variant="secondary"
                          size="sm"
                          @click="confirmDeleteGalleryId = null"
                        >
                          Cancel
                        </Button>
                      </template>
                      <Button
                        v-else
                        variant="secondary"
                        size="icon"
                        aria-label="Delete image"
                        @click="confirmDeleteGalleryId = image.id"
                      >
                        <Trash2 class="text-destructive size-4" />
                      </Button>
                    </div>
                  </div>
                </div>
              </div>
            </section>

            <section
              v-else-if="activeSection === 'links'"
              class="card-glass space-y-5 rounded-2xl p-6"
            >
              <div>
                <h2 class="section-title mb-1 flex items-center gap-2 text-lg">
                  <Link2 class="text-primary size-5" />
                  Links
                </h2>
                <p class="text-muted-foreground text-sm leading-relaxed">
                  Add external links so people can find your source code, report
                  issues, or join your community. Leave a field blank to hide
                  it.
                </p>
              </div>

              <div class="space-y-2">
                <Label for="link-website">Website</Label>
                <Input
                  id="link-website"
                  v-model="form.websiteUrl"
                  type="url"
                  placeholder="https://example.com"
                />
              </div>

              <div class="space-y-2">
                <Label for="link-source">Source code</Label>
                <Input
                  id="link-source"
                  v-model="form.sourceUrl"
                  type="url"
                  placeholder="https://github.com/you/project"
                />
              </div>

              <div class="space-y-2">
                <Label for="link-issues">Issue tracker</Label>
                <Input
                  id="link-issues"
                  v-model="form.issuesUrl"
                  type="url"
                  placeholder="https://github.com/you/project/issues"
                />
              </div>

              <div class="space-y-2">
                <Label for="link-wiki">Wiki / documentation</Label>
                <Input
                  id="link-wiki"
                  v-model="form.wikiUrl"
                  type="url"
                  placeholder="https://example.com/wiki"
                />
              </div>

              <div class="space-y-2">
                <Label for="link-discord">Discord invite</Label>
                <Input
                  id="link-discord"
                  v-model="form.discordUrl"
                  type="url"
                  placeholder="https://discord.gg/invite"
                />
              </div>

              <div
                class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center"
                :class="linksError ? 'sm:justify-between' : 'sm:justify-end'"
              >
                <p v-if="linksError" class="text-destructive text-sm">
                  {{ linksError }}
                </p>
                <Button
                  class="btn-glow shrink-0"
                  :disabled="!linksDirty || savingLinks"
                  @click="handleSaveLinks"
                >
                  <Loader2 v-if="savingLinks" class="size-4 animate-spin" />
                  Save links
                </Button>
              </div>
            </section>

            <section
              v-else-if="activeSection === 'members'"
              class="card-glass space-y-5 rounded-2xl p-6"
            >
              <div>
                <h2 class="section-title mb-1 flex items-center gap-2 text-lg">
                  <Users class="text-primary size-5" />
                  Members
                </h2>
                <p class="text-muted-foreground text-sm leading-relaxed">
                  Invite collaborators to help manage this project. Members can
                  edit the project; the owner keeps full control.
                </p>
              </div>

              <form
                class="flex flex-col gap-3 sm:flex-row sm:items-end"
                @submit.prevent="handleAddMember"
              >
                <div class="flex-1 space-y-2">
                  <Label for="member-username">Add by username</Label>
                  <Input
                    id="member-username"
                    v-model="memberUsername"
                    placeholder="username"
                  />
                </div>
                <Button
                  type="submit"
                  class="btn-glow shrink-0"
                  :disabled="addingMember"
                >
                  <Loader2 v-if="addingMember" class="size-4 animate-spin" />
                  <UserPlus v-else class="size-4" />
                  Add member
                </Button>
              </form>
              <p v-if="memberAddError" class="text-destructive -mt-2 text-sm">
                {{ memberAddError }}
              </p>

              <div class="border-t pt-5">
                <p
                  v-if="membersPending && !members.length"
                  class="text-muted-foreground text-sm"
                >
                  Loading members…
                </p>
                <ul v-else class="space-y-3">
                  <li
                    v-for="member in members"
                    :key="member.user_id"
                    class="border-border/60 bg-muted/20 flex items-center justify-between gap-3 rounded-xl border p-3"
                  >
                    <div class="flex items-center gap-3">
                      <div
                        class="bg-primary/15 text-primary flex size-9 items-center justify-center rounded-full text-sm font-semibold uppercase"
                      >
                        {{ member.username.charAt(0) }}
                      </div>
                      <div>
                        <p class="text-sm font-medium">
                          {{ member.username }}
                        </p>
                        <p class="text-muted-foreground text-xs capitalize">
                          {{ member.role }}
                        </p>
                      </div>
                    </div>

                    <div
                      v-if="member.role !== 'owner'"
                      class="flex items-center gap-2"
                    >
                      <template v-if="confirmRemoveMemberId === member.user_id">
                        <Button
                          variant="destructive"
                          size="sm"
                          @click="
                            handleRemoveMember(member.user_id);
                            confirmRemoveMemberId = null;
                          "
                        >
                          Confirm
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          @click="confirmRemoveMemberId = null"
                        >
                          Cancel
                        </Button>
                      </template>
                      <Button
                        v-else
                        variant="ghost"
                        size="icon"
                        aria-label="Remove member"
                        @click="confirmRemoveMemberId = member.user_id"
                      >
                        <Trash2 class="text-destructive size-4" />
                      </Button>
                    </div>
                  </li>
                </ul>
              </div>
            </section>

            <section v-else class="space-y-6">
              <div class="card-glass space-y-1 rounded-2xl p-6">
                <h2 class="section-title flex items-center gap-2 text-lg">
                  <BarChart3 class="text-primary size-5" />
                  Analytics
                </h2>
                <p class="text-muted-foreground text-sm leading-relaxed">
                  Views and downloads over the last
                  {{ analytics?.range_days ?? 30 }} days. Only approved, public
                  projects collect stats.
                </p>
              </div>

              <p
                v-if="analyticsError"
                class="card-glass text-destructive rounded-2xl p-6 text-sm"
              >
                {{ analyticsError }}
              </p>

              <p
                v-else-if="analyticsPending && !analytics"
                class="text-muted-foreground card-glass rounded-2xl p-6 text-sm"
              >
                Loading analytics…
              </p>

              <template v-else-if="analytics">
                <div class="grid gap-4 sm:grid-cols-3">
                  <div class="card-glass rounded-2xl p-5">
                    <p class="text-muted-foreground text-xs">
                      Views ({{ analytics.range_days }}d)
                    </p>
                    <p class="mt-1 text-2xl font-semibold">
                      {{ analytics.total_views.toLocaleString() }}
                    </p>
                  </div>
                  <div class="card-glass rounded-2xl p-5">
                    <p class="text-muted-foreground text-xs">
                      Downloads ({{ analytics.range_days }}d)
                    </p>
                    <p class="mt-1 text-2xl font-semibold">
                      {{ analytics.total_downloads.toLocaleString() }}
                    </p>
                  </div>
                  <div class="card-glass rounded-2xl p-5">
                    <p class="text-muted-foreground text-xs">
                      Downloads (all time)
                    </p>
                    <p class="mt-1 text-2xl font-semibold">
                      {{ analytics.all_time_downloads.toLocaleString() }}
                    </p>
                  </div>
                </div>

                <div class="card-glass rounded-2xl p-6">
                  <div class="mb-4 flex items-center gap-4 text-xs">
                    <span class="flex items-center gap-1.5">
                      <span class="bg-primary size-2.5 rounded-full" />
                      Views
                    </span>
                    <span class="flex items-center gap-1.5">
                      <span class="size-2.5 rounded-full bg-emerald-500" />
                      Downloads
                    </span>
                  </div>
                  <div
                    class="flex h-40 items-end gap-1"
                    role="img"
                    aria-label="Daily views and downloads"
                  >
                    <div
                      v-for="point in analytics.series"
                      :key="point.day"
                      class="group relative flex h-full flex-1 items-end justify-center gap-0.5"
                      :title="`${point.day}: ${point.views} views, ${point.downloads} downloads`"
                    >
                      <div
                        class="bg-primary/70 w-1/2 rounded-t-sm"
                        :style="{
                          height: `${(point.views / analyticsMax) * 100}%`,
                        }"
                      />
                      <div
                        class="w-1/2 rounded-t-sm bg-emerald-500/70"
                        :style="{
                          height: `${(point.downloads / analyticsMax) * 100}%`,
                        }"
                      />
                    </div>
                  </div>
                  <div
                    class="text-muted-foreground mt-2 flex justify-between text-[11px]"
                  >
                    <span>{{ analytics.series[0]?.day }}</span>
                    <span>
                      {{ analytics.series[analytics.series.length - 1]?.day }}
                    </span>
                  </div>
                </div>
              </template>
            </section>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.markdown-preview :deep(h1),
.markdown-preview :deep(h2),
.markdown-preview :deep(h3),
.markdown-preview :deep(h4) {
  font-weight: 600;
  line-height: 1.25;
  margin: 1.25em 0 0.5em;
}

.markdown-preview :deep(h1) {
  font-size: 1.5rem;
}

.markdown-preview :deep(h2) {
  font-size: 1.3rem;
}

.markdown-preview :deep(h3) {
  font-size: 1.125rem;
}

.markdown-preview :deep(:first-child) {
  margin-top: 0;
}

.markdown-preview :deep(p) {
  margin: 0.75em 0;
  line-height: 1.65;
}

.markdown-preview :deep(ul),
.markdown-preview :deep(ol) {
  margin: 0.75em 0;
  padding-left: 1.5em;
}

.markdown-preview :deep(ul) {
  list-style: disc;
}

.markdown-preview :deep(ol) {
  list-style: decimal;
}

.markdown-preview :deep(li) {
  margin: 0.25em 0;
}

.markdown-preview :deep(a) {
  color: var(--color-primary);
  text-decoration: underline;
  text-underline-offset: 2px;
}

.markdown-preview :deep(blockquote) {
  border-left: 3px solid var(--color-border);
  margin: 0.75em 0;
  padding-left: 1em;
  color: var(--color-muted-foreground);
}

.markdown-preview :deep(code) {
  background: var(--color-muted);
  border-radius: 0.25rem;
  padding: 0.1em 0.35em;
  font-size: 0.875em;
}

.markdown-preview :deep(pre) {
  background: var(--color-muted);
  border-radius: 0.5rem;
  padding: 0.75em 1em;
  overflow-x: auto;
  margin: 0.75em 0;
}

.markdown-preview :deep(pre code) {
  background: transparent;
  padding: 0;
}

.markdown-preview :deep(img) {
  max-width: 100%;
  border-radius: 0.5rem;
}

.markdown-preview :deep(hr) {
  border: none;
  border-top: 1px solid var(--color-border);
  margin: 1.5em 0;
}

.markdown-preview :deep(table) {
  border-collapse: collapse;
  margin: 0.75em 0;
}

.markdown-preview :deep(th),
.markdown-preview :deep(td) {
  border: 1px solid var(--color-border);
  padding: 0.4em 0.75em;
}
</style>
