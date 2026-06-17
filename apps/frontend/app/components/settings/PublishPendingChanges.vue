<script setup lang="ts">
import { ArrowLeft, GitCompareArrows } from "@lucide/vue";
import type { PendingChangeRow } from "~/scripts/pages/projects/settings/types";

defineProps<{
	pendingChanges: PendingChangeRow[];
	iconChanged: boolean;
	publishedIconUrl: string | null;
	iconUrl: string | null;
}>();
</script>

<template>
	<section class="card-glass rounded-2xl p-6">
		<div class="mb-5 flex items-start gap-3">
			<GitCompareArrows class="text-primary mt-0.5 size-5 shrink-0" />
			<div>
				<h2 class="section-title text-lg">Changes awaiting publish</h2>
				<p class="text-muted-foreground mt-1 text-sm">These edits are saved, but your public page still shows the last approved version. Submit them for review to make them live.</p>
			</div>
		</div>

		<div v-if="pendingChanges.length" class="overflow-hidden rounded-xl border border-border/60">
			<table class="w-full text-sm">
				<thead>
					<tr class="bg-background/40 text-muted-foreground text-left">
						<th class="w-24 px-3 py-2 font-medium">Field</th>
						<th class="px-3 py-2 font-medium">Live</th>
						<th class="px-3 py-2 font-medium">Your edit</th>
					</tr>
				</thead>
				<tbody>
					<tr v-for="row in pendingChanges" :key="row.label" class="border-border/40 border-t align-top">
						<td class="text-muted-foreground px-3 py-2 font-medium whitespace-nowrap">
							{{ row.label }}
						</td>
						<td class="text-muted-foreground px-3 py-2 break-words whitespace-pre-wrap">
							<span v-if="row.long" class="text-muted-foreground/70 italic">Previous version</span>
							<template v-else>{{ row.before || "-" }}</template>
						</td>
						<td class="text-foreground px-3 py-2 font-medium break-words whitespace-pre-wrap">
							<span v-if="row.long" class="text-primary">Updated</span>
							<template v-else>{{ row.after || "-" }}</template>
						</td>
					</tr>
				</tbody>
			</table>
		</div>

		<div v-if="iconChanged" class="mt-4 flex items-center gap-4">
			<span class="text-muted-foreground text-xs font-semibold tracking-wide uppercase"> Icon </span>
			<div class="flex items-center gap-3">
				<div v-if="publishedIconUrl" class="flex flex-col items-center gap-1">
					<img :src="publishedIconUrl" alt="Live icon" class="size-14 rounded-lg object-cover ring-1 ring-white/10" />
					<span class="text-muted-foreground text-[10px]">Live</span>
				</div>
				<ArrowLeft v-if="publishedIconUrl" class="text-muted-foreground size-4 rotate-180" />
				<div v-if="iconUrl" class="flex flex-col items-center gap-1">
					<img :src="iconUrl" alt="New icon" class="size-14 rounded-lg object-cover ring-1 ring-white/10" />
					<span class="text-primary text-[10px]">Your edit</span>
				</div>
			</div>
		</div>
	</section>
</template>
