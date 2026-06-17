import { toast } from "vue-sonner";

import { buildDecisionSignal } from "../signal";

import type { Ref } from "vue";
import type { ReviewAction } from "~/scripts/constants";
import type { PendingReview } from "~/scripts/pages/projects/types";

import { REVIEW_ACTION } from "~/scripts/constants";
import { useProjectReview } from "~/scripts/pages/moderation";

export function useReviewDecision(slug: string, pendingReview: Ref<PendingReview | null>) {
	const { submitting: reviewSubmitting, error: reviewError, review } = useProjectReview(slug);

	const reviewNotes = ref("");

	const decisionSignal = computed(() => buildDecisionSignal(pendingReview.value));

	async function handleReview(action: ReviewAction) {
		if ((action === REVIEW_ACTION.REJECT || action === REVIEW_ACTION.REQUEST_CHANGES) && !reviewNotes.value.trim()) {
			toast.error("Please add notes explaining your decision.");
			return;
		}
		const ok = await review(action, reviewNotes.value.trim());
		if (ok) {
			const labels: Record<typeof action, string> = {
				approve: "Project approved and published.",
				reject: "Project rejected.",
				request_changes: "Changes requested.",
			};
			toast.success(labels[action]);
			reviewNotes.value = "";
			await navigateTo("/moderation");
		} else if (reviewError.value) {
			toast.error(reviewError.value);
		}
	}

	return { reviewSubmitting, reviewNotes, handleReview, decisionSignal };
}
