import { useReviewDecision } from "./decision";
import { useReviewHistory } from "./history";
import { useReviewListing } from "./listing";
import { useReviewNotes } from "./notes";
import { useReviewPreview } from "./preview";
import { useReviewQueue } from "./queue";

import { useAuth } from "~/scripts/auth";
import { useProjectPendingReview } from "~/scripts/pages/moderation";
import { useProject } from "~/scripts/pages/projects";

export function useModerationReview() {
	const route = useRoute();
	const slug = String(route.params.slug ?? "");

	const { isModerator } = useAuth();

	const { project, load: loadProject } = useProject(slug);
	const { data: pendingReview, error: pendingError, pending: pendingLoading, withBase: pendingWithBase, load: loadPendingReview } = useProjectPendingReview(slug);

	const queue = useReviewQueue(slug, pendingReview);
	const history = useReviewHistory();
	const preview = useReviewPreview(slug, pendingReview);
	const notes = useReviewNotes(slug);
	const decision = useReviewDecision(slug, pendingReview);
	const listing = useReviewListing(project, pendingReview);

	async function load() {
		await Promise.all([loadProject(), loadPendingReview(), notes.loadNotes(), queue.loadQueue()]);
	}

	return {
		slug,
		isModerator,
		project,
		pendingReview,
		pendingError,
		pendingLoading,
		pendingWithBase,
		load,
		...queue,
		...history,
		...preview,
		...notes,
		...decision,
		...listing,
	};
}
