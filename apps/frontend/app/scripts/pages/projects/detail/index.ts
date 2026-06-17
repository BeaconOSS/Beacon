import { useClipboard } from "@vueuse/core";
import { toast } from "vue-sonner";
import { useProject } from "~/scripts/pages/projects";
import { useVersions } from "~/scripts/pages/projects/versions";
import { VERSION_CHANNEL } from "~/scripts/constants";
import { useGallery } from "~/scripts/pages/projects/gallery";
import { useProjectInteractions } from "~/scripts/pages/projects/interactions";
import type { Version } from "~/scripts/pages/projects/types";
import { useAuth } from "~/scripts/auth";
import { useSettings } from "~/scripts/settings";
import { TYPE_STYLES } from "./meta";
import type { ProjectLink, TypeStyle } from "./types";

export function useProjectDetail() {
  const route = useRoute();
  const slug = computed(() => String(route.params.slug ?? ""));
  const previewPending = computed(() => route.query.preview === "pending");

  const { user } = useAuth();
  const { settings } = useSettings();

  const {
    project,
    error,
    pending,
    load: loadProject,
  } = useProject(slug.value, previewPending.value);
  const { versions, load: loadVersions, downloadUrl } = useVersions(slug.value);
  const { images, load: loadGallery } = useGallery(slug.value);
  const { heartPending, savePending, toggleHeart, toggleSave } =
    useProjectInteractions(slug.value, project);

  async function load() {
    await Promise.all([loadProject(), loadVersions(), loadGallery()]);
  }

  const isOwner = computed(
    () =>
      !!user.value &&
      !!project.value &&
      user.value.username === project.value.owner,
  );

  const projectLinks = computed(() => {
    const p = project.value;
    if (!p) return [] as ProjectLink[];
    return [
      { label: "Website", url: p.website_url },
      { label: "Source code", url: p.source_url },
      { label: "Issue tracker", url: p.issues_url },
      { label: "Wiki", url: p.wiki_url },
      { label: "Discord", url: p.discord_url },
    ].filter((link): link is ProjectLink => Boolean(link.url));
  });

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
      downloadableVersions.value.find(
        (v) => v.channel === VERSION_CHANNEL.RELEASE,
      ) ??
      downloadableVersions.value[0] ??
      null,
  );

  const changelogEntries = computed(() =>
    versions.value.filter((v) => v.changelog && v.changelog.trim().length > 0),
  );

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

  const isPreview = computed(() => project.value?.preview === true);

  const showOwnerPending = computed(
    () =>
      !isPreview.value &&
      isOwner.value &&
      project.value?.has_pending_changes === true,
  );

  return {
    slug,
    settings,
    project,
    error,
    pending,
    versions,
    images,
    downloadUrl,
    heartPending,
    savePending,
    isOwner,
    isPreview,
    showOwnerPending,
    projectLinks,
    typeStyle,
    iconSrc,
    downloadableVersions,
    latestVersion,
    changelogEntries,
    copyId,
    copyLink,
    handleHeart,
    handleSave,
    reportOpen,
    reportReason,
    submitReport,
    load,
  };
}
