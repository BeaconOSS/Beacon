<script setup lang="ts">
import { Loader2, Package, Trash2, Upload } from "@lucide/vue";
import type { ProjectVisibility } from "~/scripts/constants";
import { VISIBILITY_OPTIONS } from "~/scripts/pages/projects/settings/meta";

const title = defineModel<string>("title", { required: true });
const urlSlug = defineModel<string>("urlSlug", { required: true });
const summary = defineModel<string>("summary", { required: true });
const visibility = defineModel<ProjectVisibility>("visibility", {
	required: true,
});

defineProps<{
	username: string;
	iconUrl: string | null;
	iconPending: boolean;
	iconError: string;
	locked: boolean;
	saveError: string;
	dirty: boolean;
	saving: boolean;
}>();

const emit = defineEmits<{
	upload: [file: File];
	removeIcon: [];
	save: [];
}>();

const iconInput = ref<HTMLInputElement | null>(null);
const summaryLength = computed(() => summary.value.trim().length);

function pickIcon() {
	iconInput.value?.click();
}

function onIconChange(event: Event) {
	const input = event.target as HTMLInputElement;
	const file = input.files?.[0];
	if (file) emit("upload", file);
	input.value = "";
}
</script>

<template>
	<div class="card-glass rounded-2xl p-6">
		<h2 class="section-title mb-1 text-lg">Project information</h2>
		<p class="text-muted-foreground mb-6 text-sm">The core details people see across Beacon.</p>

		<div class="space-y-6">
			<div class="space-y-2">
				<Label for="project-name">Name</Label>
				<Input id="project-name" v-model="title" placeholder="My awesome project" />
			</div>

			<div class="space-y-2">
				<Label for="project-url">URL</Label>
				<div
					class="border-input bg-background focus-within:border-ring focus-within:ring-ring/50 flex items-center rounded-md border text-sm transition-[color,box-shadow] focus-within:ring-3"
				>
					<span class="text-muted-foreground shrink-0 pl-3 select-none"> /{{ username }}/ </span>
					<input id="project-url" v-model="urlSlug" class="placeholder:text-muted-foreground min-w-0 flex-1 bg-transparent py-2 pr-3 pl-0.5 outline-none" placeholder="my-awesome-project" />
				</div>
				<p class="text-muted-foreground text-xs">Changing the URL will redirect you to the new address.</p>
			</div>

			<div class="space-y-2">
				<Label for="project-summary">Summary</Label>
				<Textarea id="project-summary" v-model="summary" :rows="3" placeholder="A short, enticing description of your project." />
				<p class="text-xs" :class="summaryLength >= 30 ? 'text-muted-foreground' : 'text-amber-500'">{{ summaryLength }} characters - at least 30 is recommended.</p>
			</div>

			<div class="space-y-2">
				<Label>Icon</Label>
				<div class="flex items-center gap-4">
					<div class="border-border/60 bg-muted/40 flex size-20 shrink-0 items-center justify-center overflow-hidden rounded-xl border">
						<img v-if="iconUrl" :src="iconUrl" alt="Project icon" class="size-full object-cover" />
						<Package v-else class="text-muted-foreground size-7" />
					</div>
					<div class="space-y-2">
						<div class="flex flex-wrap gap-2">
							<Button type="button" variant="outline" size="sm" :disabled="iconPending || locked" @click="pickIcon">
								<Loader2 v-if="iconPending" class="size-4 animate-spin" />
								<Upload v-else class="size-4" />
								{{ iconUrl ? "Replace" : "Upload" }}
							</Button>
							<Button v-if="iconUrl" type="button" variant="ghost" size="sm" :disabled="iconPending || locked" @click="$emit('removeIcon')">
								<Trash2 class="size-4" />
								Remove
							</Button>
						</div>
						<p class="text-muted-foreground text-xs">Optional. PNG, JPG, WEBP, or GIF.</p>
						<p v-if="iconError" class="text-destructive text-xs">
							{{ iconError }}
						</p>
					</div>
					<input ref="iconInput" type="file" accept="image/png,image/jpeg,image/webp,image/gif" class="hidden" @change="onIconChange" />
				</div>
			</div>

			<div class="space-y-2">
				<Label>Visibility</Label>
				<div class="grid gap-3 sm:grid-cols-3">
					<button
						v-for="option in VISIBILITY_OPTIONS"
						:key="option.value"
						type="button"
						class="flex flex-col gap-1.5 rounded-xl border p-3 text-left transition-colors"
						:class="visibility === option.value ? 'border-primary bg-primary/5' : 'border-border hover:border-border/80 hover:bg-accent/30'"
						@click="visibility = option.value"
					>
						<component :is="option.icon" class="size-4" :class="visibility === option.value ? 'text-primary' : 'text-muted-foreground'" />
						<span class="text-foreground text-sm font-semibold">
							{{ option.label }}
						</span>
						<span class="text-muted-foreground text-xs">
							{{ option.description }}
						</span>
					</button>
				</div>
			</div>

			<div class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center" :class="saveError ? 'sm:justify-between' : 'sm:justify-end'">
				<p v-if="saveError" class="text-destructive text-sm">
					{{ saveError }}
				</p>
				<Button class="btn-glow shrink-0" :disabled="!dirty || saving || locked" @click="$emit('save')">
					<Loader2 v-if="saving" class="size-4 animate-spin" />
					Save changes
				</Button>
			</div>
		</div>
	</div>
</template>
