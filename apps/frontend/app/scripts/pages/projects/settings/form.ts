import { DEFAULT_CREATOR_SHARE } from "./constants";

import type { ProjectSettings } from "../types";
import type { SettingsForm } from "./types";
import type { Ref } from "vue";

export function useSettingsForm(project: Ref<ProjectSettings | null>) {
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
	}

	const summaryLength = computed(() => form.summary.trim().length);

	const descriptionLength = computed(() => form.description.trim().length);

	const charityShare = computed(() => {
		const extra = DEFAULT_CREATOR_SHARE - form.creatorShare;
		return extra > 0 ? extra : 0;
	});

	const dirty = computed(() => {
		if (!project.value) return false;
		return form.title.trim() !== project.value.title || form.urlSlug.trim() !== project.value.slug || form.summary.trim() !== project.value.summary || form.visibility !== project.value.visibility;
	});

	const monetizationDirty = computed(() => {
		if (!project.value) return false;
		return form.monetizationEnabled !== project.value.monetization_enabled || form.creatorShare !== project.value.creator_share;
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
		return Boolean(p.website_url || p.source_url || p.issues_url || p.wiki_url || p.discord_url);
	});

	return {
		form,
		syncForm,
		summaryLength,
		descriptionLength,
		charityShare,
		dirty,
		monetizationDirty,
		descriptionDirty,
		licenseDirty,
		linksDirty,
		hasLinks,
	};
}
