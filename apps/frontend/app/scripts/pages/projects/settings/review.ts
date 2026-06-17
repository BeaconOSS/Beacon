import { useApi, apiErrorMessage } from "~/scripts/api";
import type { ProjectSettings, ProjectStatus } from "../types";
import type { PendingChangeRow, SettingsReviewContext } from "./types";

export function useSettingsReview(ctx: SettingsReviewContext) {
  const api = useApi();
  const config = useRuntimeConfig();

  const submitting = ref(false);
  const submitError = ref("");
  const changelog = ref("");
  const savingChangelog = ref(false);
  const changelogError = ref("");

  function syncChangelog(data: ProjectSettings) {
    changelog.value = data.pending_changelog ?? "";
  }

  const changelogDirty = computed(() => {
    if (!ctx.project.value) return false;
    return (
      changelog.value.trim() !== (ctx.project.value.pending_changelog ?? "")
    );
  });

  const isPublished = computed(() => ctx.project.value?.is_published === true);
  const hasPendingChanges = computed(
    () => ctx.project.value?.has_pending_changes === true,
  );
  const iconChanged = computed(() => ctx.project.value?.icon_changed === true);

  const publishedIconUrl = computed(() => {
    const path = ctx.project.value?.published?.icon_url;
    if (!path) return null;
    return `${config.public.apiBase}${path}`;
  });

  const pendingChanges = computed(() => {
    const p = ctx.project.value;
    const published = p?.published;
    if (!p || !published) return [];
    const rows: PendingChangeRow[] = [];
    const add = (
      label: string,
      before: string,
      after: string,
      long = false,
    ) => {
      if (before.trim() !== after.trim())
        rows.push({ label, before, after, long });
    };
    add("Name", published.title, p.title);
    add("Summary", published.summary, p.summary);
    add("Description", published.description, p.description, true);
    add("License", published.license, p.license);
    add(
      "Tags",
      published.categories.join(", "),
      p.categories.map((c) => c.name).join(", "),
    );
    return rows;
  });

  async function submitForReview(): Promise<boolean> {
    if (!ctx.project.value) return false;
    submitError.value = "";
    submitting.value = true;
    try {
      const result = await api<{ status: ProjectStatus }>(
        `/projects/${ctx.slug}/submit`,
        { method: "POST", body: { changelog: changelog.value.trim() } },
      );
      if (ctx.project.value) ctx.project.value.status = result.status;
      await ctx.load();
      return true;
    } catch (err) {
      submitError.value = apiErrorMessage(err, {
        fallback: "Could not submit for review. Please try again.",
        status: {
          400: "Complete every required checklist item first.",
          401: "Please sign in to submit this project.",
          403: "You do not have permission to submit this project.",
          409: "This project has already been submitted for review.",
        },
      });
      return false;
    } finally {
      submitting.value = false;
    }
  }

  async function withdrawFromReview(): Promise<boolean> {
    if (!ctx.project.value) return false;
    submitError.value = "";
    submitting.value = true;
    try {
      const result = await api<{ status: ProjectStatus }>(
        `/projects/${ctx.slug}/withdraw`,
        { method: "POST" },
      );
      if (ctx.project.value) ctx.project.value.status = result.status;
      await ctx.load();
      return true;
    } catch (err) {
      submitError.value = apiErrorMessage(err, {
        fallback: "Could not withdraw this project. Please try again.",
        status: {
          401: "Please sign in to manage this project.",
          403: "You do not have permission to manage this project.",
          409: "This project is not currently awaiting review.",
        },
      });
      return false;
    } finally {
      submitting.value = false;
    }
  }

  async function saveChangelog(): Promise<boolean> {
    if (!ctx.project.value) return false;
    changelogError.value = "";
    savingChangelog.value = true;
    try {
      await api(`/projects/${ctx.slug}`, {
        method: "PATCH",
        body: { changelog: changelog.value.trim() },
      });
      await ctx.load();
      return true;
    } catch (err) {
      changelogError.value = apiErrorMessage(err, {
        fallback: "Could not save the changelog note. Please try again.",
        status: {
          401: "Please sign in to edit this project.",
          403: "You do not have permission to edit this project.",
        },
      });
      return false;
    } finally {
      savingChangelog.value = false;
    }
  }

  return {
    submitting,
    submitError,
    changelog,
    savingChangelog,
    changelogError,
    syncChangelog,
    changelogDirty,
    isPublished,
    hasPendingChanges,
    iconChanged,
    publishedIconUrl,
    pendingChanges,
    submitForReview,
    withdrawFromReview,
    saveChangelog,
  };
}
