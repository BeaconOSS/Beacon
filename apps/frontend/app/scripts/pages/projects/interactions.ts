import { useApi, apiErrorMessage } from "~/scripts/api";
import type { ProjectDetail } from "./types";

export function useProjectInteractions(slug: string, project: Ref<ProjectDetail | null>) {
	const api = useApi();

	const heartPending = ref(false);
	const savePending = ref(false);

	async function toggleHeart() {
		if (heartPending.value || !project.value) return;
		heartPending.value = true;
		try {
			const result = await api<{ hearted: boolean; heart_count: number }>(`/projects/${slug}/heart`, { method: "POST" });
			project.value.viewer_hearted = result.hearted;
			project.value.heart_count = result.heart_count;
		} catch (err) {
			throw new Error(
				apiErrorMessage(err, {
					fallback: "Could not update your heart. Please try again.",
					status: {
						401: "Please sign in to heart this project.",
						404: "This project could not be found.",
					},
				}),
				{ cause: err }
			);
		} finally {
			heartPending.value = false;
		}
	}

	async function toggleSave() {
		if (savePending.value || !project.value) return;
		savePending.value = true;
		try {
			const result = await api<{ saved: boolean }>(`/projects/${slug}/save`, {
				method: "POST",
			});
			project.value.viewer_saved = result.saved;
		} catch (err) {
			throw new Error(
				apiErrorMessage(err, {
					fallback: "Could not update your saved projects. Please try again.",
					status: {
						401: "Please sign in to save this project.",
						404: "This project could not be found.",
					},
				}),
				{ cause: err }
			);
		} finally {
			savePending.value = false;
		}
	}

	return { heartPending, savePending, toggleHeart, toggleSave };
}
