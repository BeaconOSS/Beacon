<script setup lang="ts">
defineProps<{
  description: string;
}>();
</script>

<template>
  <div class="border-border/60 bg-card/30 rounded-xl border p-6">
    <p
      v-if="description && description.trim()"
      class="text-foreground/90 leading-relaxed break-words whitespace-pre-line"
    >
      {{ description }}
    </p>
    <p v-else class="text-muted-foreground">No description provided yet.</p>
  </div>
</template>
