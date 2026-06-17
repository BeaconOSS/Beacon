<script setup lang="ts">
import { Calendar, ExternalLink, Link2, Scale, Users } from "@lucide/vue";

import type { ProjectLink } from "~/scripts/pages/projects/detail/types";
import type { ProjectDetail } from "~/scripts/pages/projects/types";

import { relativeTime } from "~/scripts/formatters";

defineProps<{
	project: ProjectDetail;
	links: ProjectLink[];
}>();
</script>

<template>
	<aside class="space-y-4 lg:w-72 lg:shrink-0">
		<div v-if="project.categories.length" class="border-border/60 bg-card/30 rounded-xl border p-4">
			<h3 class="eyebrow mb-3">Tags</h3>
			<div class="flex flex-wrap gap-2">
				<NuxtLink
					v-for="cat in project.categories"
					:key="cat.slug"
					:to="`/projects?category=${cat.slug}`"
					class="bg-muted hover:bg-primary/15 hover:text-primary rounded-full px-3 py-1 text-xs transition-colors"
				>
					{{ cat.name }}
				</NuxtLink>
			</div>
		</div>

		<div class="border-border/60 bg-card/30 rounded-xl border p-4">
			<h3 class="eyebrow mb-3 flex items-center gap-2">
				<Link2 class="size-3.5" />
				Links
			</h3>
			<p v-if="!links.length" class="text-muted-foreground text-sm">No links yet.</p>
			<ul v-else class="space-y-2">
				<li v-for="link in links" :key="link.label">
					<a :href="link.url" target="_blank" rel="noopener noreferrer" class="text-muted-foreground hover:text-primary flex items-center justify-between gap-2 text-sm transition-colors">
						<span>{{ link.label }}</span>
						<ExternalLink class="size-3.5 shrink-0" />
					</a>
				</li>
			</ul>
		</div>

		<div class="border-border/60 bg-card/30 rounded-xl border p-4">
			<h3 class="eyebrow mb-3 flex items-center gap-2">
				<Users class="size-3.5" />
				Creators
			</h3>
			<div class="flex items-center gap-3">
				<div class="bg-primary/15 text-primary flex size-9 items-center justify-center rounded-full text-sm font-semibold uppercase">
					{{ project.owner.charAt(0) }}
				</div>
				<div>
					<p class="text-sm font-medium">{{ project.owner }}</p>
					<p class="text-muted-foreground text-xs">Owner</p>
				</div>
			</div>
		</div>

		<div class="border-border/60 bg-card/30 rounded-xl border p-4">
			<h3 class="eyebrow mb-3">Details</h3>
			<dl class="space-y-2.5 text-sm">
				<div class="flex items-center justify-between gap-2">
					<dt class="text-muted-foreground flex items-center gap-1.5">
						<Scale class="size-3.5" />
						License
					</dt>
					<dd>{{ project.license || "Not specified" }}</dd>
				</div>
				<div class="flex items-center justify-between gap-2">
					<dt class="text-muted-foreground flex items-center gap-1.5">
						<Calendar class="size-3.5" />
						Published
					</dt>
					<dd>{{ relativeTime(project.created_at) }}</dd>
				</div>
			</dl>
		</div>
	</aside>
</template>
