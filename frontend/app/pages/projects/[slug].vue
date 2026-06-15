<script setup lang="ts">
import { useProject, projectTypeLabel } from '~/scripts/pages/projects';

const route = useRoute()
const slug = route.params.slug as string
const { project, error, pending, load } = useProject(slug)

await load()
</script>

<template>
  <section class="project-detail">
    <p v-if="pending" class="projects-status">Loading…</p>
    <p v-else-if="error" class="projects-status projects-error">{{ error }}</p>

    <article v-else-if="project">
      <NuxtLink to="/projects" class="project-back">Back to projects</NuxtLink>
      <div class="project-card-head">
        <h1 class="project-title">{{ project.title }}</h1>
        <span class="project-type">{{ projectTypeLabel(project.project_type) }}</span>
      </div>
      <p class="project-summary">{{ project.summary }}</p>
      <p class="project-meta">
        by {{ project.owner }} - {{ project.download_count }} downloads
      </p>
      <div class="project-description">{{ project.description }}</div>
    </article>
  </section>
</template>
