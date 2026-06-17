import { toast } from "vue-sonner";

import { buildDecisionSignal } from "./signal";

import type { FieldDiff, FilePreviewState, LightboxState, OwnerTrust, QueueAge } from "./types";
import type { ReviewAction } from "~/scripts/constants";

import { useAuth } from "~/scripts/auth";
import { REVIEW_ACTION } from "~/scripts/constants";
import { diffLines, diffWords, previewKind } from "~/scripts/pages/diff";
import { useModeratorNotes, useModerationQueue, useProjectPendingReview, useProjectReview, useVersionFile } from "~/scripts/pages/moderation";
import { useProject, projectTypeLabel } from "~/scripts/pages/projects";

const HISTORY_NOTE_CLAMP = 160;

function fileAnchorId(path: string): string {
	return `diff-file-${path.replace(/[^a-zA-Z0-9]+/g, "-")}`;
}

export function useModerationReview() {
	const route = useRoute();
	const slug = String(route.params.slug ?? "");

	const { isModerator } = useAuth();

	const { project, load: loadProject } = useProject(slug);
	const { data: pendingReview, error: pendingError, pending: pendingLoading, withBase: pendingWithBase, load: loadPendingReview } = useProjectPendingReview(slug);
	const { submitting: reviewSubmitting, error: reviewError, review } = useProjectReview(slug);
	const { notes: moderatorNotes, submitting: noteSubmitting, load: loadModeratorNotes, add: addModeratorNote } = useModeratorNotes(slug);
	const { fetchText, fetchBlobUrl, downloadPack } = useVersionFile(slug);
	const { projects: queueProjects, load: loadQueue } = useModerationQueue();

	async function load() {
		await Promise.all([loadProject(), loadPendingReview(), loadModeratorNotes(), loadQueue()]);
	}

	const queueIndex = computed(() => queueProjects.value.findIndex((p) => p.slug === slug));
	const queuePosition = computed(() => (queueIndex.value >= 0 ? queueIndex.value + 1 : null));
	const queueTotal = computed(() => queueProjects.value.length);
	const queuePrev = computed(() => (queueIndex.value > 0 ? (queueProjects.value[queueIndex.value - 1] ?? null) : null));
	const queueNext = computed(() => (queueIndex.value >= 0 && queueIndex.value < queueProjects.value.length - 1 ? (queueProjects.value[queueIndex.value + 1] ?? null) : null));

	const queueAge = computed<QueueAge | null>(() => {
		const submitted = pendingReview.value?.submitted_at;
		if (!submitted) return null;
		const then = new Date(submitted).getTime();
		if (Number.isNaN(then)) return null;
		const hours = (Date.now() - then) / 3_600_000;
		if (hours < 24) {
			return {
				hours,
				label: "Fresh",
				class: "bg-emerald-500/15 text-emerald-400",
			};
		}
		if (hours < 72) {
			return {
				hours,
				label: "Waiting a while",
				class: "bg-amber-500/15 text-amber-500",
			};
		}
		return { hours, label: "Overdue", class: "bg-red-500/15 text-red-400" };
	});

	const expandedHistory = ref<Set<number>>(new Set());
	function toggleHistory(idx: number) {
		const next = new Set(expandedHistory.value);
		if (next.has(idx)) next.delete(idx);
		else next.add(idx);
		expandedHistory.value = next;
	}

	const ownerTrust = computed<OwnerTrust | null>(() => {
		const owner = pendingReview.value?.owner;
		if (!owner) return null;
		if (owner.rejected_count > 0) {
			return {
				label: `${owner.rejected_count} prior rejection${owner.rejected_count === 1 ? "" : "s"}`,
				class: "bg-red-500/15 text-red-400",
				icon: "warn",
			};
		}
		if (owner.approved_count > 0) {
			return {
				label: `${owner.approved_count} approved before`,
				class: "bg-emerald-500/15 text-emerald-400",
				icon: "check",
			};
		}
		return {
			label: "New creator · first review",
			class: "bg-amber-500/15 text-amber-500",
			icon: "new",
		};
	});

	const reviewNotes = ref("");
	const newNote = ref("");
	const showDiffFiles = ref(false);
	const downloadingVersion = ref<string | null>(null);

	const lightbox = ref<LightboxState | null>(null);
	function openLightbox(url: string, caption = "", pixelated = false) {
		lightbox.value = { url, caption, pixelated };
	}
	function closeLightbox() {
		lightbox.value = null;
	}

	async function downloadVersion(version: string, filename: string) {
		if (downloadingVersion.value) return;
		downloadingVersion.value = version;
		try {
			await downloadPack(version, filename);
		} catch {
			toast.error("Could not download this version.");
		} finally {
			downloadingVersion.value = null;
		}
	}

	const submittedVersion = computed(() => pendingReview.value?.versions?.[0]?.version_number ?? null);
	const previousVersion = computed(() => pendingReview.value?.versions?.[1]?.version_number ?? null);

	const activeFile = ref<FilePreviewState | null>(null);

	function releasePreview() {
		if (activeFile.value?.oldImage) URL.revokeObjectURL(activeFile.value.oldImage);
		if (activeFile.value?.newImage) URL.revokeObjectURL(activeFile.value.newImage);
	}

	async function toggleFile(path: string, status: string) {
		if (activeFile.value?.path === path) {
			releasePreview();
			activeFile.value = null;
			return;
		}

		releasePreview();
		const kind = previewKind(path);
		const state: FilePreviewState = {
			path,
			kind,
			loading: true,
			error: "",
			diff: null,
			oldImage: null,
			newImage: null,
		};
		activeFile.value = state;

		if (kind === "binary") {
			state.loading = false;
			return;
		}

		const newVer = submittedVersion.value;
		const oldVer = previousVersion.value;
		const wantOld = status !== "added" && !!oldVer;
		const wantNew = status !== "removed" && !!newVer;

		try {
			if (kind === "text") {
				const [oldText, newText] = await Promise.all([wantOld ? fetchText(oldVer as string, path) : Promise.resolve(""), wantNew ? fetchText(newVer as string, path) : Promise.resolve("")]);
				state.diff = diffLines(oldText, newText);
			} else {
				const [oldUrl, newUrl] = await Promise.all([
					wantOld ? fetchBlobUrl(oldVer as string, path) : Promise.resolve(null),
					wantNew ? fetchBlobUrl(newVer as string, path) : Promise.resolve(null),
				]);
				state.oldImage = oldUrl;
				state.newImage = newUrl;
			}
		} catch {
			state.error = "Could not load this file.";
		} finally {
			if (activeFile.value === state) state.loading = false;
		}
	}

	onBeforeUnmount(releasePreview);

	function findingFile(message: string): string | null {
		const files = pendingReview.value?.pack_diff?.files;
		if (!files?.length || !message) return null;
		let best: string | null = null;
		for (const file of files) {
			if (message.includes(file.path)) {
				if (!best || file.path.length > best.length) best = file.path;
				continue;
			}
			const base = file.path.split("/").pop() ?? "";
			if (base.length > 3 && message.includes(base)) {
				if (!best) best = file.path;
			}
		}
		return best;
	}

	async function jumpToFinding(message: string) {
		const path = findingFile(message);
		if (!path) return;
		showDiffFiles.value = true;
		if (activeFile.value?.path !== path) {
			const file = pendingReview.value?.pack_diff?.files.find((f) => f.path === path);
			if (file) await toggleFile(file.path, file.status);
		}
		await nextTick();
		const el = document.getElementById(fileAnchorId(path));
		el?.scrollIntoView({ behavior: "smooth", block: "center" });
	}

	async function submitNote() {
		const body = newNote.value.trim();
		if (!body) return;
		const ok = await addModeratorNote(body);
		if (ok) {
			newNote.value = "";
			toast.success("Note added.");
		} else {
			toast.error("Could not save your note.");
		}
	}

	const typeLabel = computed(() => (project.value ? projectTypeLabel(project.value.project_type) : ""));

	const decisionSignal = computed(() => buildDecisionSignal(pendingReview.value));

	const links = computed(() => {
		const l = pendingReview.value?.links;
		if (!l) return [] as { label: string; url: string }[];
		return [
			{ label: "Website", url: l.website_url },
			{ label: "Source code", url: l.source_url },
			{ label: "Issue tracker", url: l.issues_url },
			{ label: "Wiki", url: l.wiki_url },
			{ label: "Discord", url: l.discord_url },
		].filter((link) => link.url.trim().length > 0);
	});

	const reviewDiffs = computed<FieldDiff[]>(() => {
		const data = pendingReview.value;
		if (!data) return [];
		const before = data.published;
		const after = data.pending;
		const isFirst = data.is_first_review;

		function makeField(label: string, b: string, a: string): FieldDiff {
			const changed = !isFirst && b !== a;
			const wd = changed ? diffWords(b, a) : null;
			return {
				label,
				before: b,
				after: a,
				changed,
				segments: wd && !wd.tooLarge ? wd.segments : null,
			};
		}

		const fields: { label: string; key: keyof typeof after }[] = [
			{ label: "Title", key: "title" },
			{ label: "Summary", key: "summary" },
			{ label: "Description", key: "description" },
			{ label: "License", key: "license" },
		];
		const rows: FieldDiff[] = fields.map((field) => makeField(field.label, before ? String(before[field.key] ?? "") : "", String(after[field.key] ?? "")));
		rows.push(makeField("Categories", before ? before.categories.join(", ") : "", after.categories.join(", ")));
		return rows;
	});

	async function handleReview(action: ReviewAction) {
		if ((action === REVIEW_ACTION.REJECT || action === REVIEW_ACTION.REQUEST_CHANGES) && !reviewNotes.value.trim()) {
			toast.error("Please add notes explaining your decision.");
			return;
		}
		const ok = await review(action, reviewNotes.value.trim());
		if (ok) {
			const labels: Record<typeof action, string> = {
				approve: "Project approved and published.",
				reject: "Project rejected.",
				request_changes: "Changes requested.",
			};
			toast.success(labels[action]);
			reviewNotes.value = "";
			await navigateTo("/moderation");
		} else if (reviewError.value) {
			toast.error(reviewError.value);
		}
	}

	return {
		slug,
		isModerator,
		project,
		pendingReview,
		pendingError,
		pendingLoading,
		pendingWithBase,
		reviewSubmitting,
		moderatorNotes,
		noteSubmitting,
		load,
		queuePosition,
		queueTotal,
		queuePrev,
		queueNext,
		queueAge,
		expandedHistory,
		historyNoteClamp: HISTORY_NOTE_CLAMP,
		toggleHistory,
		ownerTrust,
		reviewNotes,
		newNote,
		showDiffFiles,
		downloadingVersion,
		lightbox,
		openLightbox,
		closeLightbox,
		downloadVersion,
		activeFile,
		toggleFile,
		fileAnchorId,
		findingFile,
		jumpToFinding,
		submitNote,
		typeLabel,
		decisionSignal,
		links,
		reviewDiffs,
		handleReview,
	};
}
