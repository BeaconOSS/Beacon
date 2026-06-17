<script setup lang="ts">
import { ArrowLeft, Lock, Rocket } from "@lucide/vue";
import { toast } from "vue-sonner";
import { useProjectSettings } from "~/scripts/pages/projects";
import { useVersions, useUploadVersionForm } from "~/scripts/pages/projects/versions";
import { useGallery, useUploadGalleryForm } from "~/scripts/pages/projects/gallery";
import { useProjectMembers } from "~/scripts/pages/projects/members";
import { useProjectAnalytics } from "~/scripts/pages/projects/analytics";
import { useSettingsChecklist } from "~/scripts/pages/projects/settings/checklist";
import type { SectionId } from "~/scripts/pages/projects/settings/types";
import SettingsNav from "~/components/settings/SettingsNav.vue";
import SettingsPublish from "~/components/settings/SettingsPublish.vue";
import SettingsGeneral from "~/components/settings/SettingsGeneral.vue";
import SettingsTags from "~/components/settings/SettingsTags.vue";
import SettingsDescription from "~/components/settings/SettingsDescription.vue";
import SettingsVersions from "~/components/settings/SettingsVersions.vue";
import SettingsLicense from "~/components/settings/SettingsLicense.vue";
import SettingsGallery from "~/components/settings/SettingsGallery.vue";
import SettingsLinks from "~/components/settings/SettingsLinks.vue";
import SettingsMembers from "~/components/settings/SettingsMembers.vue";
import SettingsAnalytics from "~/components/settings/SettingsAnalytics.vue";

const route = useRoute();
const slug = computed(() => String(route.params.slug ?? ""));
const username = computed(() => String(route.params.username ?? ""));

const {
	project,
	error,
	pending,
	load,
	form,
	saving,
	saveError,
	dirty,
	save,
	savingDescription,
	descriptionError,
	descriptionDirty,
	saveDescription,
	savingMonetization,
	monetizationError,
	monetizationDirty,
	saveMonetization,
	savingLicense,
	licenseError,
	licenseDirty,
	saveLicense,
	savingTags,
	tagsError,
	tagsDirty,
	saveTags,
	availableCategories,
	selectedCategoryIds,
	toggleCategory,
	savingLinks,
	linksError,
	linksDirty,
	hasLinks,
	saveLinks,
	submitting,
	submitError,
	submitForReview,
	withdrawFromReview,
	changelog,
	changelogDirty,
	savingChangelog,
	changelogError,
	saveChangelog,
	hasPendingChanges,
	iconChanged,
	publishedIconUrl,
	pendingChanges,
	deleting,
	deleteError,
	deleteProject,
	iconPending,
	iconError,
	iconUrl,
	uploadIcon,
	removeIcon,
	locked,
} = useProjectSettings(slug.value);

const { versions, error: versionsError, load: loadVersions, downloadUrl, remove: removeVersion } = useVersions(slug.value);
const { images, load: loadGallery, remove: removeGalleryImage } = useGallery(slug.value);
const versionForm = useUploadVersionForm(slug.value);
const galleryForm = useUploadGalleryForm(slug.value);
const {
	members,
	pending: membersPending,
	username: memberUsername,
	adding: addingMember,
	addError: memberAddError,
	load: loadMembers,
	add: addMember,
	remove: removeMember,
} = useProjectMembers(slug.value);
const { data: analytics, error: analyticsError, pending: analyticsPending, load: loadAnalytics } = useProjectAnalytics(slug.value);

const { canSubmit, requiredItems, requiredComplete, outstandingItems, completedItems, status, canSubmitNow, submitLabel, statusBanner, navStatusDot } = useSettingsChecklist(
	project,
	versions,
	images,
	hasLinks,
	hasPendingChanges
);

const activeSection = ref<SectionId>("publish");

await Promise.all([load(), loadVersions(), loadGallery(), loadMembers(), loadAnalytics()]);

async function handleSave() {
	await save();
	if (!saveError.value) {
		toast.success("Project updated.");
	}
}

async function handleSaveMonetization() {
	await saveMonetization();
	if (!monetizationError.value) {
		toast.success("Monetization settings saved.");
	}
}

async function handleSaveDescription() {
	await saveDescription();
	if (!descriptionError.value) {
		toast.success("Description saved.");
	}
}

async function handleSaveLicense() {
	await saveLicense();
	if (!licenseError.value) {
		toast.success("License saved.");
	}
}

