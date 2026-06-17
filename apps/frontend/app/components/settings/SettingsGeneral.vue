<script setup lang="ts">
import type { ProjectVisibility } from "~/scripts/constants";

import DangerZoneCard from "~/components/settings/DangerZoneCard.vue";
import GeneralInfoCard from "~/components/settings/GeneralInfoCard.vue";
import MonetizationCard from "~/components/settings/MonetizationCard.vue";

const title = defineModel<string>("title", { required: true });
const urlSlug = defineModel<string>("urlSlug", { required: true });
const summary = defineModel<string>("summary", { required: true });
const visibility = defineModel<ProjectVisibility>("visibility", {
	required: true,
});
const monetizationEnabled = defineModel<boolean>("monetizationEnabled", {
	required: true,
});
const creatorShare = defineModel<number>("creatorShare", { required: true });

defineProps<{
	username: string;
	iconUrl: string | null;
	iconPending: boolean;
	iconError: string;
	locked: boolean;
	saveError: string;
	dirty: boolean;
	saving: boolean;
	monetizationDirty: boolean;
	savingMonetization: boolean;
	monetizationError: string;
	projectTitle: string;
	deleting: boolean;
}>();

defineEmits<{
	upload: [file: File];
	removeIcon: [];
	save: [];
	saveMonetization: [];
	delete: [];
}>();
</script>

<template>
	<section class="space-y-6">
		<GeneralInfoCard
			v-model:title="title"
			v-model:url-slug="urlSlug"
			v-model:summary="summary"
			v-model:visibility="visibility"
			:username="username"
			:icon-url="iconUrl"
			:icon-pending="iconPending"
			:icon-error="iconError"
			:locked="locked"
			:save-error="saveError"
			:dirty="dirty"
			:saving="saving"
			@upload="$emit('upload', $event)"
			@remove-icon="$emit('removeIcon')"
			@save="$emit('save')"
		/>

		<MonetizationCard
			v-model:monetization-enabled="monetizationEnabled"
			v-model:creator-share="creatorShare"
			:monetization-dirty="monetizationDirty"
			:saving-monetization="savingMonetization"
			:monetization-error="monetizationError"
			:locked="locked"
			@save="$emit('saveMonetization')"
		/>

		<DangerZoneCard :project-title="projectTitle" :deleting="deleting" @delete="$emit('delete')" />
	</section>
</template>
