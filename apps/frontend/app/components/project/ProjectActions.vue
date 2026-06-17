<script setup lang="ts">
import { Bookmark, ChevronDown, Copy, Download, Flag, Heart, Link2, MoreHorizontal, Pencil } from "@lucide/vue";
import { formatBytes } from "~/scripts/formatters";
import { channelLabel } from "~/scripts/pages/projects/detail/meta";
import type { ProjectDetail, Version } from "~/scripts/pages/projects/types";

defineProps<{
	project: ProjectDetail;
	slug: string;
	isOwner: boolean;
	latestVersion: Version | null;
	downloadableVersions: Version[];
	downloadUrl: (version: Version) => string;
	heartPending: boolean;
	savePending: boolean;
}>();

defineEmits<{
	heart: [];
	save: [];
	copyId: [];
	copyLink: [];
	report: [];
}>();
</script>

<template>
	<div class="flex shrink-0 items-center gap-2">
		<Button v-if="isOwner" as-child variant="outline" class="gap-2">
			<NuxtLink :to="`/${project.owner}/${slug}/settings`">
				<Pencil class="size-4" />
				Edit
			</NuxtLink>
		</Button>

		<DropdownMenu>
			<DropdownMenuTrigger as-child>
				<Button class="btn-glow gap-2" :disabled="!latestVersion">
					<Download class="size-4" />
					{{ latestVersion ? "Download" : "No downloads" }}
					<ChevronDown class="size-4 opacity-70" />
				</Button>
			</DropdownMenuTrigger>
			<DropdownMenuContent align="end" class="w-72">
				<DropdownMenuLabel>Choose a version</DropdownMenuLabel>
				<DropdownMenuSeparator />
				<template v-if="downloadableVersions.length">
					<DropdownMenuItem v-for="v in downloadableVersions" :key="v.id" as-child>
						<a :href="downloadUrl(v)" class="flex items-center justify-between gap-3">
							<span class="flex flex-col">
								<span class="font-medium">{{ v.version_number }}</span>
								<span class="text-muted-foreground text-xs">
									{{ channelLabel(v.channel) }}
									<template v-if="v.file"> · {{ formatBytes(v.file.size) }}</template>
								</span>
							</span>
							<Download class="size-4 opacity-70" />
						</a>
					</DropdownMenuItem>
				</template>
				<DropdownMenuItem v-else disabled> No downloads yet </DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>

		<Button
			variant="outline"
			class="gap-2"
			:class="project.viewer_hearted ? 'border-rose-500/40 text-rose-500' : ''"
			:disabled="heartPending"
			:aria-pressed="project.viewer_hearted"
			aria-label="Heart project"
			@click="$emit('heart')"
		>
			<Heart class="size-4" :class="project.viewer_hearted ? 'fill-current' : ''" />
			<span class="tabular-nums">{{ project.heart_count ?? 0 }}</span>
		</Button>

		<Button
			variant="outline"
			size="icon"
			:class="project.viewer_saved ? 'border-primary/40 text-primary' : ''"
			:disabled="savePending"
			:aria-pressed="project.viewer_saved"
			aria-label="Save project"
			@click="$emit('save')"
		>
			<Bookmark class="size-4" :class="project.viewer_saved ? 'fill-current' : ''" />
		</Button>

		<DropdownMenu>
			<DropdownMenuTrigger as-child>
				<Button variant="outline" size="icon" aria-label="More actions">
					<MoreHorizontal class="size-4" />
				</Button>
			</DropdownMenuTrigger>
			<DropdownMenuContent align="end" class="w-48">
				<DropdownMenuItem variant="destructive" @click="$emit('report')">
					<Flag class="size-4" />
					Report
				</DropdownMenuItem>
				<DropdownMenuSeparator />
				<DropdownMenuItem @click="$emit('copyId')">
					<Copy class="size-4" />
					Copy ID
				</DropdownMenuItem>
				<DropdownMenuItem @click="$emit('copyLink')">
					<Link2 class="size-4" />
					Copy link
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	</div>
</template>
