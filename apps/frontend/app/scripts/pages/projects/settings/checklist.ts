import type { ComputedRef, Ref } from "vue";
import { PROJECT_STATUS, type ProjectStatus } from "~/scripts/constants";
import type { GalleryImage, ProjectSettings, Version } from "../types";
import type { ChecklistItem } from "./types";
import { NAV_STATUS_DOT, STATUS_BANNERS } from "./meta";

export function useSettingsChecklist(
  project: Ref<ProjectSettings | null>,
  versions: Ref<Version[]>,
  images: Ref<GalleryImage[]>,
  hasLinks: ComputedRef<boolean>,
  hasPendingChanges: ComputedRef<boolean>,
) {
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

  const outstandingItems = computed(() =>
    checklist.value.filter((item) => !item.complete),
  );
  const completedItems = computed(() =>
    checklist.value.filter((item) => item.complete),
  );

  const status = computed<ProjectStatus>(
    () => project.value?.status ?? PROJECT_STATUS.DRAFT,
  );

  const canSubmitNow = computed(
    () =>
      canSubmit.value &&
      (status.value === PROJECT_STATUS.DRAFT ||
        status.value === PROJECT_STATUS.CHANGES_REQUESTED ||
        status.value === PROJECT_STATUS.REJECTED ||
        (status.value === PROJECT_STATUS.APPROVED && hasPendingChanges.value)),
  );

  const submitLabel = computed(() => {
    if (status.value === PROJECT_STATUS.APPROVED)
      return "Submit changes for review";
    return status.value === PROJECT_STATUS.CHANGES_REQUESTED ||
      status.value === PROJECT_STATUS.REJECTED
      ? "Resubmit for review"
      : "Submit for review";
  });

  const statusBanner = computed(() => STATUS_BANNERS[status.value]);
  const navStatusDot = computed(() => NAV_STATUS_DOT[status.value]);

  return {
    summaryLength,
    checklist,
    requiredItems,
    requiredComplete,
    canSubmit,
    outstandingItems,
    completedItems,
    status,
    canSubmitNow,
    submitLabel,
    statusBanner,
    navStatusDot,
  };
}
