<script setup lang="ts">
import { ArrowLeft, ChevronRight, Clock, Layers } from "@lucide/vue";

import type { ModerationQueueItem } from "~/scripts/pages/moderation";
import type { QueueAge } from "~/scripts/pages/moderation/types";

import { relativeTime } from "~/scripts/formatters";

defineProps<{
	position: number;
	total: number;
	prev: ModerationQueueItem | null;
	next: ModerationQueueItem | null;
	age: QueueAge | null;
	submittedAt: string | null;
}>();
</script>

<template>
	<div class="mb-6 -mt-2 flex flex-wrap items-center gap-2 text-xs">
		<span class="border-border/50 text-muted-foreground inline-flex items-center gap-1.5 rounded-full border px-2.5 py-1 font-medium">
			<Layers class="size-3.5" />
			Pack {{ position }} of {{ total }} in queue
		</span>
		<span v-if="age && submittedAt" class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 font-semibold" :class="age.class">
			<Clock class="size-3.5" />
			{{ age.label }} · waiting
			{{ relativeTime(submittedAt) }}
		</span>
		<span class="flex-1" />
		<NuxtLink
			v-if="prev"
			:to="`/moderation/${prev.slug}`"
			class="border-border/50 hover:bg-muted/40 text-muted-foreground hover:text-foreground inline-flex items-center gap-1 rounded-full border px-2.5 py-1 font-medium transition-colors"
		>
			<ArrowLeft class="size-3.5" />
			Prev
		</NuxtLink>
		<NuxtLink
			v-if="next"
			:to="`/moderation/${next.slug}`"
			class="border-border/50 hover:bg-muted/40 text-muted-foreground hover:text-foreground inline-flex items-center gap-1 rounded-full border px-2.5 py-1 font-medium transition-colors"
		>
			Next
			<ChevronRight class="size-3.5" />
		</NuxtLink>
	</div>
</template>
