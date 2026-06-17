<script setup lang="ts">
import { ExternalLink } from "@lucide/vue";

import licenseData from "~/data/licenses.json";

interface Library {
	name: string;
	url: string;
	license: string;
	licenseUrl: string;
}

interface LicenseGroup {
	title: string;
	description: string;
	libraries: Library[];
}

const groups = licenseData.groups as LicenseGroup[];
</script>

<template>
	<div class="page-canvas">
		<div class="mx-auto max-w-4xl px-6 py-16">
			<header class="mb-10 max-w-2xl">
				<p class="text-primary eyebrow mb-2">Open source</p>
				<h1 class="display-heading mb-4 text-4xl md:text-5xl">Licenses</h1>
				<p class="text-muted-foreground">
					Beacon is built on the shoulders of the open-source community. We're grateful to the maintainers of these libraries, which are licensed under permissive open-source licenses. Below
					is a list of the projects we use, along with their licenses.
				</p>
			</header>

			<section v-for="group in groups" :key="group.title" class="mb-12 last:mb-0">
				<div class="mb-4">
					<h2 class="section-title text-xl md:text-2xl">{{ group.title }}</h2>
					<p class="text-muted-foreground mt-1 text-sm">
						{{ group.description }}
					</p>
				</div>

				<ul class="divide-border/60 border-border/60 divide-y rounded-xl border">
					<li v-for="lib in group.libraries" :key="lib.name" class="hover:bg-accent/40 flex items-center justify-between gap-4 px-4 py-3 transition-colors">
						<a :href="lib.url" target="_blank" rel="noopener noreferrer" class="group hover:text-primary inline-flex items-center gap-1.5 text-sm font-medium transition-colors">
							{{ lib.name }}
							<ExternalLink class="size-3.5 opacity-0 transition-opacity group-hover:opacity-100" />
						</a>
						<a
							:href="lib.licenseUrl"
							target="_blank"
							rel="noopener noreferrer"
							class="bg-primary/15 text-primary hover:bg-primary/25 shrink-0 rounded-full px-2.5 py-0.5 text-xs font-semibold transition-colors"
						>
							{{ lib.license }}
						</a>
					</li>
				</ul>
			</section>

			<p class="text-muted-foreground mt-12 text-sm">
				Notice an error or a missing attribution?
				<NuxtLink to="/support" class="text-primary hover:underline"> Let us know </NuxtLink>and we'll fix it.
			</p>
		</div>
	</div>
</template>
