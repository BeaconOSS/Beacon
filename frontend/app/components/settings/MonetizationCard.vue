<script setup lang="ts">
import { CircleCheck, Coins, Loader2 } from "@lucide/vue";
import {
  BEACON_SHARE,
  DEFAULT_CREATOR_SHARE,
} from "~/scripts/pages/projects/settings";
import { SHARE_PRESETS } from "~/scripts/pages/projects/settings/meta";

const monetizationEnabled = defineModel<boolean>("monetizationEnabled", {
  required: true,
});
const creatorShare = defineModel<number>("creatorShare", { required: true });

defineProps<{
  monetizationDirty: boolean;
  savingMonetization: boolean;
  monetizationError: string;
  locked: boolean;
}>();

defineEmits<{ save: [] }>();

const charityShare = computed(() => {
  const extra = DEFAULT_CREATOR_SHARE - creatorShare.value;
  return extra > 0 ? extra : 0;
});

function clampCreatorShare() {
  const value = Number(creatorShare.value);
  if (!Number.isFinite(value)) {
    creatorShare.value = 0;
    return;
  }
  creatorShare.value = Math.min(
    DEFAULT_CREATOR_SHARE,
    Math.max(0, Math.round(value)),
  );
}
</script>

<template>
  <div class="card-glass rounded-2xl p-6">
    <div class="flex items-start justify-between gap-4">
      <div>
        <h2 class="section-title mb-1 flex items-center gap-2 text-lg">
          <Coins class="text-primary size-5" />
          Monetization
        </h2>
        <p class="text-muted-foreground text-sm">
          Earn from your project through the Beacon Rewards Program.
        </p>
      </div>
      <Switch v-model="monetizationEnabled" class="mt-1 shrink-0" />
    </div>

    <div v-if="monetizationEnabled" class="mt-6 space-y-6">
      <p class="text-muted-foreground text-sm leading-relaxed">
        When monetization is on, your project earns a share of revenue through
        the
        <span class="text-foreground font-medium">Rewards Program</span>
        <span class="text-muted-foreground/70"> (coming soon)</span>. Beacon
        keeps a fixed {{ BEACON_SHARE }}% to cover running costs - any profit
        left over is donated to charity, with the breakdown published on our
        <span class="text-foreground font-medium">Beacon Finances</span>
        <span class="text-muted-foreground/70"> page (coming soon)</span>. You
        can give up part of your own share to send even more to charity.
      </p>

      <div class="space-y-3">
        <div
          class="bg-muted flex h-3 overflow-hidden rounded-full"
          role="img"
          :aria-label="`Creator ${creatorShare}%, charity ${charityShare}%, Beacon ${BEACON_SHARE}%`"
        >
          <div
            class="bg-primary h-full transition-all"
            :style="{ width: creatorShare + '%' }"
          />
          <div
            class="h-full bg-emerald-500 transition-all"
            :style="{ width: charityShare + '%' }"
          />
          <div
            class="bg-muted-foreground/40 h-full transition-all"
            :style="{ width: BEACON_SHARE + '%' }"
          />
        </div>
        <div class="flex flex-wrap gap-x-6 gap-y-2 text-xs">
          <span class="flex items-center gap-1.5">
            <span class="bg-primary size-2.5 rounded-full" />
            <span class="text-foreground font-medium"
              >You {{ creatorShare }}%</span
            >
          </span>
          <span class="flex items-center gap-1.5">
            <span class="size-2.5 rounded-full bg-emerald-500" />
            <span class="text-foreground font-medium"
              >Charity {{ charityShare }}%</span
            >
          </span>
          <span class="flex items-center gap-1.5">
            <span class="bg-muted-foreground/40 size-2.5 rounded-full" />
            <span class="text-muted-foreground"
              >Beacon {{ BEACON_SHARE }}%</span
            >
          </span>
        </div>
      </div>

      <div class="space-y-3">
        <div class="flex items-center justify-between gap-3">
          <Label class="text-sm">Your share</Label>
          <div
            class="border-input focus-within:ring-ring/50 flex items-center rounded-md border focus-within:ring-2"
          >
            <input
              v-model.number="creatorShare"
              type="number"
              min="0"
              max="80"
              step="1"
              class="w-14 bg-transparent py-1 pr-1 pl-2 text-right text-sm font-semibold outline-none"
              @change="clampCreatorShare"
            />
            <span class="text-muted-foreground pr-2 text-sm">%</span>
          </div>
        </div>
        <input
          v-model.number="creatorShare"
          type="range"
          min="0"
          max="80"
          step="1"
          class="accent-primary h-2 w-full cursor-pointer"
        />
        <div class="flex flex-wrap gap-2">
          <Button
            v-for="preset in SHARE_PRESETS"
            :key="preset.value"
            type="button"
            size="sm"
            :variant="creatorShare === preset.value ? 'default' : 'outline'"
            @click="creatorShare = preset.value"
          >
            {{ preset.label }}
          </Button>
        </div>
      </div>

      <div
        v-if="charityShare > 0"
        class="flex items-start gap-2 rounded-xl border border-emerald-500/30 bg-emerald-500/5 p-3 text-sm"
      >
        <CircleCheck class="mt-0.5 size-4 shrink-0 text-emerald-500" />
        <p class="text-muted-foreground">
          You're donating an extra
          <span class="font-semibold text-emerald-500"
            >{{ charityShare }}%</span
          >
          of revenue to charity on top of Beacon's contribution. Thank you.
        </p>
      </div>
    </div>

    <p v-else class="text-muted-foreground mt-6 text-sm leading-relaxed">
      Monetization is off, so your project earns nothing and no revenue share is
      collected. Turn it on if you'd like to earn through the Rewards Program -
      or keep it off if you'd rather not, or can't monetize for legal reasons.
    </p>

    <div
      class="mt-6 flex flex-col gap-3 border-t pt-5 sm:flex-row sm:items-center"
      :class="monetizationError ? 'sm:justify-between' : 'sm:justify-end'"
    >
      <p v-if="monetizationError" class="text-destructive text-sm">
        {{ monetizationError }}
      </p>
      <Button
        class="btn-glow shrink-0"
        :disabled="!monetizationDirty || savingMonetization || locked"
        @click="$emit('save')"
      >
        <Loader2 v-if="savingMonetization" class="size-4 animate-spin" />
        Save monetization
      </Button>
    </div>
  </div>
</template>
