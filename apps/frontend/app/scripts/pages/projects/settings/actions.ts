import type { Ref } from "vue";
import { useApi, apiErrorMessage } from "~/scripts/api";
import type { SettingsActionsContext } from "./types";

const EDIT_ERRORS: Record<number, string> = {
  401: "Please sign in to edit this project.",
  403: "You do not have permission to edit this project.",
};

export function useSettingsActions(ctx: SettingsActionsContext) {
  const api = useApi();

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

  async function patchAndReload(
    flags: { saving: Ref<boolean>; error: Ref<string> },
    body: Record<string, unknown>,
    messages: { fallback: string; status?: Record<number, string> },
  ): Promise<void> {
    if (!ctx.project.value) return;
    flags.error.value = "";
    flags.saving.value = true;
    try {
      await api(`/projects/${ctx.slug}`, { method: "PATCH", body });
      await ctx.load();
    } catch (err) {
      flags.error.value = apiErrorMessage(err, messages);
    } finally {
      flags.saving.value = false;
    }
  }

  async function save() {
    if (!ctx.project.value) return;
    saveError.value = "";
    saving.value = true;
    try {
      const body: Record<string, string> = {
        title: ctx.form.title.trim(),
        slug: ctx.form.urlSlug.trim(),
        summary: ctx.form.summary.trim(),
        visibility: ctx.form.visibility,
      };
      const result = await api<{ slug: string }>(`/projects/${ctx.slug}`, {
        method: "PATCH",
        body,
      });
      const owner = ctx.project.value.owner;
      if (result.slug !== ctx.slug) {
        await navigateTo(`/${owner}/${result.slug}/settings`);
        return;
      }
      await ctx.load();
    } catch (err) {
      saveError.value = apiErrorMessage(err, {
        fallback: "Could not save your changes. Please try again.",
        status: { ...EDIT_ERRORS, 409: "That URL is already taken." },
      });
    } finally {
      saving.value = false;
    }
  }

  function saveMonetization() {
    return patchAndReload(
      { saving: savingMonetization, error: monetizationError },
      {
        monetization_enabled: ctx.form.monetizationEnabled,
        creator_share: ctx.form.creatorShare,
      },
      {
        fallback: "Could not save monetization settings. Please try again.",
        status: EDIT_ERRORS,
      },
    );
  }

  function saveDescription() {
    return patchAndReload(
      { saving: savingDescription, error: descriptionError },
      { description: ctx.form.description },
      {
        fallback: "Could not save the description. Please try again.",
        status: EDIT_ERRORS,
      },
    );
  }

  function saveLicense() {
    return patchAndReload(
      { saving: savingLicense, error: licenseError },
      { license: ctx.form.license },
      {
        fallback: "Could not save the license. Please try again.",
        status: EDIT_ERRORS,
      },
    );
  }

  function saveLinks(): Promise<void> {
    return patchAndReload(
      { saving: savingLinks, error: linksError },
      {
        website_url: ctx.form.websiteUrl.trim(),
        source_url: ctx.form.sourceUrl.trim(),
        issues_url: ctx.form.issuesUrl.trim(),
        wiki_url: ctx.form.wikiUrl.trim(),
        discord_url: ctx.form.discordUrl.trim(),
      },
      {
        fallback: "Could not save links. Please try again.",
        status: {
          400: "Links must start with http:// or https://.",
          ...EDIT_ERRORS,
        },
      },
    );
  }

  function saveTags(): Promise<void> {
    return patchAndReload(
      { saving: savingTags, error: tagsError },
      { category_ids: ctx.selectedCategoryIds.value },
      {
        fallback: "Could not save tags. Please try again.",
        status: EDIT_ERRORS,
      },
    );
  }

  return {
    saving,
    saveError,
    save,
    savingDescription,
    descriptionError,
    saveDescription,
    savingMonetization,
    monetizationError,
    saveMonetization,
    savingLicense,
    licenseError,
    saveLicense,
    savingLinks,
    linksError,
    saveLinks,
    savingTags,
    tagsError,
    saveTags,
  };
}
