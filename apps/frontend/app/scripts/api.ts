import type { $Fetch } from "nitropack";

/// Build the request headers forwarded to the backend during SSR.
///
/// Forwards the visitor's cookie (so authenticated pages render on the server)
/// and their real IP under `x-beacon-forwarded-ip`. The IP lets the backend
/// attribute SSR-originated reads to the actual browser instead of the server's
/// own address, which keeps read rate limits per-visitor. On the client this
/// returns undefined (the browser attaches its own cookie + IP).
function ssrForwardHeaders(): Record<string, string> | undefined {
	if (!import.meta.server) {
		return undefined;
	}
	const incoming = useRequestHeaders(["cookie", "x-real-ip", "x-forwarded-for"]);
	const headers: Record<string, string> = {};
	if (incoming.cookie) {
		headers.cookie = incoming.cookie;
	}
	const forwardedFor = incoming["x-forwarded-for"]?.split(",")[0]?.trim();
	const clientIp = incoming["x-real-ip"]?.trim() || forwardedFor;
	if (clientIp) {
		headers["x-beacon-forwarded-ip"] = clientIp;
	}
	return headers;
}

export function useApi(): $Fetch {
	const config = useRuntimeConfig();
	return $fetch.create({
		baseURL: config.public.apiBase,
		credentials: "include",
		headers: ssrForwardHeaders(),
	});
}

export function apiErrorMessage(err: unknown, options: { fallback: string; status?: Record<number, string> }): string {
	const e = err as {
		response?: { status?: number };
		statusCode?: number;
		data?: { error?: string };
	};
	const status = e?.response?.status ?? e?.statusCode;
	if (status && options.status?.[status]) {
		return options.status[status];
	}
	if (e?.data?.error) {
		return e.data.error;
	}
	if (status === 429) {
		return "You're doing that too fast. Please wait a moment and try again.";
	}
	return options.fallback;
}
