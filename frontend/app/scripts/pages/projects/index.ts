import { useApi, apiErrorMessage } from "~/scripts/api";
import type {
  Category,
  CategoryTag,
  ProjectDetail,
  ProjectSummary,
} from "./types";

export * from "./types";
export * from "./versions";
export * from "./gallery";

const PROJECT_TYPE_LABELS: Record<string, string> = {
  addon: "Add-On",
  world: "World",
  resource_pack: "Resource Pack",
  skin_pack: "Skin Pack",
};

export const PROJECT_TYPES = Object.entries(PROJECT_TYPE_LABELS).map(
  ([value, label]) => ({ value, label }),
);

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

export function useProject(slug: string) {
  const api = useApi();

  const project = ref<ProjectDetail | null>(null);
  const error = ref("");
  const pending = ref(false);

  async function load() {
    error.value = "";
    pending.value = true;
    try {
      project.value = await api<ProjectDetail>(`/projects/${slug}`);
    } catch (err: any) {
      error.value =
        err?.response?.status === 404
          ? "That project could not be found."
          : "Could not load this project. Please try again.";
    } finally {
      pending.value = false;
    }
  }

  return { project, error, pending, load };
}

export function useCreateProjectForm() {
  const api = useApi();

  const title = ref("");
  const projectType = ref(PROJECT_TYPES[0]!.value);
  const summary = ref("");
  const description = ref("");
  const error = ref("");
  const pending = ref(false);

  const categories = ref<Category[]>([]);
  const selectedCategories = ref<string[]>([]);

  async function loadCategories() {
    selectedCategories.value = [];
    try {
      const data = await api<{ categories: Category[] }>("/categories", {
        query: { project_type: projectType.value },
      });
      categories.value = data.categories;
    } catch {
      categories.value = [];
    }
  }

  watch(projectType, loadCategories);

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
          description: description.value,
          category_ids: selectedCategories.value,
        },
      });
      await navigateTo(`/projects/${created.slug}`);
    } catch (err: any) {
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
    description,
    categories,
    selectedCategories,
    error,
    pending,
    loadCategories,
    submit,
  };
}
