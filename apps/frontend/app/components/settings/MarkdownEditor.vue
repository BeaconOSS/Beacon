<script setup lang="ts">
import { Eye, EyeOff } from "@lucide/vue";

import type { MarkdownAction } from "~/scripts/pages/projects/settings/types";

import { renderMarkdown } from "~/scripts/markdown";

const model = defineModel<string>({ required: true });

const props = withDefaults(
	defineProps<{
		id: string;
		actions: MarkdownAction[];
		rows: number;
		placeholder: string;
		minHeightClass?: string;
		spellcheck?: boolean;
	}>(),
	{
		minHeightClass: "min-h-24",
		spellcheck: false,
	}
);

const textarea = ref<HTMLTextAreaElement | null>(null);
const showPreview = ref(false);
const preview = computed(() => renderMarkdown(model.value));

function applyMarkdown(action: MarkdownAction) {
	const el = textarea.value;
	if (!el) return;
	const start = el.selectionStart;
	const end = el.selectionEnd;
	const value = model.value;
	const selected = value.slice(start, end);
	const text = selected || action.placeholder || "";

	let inserted: string;
	let cursorStart: number;
	let cursorEnd: number;

	if (action.block) {
		const lines = text.split("\n");
		inserted = lines.map((line) => action.before + line).join("\n");
		cursorStart = start + action.before.length;
		cursorEnd = start + inserted.length;
	} else {
		const after = action.after ?? "";
		inserted = action.before + text + after;
		cursorStart = start + action.before.length;
		cursorEnd = cursorStart + text.length;
	}

	model.value = value.slice(0, start) + inserted + value.slice(end);

	nextTick(() => {
		el.focus();
		el.setSelectionRange(cursorStart, cursorEnd);
	});
}
</script>

<template>
	<div class="border-input focus-within:ring-ring/50 overflow-hidden rounded-xl border focus-within:ring-2">
		<div class="bg-muted/40 flex flex-wrap items-center gap-1 border-b p-1.5">
			<button
				v-for="action in props.actions"
				:key="action.label"
				type="button"
				:title="action.label"
				:aria-label="action.label"
				:disabled="showPreview"
				class="text-muted-foreground hover:bg-background hover:text-foreground inline-flex size-8 items-center justify-center rounded-md transition-colors disabled:pointer-events-none disabled:opacity-40"
				@click="applyMarkdown(action)"
			>
				<component :is="action.icon" class="size-4" />
			</button>
			<div class="ml-auto">
				<Button type="button" variant="ghost" size="sm" @click="showPreview = !showPreview">
					<component :is="showPreview ? EyeOff : Eye" class="size-4" />
					{{ showPreview ? "Edit" : "Preview" }}
				</Button>
			</div>
		</div>

		<div v-if="showPreview" class="markdown-preview px-4 py-3 text-sm" :class="props.minHeightClass">
			<div v-if="model.trim()" v-html="preview" />
			<p v-else class="text-muted-foreground italic">Nothing to preview yet.</p>
		</div>
		<textarea
			v-else
			:id="props.id"
			ref="textarea"
			v-model="model"
			:rows="props.rows"
			:spellcheck="props.spellcheck"
			:placeholder="props.placeholder"
			class="placeholder:text-muted-foreground w-full resize-y bg-transparent px-4 py-3 font-mono text-sm outline-none"
			:class="props.minHeightClass"
		/>
	</div>
</template>

<style scoped>
.markdown-preview :deep(h1),
.markdown-preview :deep(h2),
.markdown-preview :deep(h3),
.markdown-preview :deep(h4) {
	font-weight: 600;
	line-height: 1.25;
	margin: 1.25em 0 0.5em;
}

.markdown-preview :deep(h1) {
	font-size: 1.5rem;
}

.markdown-preview :deep(h2) {
	font-size: 1.3rem;
}

.markdown-preview :deep(h3) {
	font-size: 1.125rem;
}

.markdown-preview :deep(:first-child) {
	margin-top: 0;
}

.markdown-preview :deep(p) {
	margin: 0.75em 0;
	line-height: 1.65;
}

.markdown-preview :deep(ul),
.markdown-preview :deep(ol) {
	margin: 0.75em 0;
	padding-left: 1.5em;
}

.markdown-preview :deep(ul) {
	list-style: disc;
}

.markdown-preview :deep(ol) {
	list-style: decimal;
}

.markdown-preview :deep(li) {
	margin: 0.25em 0;
}

.markdown-preview :deep(a) {
	color: var(--color-primary);
	text-decoration: underline;
	text-underline-offset: 2px;
}

.markdown-preview :deep(blockquote) {
	border-left: 3px solid var(--color-border);
	margin: 0.75em 0;
	padding-left: 1em;
	color: var(--color-muted-foreground);
}

.markdown-preview :deep(code) {
	background: var(--color-muted);
	border-radius: 0.25rem;
	padding: 0.1em 0.35em;
	font-size: 0.875em;
}

.markdown-preview :deep(pre) {
	background: var(--color-muted);
	border-radius: 0.5rem;
	padding: 0.75em 1em;
	overflow-x: auto;
	margin: 0.75em 0;
}

.markdown-preview :deep(pre code) {
	background: transparent;
	padding: 0;
}

.markdown-preview :deep(img) {
	max-width: 100%;
	border-radius: 0.5rem;
}

.markdown-preview :deep(hr) {
	border: none;
	border-top: 1px solid var(--color-border);
	margin: 1.5em 0;
}

.markdown-preview :deep(table) {
	border-collapse: collapse;
	margin: 0.75em 0;
}

.markdown-preview :deep(th),
.markdown-preview :deep(td) {
	border: 1px solid var(--color-border);
	padding: 0.4em 0.75em;
}
</style>
