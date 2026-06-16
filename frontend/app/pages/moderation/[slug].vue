<script setup lang="ts">
import { toast } from "vue-sonner";
import {
  ArrowLeft,
  Ban,
  Boxes,
  Calendar,
  ChevronRight,
  CircleCheck,
  CircleX,
  Coins,
  Download,
  ExternalLink,
  Eye,
  FileArchive,
  FileDiff,
  FileSearch,
  Heart,
  Images,
  Layers,
  Link2,
  Loader2,
  MessageSquareWarning,
  Package2,
  ShieldCheck,
  StickyNote,
  TriangleAlert,
  User,
} from "@lucide/vue";
import {
  useProjectReview,
  useProjectPendingReview,
  useModeratorNotes,
  useVersionFile,
} from "~/scripts/pages/moderation";
import {
  diffLines,
  previewKind,
  type LineDiff,
  type FilePreviewKind,
} from "~/scripts/pages/diff";
import { useProject, projectTypeLabel } from "~/scripts/pages/projects";
import { useAuth } from "~/scripts/auth";

const route = useRoute();
const slug = computed(() => String(route.params.slug ?? ""));

const { isModerator } = useAuth();

const { project, load: loadProject } = useProject(slug.value);
const {
  data: pendingReview,
  error: pendingError,
  pending: pendingLoading,
  withBase: pendingWithBase,
  load: loadPendingReview,
} = useProjectPendingReview(slug.value);
const {
  submitting: reviewSubmitting,
  error: reviewError,
  review,
} = useProjectReview(slug.value);
const {
  notes: moderatorNotes,
  submitting: noteSubmitting,
  load: loadModeratorNotes,
  add: addModeratorNote,
} = useModeratorNotes(slug.value);

const { fetchText, fetchBlobUrl } = useVersionFile(slug.value);

await Promise.all([loadProject(), loadPendingReview(), loadModeratorNotes()]);

const reviewNotes = ref("");
const newNote = ref("");
const showDiffFiles = ref(false);

const submittedVersion = computed(
  () => pendingReview.value?.versions?.[0]?.version_number ?? null,
);
const previousVersion = computed(
  () => pendingReview.value?.versions?.[1]?.version_number ?? null,
);

interface FilePreviewState {
  path: string;
  kind: FilePreviewKind;
  loading: boolean;
  error: string;
  diff: LineDiff | null;
  oldImage: string | null;
  newImage: string | null;
}

const activeFile = ref<FilePreviewState | null>(null);

function releasePreview() {
  if (activeFile.value?.oldImage)
    URL.revokeObjectURL(activeFile.value.oldImage);
  if (activeFile.value?.newImage)
    URL.revokeObjectURL(activeFile.value.newImage);
}

async function toggleFile(path: string, status: string) {
  if (activeFile.value?.path === path) {
    releasePreview();
    activeFile.value = null;
    return;
  }

  releasePreview();
  const kind = previewKind(path);
  const state: FilePreviewState = {
    path,
    kind,
    loading: true,
    error: "",
    diff: null,
    oldImage: null,
    newImage: null,
  };
  activeFile.value = state;

  if (kind === "binary") {
    state.loading = false;
    return;
  }

  const newVer = submittedVersion.value;
  const oldVer = previousVersion.value;
  const wantOld = status !== "added" && !!oldVer;
  const wantNew = status !== "removed" && !!newVer;

  try {
    if (kind === "text") {
      const [oldText, newText] = await Promise.all([
        wantOld ? fetchText(oldVer as string, path) : Promise.resolve(""),
        wantNew ? fetchText(newVer as string, path) : Promise.resolve(""),
      ]);
      state.diff = diffLines(oldText, newText);
    } else {
      const [oldUrl, newUrl] = await Promise.all([
        wantOld ? fetchBlobUrl(oldVer as string, path) : Promise.resolve(null),
        wantNew ? fetchBlobUrl(newVer as string, path) : Promise.resolve(null),
      ]);
      state.oldImage = oldUrl;
      state.newImage = newUrl;
    }
  } catch {
    state.error = "Could not load this file.";
  } finally {
    if (activeFile.value === state) state.loading = false;
  }
}

onBeforeUnmount(releasePreview);

async function submitNote() {
  const body = newNote.value.trim();
  if (!body) return;
  const ok = await addModeratorNote(body);
  if (ok) {
    newNote.value = "";
    toast.success("Note added.");
  } else {
    toast.error("Could not save your note.");
  }
}

const typeLabel = computed(() =>
  project.value ? projectTypeLabel(project.value.project_type) : "",
);

function relativeTime(iso: string | null): string {
  if (!iso) return "";
  const then = new Date(iso).getTime();
  if (Number.isNaN(then)) return "";
  const sec = Math.floor((Date.now() - then) / 1000);
  if (sec < 60) return "just now";
  const min = Math.floor(sec / 60);
  if (min < 60) return `${min} minute${min === 1 ? "" : "s"} ago`;
  const hr = Math.floor(min / 60);
  if (hr < 24) return `${hr} hour${hr === 1 ? "" : "s"} ago`;
  const day = Math.floor(hr / 24);
  return `${day} day${day === 1 ? "" : "s"} ago`;
}

