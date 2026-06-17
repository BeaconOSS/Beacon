<script setup lang="ts">
const open = defineModel<boolean>("open", { required: true });
const reason = defineModel<string>("reason", { required: true });

defineEmits<{
	submit: [];
}>();
</script>

<template>
	<Dialog v-model:open="open">
		<DialogContent>
			<DialogHeader>
				<DialogTitle>Report this project</DialogTitle>
				<DialogDescription> Let us know what's wrong. Reporting isn't fully wired up yet, but your interest helps us prioritize it. </DialogDescription>
			</DialogHeader>
			<Textarea v-model="reason" rows="4" placeholder="Describe the issue (optional)" />
			<DialogFooter>
				<Button variant="outline" @click="open = false">Cancel</Button>
				<Button @click="$emit('submit')">Submit report</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</template>
