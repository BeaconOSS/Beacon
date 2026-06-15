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
  categories: CategoryTag[]
}

export interface CategoryTag {
  slug: string
  name: string
}

export interface Category {
  id: string
  slug: string
  name: string
  project_type: string
}

export interface VersionFile {
  filename: string
  size: number
  sha256: string
}

export interface Version {
  id: string
  version_number: string
  name: string
  changelog: string
  channel: string
  download_count: number
  created_at: string
  file: VersionFile | null
}

export interface GalleryImage {
  id: string
  caption: string
  url: string
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

  async function load(category?: string, search?: string) {
    error.value = ''
    pending.value = true
    try {
      const query: Record<string, string> = {}
      if (category) query.category = category
      if (search) query.q = search
      const data = await $fetch<{ projects: ProjectSummary[] }>(
        `${config.public.apiBase}/projects`,
        { query },
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

export function useCategoryFilters() {
  const config = useRuntimeConfig()

  const categories = ref<CategoryTag[]>([])

  async function load() {
    try {
      const data = await $fetch<{ categories: Category[] }>(
        `${config.public.apiBase}/categories`,
      )
      const seen = new Set<string>()
      const unique: CategoryTag[] = []
      for (const category of data.categories) {
        if (!seen.has(category.slug)) {
          seen.add(category.slug)
          unique.push({ slug: category.slug, name: category.name })
        }
      }
      categories.value = unique
    } catch {
      categories.value = []
    }
  }

  return { categories, load }
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

  const categories = ref<Category[]>([])
  const selectedCategories = ref<string[]>([])

  async function loadCategories() {
    selectedCategories.value = []
    try {
      const data = await $fetch<{ categories: Category[] }>(
        `${config.public.apiBase}/categories`,
        { query: { project_type: projectType.value } },
      )
      categories.value = data.categories
    } catch {
      categories.value = []
    }
  }

  watch(projectType, loadCategories)

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
            category_ids: selectedCategories.value,
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
  }
}

export function useVersions(slug: string) {
  const config = useRuntimeConfig()

  const versions = ref<Version[]>([])
  const error = ref('')
  const pending = ref(false)

  async function load() {
    error.value = ''
    pending.value = true
    try {
      const data = await $fetch<{ versions: Version[] }>(
        `${config.public.apiBase}/projects/${slug}/versions`,
      )
      versions.value = data.versions
    } catch {
      error.value = 'Could not load versions.'
    } finally {
      pending.value = false
    }
  }

  function downloadUrl(version: Version): string {
    return `${config.public.apiBase}/projects/${slug}/versions/${encodeURIComponent(
      version.version_number,
    )}/download`
  }

  return { versions, error, pending, load, downloadUrl }
}

export function useGallery(slug: string) {
  const config = useRuntimeConfig()

  const images = ref<GalleryImage[]>([])

  async function load() {
    try {
      const data = await $fetch<{ images: GalleryImage[] }>(
        `${config.public.apiBase}/projects/${slug}/gallery`,
      )
      images.value = data.images.map((image) => ({
        ...image,
        url: `${config.public.apiBase}${image.url}`,
      }))
    } catch {
      images.value = []
    }
  }

  async function remove(id: string): Promise<boolean> {
    try {
      await $fetch(`${config.public.apiBase}/projects/${slug}/gallery/${id}`, {
        method: 'DELETE',
        credentials: 'include',
      })
      await load()
      return true
    } catch {
      return false
    }
  }

  return { images, load, remove }
}

export function useUploadGalleryForm(slug: string) {
  const config = useRuntimeConfig()

  const caption = ref('')
  const image = ref<File | null>(null)
  const error = ref('')
  const pending = ref(false)

  function onFileChange(event: Event) {
    const input = event.target as HTMLInputElement
    image.value = input.files?.[0] ?? null
  }

  async function submit(): Promise<boolean> {
    error.value = ''

    if (!image.value) {
      error.value = 'Please choose an image to upload.'
      return false
    }

    const body = new FormData()
    body.append('caption', caption.value)
    body.append('image', image.value)

    pending.value = true
    try {
      await $fetch(`${config.public.apiBase}/projects/${slug}/gallery`, {
        method: 'POST',
        credentials: 'include',
        body,
      })
      caption.value = ''
      image.value = null
      return true
    } catch (err: any) {
      error.value =
        err?.response?.status === 401
          ? 'Please sign in to upload an image.'
          : err?.data?.error ?? 'Could not upload the image. Please try again.'
      return false
    } finally {
      pending.value = false
    }
  }

  return { caption, image, error, pending, onFileChange, submit }
}

export const VERSION_CHANNELS = [
  { value: 'release', label: 'Release' },
  { value: 'beta', label: 'Beta' },
  { value: 'alpha', label: 'Alpha' },
]

export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  const units = ['KB', 'MB', 'GB']
  let size = bytes / 1024
  let unit = 0
  while (size >= 1024 && unit < units.length - 1) {
    size /= 1024
    unit += 1
  }
  return `${size.toFixed(1)} ${units[unit]}`
}

export function useUploadVersionForm(slug: string) {
  const config = useRuntimeConfig()

  const versionNumber = ref('')
  const name = ref('')
  const changelog = ref('')
  const channel = ref(VERSION_CHANNELS[0]!.value)
  const file = ref<File | null>(null)
  const error = ref('')
  const pending = ref(false)

  function onFileChange(event: Event) {
    const input = event.target as HTMLInputElement
    file.value = input.files?.[0] ?? null
  }

  async function submit(): Promise<boolean> {
    error.value = ''

    if (!versionNumber.value.trim()) {
      error.value = 'Please enter a version number.'
      return false
    }
    if (!file.value) {
      error.value = 'Please choose a file to upload.'
      return false
    }

    const body = new FormData()
    body.append('version_number', versionNumber.value.trim())
    body.append('name', name.value)
    body.append('changelog', changelog.value)
    body.append('channel', channel.value)
    body.append('file', file.value)

    pending.value = true
    try {
      await $fetch(`${config.public.apiBase}/projects/${slug}/versions`, {
        method: 'POST',
        credentials: 'include',
        body,
      })
      versionNumber.value = ''
      name.value = ''
      changelog.value = ''
      channel.value = VERSION_CHANNELS[0]!.value
      file.value = null
      return true
    } catch (err: any) {
      error.value =
        err?.response?.status === 401
          ? 'Please sign in to upload a version.'
          : err?.data?.error ?? 'Could not upload the version. Please try again.'
      return false
    } finally {
      pending.value = false
    }
  }

  return {
    versionNumber,
    name,
    changelog,
    channel,
    file,
    error,
    pending,
    onFileChange,
    submit,
  }
}


