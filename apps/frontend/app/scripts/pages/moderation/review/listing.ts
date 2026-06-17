import type { FieldDiff, OwnerTrust } from "../types";
import type { Ref } from "vue";
import type { PendingReview, ProjectDetail } from "~/scripts/pages/projects/types";

import { diffWords } from "~/scripts/pages/diff";
import { projectTypeLabel } from "~/scripts/pages/projects";

export function useReviewListing(project: Ref<ProjectDetail | null>, pendingReview: Ref<PendingReview | null>) {
	const ownerTrust = computed<OwnerTrust | null>(() => {
		const owner = pendingReview.value?.owner;
		if (!owner) return null;
		if (owner.rejected_count > 0) {
			return {
				label: `${owner.rejected_count} prior rejection${owner.rejected_count === 1 ? "" : "s"}`,
				class: "bg-red-500/15 text-red-400",
				icon: "warn",
			};
		}
		if (owner.approved_count > 0) {
			return {
				label: `${owner.approved_count} approved before`,
				class: "bg-emerald-500/15 text-emerald-400",
				icon: "check",
			};
		}
		return {
			label: "New creator · first review",
			class: "bg-amber-500/15 text-amber-500",
			icon: "new",
		};
	});

	const typeLabel = computed(() => (project.value ? projectTypeLabel(project.value.project_type) : ""));

	const links = computed(() => {
		const l = pendingReview.value?.links;
		if (!l) return [] as { label: string; url: string }[];
		return [
			{ label: "Website", url: l.website_url },
			{ label: "Source code", url: l.source_url },
			{ label: "Issue tracker", url: l.issues_url },
			{ label: "Wiki", url: l.wiki_url },
			{ label: "Discord", url: l.discord_url },
		].filter((link) => link.url.trim().length > 0);
	});

	const reviewDiffs = computed<FieldDiff[]>(() => {
		const data = pendingReview.value;
		if (!data) return [];
		const before = data.published;
		const after = data.pending;
		const isFirst = data.is_first_review;

		function makeField(label: string, b: string, a: string): FieldDiff {
			const changed = !isFirst && b !== a;
			const wd = changed ? diffWords(b, a) : null;
			return {
				label,
				before: b,
				after: a,
				changed,
				segments: wd && !wd.tooLarge ? wd.segments : null,
			};
		}

		const fields: { label: string; key: keyof typeof after }[] = [
			{ label: "Title", key: "title" },
			{ label: "Summary", key: "summary" },
			{ label: "Description", key: "description" },
			{ label: "License", key: "license" },
		];
		const rows: FieldDiff[] = fields.map((field) => makeField(field.label, before ? String(before[field.key] ?? "") : "", String(after[field.key] ?? "")));
		rows.push(makeField("Categories", before ? before.categories.join(", ") : "", after.categories.join(", ")));
		return rows;
	});

	return { ownerTrust, typeLabel, links, reviewDiffs };
}
