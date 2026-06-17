<script setup lang="ts">
import { reactiveOmit } from "@vueuse/core";
import { Primitive } from "reka-ui";

import { badgeVariants } from ".";

import type { BadgeVariants } from ".";
import type { PrimitiveProps } from "reka-ui";
import type { HTMLAttributes } from "vue";

import { cn } from "@/lib/utils";

const props = defineProps<
	PrimitiveProps & {
		variant?: BadgeVariants["variant"];
		class?: HTMLAttributes["class"];
	}
>();

const delegatedProps = reactiveOmit(props, "class");
</script>

<template>
	<Primitive data-slot="badge" :class="cn(badgeVariants({ variant }), props.class)" v-bind="delegatedProps">
		<slot />
	</Primitive>
</template>
