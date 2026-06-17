<script setup lang="ts">
import { ArrowLeft, Eye } from "@lucide/vue";

defineProps<{
	slug: string;
}>();
</script>

<template>
	<div class="border-primary/40 bg-primary/5 mb-6 flex items-start gap-3 rounded-2xl border p-5">
		<Eye class="text-primary mt-0.5 size-5 shrink-0" />
		<div class="min-w-0">
			<h2 class="text-foreground font-semibold">Preview of pending changes</h2>
			<p class="text-muted-foreground mt-1 text-sm leading-relaxed">This is how the page will look once the submitted changes are approved. It isn't the version shown publicly.</p>
			<NuxtLink :to="`/projects/${slug}`" class="text-primary mt-2 inline-flex items-center gap-1.5 text-sm hover:underline">
				<ArrowLeft class="size-3.5" />
				View the live page
			</NuxtLink>
		</div>
	</div>
</template>
