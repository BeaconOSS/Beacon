<script setup lang="ts">
import { LayoutGrid, Monitor, Moon, MoonStar, Rows3, Sun } from "@lucide/vue";

import type { ListLayout, ThemePref } from "~/scripts/settings";

import { Separator } from "@/components/ui/separator";
import { Switch } from "@/components/ui/switch";
import { useSettings } from "~/scripts/settings";

const { settings } = useSettings();

const themes: { value: ThemePref; label: string; icon: typeof Sun }[] = [
	{ value: "system", label: "Sync with system", icon: Monitor },
	{ value: "light", label: "Light", icon: Sun },
	{ value: "dark", label: "Dark", icon: Moon },
	{ value: "oled", label: "OLED", icon: MoonStar },
];

const layouts: {
	value: ListLayout;
	label: string;
	description: string;
	icon: typeof LayoutGrid;
}[] = [
	{
		value: "grid",
		label: "Grid",
		description: "Cards in a responsive grid",
		icon: LayoutGrid,
	},
	{
		value: "rows",
		label: "Rows",
		description: "One project per row",
		icon: Rows3,
	},
];

const toggles: {
	key: "advancedRendering" | "externalLinksNewTab" | "filtersSidebarRight" | "contentSidebarLeft";
	label: string;
	description: string;
}[] = [
	{
		key: "advancedRendering",
		label: "Advanced rendering",
		description: "Enables advanced rendering such as blur effects that may cause performance issues without hardware-accelerated rendering.",
	},
	{
		key: "externalLinksNewTab",
		label: "Open external links in new tab",
		description:
			"Make links which go outside of Beacon open in a new tab. No matter this setting, links on the same domain and in Markdown descriptions will open in the same tab, and links on ads and edit pages will open in a new tab.",
	},
	{
		key: "filtersSidebarRight",
		label: "Right-aligned filters sidebar on search pages",
		description: "Aligns the filters sidebar to the right of the search results.",
	},
	{
		key: "contentSidebarLeft",
		label: "Left-aligned sidebar on content pages",
		description: "Aligns the sidebar to the left of the page's content.",
	},
];
</script>

<template>
	<div class="page-canvas">
		<div class="mx-auto max-w-3xl px-6 py-16">
			<header class="mb-10">
				<p class="text-primary eyebrow mb-2">Preferences</p>
				<h1 class="display-heading text-3xl md:text-4xl">Settings</h1>
				<p class="text-muted-foreground mt-2">Personalize how Beacon looks and behaves.</p>
			</header>

			<section class="mb-10">
				<h2 class="section-title mb-1 text-base">Theme</h2>
				<p class="text-muted-foreground mb-4 text-sm">Choose your preferred color theme.</p>
				<div class="grid grid-cols-2 gap-3 sm:grid-cols-4">
					<button
						v-for="theme in themes"
						:key="theme.value"
						type="button"
						class="card-glass focus-visible:ring-ring flex flex-col items-center gap-2 rounded-xl p-4 text-center transition-colors outline-none focus-visible:ring-2"
						:class="settings.theme === theme.value ? 'ring-primary ring-2' : 'hover:border-primary/40'"
						:aria-pressed="settings.theme === theme.value"
						@click="settings.theme = theme.value"
					>
						<component :is="theme.icon" class="size-6" :class="settings.theme === theme.value ? 'text-primary' : 'text-muted-foreground'" />
						<span class="text-xs font-medium">{{ theme.label }}</span>
					</button>
				</div>
			</section>

			<Separator class="my-8" />

			<section class="mb-10">
				<h2 class="section-title mb-1 text-base">Project list layout</h2>
				<p class="text-muted-foreground mb-4 text-sm">Pick how project lists are displayed on browse and search pages.</p>
				<div class="grid grid-cols-2 gap-3">
					<button
						v-for="layout in layouts"
						:key="layout.value"
						type="button"
						class="card-glass focus-visible:ring-ring flex items-center gap-3 rounded-xl p-4 text-left transition-colors outline-none focus-visible:ring-2"
						:class="settings.listLayout === layout.value ? 'ring-primary ring-2' : 'hover:border-primary/40'"
						:aria-pressed="settings.listLayout === layout.value"
						@click="settings.listLayout = layout.value"
					>
						<component :is="layout.icon" class="size-5 shrink-0" :class="settings.listLayout === layout.value ? 'text-primary' : 'text-muted-foreground'" />
						<span>
							<span class="block text-sm font-medium">{{ layout.label }}</span>
							<span class="text-muted-foreground block text-xs">
								{{ layout.description }}
							</span>
						</span>
					</button>
				</div>
			</section>

			<Separator class="my-8" />

			<section>
				<h2 class="section-title mb-1 text-base">Features</h2>
				<p class="text-muted-foreground mb-4 text-sm">Toggle optional interface behaviors.</p>
				<ul class="divide-border/60 divide-y">
					<li v-for="toggle in toggles" :key="toggle.key" class="flex items-start justify-between gap-4 py-4">
						<div>
							<p class="text-sm font-medium">{{ toggle.label }}</p>
							<p class="text-muted-foreground mt-0.5 text-xs">
								{{ toggle.description }}
							</p>
						</div>
						<Switch v-model="settings[toggle.key]" :aria-label="toggle.label" class="mt-0.5 shrink-0" />
					</li>
				</ul>
			</section>
		</div>
	</div>
</template>
