<script setup lang="ts">
import { ArrowLeft } from "@lucide/vue";

import type { FieldDiff } from "~/scripts/pages/moderation/types";
import type { PendingReview } from "~/scripts/pages/projects/types";

const props = defineProps<{
	review: PendingReview;
	diffs: FieldDiff[];
	withBase: (path: string | null) => string | null;
}>();

const emit = defineEmits<{
	openLightbox: [url: string, caption: string, pixelated: boolean];
}>();

function zoom(path: string | null, caption: string) {
	const url = props.withBase(path);
	if (url) emit("openLightbox", url, caption, true);
}
</script>

<template>
	<!-- Creator's note -->
	<div class="border-border/60 bg-card/40 rounded-2xl border p-5">
		<p class="text-muted-foreground mb-1 text-xs font-semibold tracking-wide uppercase">
			{{ review.is_first_review ? "First submission" : "Changes submitted for review" }}
		</p>
		<p class="text-foreground text-sm whitespace-pre-wrap">
			{{ review.changelog?.trim() || "The creator did not leave a note." }}
		</p>
	</div>

	<!-- Field changes -->
	<div class="border-border/60 overflow-hidden rounded-2xl border">
		<table class="w-full text-sm">
			<thead>
				<tr class="bg-background/40 text-muted-foreground text-left">
					<th class="w-28 px-3 py-2 font-medium">Field</th>
					<th v-if="!review.is_first_review" class="px-3 py-2 font-medium">Current (live)</th>
					<th class="px-3 py-2 font-medium">
						{{ review.is_first_review ? "Submitted" : "New" }}
					</th>
				</tr>
			</thead>
			<tbody>
				<tr v-for="row in diffs" :key="row.label" class="border-border/40 border-t align-top" :class="row.changed ? 'bg-primary/5' : ''">
					<td class="text-muted-foreground px-3 py-2 font-medium whitespace-nowrap">
						{{ row.label }}
						<span v-if="row.changed" class="text-primary ml-1 text-xs">•</span>
					</td>
					<td v-if="!review.is_first_review" class="text-muted-foreground px-3 py-2 break-words whitespace-pre-wrap">
						{{ row.before || "-" }}
					</td>
					<td class="text-foreground px-3 py-2 break-words whitespace-pre-wrap" :class="row.changed ? 'font-medium' : ''">
						<template v-if="row.changed && row.segments">
							<span
								v-for="(seg, si) in row.segments"
								:key="si"
								:class="{
									'rounded bg-emerald-500/20 text-emerald-300': seg.type === 'add',
									'rounded bg-red-500/20 text-red-300/80 line-through': seg.type === 'remove',
								}"
								>{{ seg.text }}</span
							>
						</template>
						<template v-else>{{ row.after || "-" }}</template>
					</td>
				</tr>
			</tbody>
		</table>
	</div>

	<!-- Icon diff -->
	<div v-if="review.icon_changed" class="border-border/60 bg-card/40 flex items-center gap-4 rounded-2xl border p-5">
		<span class="text-muted-foreground text-xs font-semibold tracking-wide uppercase"> Icon </span>
		<div class="flex items-center gap-3">
			<div v-if="!review.is_first_review && review.published?.icon_url" class="flex flex-col items-center gap-1">
				<img
					:src="withBase(review.published.icon_url)!"
					alt="Current icon"
					class="size-14 cursor-zoom-in rounded-lg object-cover ring-1 ring-white/10"
					@click="zoom(review.published.icon_url, 'Current icon')"
				/>
				<span class="text-muted-foreground text-[10px]">Current</span>
			</div>
			<ArrowLeft v-if="!review.is_first_review" class="text-muted-foreground size-4 rotate-180" />
			<div v-if="review.pending.icon_url" class="flex flex-col items-center gap-1">
				<img
					:src="withBase(review.pending.icon_url)!"
					alt="New icon"
					class="size-14 cursor-zoom-in rounded-lg object-cover ring-1 ring-white/10"
					@click="zoom(review.pending.icon_url, 'New icon')"
				/>
				<span class="text-primary text-[10px]">New</span>
			</div>
		</div>
	</div>
</template>
