<script setup lang="ts">
import { Images, Loader2, Trash2, Upload } from "@lucide/vue";

import type { GalleryImage } from "~/scripts/pages/projects/types";

const caption = defineModel<string>({ required: true });

defineProps<{
	image: File | null;
	uploadError: string;
	uploadPending: boolean;
	locked: boolean;
	images: GalleryImage[];
}>();

defineEmits<{
	fileChange: [event: Event];
	submit: [];
	deleteImage: [id: string];
}>();

const confirmDeleteGalleryId = ref<string | null>(null);
</script>

<template>
	<section class="space-y-6">
		<div class="card-glass space-y-5 rounded-2xl p-6">
			<div>
				<h2 class="section-title mb-1 flex items-center gap-2 text-lg">
					<Images class="text-primary size-5" />
					Add a gallery image
				</h2>
				<p class="text-muted-foreground text-sm leading-relaxed">Showcase screenshots and renders of your project. Good imagery is often the first thing people notice.</p>
			</div>

			<div class="space-y-2">
				<Label for="gallery-caption">Caption (optional)</Label>
				<Input id="gallery-caption" v-model="caption" placeholder="Describe what this image shows" />
			</div>

			<div class="space-y-2">
				<Label for="gallery-file">Image</Label>
				<input
					id="gallery-file"
					type="file"
					accept="image/png,image/jpeg,image/webp,image/gif"
					class="border-input file:bg-muted file:text-foreground hover:file:bg-muted/70 block w-full cursor-pointer rounded-xl border bg-transparent text-sm file:mr-3 file:cursor-pointer file:rounded-lg file:border-0 file:px-3 file:py-2 file:text-sm file:font-medium"
					@change="$emit('fileChange', $event)"
				/>
				<p v-if="image" class="text-muted-foreground text-xs">
					{{ image.name }}
				</p>
			</div>

			<div class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center" :class="uploadError ? 'sm:justify-between' : 'sm:justify-end'">
				<p v-if="uploadError" class="text-destructive text-sm">
					{{ uploadError }}
				</p>
				<Button class="btn-glow shrink-0" :disabled="uploadPending || locked" @click="$emit('submit')">
					<Loader2 v-if="uploadPending" class="size-4 animate-spin" />
					<Upload v-else class="size-4" />
					Add image
				</Button>
			</div>
		</div>

		<div class="card-glass rounded-2xl p-6">
			<h2 class="section-title mb-1 flex items-center gap-2 text-lg">
				<Images class="text-primary size-5" />
				Gallery images
			</h2>
			<p class="text-muted-foreground mb-5 text-sm">
				{{ images.length }}
				{{ images.length === 1 ? "image" : "images" }}.
			</p>

			<p v-if="!images.length" class="text-muted-foreground border-border/60 rounded-xl border border-dashed p-8 text-center text-sm">No images yet. Upload your first image above.</p>

			<div v-else class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
				<div v-for="image in images" :key="image.id" class="border-border/60 bg-muted/20 group relative overflow-hidden rounded-xl border">
					<img :src="image.url" :alt="image.caption || 'Gallery image'" class="aspect-video w-full object-cover" />
					<div v-if="image.caption" class="text-muted-foreground p-3 text-xs">
						{{ image.caption }}
					</div>
					<div class="absolute top-2 right-2 flex items-center gap-2">
						<template v-if="confirmDeleteGalleryId === image.id">
							<Button
								variant="destructive"
								size="sm"
								@click="
									$emit('deleteImage', image.id);
									confirmDeleteGalleryId = null;
								"
							>
								Confirm
							</Button>
							<Button variant="secondary" size="sm" @click="confirmDeleteGalleryId = null"> Cancel </Button>
						</template>
						<Button v-else variant="secondary" size="icon" aria-label="Delete image" :disabled="locked" @click="confirmDeleteGalleryId = image.id">
							<Trash2 class="text-destructive size-4" />
						</Button>
					</div>
				</div>
			</div>
		</div>
	</section>
</template>
