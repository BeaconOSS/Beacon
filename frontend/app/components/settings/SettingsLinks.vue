<script setup lang="ts">
import { Link2, Loader2 } from "@lucide/vue";

const websiteUrl = defineModel<string>("websiteUrl", { required: true });
const sourceUrl = defineModel<string>("sourceUrl", { required: true });
const issuesUrl = defineModel<string>("issuesUrl", { required: true });
const wikiUrl = defineModel<string>("wikiUrl", { required: true });
const discordUrl = defineModel<string>("discordUrl", { required: true });

defineProps<{
  linksDirty: boolean;
  savingLinks: boolean;
  linksError: string;
  locked: boolean;
}>();

defineEmits<{ save: [] }>();
</script>

<template>
  <section class="card-glass space-y-5 rounded-2xl p-6">
    <div>
      <h2 class="section-title mb-1 flex items-center gap-2 text-lg">
        <Link2 class="text-primary size-5" />
        Links
      </h2>
      <p class="text-muted-foreground text-sm leading-relaxed">
        Add external links so people can find your source code, report issues,
        or join your community. Leave a field blank to hide it.
      </p>
    </div>

    <div class="space-y-2">
      <Label for="link-website">Website</Label>
      <Input
        id="link-website"
        v-model="websiteUrl"
        type="url"
        placeholder="https://example.com"
      />
    </div>

    <div class="space-y-2">
      <Label for="link-source">Source code</Label>
      <Input
        id="link-source"
        v-model="sourceUrl"
        type="url"
        placeholder="https://github.com/you/project"
      />
    </div>

    <div class="space-y-2">
      <Label for="link-issues">Issue tracker</Label>
      <Input
        id="link-issues"
        v-model="issuesUrl"
        type="url"
        placeholder="https://github.com/you/project/issues"
      />
    </div>

    <div class="space-y-2">
      <Label for="link-wiki">Wiki / documentation</Label>
      <Input
        id="link-wiki"
        v-model="wikiUrl"
        type="url"
        placeholder="https://example.com/wiki"
      />
    </div>

    <div class="space-y-2">
      <Label for="link-discord">Discord invite</Label>
      <Input
        id="link-discord"
        v-model="discordUrl"
        type="url"
        placeholder="https://discord.gg/invite"
      />
    </div>

    <div
      class="flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center"
      :class="linksError ? 'sm:justify-between' : 'sm:justify-end'"
    >
      <p v-if="linksError" class="text-destructive text-sm">
        {{ linksError }}
      </p>
      <Button
        class="btn-glow shrink-0"
        :disabled="!linksDirty || savingLinks || locked"
        @click="$emit('save')"
      >
        <Loader2 v-if="savingLinks" class="size-4 animate-spin" />
        Save links
      </Button>
    </div>
  </section>
</template>
