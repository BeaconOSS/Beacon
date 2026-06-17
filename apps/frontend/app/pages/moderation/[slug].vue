<script setup lang="ts">
import { ArrowLeft, Loader2 } from "@lucide/vue";

import ReviewDecisionBar from "~/components/moderation/ReviewDecisionBar.vue";
import ReviewDecisionSignal from "~/components/moderation/ReviewDecisionSignal.vue";
import ReviewFacts from "~/components/moderation/ReviewFacts.vue";
import ReviewGallery from "~/components/moderation/ReviewGallery.vue";
import ReviewHeader from "~/components/moderation/ReviewHeader.vue";
import ReviewHistory from "~/components/moderation/ReviewHistory.vue";
import ReviewLightbox from "~/components/moderation/ReviewLightbox.vue";
import ReviewLinks from "~/components/moderation/ReviewLinks.vue";
import ReviewListingChanges from "~/components/moderation/ReviewListingChanges.vue";
import ReviewNotes from "~/components/moderation/ReviewNotes.vue";
import ReviewOwner from "~/components/moderation/ReviewOwner.vue";
import ReviewPackAnalysis from "~/components/moderation/ReviewPackAnalysis.vue";
import ReviewPackDiff from "~/components/moderation/ReviewPackDiff.vue";
import ReviewQueueBar from "~/components/moderation/ReviewQueueBar.vue";
import ReviewVersions from "~/components/moderation/ReviewVersions.vue";
import { useModerationReview } from "~/scripts/pages/moderation/review";

const {
	slug,
	isModerator,
	project,
	pendingReview,
	pendingError,
	pendingLoading,
	pendingWithBase,
	reviewSubmitting,
	moderatorNotes,
	noteSubmitting,
	load,
	queuePosition,
	queueTotal,
	queuePrev,
	queueNext,
	queueAge,
	expandedHistory,
	historyNoteClamp,
	toggleHistory,
	ownerTrust,
	reviewNotes,
	newNote,
	showDiffFiles,
	downloadingVersion,
	lightbox,
	openLightbox,
	closeLightbox,
	downloadVersion,
	activeFile,
	toggleFile,
	fileAnchorId,
	findingFile,
	jumpToFinding,
	submitNote,
	typeLabel,
	decisionSignal,
	links,
	reviewDiffs,
	handleReview,
} = useModerationReview();

await load();
</script>

<template>
	<div class="page-canvas min-h-screen">
		<div class="mx-auto max-w-5xl px-6 py-10">
			<NuxtLink to="/moderation" class="text-muted-foreground hover:text-foreground mb-6 inline-flex items-center gap-2 text-sm transition-colors">
				<ArrowLeft class="size-4" />
				Back to queue
			</NuxtLink>

			<ReviewQueueBar
				v-if="pendingReview && queuePosition"
				:position="queuePosition"
				:total="queueTotal"
				:prev="queuePrev"
				:next="queueNext"
				:age="queueAge"
				:submitted-at="pendingReview.submitted_at"
			/>

			<div v-if="pendingLoading" class="text-muted-foreground flex items-center gap-2 py-20">
				<Loader2 class="size-5 animate-spin" />
				Loading the review…
			</div>

			<div v-else-if="pendingError" class="border-border/60 rounded-xl border p-10 text-center">
				<p class="text-muted-foreground">{{ pendingError }}</p>
				<NuxtLink v-if="!isModerator" to="/" class="text-primary mt-3 inline-block text-sm hover:underline"> Back to home </NuxtLink>
			</div>

			<template v-else-if="pendingReview && project">
				<ReviewHeader
					:title="project.title"
					:owner="project.owner"
					:type-label="typeLabel"
					:slug="slug"
					:icon-url="pendingWithBase(pendingReview.pending.icon_url)"
					:is-first-review="pendingReview.is_first_review"
					:submitted-at="pendingReview.submitted_at"
				/>

				<div class="grid gap-6 lg:grid-cols-3">
					<div class="space-y-6 lg:col-span-2">
						<ReviewDecisionSignal :signal="decisionSignal" />
						<ReviewListingChanges :review="pendingReview" :diffs="reviewDiffs" :with-base="pendingWithBase" @open-lightbox="openLightbox" />
						<ReviewLinks v-if="links.length" :links="links" />
						<ReviewGallery :gallery="pendingReview.gallery" :with-base="pendingWithBase" @open-lightbox="openLightbox" />
						<ReviewVersions :versions="pendingReview.versions" :downloading-version="downloadingVersion" @download="downloadVersion" />
						<ReviewPackAnalysis v-if="pendingReview.analysis" :analysis="pendingReview.analysis" :finding-file="findingFile" @jump-to-finding="jumpToFinding" />
						<ReviewPackDiff
							v-if="pendingReview.pack_diff"
							v-model:show-files="showDiffFiles"
							:diff="pendingReview.pack_diff"
							:active-file="activeFile"
							:file-anchor-id="fileAnchorId"
							@toggle-file="toggleFile"
						/>
					</div>

					<aside class="space-y-6">
						<ReviewOwner :owner="pendingReview.owner" :trust="ownerTrust" />
						<ReviewFacts :facts="pendingReview.facts" />
						<ReviewHistory :history="pendingReview.history" :expanded="expandedHistory" :clamp="historyNoteClamp" @toggle="toggleHistory" />
						<ReviewNotes v-model:new-note="newNote" :notes="moderatorNotes" :submitting="noteSubmitting" @submit="submitNote" />
					</aside>
				</div>

				<ReviewDecisionBar v-model:notes="reviewNotes" :submitting="reviewSubmitting" @review="handleReview" />
			</template>
		</div>

		<ReviewLightbox v-if="lightbox" :state="lightbox" @close="closeLightbox" />
	</div>
</template>