async function handleSaveTags() {
	await saveTags();
	if (!tagsError.value) {
		toast.success("Tags saved.");
	}
}

async function handleSaveLinks() {
	await saveLinks();
	if (!linksError.value) {
		toast.success("Links saved.");
	}
}

async function handleUploadVersion() {
	if (await versionForm.submit()) {
		await Promise.all([loadVersions(), load()]);
		toast.success("Version published.");
	} else if (versionForm.error.value) {
		toast.error(versionForm.error.value);
	}
}

async function handleDeleteVersion(version: (typeof versions.value)[number]) {
	if (await removeVersion(version)) {
		await load();
		toast.success("Version deleted.");
	} else if (versionsError.value) {
		toast.error(versionsError.value);
	}
}

async function handleUploadGalleryImage() {
	if (await galleryForm.submit()) {
		await loadGallery();
		toast.success("Image added to the gallery.");
	} else if (galleryForm.error.value) {
		toast.error(galleryForm.error.value);
	}
}

async function handleDeleteGalleryImage(id: string) {
	if (await removeGalleryImage(id)) {
		toast.success("Image removed.");
	} else {
		toast.error("Could not remove the image. Please try again.");
	}
}

async function handleAddMember() {
	if (await addMember()) {
		toast.success("Member added.");
	} else if (memberAddError.value) {
		toast.error(memberAddError.value);
	}
}

async function handleRemoveMember(userId: string) {
	if (await removeMember(userId)) {
		toast.success("Member removed.");
	} else {
		toast.error("Could not remove that member. Please try again.");
	}
}

async function handleSubmit() {
	if (await submitForReview()) {
		toast.success("Submitted for review.");
	} else if (submitError.value) {
		toast.error(submitError.value);
	}
}

async function handleWithdraw() {
	if (await withdrawFromReview()) {
		toast.success("Withdrawn from review. You can edit again.");
	} else if (submitError.value) {
		toast.error(submitError.value);
	}
}

async function handleSaveChangelog() {
	if (await saveChangelog()) {
		toast.success("Review note saved.");
	} else if (changelogError.value) {
		toast.error(changelogError.value);
	}
}

async function handleDeleteProject() {
	if (await deleteProject()) {
		toast.success("Project deleted.");
		await navigateTo("/profile");
	} else if (deleteError.value) {
		toast.error(deleteError.value);
	}
}
</script>

