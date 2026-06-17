import type { Ref } from "vue";
import type { Category, ProjectSettings } from "../types";

export function useSettingsCategories(project: Ref<ProjectSettings | null>) {
	const allCategories = ref<Category[]>([]);
	const selectedCategoryIds = ref<string[]>([]);
	const originalCategoryIds = ref<string[]>([]);

	const availableCategories = computed(() => {
		const type = project.value?.project_type;
		if (!type) return [] as Category[];
		return allCategories.value.filter((c) => c.project_type === type);
	});

	const tagsDirty = computed(() => {
		const current = [...selectedCategoryIds.value].sort().join(",");
		const original = [...originalCategoryIds.value].sort().join(",");
		return current !== original;
	});

	function toggleCategory(id: string) {
		const index = selectedCategoryIds.value.indexOf(id);
		if (index === -1) {
			selectedCategoryIds.value = [...selectedCategoryIds.value, id];
		} else {
			selectedCategoryIds.value = selectedCategoryIds.value.filter((value) => value !== id);
		}
	}

	function syncSelectedCategories(data: ProjectSettings) {
		const slugs = new Set(data.categories.map((c) => c.slug));
		const ids = allCategories.value.filter((c) => c.project_type === data.project_type && slugs.has(c.slug)).map((c) => c.id);
		selectedCategoryIds.value = ids;
		originalCategoryIds.value = ids;
	}

	return {
		allCategories,
		selectedCategoryIds,
		originalCategoryIds,
		availableCategories,
		tagsDirty,
		toggleCategory,
		syncSelectedCategories,
	};
}
