<script setup lang="ts">
import type { DecisionSignal } from "~/scripts/pages/moderation/types";

import { checkStatusLabel, signalMeta } from "~/scripts/pages/moderation/meta";

defineProps<{
	signal: DecisionSignal;
}>();
</script>

<template>
	<div class="rounded-2xl border p-5" :class="signalMeta(signal.overall).class">
		<div class="flex items-start gap-3">
			<component
				:is="signalMeta(signal.overall).icon"
				class="mt-0.5 size-6 shrink-0"
				:class="{
					'animate-spin': signal.overall === 'pending',
				}"
			/>
			<div class="min-w-0 flex-1">
				<p class="text-base font-semibold">
					{{ signalMeta(signal.overall).label }}
				</p>
				<p class="text-sm opacity-90">
					{{ signalMeta(signal.overall).sub }}
				</p>
			</div>
		</div>

		<ul class="border-current/15 mt-4 space-y-1.5 border-t pt-3">
			<li v-for="check in signal.checks" :key="check.label" class="flex items-center justify-between gap-3 text-sm">
				<span class="flex shrink-0 items-center gap-2">
					<span class="rounded-full px-2 py-0.5 text-xs font-semibold" :class="signalMeta(check.status).badge">
						{{ checkStatusLabel(check.status) }}
					</span>
					<span class="text-foreground">{{ check.label }}</span>
				</span>
				<span class="text-muted-foreground truncate text-right text-xs">
					{{ check.detail }}
				</span>
			</li>
		</ul>
	</div>
</template>
