<script setup lang="ts">
import { relativeTime } from "~/scripts/formatters";
import { actionMeta } from "~/scripts/pages/moderation/meta";
import type { ReviewHistoryEntry } from "~/scripts/pages/projects/types";

defineProps<{
	history: ReviewHistoryEntry[];
	expanded: Set<number>;
	clamp: number;
}>();

defineEmits<{
	toggle: [idx: number];
}>();
</script>

<template>
	<div class="border-border/60 bg-card/40 rounded-2xl border p-5">
		<p class="text-muted-foreground mb-3 text-xs font-semibold tracking-wide uppercase">Review history</p>
		<p v-if="!history.length" class="text-muted-foreground text-sm">No previous reviews.</p>
		<ul v-else class="space-y-3">
			<li v-for="(entry, idx) in history" :key="idx" class="border-border/40 border-l-2 pl-3">
				<div class="flex flex-wrap items-center gap-2">
					<span class="rounded-full px-2 py-0.5 text-xs font-semibold" :class="actionMeta(entry.action).class">
						{{ actionMeta(entry.action).label }}
					</span>
					<span class="text-muted-foreground text-xs">
						{{ entry.reviewer }} ·
						{{ relativeTime(entry.created_at) }}
					</span>
				</div>
				<p v-if="entry.notes.trim()" class="text-muted-foreground mt-1 text-sm whitespace-pre-wrap">
					{{ entry.notes.trim().length > clamp && !expanded.has(idx) ? entry.notes.trim().slice(0, clamp) + "…" : entry.notes }}
				</p>
				<button v-if="entry.notes.trim().length > clamp" type="button" class="text-primary mt-0.5 text-xs font-medium hover:underline" @click="$emit('toggle', idx)">
					{{ expanded.has(idx) ? "Show less" : "Show more" }}
				</button>
			</li>
		</ul>
	</div>
</template>
