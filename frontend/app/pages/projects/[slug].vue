<script setup lang="ts">
import type { Component } from "vue";
import { useClipboard } from "@vueuse/core";
import { toast } from "vue-sonner";
import {
  ArrowLeft,
  Bookmark,
  Calendar,
  ChevronDown,
  Clock,
  Copy,
  Download,
  Flag,
  Globe,
  Heart,
  ImagePlus,
  Link2,
  Loader2,
  MonitorSmartphone,
  MoreHorizontal,
  Package,
  Palette,
  Scale,
  Shirt,
  Trash2,
  Upload,
  Users,
} from "@lucide/vue";
import { useProject, projectTypeLabel } from "~/scripts/pages/projects";
import {
  useVersions,
  useUploadVersionForm,
  formatFileSize,
  VERSION_CHANNELS,
} from "~/scripts/pages/projects/versions";
import {
  useGallery,
  useUploadGalleryForm,
} from "~/scripts/pages/projects/gallery";
import type { Version } from "~/scripts/pages/projects/types";
import { useAuth } from "~/scripts/auth";
import { useSettings } from "~/scripts/settings";

const route = useRoute();
const slug = computed(() => String(route.params.slug ?? ""));

const { user } = useAuth();
const { settings } = useSettings();

const { project, error, pending, load: loadProject } = useProject(slug.value);
const { versions, load: loadVersions, downloadUrl } = useVersions(slug.value);
const {
  images,
  load: loadGallery,
  remove: removeImage,
} = useGallery(slug.value);

const versionForm = useUploadVersionForm(slug.value);
const galleryForm = useUploadGalleryForm(slug.value);

await Promise.all([loadProject(), loadVersions(), loadGallery()]);

const isOwner = computed(
  () =>
    !!user.value &&
    !!project.value &&
    user.value.username === project.value.owner,
);

interface TypeStyle {
  icon: Component;
  gradient: string;
}

const TYPE_STYLES: Record<string, TypeStyle> = {
  addon: {
    icon: Package,
    gradient: "from-amber-400/30 to-amber-600/10 text-amber-300",
  },
  world: {
    icon: Globe,
    gradient: "from-emerald-400/30 to-emerald-600/10 text-emerald-300",
  },
  resource_pack: {
    icon: Palette,
    gradient: "from-violet-400/30 to-violet-600/10 text-violet-300",
  },
  skin_pack: {
    icon: Shirt,
    gradient: "from-pink-400/30 to-pink-600/10 text-pink-300",
  },
};

const typeStyle = computed<TypeStyle>(
  () =>
    TYPE_STYLES[project.value?.project_type ?? "addon"] ?? TYPE_STYLES.addon!,
);

const downloadableVersions = computed(() =>
  versions.value.filter((v) => v.file),
);

const latestVersion = computed<Version | null>(
  () =>
    downloadableVersions.value.find((v) => v.channel === "release") ??
    downloadableVersions.value[0] ??
    null,
);

const changelogEntries = computed(() =>
  versions.value.filter((v) => v.changelog && v.changelog.trim().length > 0),
);

function relativeTime(iso: string): string {
  const then = new Date(iso).getTime();
  if (Number.isNaN(then)) return "";
  const sec = Math.floor((Date.now() - then) / 1000);
  if (sec < 60) return "just now";
  const min = Math.floor(sec / 60);
  if (min < 60) return `${min} minute${min === 1 ? "" : "s"} ago`;
  const hr = Math.floor(min / 60);
  if (hr < 24) return `${hr} hour${hr === 1 ? "" : "s"} ago`;
  const day = Math.floor(hr / 24);
  if (day < 30) return `${day} day${day === 1 ? "" : "s"} ago`;
  const mon = Math.floor(day / 30);
  if (mon < 12) return `${mon} month${mon === 1 ? "" : "s"} ago`;
  const yr = Math.floor(day / 365);
  return `${yr} year${yr === 1 ? "" : "s"} ago`;
}

const channelLabel = (value: string) =>
  VERSION_CHANNELS.find((c) => c.value === value)?.label ?? value;

const { copy } = useClipboard();

async function copyId() {
  if (!project.value) return;
  await copy(project.value.id);
  toast.success("Project ID copied to clipboard");
}

async function copyLink() {
  await copy(window.location.href);
  toast.success("Link copied to clipboard");
}

function comingSoon(label: string) {
  toast.info(`${label} isn't available yet - coming soon.`);
}

