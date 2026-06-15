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
