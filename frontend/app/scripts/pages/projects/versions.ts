import { useApi, apiErrorMessage } from "~/scripts/api";
import type { Version } from "./types";

export const VERSION_CHANNELS = [
  { value: "release", label: "Release" },
  { value: "beta", label: "Beta" },
  { value: "alpha", label: "Alpha" },
];

export function useVersions(slug: string) {
  const config = useRuntimeConfig();
  const api = useApi();

  const versions = ref<Version[]>([]);
  const error = ref("");
  const pending = ref(false);

  async function load() {
    error.value = "";
    pending.value = true;
    try {
      const data = await api<{ versions: Version[] }>(
        `/projects/${slug}/versions`,
      );
      versions.value = data.versions;
    } catch {
      error.value = "Could not load versions.";
    } finally {
      pending.value = false;
    }
  }

  function downloadUrl(version: Version): string {
    return `${config.public.apiBase}/projects/${slug}/versions/${encodeURIComponent(
      version.version_number,
    )}/download`;
  }

  async function remove(version: Version): Promise<boolean> {
    error.value = "";
    try {
      await api(
        `/projects/${slug}/versions/${encodeURIComponent(version.version_number)}`,
        { method: "DELETE" },
      );
      versions.value = versions.value.filter((v) => v.id !== version.id);
      return true;
    } catch (err) {
      error.value = apiErrorMessage(err, {
        fallback: "Could not delete the version. Please try again.",
        status: {
          401: "Please sign in to manage versions.",
          403: "You do not have permission to manage this project.",
        },
      });
      return false;
    }
  }

  return { versions, error, pending, load, downloadUrl, remove };
}

export function useUploadVersionForm(slug: string) {
  const api = useApi();

  const versionNumber = ref("");
  const name = ref("");
  const changelog = ref("");
  const channel = ref(VERSION_CHANNELS[0]!.value);
  const file = ref<File | null>(null);
  const error = ref("");
  const pending = ref(false);

  function onFileChange(event: Event) {
    const input = event.target as HTMLInputElement;
    file.value = input.files?.[0] ?? null;
  }

  async function submit(): Promise<boolean> {
    error.value = "";

    if (!versionNumber.value.trim()) {
      error.value = "Please enter a version number.";
      return false;
    }
    if (!file.value) {
      error.value = "Please choose a file to upload.";
      return false;
    }

    const body = new FormData();
    body.append("version_number", versionNumber.value.trim());
    body.append("name", name.value);
    body.append("changelog", changelog.value);
    body.append("channel", channel.value);
    body.append("file", file.value);

    pending.value = true;
    try {
      await api(`/projects/${slug}/versions`, {
        method: "POST",
        body,
      });
      versionNumber.value = "";
      name.value = "";
      changelog.value = "";
      channel.value = VERSION_CHANNELS[0]!.value;
      file.value = null;
      return true;
    } catch (err) {
      error.value = apiErrorMessage(err, {
        fallback: "Could not upload the version. Please try again.",
        status: { 401: "Please sign in to upload a version." },
      });
      return false;
    } finally {
      pending.value = false;
    }
  }

  return {
    versionNumber,
    name,
    changelog,
    channel,
    file,
    error,
    pending,
    onFileChange,
    submit,
  };
}
