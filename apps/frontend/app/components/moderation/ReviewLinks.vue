<script setup lang="ts">
import { ExternalLink, Link2 } from "@lucide/vue";

defineProps<{
	links: { label: string; url: string }[];
}>();
</script>

<template>
	<div class="border-border/60 bg-card/40 rounded-2xl border p-5">
		<p class="text-muted-foreground mb-3 text-xs font-semibold tracking-wide uppercase">Links</p>
		<ul class="space-y-2">
			<li v-for="link in links" :key="link.label">
				<a :href="link.url" target="_blank" rel="noopener noreferrer" class="text-primary inline-flex items-center gap-1.5 text-sm hover:underline">
					<Link2 class="size-3.5" />
					<span class="text-muted-foreground">{{ link.label }}:</span>
					<span class="truncate">{{ link.url }}</span>
					<ExternalLink class="size-3" />
				</a>
			</li>
		</ul>
	</div>
</template>
