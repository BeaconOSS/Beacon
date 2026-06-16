<script setup lang="ts">
const route = useRoute();
const slug = computed(() => String(route.params.slug ?? ""));
const username = computed(() => String(route.params.username ?? ""));
</script>

<template>
  <WipPage
    title="Project settings"
    :description="`Settings for @${username}'s '${slug}' project are coming soon. This is where you'll edit the description, categories, links, and more.`"
  />
</template>
