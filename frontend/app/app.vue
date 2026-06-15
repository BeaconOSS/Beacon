<script setup lang="ts">
const config = useRuntimeConfig()
const { data, error } = await useFetch<{ status: string }>(
  `${config.public.apiBase}/health`
)
</script>

<template>
  <div>
    <NuxtRouteAnnouncer />
    <h1>Beacon</h1>
    <p v-if="data">Backend status: {{ data.status }}</p>
    <p v-else-if="error">Backend unreachable</p>
    <p v-else>Checking backend…</p>
  </div>
</template>
