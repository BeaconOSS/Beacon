import { useApi, apiErrorMessage } from "~/scripts/api";
import type {
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
  });

  const saving = ref(false);
  const saveError = ref("");
  const savingDescription = ref(false);
  const descriptionError = ref("");
  const savingMonetization = ref(false);
  const monetizationError = ref("");
  const savingLicense = ref(false);
  const licenseError = ref("");
  const submitting = ref(false);
  const submitError = ref("");
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
  }

  const iconUrl = computed(() => {
    const path = project.value?.icon_url;
    if (!path) return null;
    return `${config.public.apiBase}${path}?v=${iconVersion.value}`;
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

  async function load() {
    error.value = "";
    pending.value = true;
    try {
      const data = await api<ProjectSettings>(`/projects/${slug}/settings`);
      project.value = data;
      syncForm(data);
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

  async function submitForReview(): Promise<boolean> {
    if (!project.value) return false;
    submitError.value = "";
    submitting.value = true;
    try {
      const result = await api<{ status: ProjectStatus }>(
        `/projects/${slug}/submit`,
        { method: "POST" },
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
    submitting,
    submitError,
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
    uploadIcon,
    removeIcon,
  };
}
