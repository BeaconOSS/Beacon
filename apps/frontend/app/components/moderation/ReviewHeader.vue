<script setup lang="ts">
import { Eye, ExternalLink, ShieldCheck } from "@lucide/vue";
import { relativeTime } from "~/scripts/formatters";

defineProps<{
	title: string;
	owner: string;
	typeLabel: string;
	slug: string;
	iconUrl: string | null;
	isFirstReview: boolean;
	submittedAt: string | null;
}>();
</script>

<template>
	<div class="mb-6 flex flex-wrap items-start gap-4">
		<div v-if="iconUrl" class="size-16 shrink-0 overflow-hidden rounded-2xl ring-1 ring-white/10">
			<img :src="iconUrl" :alt="title" class="size-full object-cover" />
		</div>
		<div v-else class="bg-primary/15 text-primary flex size-16 shrink-0 items-center justify-center rounded-2xl">
			<ShieldCheck class="size-7" />
		</div>

		<div class="min-w-0 flex-1">
			<div class="flex flex-wrap items-center gap-2">
				<h1 class="display-heading text-2xl sm:text-3xl">
					{{ title }}
				</h1>
				<span class="bg-primary/15 text-primary rounded-full px-2.5 py-0.5 text-xs font-semibold">
					{{ typeLabel }}
				</span>
				<span class="rounded-full bg-amber-500/15 px-2.5 py-0.5 text-xs font-semibold text-amber-500"> Awaiting review </span>
			</div>
			<p class="text-muted-foreground mt-1 text-sm">
				by {{ owner }} ·
				{{ isFirstReview ? "First submission" : "Update" }}
				<template v-if="submittedAt"> · submitted {{ relativeTime(submittedAt) }} </template>
			</p>
			<div class="mt-2 flex flex-wrap items-center gap-x-4 gap-y-1">
				<NuxtLink :to="`/projects/${slug}`" class="text-primary inline-flex items-center gap-1.5 text-sm hover:underline">
					<ExternalLink class="size-3.5" />
					View current PDP
				</NuxtLink>
				<NuxtLink :to="`/projects/${slug}?preview=pending`" class="text-primary inline-flex items-center gap-1.5 text-sm hover:underline">
					<Eye class="size-3.5" />
					Preview approved PDP
				</NuxtLink>
			</div>
		</div>
	</div>
</template>
