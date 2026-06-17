<script setup lang="ts">
import { Loader2, Scale } from "@lucide/vue";

import { LICENSE_OPTIONS } from "~/scripts/pages/projects/settings/meta";

const license = defineModel<string>({ required: true });

defineProps<{
	licenseDirty: boolean;
	savingLicense: boolean;
	licenseError: string;
	locked: boolean;
}>();

defineEmits<{ save: [] }>();
</script>

<template>
	<section class="card-glass space-y-5 rounded-2xl p-6">
		<div>
			<h2 class="section-title mb-1 flex items-center gap-2 text-lg">
				<Scale class="text-primary size-5" />
				License
			</h2>
			<p class="text-muted-foreground text-sm leading-relaxed">
				Choose the license your project is distributed under. This tells people what they're allowed to do with your content. If you're not sure, "All Rights Reserved" keeps every right with
				you.
			</p>
		</div>

		<div class="space-y-2">
			<Label for="project-license">License</Label>
			<Select id="project-license" v-model="license">
				<SelectTrigger class="w-full sm:max-w-md">
					<SelectValue placeholder="Select a license" />
				</SelectTrigger>
				<SelectContent>
					<SelectItem v-for="option in LICENSE_OPTIONS" :key="option.value" :value="option.value">
						{{ option.label }}
					</SelectItem>
				</SelectContent>
			</Select>
			<a href="https://choosealicense.com/" target="_blank" rel="noopener noreferrer" class="text-muted-foreground hover:text-foreground inline-block text-xs underline-offset-2 hover:underline">
				Help me choose a license
			</a>
		</div>

		<div class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center" :class="licenseError ? 'sm:justify-between' : 'sm:justify-end'">
			<p v-if="licenseError" class="text-destructive text-sm">
				{{ licenseError }}
			</p>
			<Button class="btn-glow shrink-0" :disabled="!licenseDirty || savingLicense || locked" @click="$emit('save')">
				<Loader2 v-if="savingLicense" class="size-4 animate-spin" />
				Save license
			</Button>
		</div>
	</section>
</template>
