import type { $Fetch } from 'nitropack';

export function useApi(): $Fetch {
  const config = useRuntimeConfig()
  return $fetch.create({
    baseURL: config.public.apiBase,
    credentials: 'include',
  })
}

export function apiErrorMessage(
  err: any,
  options: { fallback: string; status?: Record<number, string> },
): string {
  const status = err?.response?.status ?? err?.statusCode
  if (status && options.status?.[status]) {
    return options.status[status]
  }
  return err?.data?.error ?? options.fallback
}
