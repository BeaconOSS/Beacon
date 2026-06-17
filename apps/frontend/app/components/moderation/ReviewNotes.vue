<script setup lang="ts">
import { Loader2, StickyNote } from "@lucide/vue";

import type { ModeratorNote } from "~/scripts/pages/projects/types";

import { relativeTime } from "~/scripts/formatters";

defineProps<{
	notes: ModeratorNote[];
	newNote: string;
	submitting: boolean;
}>();

defineEmits<{
	"update:newNote": [value: string];
	submit: [];
}>();
</script>

<template>
	<div class="border-border/60 bg-card/40 rounded-2xl border p-5">
		<p class="text-muted-foreground mb-1 inline-flex items-center gap-1.5 text-xs font-semibold tracking-wide uppercase"><StickyNote class="size-3.5" /> Moderator notes</p>
		<p class="text-muted-foreground/80 mb-3 text-xs">Internal only - never shown to the creator.</p>

		<div class="mb-3">
			<Textarea :model-value="newNote" rows="3" placeholder="Add an internal note for future reviews…" class="mb-2" @update:model-value="$emit('update:newNote', String($event))" />
			<Button size="sm" class="gap-2" :disabled="submitting || !newNote.trim()" @click="$emit('submit')">
				<Loader2 v-if="submitting" class="size-4 animate-spin" />
				Add note
			</Button>
		</div>

		<p v-if="!notes.length" class="text-muted-foreground text-sm">No notes yet.</p>
		<ul v-else class="space-y-3">
			<li v-for="note in notes" :key="note.id" class="border-border/40 border-l-2 pl-3">
				<div class="text-muted-foreground text-xs">{{ note.author }} · {{ relativeTime(note.created_at) }}</div>
				<p class="text-foreground mt-1 text-sm whitespace-pre-wrap">
					{{ note.body }}
				</p>
			</li>
		</ul>
	</div>
</template>
