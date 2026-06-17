<script setup lang="ts">
import { BarChart3 } from "@lucide/vue";

import type { ProjectAnalytics } from "~/scripts/pages/projects/types";

const props = defineProps<{
	analytics: ProjectAnalytics | null;
	analyticsError: string;
	analyticsPending: boolean;
}>();

const analyticsMax = computed(() => {
	const series = props.analytics?.series ?? [];
	return Math.max(1, ...series.map((d) => Math.max(d.views, d.downloads)));
});
</script>

<template>
	<section class="space-y-6">
		<div class="card-glass space-y-1 rounded-2xl p-6">
			<h2 class="section-title flex items-center gap-2 text-lg">
				<BarChart3 class="text-primary size-5" />
				Analytics
			</h2>
			<p class="text-muted-foreground text-sm leading-relaxed">
				Views and downloads over the last
				{{ analytics?.range_days ?? 30 }} days. Only approved, public projects collect stats.
			</p>
		</div>

		<p v-if="analyticsError" class="card-glass text-destructive rounded-2xl p-6 text-sm">
			{{ analyticsError }}
		</p>

		<p v-else-if="analyticsPending && !analytics" class="text-muted-foreground card-glass rounded-2xl p-6 text-sm">Loading analytics…</p>

		<template v-else-if="analytics">
			<div class="grid gap-4 sm:grid-cols-3">
				<div class="card-glass rounded-2xl p-5">
					<p class="text-muted-foreground text-xs">Views ({{ analytics.range_days }}d)</p>
					<p class="mt-1 text-2xl font-semibold">
						{{ analytics.total_views.toLocaleString() }}
					</p>
				</div>
				<div class="card-glass rounded-2xl p-5">
					<p class="text-muted-foreground text-xs">Downloads ({{ analytics.range_days }}d)</p>
					<p class="mt-1 text-2xl font-semibold">
						{{ analytics.total_downloads.toLocaleString() }}
					</p>
				</div>
				<div class="card-glass rounded-2xl p-5">
					<p class="text-muted-foreground text-xs">Downloads (all time)</p>
					<p class="mt-1 text-2xl font-semibold">
						{{ analytics.all_time_downloads.toLocaleString() }}
					</p>
				</div>
			</div>

			<div class="card-glass rounded-2xl p-6">
				<div class="mb-4 flex items-center gap-4 text-xs">
					<span class="flex items-center gap-1.5">
						<span class="bg-primary size-2.5 rounded-full" />
						Views
					</span>
					<span class="flex items-center gap-1.5">
						<span class="size-2.5 rounded-full bg-emerald-500" />
						Downloads
					</span>
				</div>
				<div class="flex h-40 items-end gap-1" role="img" aria-label="Daily views and downloads">
					<div
						v-for="point in analytics.series"
						:key="point.day"
						class="group relative flex h-full flex-1 items-end justify-center gap-0.5"
						:title="`${point.day}: ${point.views} views, ${point.downloads} downloads`"
					>
						<div
							class="bg-primary/70 w-1/2 rounded-t-sm"
							:style="{
								height: `${(point.views / analyticsMax) * 100}%`,
							}"
						/>
						<div
							class="w-1/2 rounded-t-sm bg-emerald-500/70"
							:style="{
								height: `${(point.downloads / analyticsMax) * 100}%`,
							}"
						/>
					</div>
				</div>
				<div class="text-muted-foreground mt-2 flex justify-between text-[11px]">
					<span>{{ analytics.series[0]?.day }}</span>
					<span>
						{{ analytics.series[analytics.series.length - 1]?.day }}
					</span>
				</div>
			</div>
		</template>
	</section>
</template>
