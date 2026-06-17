import type { Ref } from "vue";
import { useApi, apiErrorMessage } from "~/scripts/api";
import type { ProjectSettings } from "../types";

export function useSettingsIcon(
  slug: string,
  project: Ref<ProjectSettings | null>,
) {
  const api = useApi();
  const config = useRuntimeConfig();

  const iconVersion = ref(0);
  const iconPending = ref(false);
  const iconError = ref("");

  const iconUrl = computed(() => {
    const path = project.value?.icon_url;
    if (!path) return null;
    return `${config.public.apiBase}${path}?v=${iconVersion.value}&revision=pending`;
  });

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

  return { iconPending, iconError, iconUrl, uploadIcon, removeIcon };
}
