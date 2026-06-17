/** Format a byte count as a human-readable size (e.g. `1.5 MB`). */
export function formatBytes(bytes: number): string {
	if (!bytes) return "0 B";
	const units = ["B", "KB", "MB", "GB"];
	const i = Math.min(units.length - 1, Math.floor(Math.log(bytes) / Math.log(1024)));
	const value = bytes / Math.pow(1024, i);
	return `${value.toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

/** Format an ISO timestamp as a localized date (e.g. `Jun 17, 2026`). */
export function formatDate(iso: string | null): string {
	if (!iso) return "-";
	const date = new Date(iso);
	if (Number.isNaN(date.getTime())) return "-";
	return date.toLocaleDateString(undefined, {
		year: "numeric",
		month: "short",
		day: "numeric",
	});
}

/** Format an ISO timestamp as a long-form relative time (e.g. `5 minutes ago`). */
export function relativeTime(iso: string | null): string {
	const sec = secondsSince(iso);
	if (sec === null) return "";
	if (sec < 60) return "just now";
	const min = Math.floor(sec / 60);
	if (min < 60) return `${min} minute${min === 1 ? "" : "s"} ago`;
	const hr = Math.floor(min / 60);
	if (hr < 24) return `${hr} hour${hr === 1 ? "" : "s"} ago`;
	const day = Math.floor(hr / 24);
	if (day < 30) return `${day} day${day === 1 ? "" : "s"} ago`;
	const mon = Math.floor(day / 30);
	if (mon < 12) return `${mon} month${mon === 1 ? "" : "s"} ago`;
	const yr = Math.floor(day / 365);
	return `${yr} year${yr === 1 ? "" : "s"} ago`;
}

/** Format an ISO timestamp as a compact relative time (e.g. `5m ago`). */
export function relativeTimeShort(iso: string | null): string {
	const sec = secondsSince(iso);
	if (sec === null) return "";
	if (sec < 60) return "just now";
	const min = Math.floor(sec / 60);
	if (min < 60) return `${min}m ago`;
	const hr = Math.floor(min / 60);
	if (hr < 24) return `${hr}h ago`;
	const day = Math.floor(hr / 24);
	if (day < 30) return `${day}d ago`;
	const mon = Math.floor(day / 30);
	if (mon < 12) return `${mon}mo ago`;
	return `${Math.floor(day / 365)}y ago`;
}

/** Seconds elapsed since an ISO timestamp, or `null` if missing/invalid. */
function secondsSince(iso: string | null): number | null {
	if (!iso) return null;
	const then = new Date(iso).getTime();
	if (Number.isNaN(then)) return null;
	return Math.floor((Date.now() - then) / 1000);
}
