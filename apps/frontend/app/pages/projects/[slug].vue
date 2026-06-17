<script setup lang="ts">
import { ArrowLeft, Loader2 } from "@lucide/vue";
import { useProjectDetail } from "~/scripts/pages/projects/detail";
import ProjectPreviewBanner from "~/components/project/ProjectPreviewBanner.vue";
import ProjectPendingBanner from "~/components/project/ProjectPendingBanner.vue";
import ProjectHero from "~/components/project/ProjectHero.vue";
import ProjectActions from "~/components/project/ProjectActions.vue";
import ProjectTabs from "~/components/project/ProjectTabs.vue";
import ProjectSidebar from "~/components/project/ProjectSidebar.vue";
import ProjectReportDialog from "~/components/project/ProjectReportDialog.vue";

const {
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
} = useProjectDetail();

await load();
</script>

<template>
	<div class="page-canvas min-h-screen">
		<div class="mx-auto max-w-6xl px-6 py-10">
			<NuxtLink to="/projects" class="text-muted-foreground hover:text-foreground mb-6 inline-flex items-center gap-2 text-sm transition-colors">
				<ArrowLeft class="size-4" />
				Back to Discover
			</NuxtLink>

			<div v-if="pending" class="text-muted-foreground flex items-center gap-2 py-20">
				<Loader2 class="size-5 animate-spin" />
				Loading project…
			</div>

			<div v-else-if="error || !project" class="border-border/60 rounded-xl border p-10 text-center">
				<p class="text-muted-foreground">{{ error || "Project not found." }}</p>
				<NuxtLink to="/projects" class="text-primary mt-3 inline-block text-sm hover:underline"> Browse all projects </NuxtLink>
			</div>

			<template v-else>
				<ProjectPreviewBanner v-if="isPreview" :slug="slug" />
				<ProjectPendingBanner v-if="showOwnerPending" />

				<section class="border-border/60 bg-card/40 rounded-2xl border p-6 backdrop-blur-sm sm:p-8">
					<div class="flex flex-col gap-6 lg:flex-row lg:items-start lg:justify-between">
						<ProjectHero :project="project" :icon-src="iconSrc" :type-style="typeStyle" />
						<ProjectActions
							:project="project"
							:slug="slug"
							:is-owner="isOwner"
							:latest-version="latestVersion"
							:downloadable-versions="downloadableVersions"
							:download-url="downloadUrl"
							:heart-pending="heartPending"
							:save-pending="savePending"
							@heart="handleHeart"
							@save="handleSave"
							@copy-id="copyId"
							@copy-link="copyLink"
							@report="reportOpen = true"
						/>
					</div>
				</section>

				<div class="mt-8 flex flex-col gap-8 lg:flex-row">
					<div class="min-w-0 flex-1" :class="settings.contentSidebarLeft ? 'lg:order-2' : ''">
						<ProjectTabs :description="project.description" :images="images" :versions="versions" :changelog-entries="changelogEntries" :download-url="downloadUrl" />
					</div>

					<ProjectSidebar :project="project" :links="projectLinks" />
				</div>
			</template>
		</div>

		<ProjectReportDialog v-model:open="reportOpen" v-model:reason="reportReason" @submit="submitReport" />
	</div>
</template>
