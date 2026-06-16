import { useApi, apiErrorMessage } from "~/scripts/api";
import type {
  Category,
  ProjectSettings,
  ProjectStatus,
  ProjectVisibility,
} from "./types";

interface SettingsForm {
  title: string;
  urlSlug: string;
  summary: string;
  description: string;
  visibility: ProjectVisibility;
  license: string;
  monetizationEnabled: boolean;
  creatorShare: number;
  websiteUrl: string;
  sourceUrl: string;
  issuesUrl: string;
  wikiUrl: string;
  discordUrl: string;
}

export const BEACON_SHARE = 20;
export const DEFAULT_CREATOR_SHARE = 80;
export const RECOMMENDED_DESCRIPTION_LENGTH = 200;

export function useProjectSettings(slug: string) {
  const api = useApi();
  const config = useRuntimeConfig();

  const project = ref<ProjectSettings | null>(null);
  const error = ref("");
  const pending = ref(false);

  const form = reactive<SettingsForm>({
    title: "",
    urlSlug: "",
    summary: "",
    description: "",
    visibility: "public",
    license: "",
    monetizationEnabled: true,
    creatorShare: DEFAULT_CREATOR_SHARE,
    websiteUrl: "",
    sourceUrl: "",
    issuesUrl: "",
    wikiUrl: "",
    discordUrl: "",
  });

  const saving = ref(false);
  const saveError = ref("");
  const savingDescription = ref(false);
  const descriptionError = ref("");
  const savingMonetization = ref(false);
  const monetizationError = ref("");
  const savingLicense = ref(false);
  const licenseError = ref("");
  const savingTags = ref(false);
  const tagsError = ref("");
  const savingLinks = ref(false);
  const linksError = ref("");
  const allCategories = ref<Category[]>([]);
  const selectedCategoryIds = ref<string[]>([]);
  const originalCategoryIds = ref<string[]>([]);
  const submitting = ref(false);
  const submitError = ref("");
  const changelog = ref("");
  const savingChangelog = ref(false);
  const changelogError = ref("");
  const deleting = ref(false);
  const deleteError = ref("");
  const iconPending = ref(false);
  const iconError = ref("");

  function syncForm(data: ProjectSettings) {
    form.title = data.title;
    form.urlSlug = data.slug;
    form.summary = data.summary;
    form.description = data.description;
    form.visibility = data.visibility;
    form.license = data.license;
    form.monetizationEnabled = data.monetization_enabled;
    form.creatorShare = data.creator_share;
    form.websiteUrl = data.website_url;
    form.sourceUrl = data.source_url;
    form.issuesUrl = data.issues_url;
    form.wikiUrl = data.wiki_url;
    form.discordUrl = data.discord_url;
    changelog.value = data.pending_changelog ?? "";
  }

  const iconUrl = computed(() => {
    const path = project.value?.icon_url;
    if (!path) return null;
    return `${config.public.apiBase}${path}?v=${iconVersion.value}&revision=pending`;
  });

  const iconVersion = ref(0);

  const summaryLength = computed(() => form.summary.trim().length);

  const descriptionLength = computed(() => form.description.trim().length);

  const charityShare = computed(() => {
    const extra = DEFAULT_CREATOR_SHARE - form.creatorShare;
    return extra > 0 ? extra : 0;
  });

  const dirty = computed(() => {
    if (!project.value) return false;
    return (
      form.title.trim() !== project.value.title ||
      form.urlSlug.trim() !== project.value.slug ||
      form.summary.trim() !== project.value.summary ||
      form.visibility !== project.value.visibility
    );
  });

  const monetizationDirty = computed(() => {
    if (!project.value) return false;
    return (
      form.monetizationEnabled !== project.value.monetization_enabled ||
      form.creatorShare !== project.value.creator_share
    );
  });

  const descriptionDirty = computed(() => {
    if (!project.value) return false;
    return form.description !== project.value.description;
  });

  const licenseDirty = computed(() => {
    if (!project.value) return false;
    return form.license !== project.value.license;
  });

  const linksDirty = computed(() => {
    if (!project.value) return false;
    return (
      form.websiteUrl.trim() !== project.value.website_url ||
      form.sourceUrl.trim() !== project.value.source_url ||
      form.issuesUrl.trim() !== project.value.issues_url ||
      form.wikiUrl.trim() !== project.value.wiki_url ||
      form.discordUrl.trim() !== project.value.discord_url
    );
  });

  const hasLinks = computed(() => {
    const p = project.value;
    if (!p) return false;
    return Boolean(
      p.website_url ||
      p.source_url ||
      p.issues_url ||
      p.wiki_url ||
      p.discord_url,
    );
  });

  const availableCategories = computed(() => {
    const type = project.value?.project_type;
    if (!type) return [] as Category[];
    return allCategories.value.filter((c) => c.project_type === type);
  });

  const tagsDirty = computed(() => {
    const current = [...selectedCategoryIds.value].sort().join(",");
    const original = [...originalCategoryIds.value].sort().join(",");
    return current !== original;
  });

  function toggleCategory(id: string) {
    const index = selectedCategoryIds.value.indexOf(id);
    if (index === -1) {
      selectedCategoryIds.value = [...selectedCategoryIds.value, id];
    } else {
      selectedCategoryIds.value = selectedCategoryIds.value.filter(
        (value) => value !== id,
      );
    }
  }

  function syncSelectedCategories(data: ProjectSettings) {
    const slugs = new Set(data.categories.map((c) => c.slug));
    const ids = allCategories.value
      .filter((c) => c.project_type === data.project_type && slugs.has(c.slug))
      .map((c) => c.id);
    selectedCategoryIds.value = ids;
    originalCategoryIds.value = ids;
  }

  async function load() {
    error.value = "";
    pending.value = true;
    try {
      const [data, categoryData] = await Promise.all([
        api<ProjectSettings>(`/projects/${slug}/settings`),
        api<{ categories: Category[] }>("/categories"),
      ]);
      allCategories.value = categoryData.categories;
      project.value = data;
      syncForm(data);
      syncSelectedCategories(data);
    } catch (err) {
      const status = (err as { response?: { status?: number } })?.response
        ?.status;
      error.value =
        status === 401 || status === 403
          ? "You do not have access to this project's settings."
          : status === 404
            ? "That project could not be found."
            : "Could not load this project. Please try again.";
    } finally {
      pending.value = false;
    }
  }

  async function save() {
    if (!project.value) return;
    saveError.value = "";
    saving.value = true;
    try {
      const body: Record<string, string> = {
        title: form.title.trim(),
        slug: form.urlSlug.trim(),
        summary: form.summary.trim(),
        visibility: form.visibility,
      };
      const result = await api<{ slug: string }>(`/projects/${slug}`, {
        method: "PATCH",
        body,
      });
      const owner = project.value.owner;
      if (result.slug !== slug) {
        await navigateTo(`/${owner}/${result.slug}/settings`);
        return;
      }
      await load();
    } catch (err) {
      saveError.value = apiErrorMessage(err, {
        fallback: "Could not save your changes. Please try again.",
        status: {
          401: "Please sign in to edit this project.",
          403: "You do not have permission to edit this project.",
          409: "That URL is already taken.",
        },
      });
    } finally {
      saving.value = false;
    }
  }

  async function uploadIcon(file: File) {
    iconError.value = "";
    iconPending.value = true;
    try {
      const data = new FormData();
      data.append("icon", file);
      const result = await api<{ icon_url: string }>(`/projects/${slug}/icon`, {
        method: "POST",
        body: data,
      });
      if (project.value) project.value.icon_url = result.icon_url;
      iconVersion.value += 1;
    } catch (err) {
      iconError.value = apiErrorMessage(err, {
        fallback: "Could not upload the icon. Please try again.",
        status: { 400: "Please choose a PNG, JPG, WEBP, or GIF image." },
      });
    } finally {
      iconPending.value = false;
    }
  }

  async function removeIcon() {
    iconError.value = "";
    iconPending.value = true;
    try {
      await api(`/projects/${slug}/icon`, { method: "DELETE" });
      if (project.value) project.value.icon_url = null;
      iconVersion.value += 1;
    } catch (err) {
      iconError.value = apiErrorMessage(err, {
        fallback: "Could not remove the icon. Please try again.",
      });
    } finally {
      iconPending.value = false;
    }
  }

  async function saveMonetization() {
    if (!project.value) return;
    monetizationError.value = "";
    savingMonetization.value = true;
    try {
      await api(`/projects/${slug}`, {
        method: "PATCH",
        body: {
          monetization_enabled: form.monetizationEnabled,
          creator_share: form.creatorShare,
        },
      });
      await load();
    } catch (err) {
      monetizationError.value = apiErrorMessage(err, {
        fallback: "Could not save monetization settings. Please try again.",
        status: {
          401: "Please sign in to edit this project.",
          403: "You do not have permission to edit this project.",
        },
      });
    } finally {
      savingMonetization.value = false;
    }
  }

  async function saveDescription() {
    if (!project.value) return;
    descriptionError.value = "";
    savingDescription.value = true;
    try {
      await api(`/projects/${slug}`, {
        method: "PATCH",
        body: { description: form.description },
      });
      await load();
    } catch (err) {
      descriptionError.value = apiErrorMessage(err, {
        fallback: "Could not save the description. Please try again.",
        status: {
          401: "Please sign in to edit this project.",
          403: "You do not have permission to edit this project.",
        },
      });
    } finally {
      savingDescription.value = false;
    }
  }

  async function saveLicense() {
    if (!project.value) return;
    licenseError.value = "";
    savingLicense.value = true;
    try {
      await api(`/projects/${slug}`, {
        method: "PATCH",
        body: { license: form.license },
      });
      await load();
    } catch (err) {
      licenseError.value = apiErrorMessage(err, {
        fallback: "Could not save the license. Please try again.",
        status: {
          401: "Please sign in to edit this project.",
          403: "You do not have permission to edit this project.",
        },
      });
    } finally {
      savingLicense.value = false;
    }
  }

  async function saveLinks(): Promise<void> {
    if (!project.value) return;
    linksError.value = "";
    savingLinks.value = true;
    try {
      await api(`/projects/${slug}`, {
        method: "PATCH",
        body: {
          website_url: form.websiteUrl.trim(),
          source_url: form.sourceUrl.trim(),
          issues_url: form.issuesUrl.trim(),
          wiki_url: form.wikiUrl.trim(),
          discord_url: form.discordUrl.trim(),
        },
      });
      await load();
    } catch (err) {
      linksError.value = apiErrorMessage(err, {
        fallback: "Could not save links. Please try again.",
        status: {
          400: "Links must start with http:// or https://.",
          401: "Please sign in to edit this project.",
          403: "You do not have permission to edit this project.",
        },
      });
    } finally {
      savingLinks.value = false;
    }
  }

  async function saveTags(): Promise<void> {
    if (!project.value) return;
    tagsError.value = "";
    savingTags.value = true;
    try {
      await api(`/projects/${slug}`, {
        method: "PATCH",
        body: { category_ids: selectedCategoryIds.value },
      });
      await load();
    } catch (err) {
      tagsError.value = apiErrorMessage(err, {
        fallback: "Could not save tags. Please try again.",
        status: {
          401: "Please sign in to edit this project.",
          403: "You do not have permission to edit this project.",
        },
      });
    } finally {
      savingTags.value = false;
    }
  }

  async function submitForReview(): Promise<boolean> {
    if (!project.value) return false;
    submitError.value = "";
    submitting.value = true;
    try {
      const result = await api<{ status: ProjectStatus }>(
        `/projects/${slug}/submit`,
        { method: "POST", body: { changelog: changelog.value.trim() } },
      );
      if (project.value) project.value.status = result.status;
      await load();
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

  const changelogDirty = computed(() => {
    if (!project.value) return false;
    return changelog.value.trim() !== (project.value.pending_changelog ?? "");
  });

  async function saveChangelog(): Promise<boolean> {
    if (!project.value) return false;
    changelogError.value = "";
    savingChangelog.value = true;
    try {
      await api(`/projects/${slug}`, {
        method: "PATCH",
        body: { changelog: changelog.value.trim() },
      });
      await load();
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

  async function deleteProject(): Promise<boolean> {
    deleteError.value = "";
    deleting.value = true;
    try {
      await api(`/projects/${slug}`, { method: "DELETE" });
      return true;
    } catch (err) {
      deleteError.value = apiErrorMessage(err, {
        fallback: "Could not delete this project. Please try again.",
        status: {
          401: "Please sign in to delete this project.",
          403: "You do not have permission to delete this project.",
          404: "This project no longer exists.",
        },
      });
      return false;
    } finally {
      deleting.value = false;
    }
  }

  return {
    project,
    error,
    pending,
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
    iconPending,
    iconError,
    iconUrl,
    summaryLength,
    descriptionLength,
    charityShare,
    dirty,
    descriptionDirty,
    monetizationDirty,
    licenseDirty,
    load,
    save,
    saveDescription,
    saveLicense,
    saveMonetization,
    submitForReview,
    deleteProject,
    uploadIcon,
    removeIcon,
  };
}
