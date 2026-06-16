<script setup lang="ts">
import {
  useProject,
  useVersions,
  useGallery,
  useUploadVersionForm,
  useUploadGalleryForm,
  projectTypeLabel,
  formatFileSize,
  VERSION_CHANNELS,
} from "~/scripts/pages/projects";
import { useAuth } from "~/scripts/auth";

const route = useRoute();
const slug = route.params.slug as string;
const { project, error, pending, load } = useProject(slug);
const { versions, load: loadVersions, downloadUrl } = useVersions(slug);
const { images, load: loadGallery, remove: removeImage } = useGallery(slug);
const { user, fetchUser } = useAuth();
const upload = useUploadVersionForm(slug);
const galleryUpload = useUploadGalleryForm(slug);

await load();
if (project.value) {
  await Promise.all([loadVersions(), loadGallery()]);
}

onMounted(fetchUser);

const isOwner = computed(
  () => !!user.value && user.value.username === project.value?.owner,
);

async function submitVersion() {
  if (await upload.submit()) {
    await loadVersions();
  }
}

async function submitGalleryImage() {
  if (await galleryUpload.submit()) {
    await loadGallery();
  }
}

async function deleteGalleryImage(id: string) {
  await removeImage(id);
}
</script>

<template>
  <section class="project-detail">
    <p v-if="pending" class="projects-status">Loading…</p>
    <p v-else-if="error" class="projects-status projects-error">{{ error }}</p>

    <article v-else-if="project">
      <NuxtLink to="/projects" class="project-back">Back to projects</NuxtLink>
      <div class="project-card-head">
        <h1 class="project-title">{{ project.title }}</h1>
        <span class="project-type">{{
          projectTypeLabel(project.project_type)
        }}</span>
      </div>
      <p class="project-summary">{{ project.summary }}</p>
      <p class="project-meta">
        by {{ project.owner }} - {{ project.download_count }} downloads
      </p>
      <ul v-if="project.categories.length" class="project-tags">
        <li
          v-for="category in project.categories"
          :key="category.slug"
          class="project-tag"
        >
          {{ category.name }}
        </li>
      </ul>
      <div class="project-description">{{ project.description }}</div>

      <section v-if="images.length || isOwner" class="gallery">
        <h2 class="gallery-title">Gallery</h2>

        <form
          v-if="isOwner"
          class="gallery-form"
          @submit.prevent="submitGalleryImage"
        >
          <p
            v-if="galleryUpload.error.value"
            class="projects-status projects-error"
          >
            {{ galleryUpload.error.value }}
          </p>
          <label class="version-field">
            <span>Caption (optional)</span>
            <input
              v-model="galleryUpload.caption.value"
              type="text"
              placeholder="What does this show?"
            />
          </label>
          <label class="version-field">
            <span>Image</span>
            <input
              type="file"
              accept="image/*"
              required
              @change="galleryUpload.onFileChange"
            />
          </label>
          <button
            type="submit"
            class="version-submit"
            :disabled="galleryUpload.pending.value"
          >
            {{ galleryUpload.pending.value ? "Uploading…" : "Add image" }}
          </button>
        </form>

        <ul v-if="images.length" class="gallery-grid">
          <li v-for="image in images" :key="image.id" class="gallery-item">
            <img :src="image.url" :alt="image.caption" class="gallery-image" />
            <p v-if="image.caption" class="gallery-caption">
              {{ image.caption }}
            </p>
            <button
              v-if="isOwner"
              type="button"
              class="gallery-delete"
              @click="deleteGalleryImage(image.id)"
            >
              Delete
            </button>
          </li>
        </ul>
      </section>

      <section class="versions">
        <h2 class="versions-title">Versions</h2>

        <form
          v-if="isOwner"
          class="version-form"
          @submit.prevent="submitVersion"
        >
          <h3 class="version-form-title">Upload a new version</h3>
          <p v-if="upload.error.value" class="projects-status projects-error">
            {{ upload.error.value }}
          </p>
          <div class="version-form-row">
            <label class="version-field">
              <span>Version number</span>
              <input
                v-model="upload.versionNumber.value"
                type="text"
                placeholder="1.0.0"
                required
              />
            </label>
            <label class="version-field">
              <span>Channel</span>
              <select v-model="upload.channel.value">
                <option
                  v-for="option in VERSION_CHANNELS"
                  :key="option.value"
                  :value="option.value"
                >
                  {{ option.label }}
                </option>
              </select>
            </label>
          </div>
          <label class="version-field">
            <span>Title (optional)</span>
            <input
              v-model="upload.name.value"
              type="text"
              placeholder="Summer update"
            />
          </label>
          <label class="version-field">
            <span>Changelog (optional)</span>
            <textarea v-model="upload.changelog.value" rows="3"></textarea>
          </label>
          <label class="version-field">
            <span>File</span>
            <input type="file" required @change="upload.onFileChange" />
          </label>
          <button
            type="submit"
            class="version-submit"
            :disabled="upload.pending.value"
          >
            {{ upload.pending.value ? "Uploading…" : "Upload version" }}
          </button>
        </form>

        <p v-if="versions.length === 0" class="projects-status">
          No versions yet.
        </p>
        <ul v-else class="version-list">
          <li
            v-for="version in versions"
            :key="version.id"
            class="version-card"
          >
            <div class="version-head">
              <span class="version-number">{{ version.version_number }}</span>
              <span class="version-channel">{{ version.channel }}</span>
            </div>
            <p v-if="version.name" class="version-name">{{ version.name }}</p>
            <p v-if="version.changelog" class="version-changelog">
              {{ version.changelog }}
            </p>
            <div class="version-meta">
              <a
                v-if="version.file"
                :href="downloadUrl(version)"
                class="version-download"
              >
                Download
                <span class="version-size"
                  >({{ formatFileSize(version.file.size) }})</span
                >
              </a>
              <span class="version-downloads"
                >{{ version.download_count }} downloads</span
              >
            </div>
          </li>
        </ul>
      </section>
    </article>
  </section>
</template>
