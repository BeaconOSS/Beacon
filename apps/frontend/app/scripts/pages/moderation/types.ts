import type { FilePreviewKind, LineDiff, WordDiffSegment } from "~/scripts/pages/diff";

export type SignalStatus = "pass" | "warn" | "fail" | "pending" | "neutral";

export interface DecisionCheck {
	label: string;
	status: SignalStatus;
	detail: string;
}

export interface DecisionSignal {
	overall: SignalStatus;
	checks: DecisionCheck[];
}

export interface FilePreviewState {
	path: string;
	kind: FilePreviewKind;
	loading: boolean;
	error: string;
	diff: LineDiff | null;
	oldImage: string | null;
	newImage: string | null;
}

export interface FieldDiff {
	label: string;
	before: string;
	after: string;
	changed: boolean;
	segments: WordDiffSegment[] | null;
}

export interface QueueAge {
	hours: number;
	label: string;
	class: string;
}

export interface OwnerTrust {
	label: string;
	class: string;
	icon: "check" | "warn" | "new";
}

export interface LightboxState {
	url: string;
	caption: string;
	pixelated: boolean;
}