function formatDate(iso: string | null): string {
  if (!iso) return "-";
  const d = new Date(iso);
  if (Number.isNaN(d.getTime())) return "-";
  return d.toLocaleDateString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

const VISIBILITY_LABELS: Record<string, string> = {
  public: "Public",
  unlisted: "Unlisted",
  private: "Private",
};

const REVIEW_ACTION_META: Record<string, { label: string; class: string }> = {
  approve: {
    label: "Approved",
    class: "bg-emerald-500/15 text-emerald-400",
  },
  reject: { label: "Rejected", class: "bg-red-500/15 text-red-400" },
  request_changes: {
    label: "Changes requested",
    class: "bg-amber-500/15 text-amber-500",
  },
};

function actionMeta(action: string) {
  return (
    REVIEW_ACTION_META[action] ?? {
      label: action,
      class: "bg-muted text-muted-foreground",
    }
  );
}

const CHANNEL_META: Record<string, { label: string; class: string }> = {
  release: { label: "Release", class: "bg-emerald-500/15 text-emerald-400" },
  beta: { label: "Beta", class: "bg-sky-500/15 text-sky-400" },
  alpha: { label: "Alpha", class: "bg-amber-500/15 text-amber-500" },
};

function channelMeta(channel: string) {
  return (
    CHANNEL_META[channel] ?? {
      label: channel,
      class: "bg-muted text-muted-foreground",
    }
  );
}

function formatBytes(bytes: number): string {
  if (!bytes) return "0 B";
  const units = ["B", "KB", "MB", "GB"];
  const i = Math.min(
    units.length - 1,
    Math.floor(Math.log(bytes) / Math.log(1024)),
  );
  const value = bytes / Math.pow(1024, i);
  return `${value.toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

const DECISION_META: Record<
  string,
  { label: string; class: string; icon: typeof CircleCheck }
> = {
  pass: {
    label: "Passed validation",
    class: "border-emerald-500/30 bg-emerald-500/10 text-emerald-400",
    icon: CircleCheck,
  },
  warn: {
    label: "Passed with warnings",
    class: "border-amber-500/30 bg-amber-500/10 text-amber-500",
    icon: TriangleAlert,
  },
  fail: {
    label: "Validation errors found",
    class: "border-red-500/30 bg-red-500/10 text-red-400",
    icon: CircleX,
  },
};

function decisionMeta(decision: string) {
  return (
    DECISION_META[decision] ?? {
      label: "Passed with warnings",
      class: "border-amber-500/30 bg-amber-500/10 text-amber-500",
      icon: TriangleAlert,
    }
  );
}

const FINDING_CLASS: Record<string, string> = {
  error: "text-red-400",
  testFail: "text-red-400",
  warn: "text-amber-500",
  warning: "text-amber-500",
  testWarn: "text-amber-500",
  recommendation: "text-sky-400",
};

function findingClass(type: string): string {
  return FINDING_CLASS[type] ?? "text-muted-foreground";
}

const DIFF_STATUS_META: Record<string, { label: string; class: string }> = {
  added: { label: "Added", class: "bg-emerald-500/15 text-emerald-400" },
  removed: { label: "Removed", class: "bg-red-500/15 text-red-400" },
  modified: { label: "Modified", class: "bg-amber-500/15 text-amber-500" },
};

function diffStatusMeta(status: string) {
  return (
    DIFF_STATUS_META[status] ?? {
      label: status,
      class: "bg-muted text-muted-foreground",
    }
  );
}

const KIND_LABELS: Record<string, string> = {
  manifest: "Manifests",
  model: "Models",
  entity: "Entities",
  block: "Blocks",
  item: "Items",
  texture: "Textures",
  animation: "Animations",
  render_controller: "Render controllers",
  particle: "Particles",
  sound: "Sounds",
  function: "Functions",
  lang: "Language files",
  material: "Materials",
  other: "Other",
};

function kindLabel(kind: string): string {
  return KIND_LABELS[kind] ?? kind;
}

type SignalStatus = "pass" | "warn" | "fail" | "pending" | "neutral";

interface DecisionCheck {
  label: string;
  status: SignalStatus;
  detail: string;
}

const SIGNAL_RANK: Record<SignalStatus, number> = {
  pass: 0,
  neutral: 0,
  pending: 1,
  warn: 2,
  fail: 3,
};

const SIGNAL_META: Record<
  SignalStatus,
  {
    label: string;
    sub: string;
    class: string;
    badge: string;
    icon: typeof CircleCheck;
  }
> = {
  pass: {
    label: "Looks good",
    sub: "No blocking issues detected by the automated checks.",
    class: "border-emerald-500/30 bg-emerald-500/10 text-emerald-400",
    badge: "bg-emerald-500/15 text-emerald-400",
    icon: CircleCheck,
  },
  warn: {
    label: "Passed with warnings",
    sub: "Review the flagged checks below before approving.",
    class: "border-amber-500/30 bg-amber-500/10 text-amber-500",
    badge: "bg-amber-500/15 text-amber-500",
    icon: TriangleAlert,
  },
  fail: {
    label: "Issues found",
    sub: "Automated checks flagged problems - review carefully.",
    class: "border-red-500/30 bg-red-500/10 text-red-400",
    badge: "bg-red-500/15 text-red-400",
    icon: CircleX,
  },
  pending: {
    label: "Validation pending",
    sub: "Full signal not ready - analysis is still running.",
    class: "border-sky-500/30 bg-sky-500/10 text-sky-400",
    badge: "bg-sky-500/15 text-sky-400",
    icon: Loader2,
  },
  neutral: {
    label: "No signal yet",
    sub: "Not enough data to make an automated assessment.",
    class: "border-border/60 bg-card/40 text-muted-foreground",
    badge: "bg-muted text-muted-foreground",
    icon: TriangleAlert,
  },
};

function signalMeta(status: SignalStatus) {
  return SIGNAL_META[status];
}

const CHECK_STATUS_LABEL: Record<SignalStatus, string> = {
  pass: "Pass",
  warn: "Warn",
  fail: "Fail",
  pending: "Pending",
  neutral: "N/A",
};

function checkStatusLabel(status: SignalStatus): string {
  return CHECK_STATUS_LABEL[status];
}

function plural(n: number, word: string): string {
  return `${n} ${word}${n === 1 ? "" : "s"}`;
}

const decisionSignal = computed<{
  overall: SignalStatus;
  checks: DecisionCheck[];
}>(() => {
  const review = pendingReview.value;
  const checks: DecisionCheck[] = [];

  const analysis = review?.analysis ?? null;
  let validationStatus: SignalStatus = "pending";
  if (!analysis || analysis.status === "pending") {
    checks.push({
      label: "MCTools validation",
      status: "pending",
      detail: analysis ? "Analysis in progress" : "Not yet run",
    });
  } else if (analysis.status === "error") {
    validationStatus = "warn";
    checks.push({
      label: "MCTools validation",
      status: "warn",
      detail: "Could not validate - review manually",
    });
  } else if (analysis.report) {
    const decision = analysis.report.decision;
    validationStatus =
      decision === "fail" ? "fail" : decision === "warn" ? "warn" : "pass";
    const counts = analysis.report.counts;
    checks.push({
      label: "MCTools validation",
      status: validationStatus,
      detail:
        decision === "pass"
          ? "No errors or warnings"
          : `${plural(counts.errors, "error")}, ${plural(counts.warnings, "warning")}`,
    });
  } else {
    checks.push({
      label: "MCTools validation",
      status: "pending",
      detail: "No report available",
    });
  }

  if (analysis?.status === "ready" && analysis.report) {
    const counts = analysis.report.counts;
    if (counts.testFail > 0) {
      checks.push({
        label: "Automated tests",
        status: "fail",
        detail: `${counts.testFail} failed, ${counts.testSuccess} passed`,
      });
    } else if (counts.testSuccess > 0) {
      checks.push({
        label: "Automated tests",
        status: "pass",
        detail: plural(counts.testSuccess, "test") + " passed",
      });
    } else {
      checks.push({
        label: "Automated tests",
        status: "neutral",
        detail: "No tests applicable",
      });
    }
  }

  const diff = review?.pack_diff ?? null;
  if (diff?.files_truncated) {
    checks.push({
      label: "Pack contents",
      status: "warn",
      detail: "Too many changes to index fully",
    });
  } else if (diff && diff.added + diff.removed + diff.modified > 0) {
    checks.push({
      label: "Pack contents",
      status: "pass",
      detail:
        plural(diff.added + diff.removed + diff.modified, "file change") +
        " indexed",
    });
  }

  const note = review?.changelog?.trim() ?? "";
  checks.push(
    note
      ? { label: "Creator note", status: "pass", detail: "Provided" }
      : {
          label: "Creator note",
          status: "warn",
          detail: "No changelog provided",
        },
  );

  const overall: SignalStatus =
    validationStatus === "pending"
      ? "pending"
      : checks.reduce<SignalStatus>(
          (worst, check) =>
            SIGNAL_RANK[check.status] > SIGNAL_RANK[worst]
              ? check.status
              : worst,
          "pass",
        );

  return { overall, checks };
});

const links = computed(() => {
  const l = pendingReview.value?.links;
  if (!l) return [] as { label: string; url: string }[];
  return [
    { label: "Website", url: l.website_url },
    { label: "Source code", url: l.source_url },
    { label: "Issue tracker", url: l.issues_url },
    { label: "Wiki", url: l.wiki_url },
    { label: "Discord", url: l.discord_url },
  ].filter((link) => link.url.trim().length > 0);
});

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
    await navigateTo("/moderation");
  } else if (reviewError.value) {
    toast.error(reviewError.value);
  }
}
</script>

<template>
  <div class="page-canvas min-h-screen">
    <div class="mx-auto max-w-5xl px-6 py-10">
      <NuxtLink
        to="/moderation"
        class="text-muted-foreground hover:text-foreground mb-6 inline-flex items-center gap-2 text-sm transition-colors"
      >
        <ArrowLeft class="size-4" />
        Back to queue
      </NuxtLink>

      <div
        v-if="pendingLoading"
        class="text-muted-foreground flex items-center gap-2 py-20"
      >
        <Loader2 class="size-5 animate-spin" />
        Loading the review…
      </div>

      <div
        v-else-if="pendingError"
        class="border-border/60 rounded-xl border p-10 text-center"
      >
        <p class="text-muted-foreground">{{ pendingError }}</p>
        <NuxtLink
          v-if="!isModerator"
          to="/"
          class="text-primary mt-3 inline-block text-sm hover:underline"
        >
          Back to home
        </NuxtLink>
      </div>

      <template v-else-if="pendingReview && project">
        <!-- Header -->
        <div class="mb-6 flex flex-wrap items-start gap-4">
          <div
            v-if="pendingWithBase(pendingReview.pending.icon_url)"
            class="size-16 shrink-0 overflow-hidden rounded-2xl ring-1 ring-white/10"
          >
            <img
              :src="pendingWithBase(pendingReview.pending.icon_url)!"
              :alt="project.title"
              class="size-full object-cover"
            />
          </div>
          <div
            v-else
            class="bg-primary/15 text-primary flex size-16 shrink-0 items-center justify-center rounded-2xl"
          >
            <ShieldCheck class="size-7" />
          </div>

          <div class="min-w-0 flex-1">
            <div class="flex flex-wrap items-center gap-2">
              <h1 class="display-heading text-2xl sm:text-3xl">
                {{ project.title }}
              </h1>
              <span
                class="bg-primary/15 text-primary rounded-full px-2.5 py-0.5 text-xs font-semibold"
              >
                {{ typeLabel }}
              </span>
              <span
                class="rounded-full bg-amber-500/15 px-2.5 py-0.5 text-xs font-semibold text-amber-500"
              >
                Awaiting review
              </span>
            </div>
            <p class="text-muted-foreground mt-1 text-sm">
              by {{ project.owner }} ·
              {{
                pendingReview.is_first_review ? "First submission" : "Update"
              }}
              <template v-if="pendingReview.submitted_at">
                · submitted {{ relativeTime(pendingReview.submitted_at) }}
              </template>
            </p>
            <div class="mt-2 flex flex-wrap items-center gap-x-4 gap-y-1">
              <NuxtLink
                :to="`/projects/${slug}`"
                class="text-primary inline-flex items-center gap-1.5 text-sm hover:underline"
              >
                <ExternalLink class="size-3.5" />
                View current PDP
              </NuxtLink>
              <NuxtLink
                :to="`/projects/${slug}?preview=pending`"
                class="text-primary inline-flex items-center gap-1.5 text-sm hover:underline"
              >
                <Eye class="size-3.5" />
                Preview approved PDP
              </NuxtLink>
            </div>
          </div>
        </div>

        <div class="grid gap-6 lg:grid-cols-3">
          <!-- Main report column -->
          <div class="space-y-6 lg:col-span-2">
            <!-- Decision signal -->
            <div
              class="rounded-2xl border p-5"
              :class="signalMeta(decisionSignal.overall).class"
            >
              <div class="flex items-start gap-3">
                <component
                  :is="signalMeta(decisionSignal.overall).icon"
                  class="mt-0.5 size-6 shrink-0"
                  :class="{
                    'animate-spin': decisionSignal.overall === 'pending',
                  }"
                />
                <div class="min-w-0 flex-1">
                  <p class="text-base font-semibold">
                    {{ signalMeta(decisionSignal.overall).label }}
                  </p>
                  <p class="text-sm opacity-90">
                    {{ signalMeta(decisionSignal.overall).sub }}
                  </p>
                </div>
              </div>

              <ul class="border-current/15 mt-4 space-y-1.5 border-t pt-3">
                <li
                  v-for="check in decisionSignal.checks"
                  :key="check.label"
                  class="flex items-center justify-between gap-3 text-sm"
                >
                  <span class="flex shrink-0 items-center gap-2">
                    <span
                      class="rounded-full px-2 py-0.5 text-xs font-semibold"
                      :class="signalMeta(check.status).badge"
                    >
                      {{ checkStatusLabel(check.status) }}
                    </span>
                    <span class="text-foreground">{{ check.label }}</span>
                  </span>
                  <span
                    class="text-muted-foreground truncate text-right text-xs"
                  >
                    {{ check.detail }}
                  </span>
                </li>
              </ul>
            </div>

            <!-- Creator's note -->
            <div class="border-border/60 bg-card/40 rounded-2xl border p-5">
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

            <!-- Field changes -->
            <div class="border-border/60 overflow-hidden rounded-2xl border">
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
                      {{ row.before || "-" }}
                    </td>
                    <td
                      class="text-foreground px-3 py-2 break-words whitespace-pre-wrap"
                      :class="row.changed ? 'font-medium' : ''"
                    >
                      {{ row.after || "-" }}
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>

            <!-- Icon diff -->
            <div
              v-if="pendingReview.icon_changed"
              class="border-border/60 bg-card/40 flex items-center gap-4 rounded-2xl border p-5"
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

            <!-- Links -->
            <div
              v-if="links.length"
              class="border-border/60 bg-card/40 rounded-2xl border p-5"
            >
              <p
                class="text-muted-foreground mb-3 text-xs font-semibold tracking-wide uppercase"
              >
                Links
              </p>
              <ul class="space-y-2">
                <li v-for="link in links" :key="link.label">
                  <a
                    :href="link.url"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-primary inline-flex items-center gap-1.5 text-sm hover:underline"
                  >
                    <Link2 class="size-3.5" />
                    <span class="text-muted-foreground">{{ link.label }}:</span>
                    <span class="truncate">{{ link.url }}</span>
                    <ExternalLink class="size-3" />
                  </a>
                </li>
              </ul>
            </div>

            <!-- Gallery -->
            <div class="border-border/60 bg-card/40 rounded-2xl border p-5">
              <p
                class="text-muted-foreground mb-3 inline-flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
              >
                <Images class="size-3.5" /> Gallery
                <span class="text-muted-foreground/70 normal-case"
                  >({{ pendingReview.gallery.length }})</span
                >
              </p>
              <p
                v-if="!pendingReview.gallery.length"
                class="text-muted-foreground text-sm"
              >
                No gallery images.
              </p>
              <div v-else class="grid grid-cols-2 gap-3 sm:grid-cols-3">
                <figure
                  v-for="image in pendingReview.gallery"
                  :key="image.id"
                  class="border-border/40 bg-background/40 overflow-hidden rounded-lg border"
                >
                  <a
                    :href="pendingWithBase(image.url)!"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="block aspect-video overflow-hidden"
                  >
                    <img
                      :src="pendingWithBase(image.url)!"
                      :alt="image.caption || 'Gallery image'"
                      loading="lazy"
                      class="size-full object-cover transition-transform hover:scale-105"
                    />
                  </a>
                  <figcaption
                    v-if="image.caption.trim()"
                    class="text-muted-foreground px-2 py-1.5 text-xs"
                  >
                    {{ image.caption }}
                  </figcaption>
                </figure>
              </div>
            </div>

            <!-- Versions -->
            <div class="border-border/60 bg-card/40 rounded-2xl border p-5">
              <p
                class="text-muted-foreground mb-3 inline-flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
              >
                <Layers class="size-3.5" /> Versions
                <span class="text-muted-foreground/70 normal-case"
                  >({{ pendingReview.versions.length }})</span
                >
              </p>
              <p
                v-if="!pendingReview.versions.length"
                class="text-muted-foreground text-sm"
              >
                No versions uploaded.
              </p>
              <ul v-else class="space-y-3">
                <li
                  v-for="version in pendingReview.versions"
                  :key="version.version_number"
                  class="border-border/40 border-l-2 pl-3"
                >
                  <div class="flex flex-wrap items-center gap-2">
                    <span class="text-foreground font-semibold">
                      {{ version.version_number }}
                    </span>
                    <span
                      v-if="version.name.trim()"
                      class="text-muted-foreground text-sm"
                    >
                      {{ version.name }}
                    </span>
                    <span
                      class="rounded-full px-2 py-0.5 text-xs font-semibold"
                      :class="channelMeta(version.channel).class"
                    >
                      {{ channelMeta(version.channel).label }}
                    </span>
                    <span class="text-muted-foreground text-xs">
                      {{ relativeTime(version.created_at) }}
                    </span>
                  </div>
                  <p
                    v-if="version.changelog.trim()"
                    class="text-muted-foreground mt-1 text-sm whitespace-pre-wrap"
                  >
                    {{ version.changelog }}
                  </p>
                  <p
                    v-if="version.file"
                    class="text-muted-foreground/80 mt-1.5 inline-flex items-center gap-1.5 text-xs"
                  >
                    <FileArchive class="size-3.5" />
                    {{ version.file.filename }} ·
                    {{ formatBytes(version.file.size) }}
                  </p>
                </li>
              </ul>
            </div>

            <!-- Pack analysis -->
            <div
              v-if="pendingReview.analysis"
              class="border-border/60 bg-card/40 rounded-2xl border p-5"
            >
              <p
                class="text-muted-foreground mb-3 inline-flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
              >
                <FileSearch class="size-3.5" /> Pack analysis
                <span
                  v-if="pendingReview.analysis.mctools_version"
                  class="text-muted-foreground/70 normal-case"
                  >(Creator Tools
                  {{ pendingReview.analysis.mctools_version }})</span
                >
              </p>

              <p
                v-if="pendingReview.analysis.status === 'pending'"
                class="text-muted-foreground inline-flex items-center gap-2 text-sm"
              >
                <Loader2 class="size-4 animate-spin" /> Analysis in progress -
                check back shortly.
              </p>

              <div
                v-else-if="pendingReview.analysis.status === 'error'"
                class="rounded-lg border border-red-500/30 bg-red-500/10 p-3 text-sm text-red-400"
              >
                <p class="inline-flex items-center gap-1.5 font-medium">
                  <CircleX class="size-4" /> Analysis failed
                </p>
                <p class="text-muted-foreground mt-1 text-xs break-words">
                  {{ pendingReview.analysis.error || "Unknown error" }}
                </p>
              </div>

              <template
                v-else-if="
                  pendingReview.analysis.status === 'ready' &&
                  pendingReview.analysis.report
                "
              >
                <!-- Decision banner -->
                <div
                  class="inline-flex items-center gap-2 rounded-lg border px-3 py-2 text-sm font-semibold"
                  :class="
                    decisionMeta(pendingReview.analysis.report.decision).class
                  "
                >
                  <component
                    :is="
                      decisionMeta(pendingReview.analysis.report.decision).icon
                    "
                    class="size-4"
                  />
                  {{
                    decisionMeta(pendingReview.analysis.report.decision).label
                  }}
                </div>

                <!-- Counts -->
                <div class="mt-3 flex flex-wrap gap-2 text-xs">
                  <span
                    class="rounded-full bg-red-500/15 px-2 py-0.5 font-medium text-red-400"
                  >
                    {{ pendingReview.analysis.report.counts.errors }} errors
                  </span>
                  <span
                    class="rounded-full bg-amber-500/15 px-2 py-0.5 font-medium text-amber-500"
                  >
                    {{ pendingReview.analysis.report.counts.warnings }} warnings
                  </span>
                  <span
                    class="rounded-full bg-sky-500/15 px-2 py-0.5 font-medium text-sky-400"
                  >
                    {{ pendingReview.analysis.report.counts.recommendations }}
                    recommendations
                  </span>
                  <span
                    class="bg-muted text-muted-foreground rounded-full px-2 py-0.5 font-medium"
                  >
                    {{ pendingReview.analysis.report.counts.testSuccess }}/{{
                      pendingReview.analysis.report.counts.testSuccess +
                      pendingReview.analysis.report.counts.testFail
                    }}
                    checks passed
                  </span>
                </div>

                <!-- Pack info -->
                <dl
                  class="text-muted-foreground mt-4 grid grid-cols-2 gap-x-4 gap-y-1.5 text-xs"
                >
                  <div class="flex items-center justify-between gap-2">
                    <dt class="inline-flex items-center gap-1">
                      <Package2 class="size-3" /> Behavior packs
                    </dt>
                    <dd class="text-foreground font-medium">
                      {{
                        pendingReview.analysis.report.info
                          .behaviorPackManifestCount
                      }}
                    </dd>
                  </div>
                  <div class="flex items-center justify-between gap-2">
                    <dt class="inline-flex items-center gap-1">
                      <Images class="size-3" /> Resource packs
                    </dt>
                    <dd class="text-foreground font-medium">
                      {{
                        pendingReview.analysis.report.info
                          .resourcePackManifestCount
                      }}
                    </dd>
                  </div>
                  <div class="flex items-center justify-between gap-2">
                    <dt class="inline-flex items-center gap-1">
                      <FileArchive class="size-3" /> Total size
                    </dt>
                    <dd class="text-foreground font-medium">
                      {{
                        formatBytes(
                          pendingReview.analysis.report.info.overallSize,
                        )
                      }}
                    </dd>
                  </div>
                  <div class="flex items-center justify-between gap-2">
                    <dt class="inline-flex items-center gap-1">
                      <Boxes class="size-3" /> Textures
                    </dt>
                    <dd class="text-foreground font-medium">
                      {{ pendingReview.analysis.report.info.textureCount }}
                    </dd>
                  </div>
                </dl>

                <!-- Capabilities / APIs -->
                <div
                  v-if="
                    pendingReview.analysis.report.info.capabilities.length ||
                    pendingReview.analysis.report.info.apisUsed.length
                  "
                  class="mt-3 flex flex-wrap gap-1.5"
                >
                  <span
                    v-for="cap in pendingReview.analysis.report.info
                      .capabilities"
                    :key="`cap-${cap}`"
                    class="bg-primary/10 text-primary rounded-full px-2 py-0.5 text-xs font-medium"
                  >
                    {{ cap }}
                  </span>
                  <span
                    v-for="api in pendingReview.analysis.report.info.apisUsed"
                    :key="`api-${api}`"
                    class="bg-muted text-muted-foreground rounded-full px-2 py-0.5 text-xs font-medium"
                  >
                    {{ api }}
                  </span>
                </div>

                <!-- Findings -->
                <ul
                  v-if="pendingReview.analysis.report.findings.length"
                  class="border-border/40 mt-4 space-y-1.5 border-t pt-3"
                >
                  <li
                    v-for="(finding, i) in pendingReview.analysis.report
                      .findings"
                    :key="`finding-${i}`"
                    class="flex items-start gap-2 text-xs"
                  >
                    <span
                      class="mt-px font-semibold"
                      :class="findingClass(finding.type)"
                    >
                      {{ finding.generatorId }}
                    </span>
                    <span class="text-muted-foreground break-words">
                      {{ finding.message }}
                    </span>
                  </li>
                </ul>
              </template>
            </div>

            <!-- Pack file diff -->
            <div
              v-if="pendingReview.pack_diff"
              class="border-border/60 bg-card/40 rounded-2xl border p-5"
            >
              <p
                class="text-muted-foreground mb-3 inline-flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
              >
                <FileDiff class="size-3.5" /> Pack changes
                <span class="text-muted-foreground/70 normal-case">
                  vs previous version
                </span>
              </p>

              <p
                v-if="
                  !pendingReview.pack_diff.added &&
                  !pendingReview.pack_diff.removed &&
                  !pendingReview.pack_diff.modified
                "
                class="text-muted-foreground text-sm"
              >
                No file changes detected
                <template v-if="!pendingReview.pack_diff.unchanged">
                  - file index not yet available.
                </template>
                <template v-else>
                  ({{ pendingReview.pack_diff.unchanged }} files unchanged).
                </template>
              </p>

              <template v-else>
                <!-- Totals -->
                <div class="flex flex-wrap gap-2 text-xs">
                  <span
                    class="rounded-full bg-emerald-500/15 px-2 py-0.5 font-medium text-emerald-400"
                  >
                    +{{ pendingReview.pack_diff.added }} added
                  </span>
                  <span
                    class="rounded-full bg-red-500/15 px-2 py-0.5 font-medium text-red-400"
                  >
                    −{{ pendingReview.pack_diff.removed }} removed
                  </span>
                  <span
                    class="rounded-full bg-amber-500/15 px-2 py-0.5 font-medium text-amber-500"
                  >
                    ~{{ pendingReview.pack_diff.modified }} modified
                  </span>
                  <span
                    class="bg-muted text-muted-foreground rounded-full px-2 py-0.5 font-medium"
                  >
                    {{ pendingReview.pack_diff.unchanged }} unchanged
                  </span>
                </div>

                <!-- Per-kind rollup -->
                <ul
                  v-if="pendingReview.pack_diff.by_kind.length"
                  class="mt-3 space-y-1 text-xs"
                >
                  <li
                    v-for="row in pendingReview.pack_diff.by_kind"
                    :key="`kind-${row.kind}`"
                    class="flex items-center justify-between gap-2"
                  >
                    <span class="text-muted-foreground">
                      {{ kindLabel(row.kind) }}
                    </span>
                    <span class="flex items-center gap-1.5 font-medium">
                      <span v-if="row.added" class="text-emerald-400"
                        >+{{ row.added }}</span
                      >
                      <span v-if="row.removed" class="text-red-400"
                        >−{{ row.removed }}</span
                      >
                      <span v-if="row.modified" class="text-amber-500"
                        >~{{ row.modified }}</span
                      >
                    </span>
                  </li>
                </ul>

                <!-- File list (collapsible) -->
                <div class="border-border/40 mt-4 border-t pt-3">
                  <button
                    type="button"
                    class="text-muted-foreground hover:text-foreground inline-flex items-center gap-1.5 text-xs font-medium"
                    @click="showDiffFiles = !showDiffFiles"
                  >
                    <Layers class="size-3.5" />
                    {{ showDiffFiles ? "Hide" : "Show" }} changed files ({{
                      pendingReview.pack_diff.files.length
                    }})
                  </button>

                  <ul v-if="showDiffFiles" class="mt-2 space-y-1">
                    <li
                      v-for="file in pendingReview.pack_diff.files"
                      :key="`diff-${file.status}-${file.path}`"
                    >
                      <button
                        type="button"
                        class="hover:bg-muted/40 flex w-full items-center justify-between gap-2 rounded px-1 py-0.5 text-left text-xs"
                        @click="toggleFile(file.path, file.status)"
                      >
                        <span class="flex min-w-0 items-center gap-2">
                          <ChevronRight
                            class="size-3 shrink-0 transition-transform"
                            :class="{
                              'rotate-90': activeFile?.path === file.path,
                            }"
                          />
                          <span
                            class="rounded px-1.5 py-0.5 font-semibold"
                            :class="diffStatusMeta(file.status).class"
                          >
                            {{ diffStatusMeta(file.status).label }}
                          </span>
                          <span
                            class="text-muted-foreground truncate font-mono"
                            :title="file.path"
                          >
                            {{ file.path }}
                          </span>
                        </span>
                        <span
                          v-if="file.status === 'modified'"
                          class="text-muted-foreground/70 shrink-0"
                        >
                          {{ formatBytes(file.old_size ?? 0) }} →
                          {{ formatBytes(file.new_size ?? 0) }}
                        </span>
                        <span
                          v-else-if="file.status === 'added'"
                          class="shrink-0 text-emerald-400/80"
                        >
                          {{ formatBytes(file.new_size ?? 0) }}
                        </span>
                        <span v-else class="shrink-0 text-red-400/80">
                          {{ formatBytes(file.old_size ?? 0) }}
                        </span>
                      </button>

                      <!-- Per-file preview -->
                      <div
                        v-if="activeFile?.path === file.path"
                        class="border-border/40 bg-background/40 mt-1 rounded-lg border p-2"
                      >
                        <p
                          v-if="activeFile.loading"
                          class="text-muted-foreground inline-flex items-center gap-2 text-xs"
                        >
                          <Loader2 class="size-3.5 animate-spin" /> Loading…
                        </p>
                        <p
                          v-else-if="activeFile.error"
                          class="text-xs text-red-400"
                        >
                          {{ activeFile.error }}
                        </p>

                        <!-- Text diff -->
                        <template
                          v-else-if="
                            activeFile.kind === 'text' && activeFile.diff
                          "
                        >
                          <p
                            v-if="activeFile.diff.tooLarge"
                            class="text-muted-foreground text-xs"
                          >
                            File too large to diff inline - sha changed.
                          </p>
                          <template v-else>
                            <p class="text-muted-foreground mb-1 text-xs">
                              <span class="text-emerald-400"
                                >+{{ activeFile.diff.added }}</span
                              >
                              /
                              <span class="text-red-400"
                                >−{{ activeFile.diff.removed }}</span
                              >
                              lines
                            </p>
                            <div
                              class="max-h-80 overflow-auto rounded font-mono text-[11px] leading-relaxed"
                            >
                              <div
                                v-for="(line, li) in activeFile.diff.lines"
                                :key="`line-${li}`"
                                class="flex"
                                :class="{
                                  'bg-emerald-500/10 text-emerald-300':
                                    line.type === 'add',
                                  'bg-red-500/10 text-red-300':
                                    line.type === 'remove',
                                  'text-muted-foreground':
                                    line.type === 'context',
                                }"
                              >
                                <span
                                  class="text-muted-foreground/50 w-8 shrink-0 select-none pr-1 text-right"
                                  >{{ line.oldNumber ?? "" }}</span
                                >
                                <span
                                  class="text-muted-foreground/50 w-8 shrink-0 select-none pr-2 text-right"
                                  >{{ line.newNumber ?? "" }}</span
                                >
                                <span class="w-3 shrink-0 select-none">{{
                                  line.type === "add"
                                    ? "+"
                                    : line.type === "remove"
                                      ? "−"
                                      : " "
                                }}</span>
                                <span class="whitespace-pre-wrap break-all">{{
                                  line.text
                                }}</span>
                              </div>
                            </div>
                            <p
                              v-if="activeFile.diff.truncated"
                              class="text-muted-foreground/70 mt-1 text-xs italic"
                            >
                              Diff truncated - download to view the full file.
                            </p>
                          </template>
                        </template>

                        <!-- Image before/after -->
                        <div
                          v-else-if="activeFile.kind === 'image'"
                          class="grid grid-cols-2 gap-2"
                        >
                          <div>
                            <p class="text-muted-foreground mb-1 text-xs">
                              Before
                            </p>
                            <img
                              v-if="activeFile.oldImage"
                              :src="activeFile.oldImage"
                              alt="previous version"
                              class="border-border/40 max-h-40 rounded border bg-[repeating-conic-gradient(#0002_0_25%,transparent_0_50%)] bg-[length:16px_16px] object-contain"
                            />
                            <p v-else class="text-muted-foreground/60 text-xs">
                              (new file)
                            </p>
                          </div>
                          <div>
                            <p class="text-muted-foreground mb-1 text-xs">
                              After
                            </p>
                            <img
                              v-if="activeFile.newImage"
                              :src="activeFile.newImage"
                              alt="submitted version"
                              class="border-border/40 max-h-40 rounded border bg-[repeating-conic-gradient(#0002_0_25%,transparent_0_50%)] bg-[length:16px_16px] object-contain"
                            />
                            <p v-else class="text-muted-foreground/60 text-xs">
                              (removed)
                            </p>
                          </div>
                        </div>

                        <!-- Binary -->
                        <p v-else class="text-muted-foreground text-xs">
                          Binary file - contents changed (no inline preview).
                        </p>
                      </div>
                    </li>
                  </ul>

                  <p
                    v-if="
                      showDiffFiles && pendingReview.pack_diff.files_truncated
                    "
                    class="text-muted-foreground/70 mt-2 text-xs italic"
                  >
                    File list truncated - too many changes to display in full.
                  </p>
                </div>
              </template>
            </div>
          </div>

          <!-- Sidebar context -->
          <aside class="space-y-6">
            <!-- Owner -->
            <div class="border-border/60 bg-card/40 rounded-2xl border p-5">
              <p
                class="text-muted-foreground mb-3 text-xs font-semibold tracking-wide uppercase"
              >
                Creator
              </p>
              <div class="mb-3 flex items-center gap-2">
                <div
                  class="bg-primary/15 text-primary flex size-9 items-center justify-center rounded-full"
                >
                  <User class="size-4" />
                </div>
                <div class="min-w-0">
                  <p class="text-foreground truncate font-medium">
                    {{ pendingReview.owner.username }}
                  </p>
                  <p class="text-muted-foreground text-xs">
                    Joined {{ formatDate(pendingReview.owner.member_since) }}
                  </p>
                </div>
              </div>
              <dl class="space-y-1.5 text-sm">
                <div class="flex items-center justify-between">
                  <dt class="text-muted-foreground">Projects</dt>
                  <dd class="text-foreground font-medium">
                    {{ pendingReview.owner.project_count }}
                  </dd>
                </div>
                <div class="flex items-center justify-between">
                  <dt class="text-muted-foreground">Approved before</dt>
                  <dd class="font-medium text-emerald-400">
                    {{ pendingReview.owner.approved_count }}
                  </dd>
                </div>
                <div class="flex items-center justify-between">
                  <dt class="text-muted-foreground">Rejected before</dt>
                  <dd class="font-medium text-red-400">
                    {{ pendingReview.owner.rejected_count }}
                  </dd>
                </div>
              </dl>
            </div>

            <!-- Project facts -->
            <div class="border-border/60 bg-card/40 rounded-2xl border p-5">
              <p
                class="text-muted-foreground mb-3 text-xs font-semibold tracking-wide uppercase"
              >
                Project facts
              </p>
              <dl class="space-y-2 text-sm">
                <div class="flex items-center justify-between gap-2">
                  <dt
                    class="text-muted-foreground inline-flex items-center gap-1.5"
                  >
                    <Eye class="size-3.5" /> Visibility
                  </dt>
                  <dd class="text-foreground font-medium">
                    {{
                      VISIBILITY_LABELS[pendingReview.facts.visibility] ??
                      pendingReview.facts.visibility
                    }}
                  </dd>
                </div>
                <div class="flex items-center justify-between gap-2">
                  <dt
                    class="text-muted-foreground inline-flex items-center gap-1.5"
                  >
                    <Coins class="size-3.5" /> Monetization
                  </dt>
                  <dd class="text-foreground font-medium">
                    {{
                      pendingReview.facts.monetization_enabled
                        ? `On · ${pendingReview.facts.creator_share}% creator`
                        : "Off"
                    }}
                  </dd>
                </div>
                <div class="flex items-center justify-between gap-2">
                  <dt
                    class="text-muted-foreground inline-flex items-center gap-1.5"
                  >
                    <Package2 class="size-3.5" /> Versions
                  </dt>
                  <dd class="text-foreground font-medium">
                    {{ pendingReview.facts.version_count }}
                  </dd>
                </div>
                <div class="flex items-center justify-between gap-2">
                  <dt
                    class="text-muted-foreground inline-flex items-center gap-1.5"
                  >
                    <Images class="size-3.5" /> Gallery
                  </dt>
                  <dd class="text-foreground font-medium">
                    {{ pendingReview.facts.gallery_count }}
                  </dd>
                </div>
                <div class="flex items-center justify-between gap-2">
                  <dt
                    class="text-muted-foreground inline-flex items-center gap-1.5"
                  >
                    <Heart class="size-3.5" /> Hearts
                  </dt>
                  <dd class="text-foreground font-medium">
                    {{ pendingReview.facts.heart_count }}
                  </dd>
                </div>
                <div class="flex items-center justify-between gap-2">
                  <dt
                    class="text-muted-foreground inline-flex items-center gap-1.5"
                  >
                    <Download class="size-3.5" /> Downloads
                  </dt>
                  <dd class="text-foreground font-medium">
                    {{ pendingReview.facts.download_count }}
                  </dd>
                </div>
                <div class="flex items-center justify-between gap-2">
                  <dt
                    class="text-muted-foreground inline-flex items-center gap-1.5"
                  >
                    <Calendar class="size-3.5" /> Created
                  </dt>
                  <dd class="text-foreground font-medium">
                    {{ formatDate(pendingReview.facts.created_at) }}
                  </dd>
                </div>
              </dl>
            </div>

            <!-- Review history -->
            <div class="border-border/60 bg-card/40 rounded-2xl border p-5">
              <p
                class="text-muted-foreground mb-3 text-xs font-semibold tracking-wide uppercase"
              >
                Review history
              </p>
              <p
                v-if="!pendingReview.history.length"
                class="text-muted-foreground text-sm"
              >
                No previous reviews.
              </p>
              <ul v-else class="space-y-3">
                <li
                  v-for="(entry, idx) in pendingReview.history"
                  :key="idx"
                  class="border-border/40 border-l-2 pl-3"
                >
                  <div class="flex flex-wrap items-center gap-2">
                    <span
                      class="rounded-full px-2 py-0.5 text-xs font-semibold"
                      :class="actionMeta(entry.action).class"
                    >
                      {{ actionMeta(entry.action).label }}
                    </span>
                    <span class="text-muted-foreground text-xs">
                      {{ entry.reviewer }} ·
                      {{ relativeTime(entry.created_at) }}
                    </span>
                  </div>
                  <p
                    v-if="entry.notes.trim()"
                    class="text-muted-foreground mt-1 text-sm whitespace-pre-wrap"
                  >
                    {{ entry.notes }}
                  </p>
                </li>
              </ul>
            </div>

            <!-- Private moderator notes -->
            <div class="border-border/60 bg-card/40 rounded-2xl border p-5">
              <p
                class="text-muted-foreground mb-1 inline-flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"
              >
                <StickyNote class="size-3.5" /> Moderator notes
              </p>
              <p class="text-muted-foreground/80 mb-3 text-xs">
                Internal only - never shown to the creator.
              </p>

              <div class="mb-3">
                <Textarea
                  v-model="newNote"
                  rows="3"
                  placeholder="Add an internal note for future reviews…"
                  class="mb-2"
                />
                <Button
                  size="sm"
                  class="gap-2"
                  :disabled="noteSubmitting || !newNote.trim()"
                  @click="submitNote"
                >
                  <Loader2 v-if="noteSubmitting" class="size-4 animate-spin" />
                  Add note
                </Button>
              </div>

              <p
                v-if="!moderatorNotes.length"
                class="text-muted-foreground text-sm"
              >
                No notes yet.
              </p>
              <ul v-else class="space-y-3">
                <li
                  v-for="note in moderatorNotes"
                  :key="note.id"
                  class="border-border/40 border-l-2 pl-3"
                >
                  <div class="text-muted-foreground text-xs">
                    {{ note.author }} · {{ relativeTime(note.created_at) }}
                  </div>
                  <p class="text-foreground mt-1 text-sm whitespace-pre-wrap">
                    {{ note.body }}
                  </p>
                </li>
              </ul>
            </div>
          </aside>
        </div>

        <!-- Decision -->
        <div
          class="border-primary/40 bg-primary/5 sticky bottom-4 rounded-2xl border p-5 backdrop-blur-sm"
        >
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
              Approve &amp; publish
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
      </template>
    </div>
  </div>
</template>
