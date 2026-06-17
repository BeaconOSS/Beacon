import type { QueueAge } from "../types";
import type { Ref } from "vue";
import type { PendingReview } from "~/scripts/pages/projects/types";

import { useModerationQueue } from "~/scripts/pages/moderation";

export function useReviewQueue(slug: string, pendingReview: Ref<PendingReview | null>) {
	const { projects: queueProjects, load: loadQueue } = useModerationQueue();

	const queueIndex = computed(() => queueProjects.value.findIndex((p) => p.slug === slug));
	const queuePosition = computed(() => (queueIndex.value >= 0 ? queueIndex.value + 1 : null));
	const queueTotal = computed(() => queueProjects.value.length);
	const queuePrev = computed(() => (queueIndex.value > 0 ? (queueProjects.value[queueIndex.value - 1] ?? null) : null));
	const queueNext = computed(() => (queueIndex.value >= 0 && queueIndex.value < queueProjects.value.length - 1 ? (queueProjects.value[queueIndex.value + 1] ?? null) : null));

	const queueAge = computed<QueueAge | null>(() => {
		const submitted = pendingReview.value?.submitted_at;
		if (!submitted) return null;
		const then = new Date(submitted).getTime();
		if (Number.isNaN(then)) return null;
		const hours = (Date.now() - then) / 3_600_000;
		if (hours < 24) {
			return {
				hours,
				label: "Fresh",
				class: "bg-emerald-500/15 text-emerald-400",
			};
		}
		if (hours < 72) {
			return {
				hours,
				label: "Waiting a while",
				class: "bg-amber-500/15 text-amber-500",
			};
		}
		return { hours, label: "Overdue", class: "bg-red-500/15 text-red-400" };
	});

	return { queuePosition, queueTotal, queuePrev, queueNext, queueAge, loadQueue };
}
