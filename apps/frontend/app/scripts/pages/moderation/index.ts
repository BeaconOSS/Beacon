import { useApi, apiErrorMessage } from "~/scripts/api";
import type { ModeratorNote, PendingReview, ReviewAction } from "~/scripts/pages/projects/types";

export interface ModerationQueueItem {
	id: string;
	slug: string;
	title: string;
	summary: string;
	project_type: string;
	owner: string;
	icon_url: string | null;
	submitted_at: string | null;
}

export function useModerationQueue() {
	const api = useApi();
	const config = useRuntimeConfig();

	const projects = ref<ModerationQueueItem[]>([]);
	const error = ref("");
	const pending = ref(false);

	function iconUrl(item: ModerationQueueItem): string | null {
		if (!item.icon_url) return null;
		return `${config.public.apiBase}${item.icon_url}`;
	}

	async function load() {
		error.value = "";
		pending.value = true;
		try {
			const data = await api<{ projects: ModerationQueueItem[] }>("/moderation/projects");
			projects.value = data.projects;
		} catch (err) {
			error.value = apiErrorMessage(err, {
				fallback: "Could not load the review queue. Please try again.",
				status: {
					401: "Please sign in to access moderation.",
					403: "You do not have moderator access.",
				},
			});
		} finally {
			pending.value = false;
		}
	}

	return { projects, error, pending, iconUrl, load };
}

export function useProjectReview(slug: string) {
	const api = useApi();
	const submitting = ref(false);
	const error = ref("");

	async function review(action: ReviewAction, notes: string): Promise<boolean> {
		error.value = "";
		submitting.value = true;
		try {
			await api(`/projects/${slug}/review`, {
				method: "POST",
				body: { action, notes },
			});
			return true;
		} catch (err) {
			error.value = apiErrorMessage(err, {
				fallback: "Could not submit your review. Please try again.",
				status: {
					401: "Please sign in to review projects.",
					403: "You do not have moderator access.",
					409: "This project is no longer awaiting review.",
				},
			});
			return false;
		} finally {
			submitting.value = false;
		}
	}

	return { submitting, error, review };
}

export function useProjectPendingReview(slug: string) {
	const api = useApi();
	const config = useRuntimeConfig();

	const data = ref<PendingReview | null>(null);
	const error = ref("");
	const pending = ref(false);

	function withBase(path: string | null): string | null {
		if (!path) return null;
		return `${config.public.apiBase}${path}`;
	}

	async function load() {
		error.value = "";
		pending.value = true;
		try {
			data.value = await api<PendingReview>(`/projects/${slug}/pending`);
		} catch (err) {
			error.value = apiErrorMessage(err, {
				fallback: "Could not load the pending changes. Please try again.",
				status: {
					401: "Please sign in to access moderation.",
					403: "You do not have moderator access.",
				},
			});
		} finally {
			pending.value = false;
		}
	}

	return { data, error, pending, withBase, load };
}

export function useModeratorNotes(slug: string) {
	const api = useApi();

	const notes = ref<ModeratorNote[]>([]);
	const error = ref("");
	const pending = ref(false);
	const submitting = ref(false);

	async function load() {
		error.value = "";
		pending.value = true;
		try {
			const data = await api<{ notes: ModeratorNote[] }>(`/projects/${slug}/moderator-notes`);
			notes.value = data.notes;
		} catch (err) {
			error.value = apiErrorMessage(err, {
				fallback: "Could not load moderator notes. Please try again.",
				status: {
					401: "Please sign in to access moderation.",
					403: "You do not have moderator access.",
				},
			});
		} finally {
			pending.value = false;
		}
	}

	async function add(body: string): Promise<boolean> {
		error.value = "";
		submitting.value = true;
		try {
			const note = await api<ModeratorNote>(`/projects/${slug}/moderator-notes`, {
				method: "POST",
				body: { body },
			});
			notes.value = [note, ...notes.value];
			return true;
		} catch (err) {
			error.value = apiErrorMessage(err, {
				fallback: "Could not save your note. Please try again.",
				status: {
					400: "The note cannot be empty.",
					401: "Please sign in to access moderation.",
					403: "You do not have moderator access.",
				},
			});
			return false;
		} finally {
			submitting.value = false;
		}
	}

	return { notes, error, pending, submitting, load, add };
}

export function useVersionFile(slug: string) {
	const api = useApi();

	function url(version: string, path: string): string {
		return `/projects/${slug}/versions/${encodeURIComponent(version)}/file?path=${encodeURIComponent(path)}`;
	}

	async function fetchText(version: string, path: string): Promise<string> {
		return api<string>(url(version, path), { responseType: "text" });
	}

	async function fetchBlobUrl(version: string, path: string): Promise<string> {
		const blob = await api<Blob>(url(version, path), { responseType: "blob" });
		return URL.createObjectURL(blob);
	}

	async function downloadPack(version: string, filename: string): Promise<void> {
		const blob = await api<Blob>(`/projects/${slug}/versions/${encodeURIComponent(version)}/moderator-download`, { responseType: "blob" });
		const objectUrl = URL.createObjectURL(blob);
		const link = document.createElement("a");
		link.href = objectUrl;
		link.download = filename || `${slug}-${version}.mcpack`;
		document.body.appendChild(link);
		link.click();
		link.remove();
		URL.revokeObjectURL(objectUrl);
	}

	return { fetchText, fetchBlobUrl, downloadPack };
}
