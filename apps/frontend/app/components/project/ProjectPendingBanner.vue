<script setup lang="ts">
import { Clock } from "@lucide/vue";
</script>

<template>
  <div
    class="border-amber-500/40 bg-amber-500/5 mb-6 flex items-start gap-3 rounded-2xl border p-5"
  >
    <Clock class="mt-0.5 size-5 shrink-0 text-amber-500" />
    <div>
      <h2 class="text-foreground font-semibold">
        Your changes are awaiting review
      </h2>
      <p class="text-muted-foreground mt-1 text-sm leading-relaxed">
        The version shown publicly is your last approved one. Your latest edits
        will go live once a moderator approves them.
      </p>
    </div>
  </div>
</template>
