<script setup lang="ts">
import { Loader2 } from "@lucide/vue";

import type { FilePreviewState } from "~/scripts/pages/moderation/types";

defineProps<{
	preview: FilePreviewState;
}>();
</script>

<template>
	<div class="border-border/40 bg-background/40 mt-1 rounded-lg border p-2">
		<p v-if="preview.loading" class="text-muted-foreground inline-flex items-center gap-2 text-xs"><Loader2 class="size-3.5 animate-spin" /> Loading…</p>
		<p v-else-if="preview.error" class="text-xs text-red-400">
			{{ preview.error }}
		</p>

		<!-- Text diff -->
		<template v-else-if="preview.kind === 'text' && preview.diff">
			<p v-if="preview.diff.tooLarge" class="text-muted-foreground text-xs">File too large to diff inline - sha changed.</p>
			<template v-else>
				<p class="text-muted-foreground mb-1 text-xs">
					<span class="text-emerald-400">+{{ preview.diff.added }}</span>
					/
					<span class="text-red-400">−{{ preview.diff.removed }}</span>
					lines
				</p>
				<div class="max-h-80 overflow-auto rounded font-mono text-[11px] leading-relaxed">
					<div
						v-for="(line, li) in preview.diff.lines"
						:key="`line-${li}`"
						class="flex"
						:class="{
							'bg-emerald-500/10 text-emerald-300': line.type === 'add',
							'bg-red-500/10 text-red-300': line.type === 'remove',
							'text-muted-foreground': line.type === 'context',
						}"
					>
						<span class="text-muted-foreground/50 w-8 shrink-0 select-none pr-1 text-right">{{ line.oldNumber ?? "" }}</span>
						<span class="text-muted-foreground/50 w-8 shrink-0 select-none pr-2 text-right">{{ line.newNumber ?? "" }}</span>
						<span class="w-3 shrink-0 select-none">{{ line.type === "add" ? "+" : line.type === "remove" ? "−" : " " }}</span>
						<span class="whitespace-pre-wrap break-all">{{ line.text }}</span>
					</div>
				</div>
				<p v-if="preview.diff.truncated" class="text-muted-foreground/70 mt-1 text-xs italic">Diff truncated - download to view the full file.</p>
			</template>
		</template>

		<!-- Image before/after -->
		<div v-else-if="preview.kind === 'image'" class="grid grid-cols-2 gap-2">
			<div>
				<p class="text-muted-foreground mb-1 text-xs">Before</p>
				<img
					v-if="preview.oldImage"
					:src="preview.oldImage"
					alt="previous version"
					class="border-border/40 max-h-40 rounded border bg-[repeating-conic-gradient(#0002_0_25%,transparent_0_50%)] bg-[length:16px_16px] object-contain"
				/>
				<p v-else class="text-muted-foreground/60 text-xs">(new file)</p>
			</div>
			<div>
				<p class="text-muted-foreground mb-1 text-xs">After</p>
				<img
					v-if="preview.newImage"
					:src="preview.newImage"
					alt="submitted version"
					class="border-border/40 max-h-40 rounded border bg-[repeating-conic-gradient(#0002_0_25%,transparent_0_50%)] bg-[length:16px_16px] object-contain"
				/>
				<p v-else class="text-muted-foreground/60 text-xs">(removed)</p>
			</div>
		</div>

		<!-- Binary -->
		<p v-else class="text-muted-foreground text-xs">Binary file - contents changed (no inline preview).</p>
	</div>
</template>
