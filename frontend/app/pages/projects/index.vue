<script setup lang="ts">
import { useProjects, useCategoryFilters, projectTypeLabel } from '~/scripts/pages/projects';
const { projects, error, pending, load } = useProjects()
const { categories, load: loadCategories } = useCategoryFilters()

const selectedCategory = ref('')
const searchTerm = ref('')

await Promise.all([load(), loadCategories()])

async function reload() {
  await load(selectedCategory.value || undefined, searchTerm.value.trim() || undefined)
}

async function filterBy(slug: string) {
  if (selectedCategory.value === slug) return
  selectedCategory.value = slug
  await reload()
}
</script>

<template>
  <section class="projects">
    <h1>Browse projects</h1>

    <form class="projects-search" @submit.prevent="reload">
      <input
        v-model="searchTerm"
        type="search"
        class="projects-search-input"
        placeholder="Search projects…"
      />
      <button type="submit" class="projects-search-button">Search</button>
    </form>

    <div class="category-filters">
      <button
        type="button"
        class="category-filter"
        :class="{ active: selectedCategory === '' }"
        @click="filterBy('')"
      >
        All
      </button>
      <button
        v-for="category in categories"
        :key="category.slug"
        type="button"
        class="category-filter"
        :class="{ active: selectedCategory === category.slug }"
        @click="filterBy(category.slug)"
      >
        {{ category.name }}
      </button>
    </div>

    <p v-if="pending" class="projects-status">Loading projects…</p>
    <p v-else-if="error" class="projects-status projects-error">{{ error }}</p>
    <p v-else-if="projects.length === 0" class="projects-status">
      {{ searchTerm.trim() || selectedCategory ? 'No projects match your search.' : 'No projects yet.' }}
    </p>

    <ul v-else class="project-list">
      <li v-for="project in projects" :key="project.id" class="project-card">
        <NuxtLink :to="`/projects/${project.slug}`" class="project-card-link">
          <div class="project-card-head">
            <h2 class="project-title">{{ project.title }}</h2>
            <span class="project-type">{{ projectTypeLabel(project.project_type) }}</span>
          </div>
          <p class="project-summary">{{ project.summary }}</p>
          <p class="project-meta">{{ project.download_count }} downloads</p>
        </NuxtLink>
      </li>
    </ul>
  </section>
</template>
