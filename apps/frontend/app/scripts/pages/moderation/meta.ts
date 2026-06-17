import { CircleCheck, CircleX, Loader2, TriangleAlert } from "@lucide/vue";

import type { SignalStatus } from "./types";
import type { LucideIcon } from "@lucide/vue";

import { PROJECT_VISIBILITY, REVIEW_ACTION, VERSION_CHANNEL } from "~/scripts/constants";

interface Badge {
	label: string;
	class: string;
}

interface Banner {
	label: string;
	class: string;
	icon: LucideIcon;
}

interface SignalAppearance {
	label: string;
	sub: string;
	class: string;
	badge: string;
	icon: LucideIcon;
}

const NEUTRAL_BADGE: Badge = {
	label: "",
	class: "bg-muted text-muted-foreground",
};

const VISIBILITY_LABELS: Record<string, string> = {
	[PROJECT_VISIBILITY.PUBLIC]: "Public",
	[PROJECT_VISIBILITY.UNLISTED]: "Unlisted",
	[PROJECT_VISIBILITY.PRIVATE]: "Private",
};

export function visibilityLabel(visibility: string): string {
	return VISIBILITY_LABELS[visibility] ?? visibility;
}

const REVIEW_ACTION_META: Record<string, Badge> = {
	[REVIEW_ACTION.APPROVE]: {
		label: "Approved",
		class: "bg-emerald-500/15 text-emerald-400",
	},
	[REVIEW_ACTION.REJECT]: {
		label: "Rejected",
		class: "bg-red-500/15 text-red-400",
	},
	[REVIEW_ACTION.REQUEST_CHANGES]: {
		label: "Changes requested",
		class: "bg-amber-500/15 text-amber-500",
	},
};

export function actionMeta(action: string): Badge {
	return REVIEW_ACTION_META[action] ?? { ...NEUTRAL_BADGE, label: action };
}

const CHANNEL_META: Record<string, Badge> = {
	[VERSION_CHANNEL.RELEASE]: {
		label: "Release",
		class: "bg-emerald-500/15 text-emerald-400",
	},
	[VERSION_CHANNEL.BETA]: {
		label: "Beta",
		class: "bg-sky-500/15 text-sky-400",
	},
	[VERSION_CHANNEL.ALPHA]: {
		label: "Alpha",
		class: "bg-amber-500/15 text-amber-500",
	},
};

export function channelMeta(channel: string): Badge {
	return CHANNEL_META[channel] ?? { ...NEUTRAL_BADGE, label: channel };
}

const DECISION_META: Record<string, Banner> = {
	pass: {
		label: "Passed validation",
		class: "border-emerald-500/30 bg-emerald-500/10 text-emerald-400",
		icon: CircleCheck,
	},
	warn: {
		label: "Passed with warnings",
		class: "border-amber-500/30 bg-amber-500/10 text-amber-500",
		icon: TriangleAlert,
	},
	fail: {
		label: "Validation errors found",
		class: "border-red-500/30 bg-red-500/10 text-red-400",
		icon: CircleX,
	},
};

export function decisionMeta(decision: string): Banner {
	return DECISION_META[decision] ?? DECISION_META.warn!;
}

const FINDING_CLASS: Record<string, string> = {
	error: "text-red-400",
	testFail: "text-red-400",
	warn: "text-amber-500",
	warning: "text-amber-500",
	testWarn: "text-amber-500",
	recommendation: "text-sky-400",
};

export function findingClass(type: string): string {
	return FINDING_CLASS[type] ?? "text-muted-foreground";
}

const DIFF_STATUS_META: Record<string, Badge> = {
	added: { label: "Added", class: "bg-emerald-500/15 text-emerald-400" },
	removed: { label: "Removed", class: "bg-red-500/15 text-red-400" },
	modified: { label: "Modified", class: "bg-amber-500/15 text-amber-500" },
};

export function diffStatusMeta(status: string): Badge {
	return DIFF_STATUS_META[status] ?? { ...NEUTRAL_BADGE, label: status };
}

const KIND_LABELS: Record<string, string> = {
	manifest: "Manifests",
	model: "Models",
	entity: "Entities",
	block: "Blocks",
	item: "Items",
	texture: "Textures",
	animation: "Animations",
	render_controller: "Render controllers",
	particle: "Particles",
	sound: "Sounds",
	function: "Functions",
	lang: "Language files",
	material: "Materials",
	other: "Other",
};

export function kindLabel(kind: string): string {
	return KIND_LABELS[kind] ?? kind;
}

export const SIGNAL_RANK: Record<SignalStatus, number> = {
	pass: 0,
	neutral: 0,
	pending: 1,
	warn: 2,
	fail: 3,
};

const SIGNAL_META: Record<SignalStatus, SignalAppearance> = {
	pass: {
		label: "Looks good",
		sub: "No blocking issues detected by the automated checks.",
		class: "border-emerald-500/30 bg-emerald-500/10 text-emerald-400",
		badge: "bg-emerald-500/15 text-emerald-400",
		icon: CircleCheck,
	},
	warn: {
		label: "Passed with warnings",
		sub: "Review the flagged checks below before approving.",
		class: "border-amber-500/30 bg-amber-500/10 text-amber-500",
		badge: "bg-amber-500/15 text-amber-500",
		icon: TriangleAlert,
	},
	fail: {
		label: "Issues found",
		sub: "Automated checks flagged problems - review carefully.",
		class: "border-red-500/30 bg-red-500/10 text-red-400",
		badge: "bg-red-500/15 text-red-400",
		icon: CircleX,
	},
	pending: {
		label: "Validation pending",
		sub: "Full signal not ready - analysis is still running.",
		class: "border-sky-500/30 bg-sky-500/10 text-sky-400",
		badge: "bg-sky-500/15 text-sky-400",
		icon: Loader2,
	},
	neutral: {
		label: "No signal yet",
		sub: "Not enough data to make an automated assessment.",
		class: "border-border/60 bg-card/40 text-muted-foreground",
		badge: "bg-muted text-muted-foreground",
		icon: TriangleAlert,
	},
};

export function signalMeta(status: SignalStatus): SignalAppearance {
	return SIGNAL_META[status];
}

const CHECK_STATUS_LABEL: Record<SignalStatus, string> = {
	pass: "Pass",
	warn: "Warn",
	fail: "Fail",
	pending: "Pending",
	neutral: "N/A",
};

export function checkStatusLabel(status: SignalStatus): string {
	return CHECK_STATUS_LABEL[status];
}

export function plural(n: number, word: string): string {
	return `${n} ${word}${n === 1 ? "" : "s"}`;
}
