<script setup lang="ts">
import { Loader2 } from "@lucide/vue";

defineProps<{
	projectTitle: string;
	deleting: boolean;
}>();

defineEmits<{ delete: [] }>();

const confirming = ref(false);
</script>

<template>
	<div class="border-destructive/40 rounded-2xl border p-6">
		<h2 class="text-destructive mb-1 text-lg font-semibold">Delete project</h2>
		<p class="text-muted-foreground mb-4 text-sm">Removes your project from Beacon's servers and search. This action is permanent, so be extra careful.</p>
		<template v-if="confirming">
			<p class="mb-3 text-sm font-medium">
				This permanently deletes
				<span class="font-semibold">{{ projectTitle }}</span>
				and all its versions, gallery images, and stats. This cannot be undone.
			</p>
			<div class="flex flex-wrap gap-3">
				<Button variant="destructive" :disabled="deleting" @click="$emit('delete')">
					<Loader2 v-if="deleting" class="size-4 animate-spin" />
					Yes, delete this project
				</Button>
				<Button variant="outline" :disabled="deleting" @click="confirming = false"> Cancel </Button>
			</div>
		</template>
		<Button v-else variant="destructive" @click="confirming = true"> Delete project </Button>
	</div>
</template>
