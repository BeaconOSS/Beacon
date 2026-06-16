import { useApi, apiErrorMessage } from "~/scripts/api";
import type { ProjectSettings, ProjectVisibility } from "./types";

interface SettingsForm {
  title: string;
  urlSlug: string;
  summary: string;
  visibility: ProjectVisibility;
}

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
    visibility: "public",
  });

  const saving = ref(false);
  const saveError = ref("");
  const iconPending = ref(false);
  const iconError = ref("");

  function syncForm(data: ProjectSettings) {
    form.title = data.title;
    form.urlSlug = data.slug;
    form.summary = data.summary;
    form.visibility = data.visibility;
  }

  const iconUrl = computed(() => {
    const path = project.value?.icon_url;
    if (!path) return null;
    return `${config.public.apiBase}${path}?v=${iconVersion.value}`;
  });

  const iconVersion = ref(0);

  const summaryLength = computed(() => form.summary.trim().length);

  const dirty = computed(() => {
    if (!project.value) return false;
    return (
      form.title.trim() !== project.value.title ||
      form.urlSlug.trim() !== project.value.slug ||
      form.summary.trim() !== project.value.summary ||
      form.visibility !== project.value.visibility
    );
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

  return {
    project,
    error,
    pending,
    form,
    saving,
    saveError,
    iconPending,
    iconError,
    iconUrl,
    summaryLength,
    dirty,
    load,
    save,
    uploadIcon,
    removeIcon,
  };
}
