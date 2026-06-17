<script setup lang="ts">
import { Clock, Download, Heart } from "@lucide/vue";
import { projectTypeLabel } from "~/scripts/pages/projects";
import { relativeTime } from "~/scripts/formatters";
import type { ProjectDetail } from "~/scripts/pages/projects/types";
import type { TypeStyle } from "~/scripts/pages/projects/detail/types";

defineProps<{
	project: ProjectDetail;
	iconSrc: string | null;
	typeStyle: TypeStyle;
}>();
</script>

<template>
	<div class="flex gap-5">
		<div class="flex size-28 shrink-0 items-center justify-center overflow-hidden rounded-2xl bg-gradient-to-br ring-1 ring-white/10" :class="iconSrc ? '' : typeStyle.gradient">
			<img v-if="iconSrc" :src="iconSrc" :alt="project.title" class="size-full object-cover" />
			<component :is="typeStyle.icon" v-else class="size-12" />
		</div>
		<div class="min-w-0">
			<div class="flex flex-wrap items-center gap-3">
				<h1 class="display-heading text-2xl sm:text-3xl">
					{{ project.title }}
				</h1>
				<span class="bg-primary/15 text-primary rounded-full px-3 py-1 text-xs font-semibold">
					{{ projectTypeLabel(project.project_type) }}
				</span>
			</div>
			<p class="text-muted-foreground mt-2 max-w-2xl break-words">
				{{ project.summary }}
			</p>
			<div class="text-muted-foreground mt-4 flex flex-wrap items-center gap-5 text-sm">
				<span class="inline-flex items-center gap-1.5">
					<Download class="size-4" />
					{{ project.download_count.toLocaleString() }} downloads
				</span>
				<span class="inline-flex items-center gap-1.5">
					<Heart class="size-4" />
					{{ (project.heart_count ?? 0).toLocaleString() }} hearts
				</span>
				<span class="inline-flex items-center gap-1.5">
					<Clock class="size-4" />
					{{ relativeTime(project.created_at) }}
				</span>
			</div>
		</div>
	</div>
</template>
