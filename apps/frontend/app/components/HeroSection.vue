<script setup lang="ts">
import { ArrowRight, Compass, Search } from "@lucide/vue";
import { useAuth } from "~/scripts/auth";
import { Button } from "@/components/ui/button";

const { user } = useAuth();

const query = ref("");

const categories = [
	{ label: "Add-Ons", to: "/projects?type=addon" },
	{ label: "Worlds", to: "/projects?type=world" },
	{ label: "Resource packs", to: "/projects?type=resource_pack" },
	{ label: "Skins", to: "/projects?type=skin_pack" },
];

function search() {
	const q = query.value.trim();
	navigateTo(q ? `/projects?q=${encodeURIComponent(q)}` : "/projects");
}
</script>

<template>
	<section class="relative overflow-hidden px-6 py-20 md:py-28">
		<div class="mx-auto max-w-5xl text-center">
			<div class="border-border bg-card/60 text-muted-foreground mb-6 inline-flex items-center gap-2 rounded-full border px-3 py-1 text-xs font-medium backdrop-blur">
				<span class="bg-primary size-1.5 rounded-full" />
				Open-source Bedrock content, for a good cause
			</div>

			<h1 class="display-heading mb-5 text-5xl md:text-7xl">
				Browse the best of
				<span class="gradient-brand-text-shimmer">Minecraft Bedrock</span>
			</h1>

			<p class="text-muted-foreground mx-auto mb-9 max-w-2xl text-lg md:text-xl">
				Discover Add-Ons, worlds, resource packs, and skins - or publish your own. Every BeaconOSS project is open source, and our profits go to charity.
			</p>

			<form class="mx-auto mb-5 flex max-w-2xl items-center gap-2" @submit.prevent="search">
				<div class="relative flex-1">
					<Search class="text-muted-foreground pointer-events-none absolute top-1/2 left-4 size-5 -translate-y-1/2" />
					<input
						v-model="query"
						type="search"
						placeholder="Search Add-Ons, worlds, skins..."
						class="border-border bg-card/70 focus:border-primary focus:ring-primary/30 h-13 w-full rounded-full border py-3 pr-4 pl-12 text-base shadow-sm backdrop-blur outline-none focus:ring-4"
					/>
				</div>
				<Button type="submit" size="lg" class="btn-glow h-13 px-6">
					<Search class="md:hidden" />
					<span class="hidden md:inline">Search</span>
				</Button>
			</form>

			<div class="mb-9 flex flex-wrap justify-center gap-2">
				<NuxtLink v-for="category in categories" :key="category.label" :to="category.to" class="brand-pill">
					{{ category.label }}
				</NuxtLink>
			</div>

			<div class="flex flex-wrap justify-center gap-3">
				<Button as-child size="lg" variant="outline">
					<NuxtLink to="/projects">
						<Compass />
						Discover everything
					</NuxtLink>
				</Button>
				<Button v-if="!user" as-child size="lg" variant="ghost">
					<NuxtLink to="/register">
						Create an account
						<ArrowRight />
					</NuxtLink>
				</Button>
			</div>
		</div>
	</section>
</template>
