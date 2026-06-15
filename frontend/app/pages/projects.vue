<script setup lang="ts">
import { useProjects, projectTypeLabel } from '~/scripts/pages/projects';
const { projects, error, pending, load } = useProjects()

await load()
</script>

<template>
  <section class="projects">
    <h1>Browse projects</h1>

    <p v-if="pending" class="projects-status">Loading projects…</p>
    <p v-else-if="error" class="projects-status projects-error">{{ error }}</p>
    <p v-else-if="projects.length === 0" class="projects-status">No projects yet.</p>

    <ul v-else class="project-list">
      <li v-for="project in projects" :key="project.id" class="project-card">
        <div class="project-card-head">
          <h2 class="project-title">{{ project.title }}</h2>
          <span class="project-type">{{ projectTypeLabel(project.project_type) }}</span>
        </div>
        <p class="project-summary">{{ project.summary }}</p>
        <p class="project-meta">{{ project.download_count }} downloads</p>
      </li>
    </ul>
  </section>
</template>