const reportOpen = ref(false);
const reportReason = ref("");

function submitReport() {
  reportOpen.value = false;
  reportReason.value = "";
  toast.info(
    "Reporting isn't available yet - thanks, we've noted your interest.",
  );
}

async function submitVersion() {
  if (await versionForm.submit()) {
    await loadVersions();
    toast.success("Version published");
  }
}

async function submitGallery() {
  if (await galleryForm.submit()) {
    await loadGallery();
    toast.success("Image added to the gallery");
  }
}

async function deleteImage(id: string) {
  if (await removeImage(id)) {
    toast.success("Image removed");
  } else {
    toast.error("Could not remove the image");
  }
}
</script>

<template>
  <div class="page-canvas min-h-screen">
    <div class="mx-auto max-w-6xl px-6 py-10">
      <NuxtLink
        to="/projects"
        class="text-muted-foreground hover:text-foreground mb-6 inline-flex items-center gap-2 text-sm transition-colors"
      >
        <ArrowLeft class="size-4" />
        Back to browse
      </NuxtLink>

      <div
        v-if="pending"
        class="text-muted-foreground flex items-center gap-2 py-20"
      >
        <Loader2 class="size-5 animate-spin" />
        Loading project…
      </div>

      <div
        v-else-if="error || !project"
        class="border-border/60 rounded-xl border p-10 text-center"
      >
        <p class="text-muted-foreground">{{ error || "Project not found." }}</p>
        <NuxtLink
          to="/projects"
          class="text-primary mt-3 inline-block text-sm hover:underline"
        >
          Browse all projects
        </NuxtLink>
      </div>

      <template v-else>
        <section
          class="border-border/60 bg-card/40 rounded-2xl border p-6 backdrop-blur-sm sm:p-8"
        >
          <div
            class="flex flex-col gap-6 lg:flex-row lg:items-start lg:justify-between"
          >
            <div class="flex gap-5">
              <div
                class="flex size-20 shrink-0 items-center justify-center rounded-2xl bg-gradient-to-br ring-1 ring-white/10"
                :class="typeStyle.gradient"
              >
                <component :is="typeStyle.icon" class="size-9" />
              </div>
              <div class="min-w-0">
                <div class="flex flex-wrap items-center gap-3">
                  <h1 class="display-heading text-2xl sm:text-3xl">
                    {{ project.title }}
                  </h1>
                  <span
                    class="bg-primary/15 text-primary rounded-full px-3 py-1 text-xs font-semibold"
                  >
                    {{ projectTypeLabel(project.project_type) }}
                  </span>
                </div>
                <p class="text-muted-foreground mt-2 max-w-2xl">
                  {{ project.summary }}
                </p>
                <div
                  class="text-muted-foreground mt-4 flex flex-wrap items-center gap-5 text-sm"
                >
                  <span class="inline-flex items-center gap-1.5">
                    <Download class="size-4" />
                    {{ project.download_count.toLocaleString() }} downloads
                  </span>
                  <span class="inline-flex items-center gap-1.5">
                    <Heart class="size-4" />
                    0 hearts
                  </span>
                  <span class="inline-flex items-center gap-1.5">
                    <Clock class="size-4" />
                    {{ relativeTime(project.created_at) }}
                  </span>
                </div>
              </div>
            </div>

            <div class="flex shrink-0 items-center gap-2">
              <DropdownMenu>
                <DropdownMenuTrigger as-child>
                  <Button class="btn-glow gap-2" :disabled="!latestVersion">
                    <Download class="size-4" />
                    {{ latestVersion ? "Download" : "No downloads" }}
                    <ChevronDown class="size-4 opacity-70" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end" class="w-72">
                  <DropdownMenuLabel>Choose a version</DropdownMenuLabel>
                  <DropdownMenuSeparator />
                  <template v-if="downloadableVersions.length">
                    <DropdownMenuItem
                      v-for="v in downloadableVersions"
                      :key="v.id"
                      as-child
                    >
                      <a
                        :href="downloadUrl(v)"
                        class="flex items-center justify-between gap-3"
                      >
                        <span class="flex flex-col">
                          <span class="font-medium">{{
                            v.version_number
                          }}</span>
                          <span class="text-muted-foreground text-xs">
                            {{ channelLabel(v.channel) }}
                            <template v-if="v.file">
                              · {{ formatFileSize(v.file.size) }}</template
                            >
                          </span>
                        </span>
                        <Download class="size-4 opacity-70" />
                      </a>
                    </DropdownMenuItem>
                  </template>
                  <DropdownMenuItem v-else disabled>
                    No downloads yet
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>

              <Button
                variant="outline"
                class="gap-2"
                @click="comingSoon('Hearts')"
              >
                <Heart class="size-4" />
                <span class="tabular-nums">0</span>
              </Button>

              <Button
                variant="outline"
                size="icon"
                aria-label="Save project"
                @click="comingSoon('Saving')"
              >
                <Bookmark class="size-4" />
              </Button>

              <DropdownMenu>
                <DropdownMenuTrigger as-child>
                  <Button
                    variant="outline"
                    size="icon"
                    aria-label="More actions"
                  >
                    <MoreHorizontal class="size-4" />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end" class="w-48">
                  <DropdownMenuItem
                    variant="destructive"
                    @click="reportOpen = true"
                  >
                    <Flag class="size-4" />
                    Report
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem @click="copyId">
                    <Copy class="size-4" />
                    Copy ID
                  </DropdownMenuItem>
                  <DropdownMenuItem @click="copyLink">
                    <Link2 class="size-4" />
                    Copy link
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
          </div>
        </section>

        <div class="mt-8 flex flex-col gap-8 lg:flex-row">
          <div
            class="min-w-0 flex-1"
            :class="settings.contentSidebarLeft ? 'lg:order-2' : ''"
          >
            <Tabs default-value="description">
              <TabsList class="flex-wrap">
                <TabsTrigger value="description">Description</TabsTrigger>
                <TabsTrigger value="gallery">
                  Gallery
                  <span
                    v-if="images.length"
                    class="text-muted-foreground ml-1 text-xs"
                  >
                    {{ images.length }}
                  </span>
                </TabsTrigger>
                <TabsTrigger value="changelog">Changelog</TabsTrigger>
                <TabsTrigger value="versions">
                  Versions
                  <span
                    v-if="versions.length"
                    class="text-muted-foreground ml-1 text-xs"
                  >
                    {{ versions.length }}
                  </span>
                </TabsTrigger>
              </TabsList>

              <TabsContent value="description">
                <div class="border-border/60 bg-card/30 rounded-xl border p-6">
                  <p
                    v-if="project.description && project.description.trim()"
                    class="text-foreground/90 leading-relaxed whitespace-pre-line"
                  >
                    {{ project.description }}
                  </p>
                  <p v-else class="text-muted-foreground">
                    No description provided yet.
                  </p>
                </div>
              </TabsContent>

              <TabsContent value="gallery">
                <div class="border-border/60 bg-card/30 rounded-xl border p-6">
                  <div v-if="images.length" class="grid gap-4 sm:grid-cols-2">
                    <figure
                      v-for="image in images"
                      :key="image.id"
                      class="border-border/60 group relative overflow-hidden rounded-lg border"
                    >
                      <img
                        :src="image.url"
                        :alt="image.caption"
                        class="aspect-video w-full object-cover"
                      />
                      <figcaption
                        v-if="image.caption"
                        class="text-muted-foreground bg-card/80 px-3 py-2 text-sm"
                      >
                        {{ image.caption }}
                      </figcaption>
                      <Button
                        v-if="isOwner"
                        variant="destructive"
                        size="icon"
                        class="absolute top-2 right-2 opacity-0 transition-opacity group-hover:opacity-100"
                        aria-label="Delete image"
                        @click="deleteImage(image.id)"
                      >
                        <Trash2 class="size-4" />
                      </Button>
                    </figure>
                  </div>
                  <p v-else class="text-muted-foreground">
                    No screenshots yet.
                  </p>

                  <form
                    v-if="isOwner"
                    class="border-border/60 mt-6 space-y-4 rounded-lg border border-dashed p-4"
                    @submit.prevent="submitGallery"
                  >
                    <p class="flex items-center gap-2 text-sm font-medium">
                      <ImagePlus class="size-4" />
                      Add a screenshot
                    </p>
                    <div class="space-y-2">
                      <Label for="gallery-caption">Caption (optional)</Label>
                      <Input
                        id="gallery-caption"
                        v-model="galleryForm.caption.value"
                        placeholder="A short caption"
                      />
                    </div>
                    <div class="space-y-2">
                      <Label for="gallery-file">Image</Label>
                      <Input
                        id="gallery-file"
                        type="file"
                        accept="image/*"
                        @change="galleryForm.onFileChange"
                      />
                    </div>
                    <p
                      v-if="galleryForm.error.value"
                      class="text-destructive text-sm"
                    >
                      {{ galleryForm.error.value }}
                    </p>
                    <Button
                      type="submit"
                      class="gap-2"
                      :disabled="galleryForm.pending.value"
                    >
                      <Loader2
                        v-if="galleryForm.pending.value"
                        class="size-4 animate-spin"
                      />
                      <Upload v-else class="size-4" />
                      Upload image
                    </Button>
                  </form>
                </div>
              </TabsContent>

              <TabsContent value="changelog">
                <div class="border-border/60 bg-card/30 rounded-xl border p-6">
                  <ol v-if="changelogEntries.length" class="space-y-6">
                    <li
                      v-for="v in changelogEntries"
                      :key="v.id"
                      class="border-border/40 border-l-2 pl-4"
                    >
                      <div class="flex flex-wrap items-center gap-2">
                        <span class="font-semibold">{{
                          v.version_number
                        }}</span>
                        <span
                          class="bg-primary/15 text-primary rounded-full px-2 py-0.5 text-xs"
                        >
                          {{ channelLabel(v.channel) }}
                        </span>
                        <span class="text-muted-foreground text-xs">
                          {{ relativeTime(v.created_at) }}
                        </span>
                      </div>
                      <p class="text-foreground/85 mt-2 whitespace-pre-line">
                        {{ v.changelog }}
                      </p>
                    </li>
                  </ol>
                  <p v-else class="text-muted-foreground">
                    No changelog entries yet.
                  </p>
                </div>
              </TabsContent>

              <TabsContent value="versions">
                <div class="border-border/60 bg-card/30 rounded-xl border p-6">
                  <ul v-if="versions.length" class="divide-border/60 divide-y">
                    <li
                      v-for="v in versions"
                      :key="v.id"
                      class="flex flex-wrap items-center justify-between gap-3 py-3 first:pt-0"
                    >
                      <div class="min-w-0">
                        <div class="flex flex-wrap items-center gap-2">
                          <span class="font-medium">{{
                            v.version_number
                          }}</span>
                          <span
                            v-if="v.name"
                            class="text-muted-foreground text-sm"
                          >
                            {{ v.name }}
                          </span>
                          <span
                            class="bg-muted text-muted-foreground rounded-full px-2 py-0.5 text-xs"
                          >
                            {{ channelLabel(v.channel) }}
                          </span>
                        </div>
                        <div
                          class="text-muted-foreground mt-1 flex flex-wrap items-center gap-3 text-xs"
                        >
                          <span>{{ relativeTime(v.created_at) }}</span>
                          <span v-if="v.file">{{
                            formatFileSize(v.file.size)
                          }}</span>
                          <span class="inline-flex items-center gap-1">
                            <Download class="size-3" />
                            {{ v.download_count.toLocaleString() }}
                          </span>
                        </div>
                      </div>
                      <Button
                        v-if="v.file"
                        as-child
                        variant="outline"
                        size="sm"
                        class="gap-2"
                      >
                        <a :href="downloadUrl(v)">
                          <Download class="size-4" />
                          Download
                        </a>
                      </Button>
                    </li>
                  </ul>
                  <p v-else class="text-muted-foreground">
                    No versions published yet.
                  </p>

                  <form
                    v-if="isOwner"
                    class="border-border/60 mt-6 space-y-4 rounded-lg border border-dashed p-4"
                    @submit.prevent="submitVersion"
                  >
                    <p class="flex items-center gap-2 text-sm font-medium">
                      <Upload class="size-4" />
                      Publish a new version
                    </p>
                    <div class="grid gap-4 sm:grid-cols-2">
                      <div class="space-y-2">
                        <Label for="version-number">Version number</Label>
                        <Input
                          id="version-number"
                          v-model="versionForm.versionNumber.value"
                          placeholder="1.0.0"
                        />
                      </div>
                      <div class="space-y-2">
                        <Label for="version-name">Name (optional)</Label>
                        <Input
                          id="version-name"
                          v-model="versionForm.name.value"
                          placeholder="Release title"
                        />
                      </div>
                    </div>
                    <div class="space-y-2">
                      <Label for="version-channel">Channel</Label>
                      <select
                        id="version-channel"
                        v-model="versionForm.channel.value"
                        class="border-input bg-background h-9 w-full rounded-md border px-3 text-sm"
                      >
                        <option
                          v-for="c in VERSION_CHANNELS"
                          :key="c.value"
                          :value="c.value"
                        >
                          {{ c.label }}
                        </option>
                      </select>
                    </div>
                    <div class="space-y-2">
                      <Label for="version-changelog"
                        >Changelog (optional)</Label
                      >
                      <Textarea
                        id="version-changelog"
                        v-model="versionForm.changelog.value"
                        rows="3"
                        placeholder="What changed in this version?"
                      />
                    </div>
                    <div class="space-y-2">
                      <Label for="version-file">File</Label>
                      <Input
                        id="version-file"
                        type="file"
                        @change="versionForm.onFileChange"
                      />
                    </div>
                    <p
                      v-if="versionForm.error.value"
                      class="text-destructive text-sm"
                    >
                      {{ versionForm.error.value }}
                    </p>
                    <Button
                      type="submit"
                      class="gap-2"
                      :disabled="versionForm.pending.value"
                    >
                      <Loader2
                        v-if="versionForm.pending.value"
                        class="size-4 animate-spin"
                      />
                      <Upload v-else class="size-4" />
                      Publish version
                    </Button>
                  </form>
                </div>
              </TabsContent>
            </Tabs>
          </div>

          <aside class="space-y-4 lg:w-72 lg:shrink-0">
            <div class="border-border/60 bg-card/30 rounded-xl border p-4">
              <h3 class="eyebrow mb-3 flex items-center gap-2">
                <MonitorSmartphone class="size-3.5" />
                Compatibility
              </h3>
              <p class="text-sm">Minecraft: Bedrock Edition</p>
              <p class="text-muted-foreground mt-1 text-xs">
                Version details coming soon
              </p>
            </div>

            <div
              v-if="project.categories.length"
              class="border-border/60 bg-card/30 rounded-xl border p-4"
            >
              <h3 class="eyebrow mb-3">Tags</h3>
              <div class="flex flex-wrap gap-2">
                <NuxtLink
                  v-for="cat in project.categories"
                  :key="cat.slug"
                  :to="`/projects?category=${cat.slug}`"
                  class="bg-muted hover:bg-primary/15 hover:text-primary rounded-full px-3 py-1 text-xs transition-colors"
                >
                  {{ cat.name }}
                </NuxtLink>
              </div>
            </div>

            <div class="border-border/60 bg-card/30 rounded-xl border p-4">
              <h3 class="eyebrow mb-3 flex items-center gap-2">
                <Link2 class="size-3.5" />
                Links
              </h3>
              <p class="text-muted-foreground text-sm">No links yet.</p>
            </div>

            <div class="border-border/60 bg-card/30 rounded-xl border p-4">
              <h3 class="eyebrow mb-3 flex items-center gap-2">
                <Users class="size-3.5" />
                Creators
              </h3>
              <div class="flex items-center gap-3">
                <div
                  class="bg-primary/15 text-primary flex size-9 items-center justify-center rounded-full text-sm font-semibold uppercase"
                >
                  {{ project.owner.charAt(0) }}
                </div>
                <div>
                  <p class="text-sm font-medium">{{ project.owner }}</p>
                  <p class="text-muted-foreground text-xs">Owner</p>
                </div>
              </div>
            </div>

            <div class="border-border/60 bg-card/30 rounded-xl border p-4">
              <h3 class="eyebrow mb-3">Details</h3>
              <dl class="space-y-2.5 text-sm">
                <div class="flex items-center justify-between gap-2">
                  <dt class="text-muted-foreground flex items-center gap-1.5">
                    <Scale class="size-3.5" />
                    License
                  </dt>
                  <dd>Coming soon</dd>
                </div>
                <div class="flex items-center justify-between gap-2">
                  <dt class="text-muted-foreground flex items-center gap-1.5">
                    <Calendar class="size-3.5" />
                    Published
                  </dt>
                  <dd>{{ relativeTime(project.created_at) }}</dd>
                </div>
              </dl>
            </div>
          </aside>
        </div>
      </template>
    </div>

    <Dialog v-model:open="reportOpen">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Report this project</DialogTitle>
          <DialogDescription>
            Let us know what's wrong. Reporting isn't fully wired up yet, but
            your interest helps us prioritize it.
          </DialogDescription>
        </DialogHeader>
        <Textarea
          v-model="reportReason"
          rows="4"
          placeholder="Describe the issue (optional)"
        />
        <DialogFooter>
          <Button variant="outline" @click="reportOpen = false">Cancel</Button>
          <Button @click="submitReport">Submit report</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>
