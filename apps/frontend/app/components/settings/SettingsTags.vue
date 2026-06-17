<script setup lang="ts">
import { Loader2, Tags } from "@lucide/vue";
import type { Category } from "~/scripts/pages/projects/types";

defineProps<{
	availableCategories: Category[];
	selectedCategoryIds: string[];
	tagsDirty: boolean;
	savingTags: boolean;
	tagsError: string;
	locked: boolean;
}>();

defineEmits<{
	toggle: [id: string];
	save: [];
}>();
</script>

<template>
	<section class="card-glass space-y-5 rounded-2xl p-6">
		<div>
			<h2 class="section-title mb-1 flex items-center gap-2 text-lg">
				<Tags class="text-primary size-5" />
				Tags
			</h2>
			<p class="text-muted-foreground text-sm leading-relaxed">Pick the categories that best describe your project so people can find it when browsing. Choose the ones that genuinely fit.</p>
		</div>

		<p v-if="!availableCategories.length" class="text-muted-foreground border-border/60 rounded-xl border border-dashed p-8 text-center text-sm">
			No categories are available for this project type yet.
		</p>

		<div v-else class="flex flex-wrap gap-2">
			<button
				v-for="category in availableCategories"
				:key="category.id"
				type="button"
				class="rounded-full border px-3 py-1.5 text-sm font-medium transition-colors"
				:class="selectedCategoryIds.includes(category.id) ? 'border-primary bg-primary/15 text-primary' : 'border-border/60 text-muted-foreground hover:border-border hover:text-foreground'"
				@click="$emit('toggle', category.id)"
			>
				{{ category.name }}
			</button>
		</div>

		<div class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center" :class="tagsError ? 'sm:justify-between' : 'sm:justify-end'">
			<p v-if="tagsError" class="text-destructive text-sm">
				{{ tagsError }}
			</p>
			<Button class="btn-glow shrink-0" :disabled="!tagsDirty || savingTags || locked" @click="$emit('save')">
				<Loader2 v-if="savingTags" class="size-4 animate-spin" />
				Save tags
			</Button>
		</div>
	</section>
</template>
