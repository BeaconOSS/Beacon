<script setup lang="ts">
import { Rocket } from "@lucide/vue";
import { NAV_ITEMS } from "~/scripts/pages/projects/settings/meta";
import type { SectionId } from "~/scripts/pages/projects/settings/types";

const active = defineModel<SectionId>({ required: true });

defineProps<{
	navStatusDot: string;
	statusLabel: string;
}>();
</script>

<template>
	<aside class="shrink-0 lg:w-56">
		<nav class="flex flex-col gap-0.5 lg:sticky lg:top-24">
			<button
				type="button"
				class="flex items-center gap-2.5 rounded-lg px-3 py-2 text-left text-sm font-medium transition-colors"
				:class="active === 'publish' ? 'bg-primary/10 text-primary' : 'text-muted-foreground hover:bg-accent/40 hover:text-foreground'"
				@click="active = 'publish'"
			>
				<Rocket class="size-4 shrink-0" />
				<span class="flex-1">Publish</span>
				<span class="size-2 shrink-0 rounded-full" :class="navStatusDot" :title="statusLabel" />
			</button>
			<div class="bg-border/60 my-2 h-px" />
			<button
				v-for="item in NAV_ITEMS"
				:key="item.id"
				type="button"
				class="flex items-center gap-2.5 rounded-lg px-3 py-2 text-left text-sm font-medium transition-colors"
				:class="active === item.id ? 'bg-primary/10 text-primary' : 'text-muted-foreground hover:bg-accent/40 hover:text-foreground'"
				@click="active = item.id"
			>
				<component :is="item.icon" class="size-4" />
				{{ item.label }}
			</button>
		</nav>
	</aside>
</template>
