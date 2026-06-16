<script setup lang="ts">
import { useCreateProjectForm, PROJECT_TYPES } from "~/scripts/pages/projects";
const {
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
} = useCreateProjectForm();

await loadCategories();
</script>

<template>
  <section class="login">
    <h1>New project</h1>
    <form class="login-form" @submit.prevent="submit">
      <label class="field">
        <span>Title</span>
        <input v-model="title" name="title" />
      </label>
      <label class="field">
        <span>Type</span>
        <select v-model="projectType" name="project_type">
          <option
            v-for="type in PROJECT_TYPES"
            :key="type.value"
            :value="type.value"
          >
            {{ type.label }}
          </option>
        </select>
      </label>
      <div v-if="categories.length" class="field">
        <span>Categories</span>
        <div class="category-options">
          <label
            v-for="category in categories"
            :key="category.id"
            class="category-option"
          >
            <input
              v-model="selectedCategories"
              type="checkbox"
              :value="category.id"
            />
            {{ category.name }}
          </label>
        </div>
      </div>
      <label class="field">
        <span>Summary</span>
        <input v-model="summary" name="summary" />
      </label>
      <label class="field">
        <span>Description</span>
        <textarea v-model="description" name="description" rows="6" />
      </label>
      <p v-if="error" class="form-error">{{ error }}</p>
      <button class="submit" type="submit" :disabled="pending">
        {{ pending ? "Creating…" : "Create project" }}
      </button>
    </form>
  </section>
</template>
