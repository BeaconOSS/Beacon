export interface AuthUser {
  id: string
  username: string
  email: string
}

export function useAuth() {
  const config = useRuntimeConfig()
  const user = useState<AuthUser | null>('auth-user', () => null)

  async function fetchUser() {
    try {
      user.value = await $fetch<AuthUser>(`${config.public.apiBase}/me`, {
        credentials: 'include',
      })
    } catch {
      user.value = null
    }
  }

  async function logout() {
    try {
      await $fetch(`${config.public.apiBase}/logout`, {
        method: 'POST',
        credentials: 'include',
      })
    } finally {
      user.value = null
      await navigateTo('/')
    }
  }

  return { user, fetchUser, logout }
}
