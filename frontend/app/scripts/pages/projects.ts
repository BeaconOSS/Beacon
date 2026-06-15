export interface ProjectSummary {
  id: string
  slug: string
  title: string
  summary: string
  project_type: string
  download_count: number
  created_at: string
}

export interface ProjectDetail extends ProjectSummary {
  description: string
  owner: string
}

const PROJECT_TYPE_LABELS: Record<string, string> = {
  addon: 'Add-On',
  world: 'World',
  resource_pack: 'Resource Pack',
  skin_pack: 'Skin Pack',
}

export const PROJECT_TYPES = Object.entries(PROJECT_TYPE_LABELS).map(
  ([value, label]) => ({ value, label }),
)

export function projectTypeLabel(type: string): string {
  return PROJECT_TYPE_LABELS[type] ?? type
}

export function useProjects() {
  const config = useRuntimeConfig()

  const projects = ref<ProjectSummary[]>([])
  const error = ref('')
  const pending = ref(false)

  async function load() {
    error.value = ''
    pending.value = true
    try {
      const data = await $fetch<{ projects: ProjectSummary[] }>(
        `${config.public.apiBase}/projects`,
      )
      projects.value = data.projects
    } catch {
      error.value = 'Could not load projects. Please try again.'
    } finally {
      pending.value = false
    }
  }

  return { projects, error, pending, load }
}

export function useProject(slug: string) {
  const config = useRuntimeConfig()

  const project = ref<ProjectDetail | null>(null)
  const error = ref('')
  const pending = ref(false)

  async function load() {
    error.value = ''
    pending.value = true
    try {
      project.value = await $fetch<ProjectDetail>(
        `${config.public.apiBase}/projects/${slug}`,
      )
    } catch (err: any) {
      error.value =
        err?.response?.status === 404
          ? 'That project could not be found.'
          : 'Could not load this project. Please try again.'
    } finally {
      pending.value = false
    }
  }

  return { project, error, pending, load }
}

export function useCreateProjectForm() {
  const config = useRuntimeConfig()

  const title = ref('')
  const projectType = ref(PROJECT_TYPES[0]!.value)
  const summary = ref('')
  const description = ref('')
  const error = ref('')
  const pending = ref(false)

  async function submit() {
    error.value = ''
    pending.value = true
    try {
      const created = await $fetch<{ id: string; slug: string }>(
        `${config.public.apiBase}/projects`,
        {
          method: 'POST',
          credentials: 'include',
          body: {
            title: title.value,
            project_type: projectType.value,
            summary: summary.value,
            description: description.value,
          },
        },
      )
      await navigateTo(`/projects/${created.slug}`)
    } catch (err: any) {
      error.value =
        err?.response?.status === 401
          ? 'Please sign in to create a project.'
          : err?.data?.error ?? 'Could not create the project. Please try again.'
    } finally {
      pending.value = false
    }
  }

  return { title, projectType, summary, description, error, pending, submit }
}
