import type { $Fetch } from "nitropack";

export function useApi(): $Fetch {
	const config = useRuntimeConfig();
	const headers = import.meta.server ? useRequestHeaders(["cookie"]) : undefined;
	return $fetch.create({
		baseURL: config.public.apiBase,
		credentials: "include",
		headers,
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
	return e?.data?.error ?? options.fallback;
}
