<script setup lang="ts">
import { CircleCheck, Clock, Loader2, Send, Undo2 } from "@lucide/vue";

import type { ProjectStatus } from "~/scripts/constants";
import type { ChecklistItem } from "~/scripts/pages/projects/settings/types";

import MarkdownEditor from "~/components/settings/MarkdownEditor.vue";
import { PROJECT_STATUS } from "~/scripts/constants";
import { LEVEL_STYLES, NOTE_MARKDOWN_ACTIONS } from "~/scripts/pages/projects/settings/meta";

const changelog = defineModel<string>("changelog", { required: true });

defineProps<{
	canSubmit: boolean;
	requiredComplete: number;
	requiredTotal: number;
	outstandingItems: ChecklistItem[];
	completedItems: ChecklistItem[];
	status: ProjectStatus;
	changelogDirty: boolean;
	savingChangelog: boolean;
	changelogError: string;
	canSubmitNow: boolean;
	submitting: boolean;
	submitLabel: string;
	hasPendingChanges: boolean;
}>();

defineEmits<{
	saveChangelog: [];
	submit: [];
	withdraw: [];
}>();
</script>

<template>
	<section class="card-glass rounded-2xl p-6">
		<div class="mb-5 flex items-start justify-between gap-4">
			<div>
				<h2 class="section-title text-lg">Publish checklist</h2>
				<p class="text-muted-foreground mt-1 text-sm">Complete every required item before submitting for review.</p>
			</div>
			<span class="shrink-0 rounded-full px-3 py-1 text-xs font-semibold" :class="canSubmit ? 'bg-primary/15 text-primary' : 'bg-destructive/10 text-destructive'">
				{{ requiredComplete }} / {{ requiredTotal }} required
			</span>
		</div>

		<ul v-if="outstandingItems.length" class="space-y-2.5">
			<li v-for="item in outstandingItems" :key="item.title" class="bg-muted/30 flex items-start gap-3 rounded-xl border-l-2 p-3.5" :class="LEVEL_STYLES[item.level].accent">
				<component :is="LEVEL_STYLES[item.level].icon" class="mt-0.5 size-5 shrink-0" :class="LEVEL_STYLES[item.level].tone" />
				<div class="min-w-0 flex-1">
					<div class="flex flex-wrap items-center gap-2">
						<span class="text-foreground text-sm font-semibold">
							{{ item.title }}
						</span>
						<span class="rounded-full px-1.5 py-0.5 text-[10px] font-semibold tracking-wide uppercase" :class="LEVEL_STYLES[item.level].pill">
							{{ LEVEL_STYLES[item.level].label }}
						</span>
					</div>
					<p class="text-muted-foreground mt-1 text-xs leading-relaxed">
						{{ item.description }}
					</p>
				</div>
			</li>
		</ul>

		<div v-if="completedItems.length" class="mt-4 flex flex-wrap gap-2">
			<span v-for="item in completedItems" :key="item.title" class="bg-primary/10 text-primary inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium">
				<CircleCheck class="size-3.5" />
				{{ item.title }}
			</span>
		</div>

		<div v-if="!outstandingItems.length" class="text-primary mt-2 flex items-center gap-2 text-sm font-medium">
			<CircleCheck class="size-5" />
			Everything looks ready - submit your project for review.
		</div>

		<div class="mt-6 space-y-2 border-t pt-5">
			<Label for="changelog-note">Note for reviewers</Label>
			<p class="text-muted-foreground text-xs">Tell moderators what changed in this submission. Shown to the review team alongside a diff of your edits.</p>
			<MarkdownEditor id="changelog-note" v-model="changelog" :actions="NOTE_MARKDOWN_ACTIONS" :rows="4" placeholder="e.g. Updated the description and added two new categories." />
			<div v-if="status === PROJECT_STATUS.IN_REVIEW || status === PROJECT_STATUS.APPROVED" class="flex items-center justify-end gap-3">
				<span v-if="changelogError" class="text-destructive text-xs">{{ changelogError }}</span>
				<Button variant="outline" size="sm" :disabled="!changelogDirty || savingChangelog" @click="$emit('saveChangelog')">
					<Loader2 v-if="savingChangelog" class="size-4 animate-spin" />
					Save note
				</Button>
			</div>
		</div>

		<div class="mt-6 flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center sm:justify-between">
			<p class="text-muted-foreground text-xs">Your project stays private until a moderator reviews and approves it. Approved projects re-enter review when key details change.</p>
			<div v-if="status === PROJECT_STATUS.IN_REVIEW" class="flex shrink-0 flex-col items-stretch gap-2 sm:flex-row sm:items-center">
				<span class="text-muted-foreground inline-flex items-center gap-2 text-sm font-medium">
					<Clock class="size-4 text-amber-500" />
					In review
				</span>
				<Button variant="outline" class="gap-2" :disabled="submitting" @click="$emit('withdraw')">
					<Loader2 v-if="submitting" class="size-4 animate-spin" />
					<Undo2 v-else class="size-4" />
					Withdraw
				</Button>
			</div>
			<div v-else-if="status === PROJECT_STATUS.APPROVED && !hasPendingChanges" class="text-primary inline-flex shrink-0 items-center gap-2 text-sm font-medium">
				<CircleCheck class="size-4" />
				Live
			</div>
			<Button v-else class="btn-glow shrink-0" :disabled="!canSubmitNow || submitting" @click="$emit('submit')">
				<Loader2 v-if="submitting" class="size-4 animate-spin" />
				<Send v-else class="size-4" />
				{{ submitLabel }}
			</Button>
		</div>
	</section>
</template>
