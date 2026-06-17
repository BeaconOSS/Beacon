import { useSettingsActions } from "./actions";
import { useSettingsCategories } from "./categories";
import { useSettingsDelete } from "./delete";
import { useSettingsForm } from "./form";
import { useSettingsIcon } from "./icon";
import { useSettingsReview } from "./review";

import type { Category, ProjectSettings } from "../types";

import { useApi } from "~/scripts/api";
import { PROJECT_STATUS } from "~/scripts/constants";

export * from "./constants";

export function useProjectSettings(slug: string) {
	const api = useApi();

	const project = ref<ProjectSettings | null>(null);
	const error = ref("");
	const pending = ref(false);

	const formApi = useSettingsForm(project);
	const categories = useSettingsCategories(project);
	const icon = useSettingsIcon(slug, project);
	const review = useSettingsReview({ slug, project, load });
	const actions = useSettingsActions({
		slug,
		project,
		form: formApi.form,
		selectedCategoryIds: categories.selectedCategoryIds,
		load,
	});
	const danger = useSettingsDelete(slug);

	const locked = computed(() => project.value?.status === PROJECT_STATUS.IN_REVIEW);

	async function load() {
		error.value = "";
		pending.value = true;
		try {
			const [data, categoryData] = await Promise.all([api<ProjectSettings>(`/projects/${slug}/settings`), api<{ categories: Category[] }>("/categories")]);
			categories.allCategories.value = categoryData.categories;
			project.value = data;
			formApi.syncForm(data);
			categories.syncSelectedCategories(data);
			review.syncChangelog(data);
		} catch (err) {
			const status = (err as { response?: { status?: number } })?.response?.status;
			error.value =
				status === 401 || status === 403
					? "You do not have access to this project's settings."
					: status === 404
						? "That project could not be found."
						: "Could not load this project. Please try again.";
		} finally {
			pending.value = false;
		}
	}

	return {
		project,
		error,
		pending,
		load,
		locked,
		...formApi,
		...categories,
		...icon,
		...actions,
		...review,
		...danger,
	};
}
