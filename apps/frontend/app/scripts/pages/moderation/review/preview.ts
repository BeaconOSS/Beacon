import { toast } from "vue-sonner";

import type { FilePreviewState, LightboxState } from "../types";
import type { Ref } from "vue";
import type { PendingReview } from "~/scripts/pages/projects/types";

import { diffLines, previewKind } from "~/scripts/pages/diff";
import { useVersionFile } from "~/scripts/pages/moderation";

function fileAnchorId(path: string): string {
	return `diff-file-${path.replace(/[^a-zA-Z0-9]+/g, "-")}`;
}

export function useReviewPreview(slug: string, pendingReview: Ref<PendingReview | null>) {
	const { fetchText, fetchBlobUrl, downloadPack } = useVersionFile(slug);

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

	return {
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
	};
}
