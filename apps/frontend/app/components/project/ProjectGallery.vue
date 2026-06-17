<script setup lang="ts">
import type { GalleryImage } from "~/scripts/pages/projects/types";

defineProps<{
	images: GalleryImage[];
}>();
</script>

<template>
	<div class="border-border/60 bg-card/30 rounded-xl border p-6">
		<div v-if="images.length" class="grid gap-4 sm:grid-cols-2">
			<figure v-for="image in images" :key="image.id" class="border-border/60 group relative overflow-hidden rounded-lg border">
				<img :src="image.url" :alt="image.caption" class="aspect-video w-full object-cover" />
				<figcaption v-if="image.caption" class="text-muted-foreground bg-card/80 px-3 py-2 text-sm">
					{{ image.caption }}
				</figcaption>
			</figure>
		</div>
		<p v-else class="text-muted-foreground">No screenshots yet.</p>
	</div>
</template>