<template>
	<div class="page-canvas">
		<div class="mx-auto max-w-6xl px-6 py-12">
			<NuxtLink :to="`/projects/${slug}`" class="text-muted-foreground hover:text-foreground mb-6 inline-flex items-center gap-1.5 text-sm transition-colors">
				<ArrowLeft class="size-4" />
				Back to project
			</NuxtLink>

			<p v-if="pending" class="text-muted-foreground py-12 text-center">Loading project...</p>
			<p v-else-if="error" class="text-destructive py-12 text-center">
				{{ error }}
			</p>

			<template v-else-if="project">
				<header class="mb-8">
					<p class="text-primary eyebrow mb-2">Project settings</p>
					<h1 class="display-heading text-3xl md:text-4xl">
						{{ project.title }}
					</h1>
					<p class="text-muted-foreground mt-1 text-sm">@{{ username }} / {{ slug }}</p>
				</header>

				<div class="flex flex-col gap-8 lg:flex-row">
					<SettingsNav v-model="activeSection" :nav-status-dot="navStatusDot" :status-label="statusBanner.label" />

					<div class="min-w-0 flex-1 space-y-8">
						<div v-if="locked && activeSection !== 'publish'" class="flex flex-wrap items-center gap-3 rounded-xl border border-amber-500/30 bg-amber-500/10 px-4 py-3 text-sm">
							<Lock class="size-4 shrink-0 text-amber-500" />
							<span class="text-foreground flex-1 font-medium"> This project is locked while it is under review. </span>
							<Button variant="outline" size="sm" class="gap-2" @click="activeSection = 'publish'">
								<Rocket class="size-4" />
								Manage review
							</Button>
						</div>

						<SettingsPublish
							v-if="activeSection === 'publish'"
							v-model:changelog="changelog"
							:banner="statusBanner"
							:review-notes="project.review?.notes ?? null"
							:has-pending-changes="hasPendingChanges"
							:pending-changes="pendingChanges"
							:icon-changed="iconChanged"
							:published-icon-url="publishedIconUrl"
							:icon-url="iconUrl"
							:can-submit="canSubmit"
							:required-complete="requiredComplete"
							:required-total="requiredItems.length"
							:outstanding-items="outstandingItems"
							:completed-items="completedItems"
							:status="status"
							:changelog-dirty="changelogDirty"
							:saving-changelog="savingChangelog"
							:changelog-error="changelogError"
							:can-submit-now="canSubmitNow"
							:submitting="submitting"
							:submit-label="submitLabel"
							@save-changelog="handleSaveChangelog"
							@submit="handleSubmit"
							@withdraw="handleWithdraw"
						/>

						<SettingsGeneral
							v-else-if="activeSection === 'general'"
							v-model:title="form.title"
							v-model:url-slug="form.urlSlug"
							v-model:summary="form.summary"
							v-model:visibility="form.visibility"
							v-model:monetization-enabled="form.monetizationEnabled"
							v-model:creator-share="form.creatorShare"
							:username="username"
							:icon-url="iconUrl"
							:icon-pending="iconPending"
							:icon-error="iconError"
							:locked="locked"
							:save-error="saveError"
							:dirty="dirty"
							:saving="saving"
							:monetization-dirty="monetizationDirty"
							:saving-monetization="savingMonetization"
							:monetization-error="monetizationError"
							:project-title="project.title"
							:deleting="deleting"
							@upload="uploadIcon"
							@remove-icon="removeIcon"
							@save="handleSave"
							@save-monetization="handleSaveMonetization"
							@delete="handleDeleteProject"
						/>

						<SettingsTags
							v-else-if="activeSection === 'tags'"
							:available-categories="availableCategories"
							:selected-category-ids="selectedCategoryIds"
							:tags-dirty="tagsDirty"
							:saving-tags="savingTags"
							:tags-error="tagsError"
							:locked="locked"
							@toggle="toggleCategory"
							@save="handleSaveTags"
						/>

						<SettingsDescription
							v-else-if="activeSection === 'description'"
							v-model="form.description"
							:description-dirty="descriptionDirty"
							:saving-description="savingDescription"
							:description-error="descriptionError"
							:locked="locked"
							@save="handleSaveDescription"
						/>

						<SettingsVersions
							v-else-if="activeSection === 'versions'"
							v-model:version-number="versionForm.versionNumber.value"
							v-model:channel="versionForm.channel.value"
							v-model:name="versionForm.name.value"
							v-model:changelog="versionForm.changelog.value"
							:file="versionForm.file.value"
							:upload-error="versionForm.error.value"
							:upload-pending="versionForm.pending.value"
							:locked="locked"
							:versions="versions"
							:download-url="downloadUrl"
							@file-change="versionForm.onFileChange"
							@submit="handleUploadVersion"
							@delete-version="handleDeleteVersion"
						/>

						<SettingsLicense
							v-else-if="activeSection === 'license'"
							v-model="form.license"
							:license-dirty="licenseDirty"
							:saving-license="savingLicense"
							:license-error="licenseError"
							:locked="locked"
							@save="handleSaveLicense"
						/>

						<SettingsGallery
							v-else-if="activeSection === 'gallery'"
							v-model="galleryForm.caption.value"
							:image="galleryForm.image.value"
							:upload-error="galleryForm.error.value"
							:upload-pending="galleryForm.pending.value"
							:locked="locked"
							:images="images"
							@file-change="galleryForm.onFileChange"
							@submit="handleUploadGalleryImage"
							@delete-image="handleDeleteGalleryImage"
						/>

						<SettingsLinks
							v-else-if="activeSection === 'links'"
							v-model:website-url="form.websiteUrl"
							v-model:source-url="form.sourceUrl"
							v-model:issues-url="form.issuesUrl"
							v-model:wiki-url="form.wikiUrl"
							v-model:discord-url="form.discordUrl"
							:links-dirty="linksDirty"
							:saving-links="savingLinks"
							:links-error="linksError"
							:locked="locked"
							@save="handleSaveLinks"
						/>

						<SettingsMembers
							v-else-if="activeSection === 'members'"
							v-model="memberUsername"
							:members="members"
							:members-pending="membersPending"
							:adding-member="addingMember"
							:member-add-error="memberAddError"
							@add="handleAddMember"
							@remove="handleRemoveMember"
						/>

						<SettingsAnalytics v-else-if="activeSection === 'analytics'" :analytics="analytics" :analytics-error="analyticsError" :analytics-pending="analyticsPending" />
					</div>
				</div>
			</template>
		</div>
	</div>
</template>
