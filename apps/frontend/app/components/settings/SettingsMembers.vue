<script setup lang="ts">
import { Loader2, Trash2, UserPlus, Users } from "@lucide/vue";

import type { ProjectMember } from "~/scripts/pages/projects/types";

const memberUsername = defineModel<string>({ required: true });

defineProps<{
	members: ProjectMember[];
	membersPending: boolean;
	addingMember: boolean;
	memberAddError: string;
}>();

defineEmits<{
	add: [];
	remove: [userId: string];
}>();

const confirmRemoveMemberId = ref<string | null>(null);
</script>

<template>
	<section class="card-glass space-y-5 rounded-2xl p-6">
		<div>
			<h2 class="section-title mb-1 flex items-center gap-2 text-lg">
				<Users class="text-primary size-5" />
				Members
			</h2>
			<p class="text-muted-foreground text-sm leading-relaxed">Invite collaborators to help manage this project. Members can edit the project; the owner keeps full control.</p>
		</div>

		<form class="flex flex-col gap-3 sm:flex-row sm:items-end" @submit.prevent="$emit('add')">
			<div class="flex-1 space-y-2">
				<Label for="member-username">Add by username</Label>
				<Input id="member-username" v-model="memberUsername" placeholder="username" />
			</div>
			<Button type="submit" class="btn-glow shrink-0" :disabled="addingMember">
				<Loader2 v-if="addingMember" class="size-4 animate-spin" />
				<UserPlus v-else class="size-4" />
				Add member
			</Button>
		</form>
		<p v-if="memberAddError" class="text-destructive -mt-2 text-sm">
			{{ memberAddError }}
		</p>

		<div class="border-t pt-5">
			<p v-if="membersPending && !members.length" class="text-muted-foreground text-sm">Loading members…</p>
			<ul v-else class="space-y-3">
				<li v-for="member in members" :key="member.user_id" class="border-border/60 bg-muted/20 flex items-center justify-between gap-3 rounded-xl border p-3">
					<div class="flex items-center gap-3">
						<div class="bg-primary/15 text-primary flex size-9 items-center justify-center rounded-full text-sm font-semibold uppercase">
							{{ member.username.charAt(0) }}
						</div>
						<div>
							<p class="text-sm font-medium">
								{{ member.username }}
							</p>
							<p class="text-muted-foreground text-xs capitalize">
								{{ member.role }}
							</p>
						</div>
					</div>

					<div v-if="member.role !== 'owner'" class="flex items-center gap-2">
						<template v-if="confirmRemoveMemberId === member.user_id">
							<Button
								variant="destructive"
								size="sm"
								@click="
									$emit('remove', member.user_id);
									confirmRemoveMemberId = null;
								"
							>
								Confirm
							</Button>
							<Button variant="ghost" size="sm" @click="confirmRemoveMemberId = null"> Cancel </Button>
						</template>
						<Button v-else variant="ghost" size="icon" aria-label="Remove member" @click="confirmRemoveMemberId = member.user_id">
							<Trash2 class="text-destructive size-4" />
						</Button>
					</div>
				</li>
			</ul>
		</div>
	</section>
</template>
