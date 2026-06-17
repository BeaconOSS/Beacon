import { useApi, apiErrorMessage } from "~/scripts/api";
import { useAuth } from "~/scripts/auth";
import { PROJECT_VISIBILITY } from "~/scripts/constants";
import type { Category, CategoryTag, ProjectDetail, ProjectSummary, ProjectVisibility } from "./types";

export * from "./types";
export * from "./versions";
export * from "./gallery";
export * from "./settings";

const PROJECT_TYPE_LABELS: Record<string, string> = {
	addon: "Add-On",
	world: "World",
	resource_pack: "Resource Pack",
	skin_pack: "Skin Pack",
};

export const PROJECT_TYPES = Object.entries(PROJECT_TYPE_LABELS).map(([value, label]) => ({ value, label }));

export function projectTypeLabel(type: string): string {
	return PROJECT_TYPE_LABELS[type] ?? type;
}

export function useProjects() {
	const api = useApi();

	const projects = ref<ProjectSummary[]>([]);
	const error = ref("");
	const pending = ref(false);

	async function load(category?: string, search?: string) {
		error.value = "";
		pending.value = true;
		try {
			const query: Record<string, string> = {};
			if (category) query.category = category;
			if (search) query.q = search;
			const data = await api<{ projects: ProjectSummary[] }>("/projects", {
				query,
			});
			projects.value = data.projects;
		} catch {
			error.value = "Could not load projects. Please try again.";
		} finally {
			pending.value = false;
		}
	}

	return { projects, error, pending, load };
}

export function useCategoryFilters() {
	const api = useApi();

	const categories = ref<CategoryTag[]>([]);

	async function load() {
		try {
			const data = await api<{ categories: Category[] }>("/categories");
			const seen = new Set<string>();
			const unique: CategoryTag[] = [];
			for (const category of data.categories) {
				if (!seen.has(category.slug)) {
					seen.add(category.slug);
					unique.push({ slug: category.slug, name: category.name });
				}
			}
			categories.value = unique;
		} catch {
			categories.value = [];
		}
	}

	return { categories, load };
}

export function useProject(slug: string, preview = false) {
	const api = useApi();

	const project = ref<ProjectDetail | null>(null);
	const error = ref("");
	const pending = ref(false);

	async function load() {
		error.value = "";
		pending.value = true;
		try {
			const path = preview ? `/projects/${slug}?preview=pending` : `/projects/${slug}`;
			project.value = await api<ProjectDetail>(path);
		} catch (err) {
			const status = (err as { response?: { status?: number } })?.response?.status;
			error.value = status === 404 ? "That project could not be found." : "Could not load this project. Please try again.";
		} finally {
			pending.value = false;
		}
	}

	return { project, error, pending, load };
}

export function useCreateProjectForm() {
	const api = useApi();
	const { user, fetchUser } = useAuth();

	const title = ref("");
	const projectType = ref(PROJECT_TYPES[0]!.value);
	const summary = ref("");
	const visibility = ref<ProjectVisibility>(PROJECT_VISIBILITY.PUBLIC);
	const error = ref("");
	const pending = ref(false);

	async function submit() {
		error.value = "";
		pending.value = true;
		try {
			const created = await api<{ id: string; slug: string }>("/projects", {
				method: "POST",
				body: {
					title: title.value,
					project_type: projectType.value,
					summary: summary.value,
					visibility: visibility.value,
				},
			});
			if (!user.value) {
				await fetchUser();
			}
			const username = user.value?.username;
			await navigateTo(username ? `/${username}/${created.slug}/settings` : `/projects/${created.slug}`);
		} catch (err) {
			error.value = apiErrorMessage(err, {
				fallback: "Could not create the project. Please try again.",
				status: { 401: "Please sign in to create a project." },
			});
		} finally {
			pending.value = false;
		}
	}

	return {
		title,
		projectType,
		summary,
		visibility,
		user,
		error,
		pending,
		submit,
	};
}
