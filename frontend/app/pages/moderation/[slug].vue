<script setup lang="ts">
import { toast } from "vue-sonner";
import {
  ArrowLeft,
  Ban,
  CircleCheck,
  ExternalLink,
  Loader2,
  MessageSquareWarning,
  ShieldCheck,
} from "@lucide/vue";
import {
  useProjectReview,
  useProjectPendingReview,
} from "~/scripts/pages/moderation";
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

await Promise.all([loadProject(), loadPendingReview()]);

const reviewNotes = ref("");

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
  if (!iso) return "—";
  const d = new Date(iso);
  if (Number.isNaN(d.getTime())) return "—";
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
            <NuxtLink
              :to="`/projects/${slug}`"
              class="text-primary mt-2 inline-flex items-center gap-1.5 text-sm hover:underline"
            >
              <ExternalLink class="size-3.5" />
              View current PDP
            </NuxtLink>
          </div>
        </div>

        <div class="grid gap-6 lg:grid-cols-3">
          <!-- Main report column -->
          <div class="space-y-6 lg:col-span-2">
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
