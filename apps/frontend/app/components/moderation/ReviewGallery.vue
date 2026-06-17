<script setup lang="ts">
import { Images } from "@lucide/vue";
import type { GalleryItem } from "~/scripts/pages/projects/types";

const props = defineProps<{
	gallery: GalleryItem[];
	withBase: (path: string | null) => string | null;
}>();

const emit = defineEmits<{
	openLightbox: [url: string, caption: string, pixelated: boolean];
}>();

function zoom(image: GalleryItem) {
	const url = props.withBase(image.url);
	if (url) emit("openLightbox", url, image.caption, false);
}
</script>

<template>
	<div class="border-border/60 bg-card/40 rounded-2xl border p-5">
		<p class="text-muted-foreground mb-3 inline-flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase">
			<Images class="size-3.5" /> Gallery
			<span class="text-muted-foreground/70 normal-case">({{ gallery.length }})</span>
		</p>
		<p v-if="!gallery.length" class="text-muted-foreground text-sm">No gallery images.</p>
		<div v-else class="grid grid-cols-2 gap-3 sm:grid-cols-3">
			<figure v-for="image in gallery" :key="image.id" class="border-border/40 bg-background/40 overflow-hidden rounded-lg border">
				<button type="button" class="block aspect-video w-full overflow-hidden" @click="zoom(image)">
					<img :src="withBase(image.url)!" :alt="image.caption || 'Gallery image'" loading="lazy" class="size-full cursor-zoom-in object-cover transition-transform hover:scale-105" />
				</button>
				<figcaption v-if="image.caption.trim()" class="text-muted-foreground px-2 py-1.5 text-xs">
					{{ image.caption }}
				</figcaption>
			</figure>
		</div>
	</div>
</template>
