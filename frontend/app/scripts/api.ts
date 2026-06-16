import type { $Fetch } from 'nitropack'

export function useApi(): $Fetch {
  const config = useRuntimeConfig()
  return $fetch.create({
    baseURL: config.public.apiBase,
    credentials: 'include',
  })
}
