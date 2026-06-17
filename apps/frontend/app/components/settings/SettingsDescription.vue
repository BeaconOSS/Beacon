<script setup lang="ts">
import { FileText, Loader2 } from "@lucide/vue";

import MarkdownEditor from "~/components/settings/MarkdownEditor.vue";
import { RECOMMENDED_DESCRIPTION_LENGTH } from "~/scripts/pages/projects/settings";
import { MARKDOWN_ACTIONS } from "~/scripts/pages/projects/settings/meta";

const description = defineModel<string>({ required: true });

defineProps<{
	descriptionDirty: boolean;
	savingDescription: boolean;
	descriptionError: string;
	locked: boolean;
}>();

defineEmits<{ save: [] }>();

const descriptionLength = computed(() => description.value.trim().length);
</script>

<template>
	<section class="space-y-6">
		<div class="card-glass space-y-5 rounded-2xl p-6">
			<div>
				<h2 class="section-title mb-1 flex items-center gap-2 text-lg">
					<FileText class="text-primary size-5" />
					Description
				</h2>
				<p class="text-muted-foreground text-sm leading-relaxed">
					Use this space for a full, extended description of your project - what it is, what it adds, how it works, and how to use it. It must be honest and accurately reflect the actual
					project: don't promise features it doesn't have or misrepresent what players will get. Full Markdown formatting is supported.
				</p>
			</div>

			<MarkdownEditor
				id="project-description"
				v-model="description"
				:actions="MARKDOWN_ACTIONS"
				:rows="14"
				:spellcheck="true"
				min-height-class="min-h-64"
				placeholder="# My project&#10;&#10;Describe your project in detail using Markdown..."
			/>

			<div class="flex flex-wrap items-center justify-between gap-2">
				<p class="text-xs" :class="descriptionLength < RECOMMENDED_DESCRIPTION_LENGTH ? 'text-amber-500' : 'text-muted-foreground'">
					{{ descriptionLength }} characters · {{ RECOMMENDED_DESCRIPTION_LENGTH }}+ recommended
				</p>
				<a
					href="https://www.markdownguide.org/basic-syntax/"
					target="_blank"
					rel="noopener noreferrer"
					class="text-muted-foreground hover:text-foreground text-xs underline-offset-2 hover:underline"
				>
					Markdown formatting help
				</a>
			</div>

			<div class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center" :class="descriptionError ? 'sm:justify-between' : 'sm:justify-end'">
				<p v-if="descriptionError" class="text-destructive text-sm">
					{{ descriptionError }}
				</p>
				<Button class="btn-glow shrink-0" :disabled="!descriptionDirty || savingDescription || locked" @click="$emit('save')">
					<Loader2 v-if="savingDescription" class="size-4 animate-spin" />
					Save description
				</Button>
			</div>
		</div>
	</section>
</template>
