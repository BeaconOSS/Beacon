<script setup lang="ts">
import ProjectChangelog from "./ProjectChangelog.vue";
import ProjectDescription from "./ProjectDescription.vue";
import ProjectGallery from "./ProjectGallery.vue";
import ProjectVersions from "./ProjectVersions.vue";

import type { GalleryImage, Version } from "~/scripts/pages/projects/types";

defineProps<{
	description: string;
	images: GalleryImage[];
	versions: Version[];
	changelogEntries: Version[];
	downloadUrl: (version: Version) => string;
}>();
</script>

<template>
	<Tabs default-value="description">
		<TabsList class="flex-wrap">
			<TabsTrigger value="description">Description</TabsTrigger>
			<TabsTrigger value="gallery">
				Gallery
				<span v-if="images.length" class="text-muted-foreground ml-1 text-xs">
					{{ images.length }}
				</span>
			</TabsTrigger>
			<TabsTrigger value="changelog">Changelog</TabsTrigger>
			<TabsTrigger value="versions">
				Versions
				<span v-if="versions.length" class="text-muted-foreground ml-1 text-xs">
					{{ versions.length }}
				</span>
			</TabsTrigger>
		</TabsList>

		<TabsContent value="description">
			<ProjectDescription :description="description" />
		</TabsContent>

		<TabsContent value="gallery">
			<ProjectGallery :images="images" />
		</TabsContent>

		<TabsContent value="changelog">
			<ProjectChangelog :entries="changelogEntries" />
		</TabsContent>

		<TabsContent value="versions">
			<ProjectVersions :versions="versions" :download-url="downloadUrl" />
		</TabsContent>
	</Tabs>
</template>
