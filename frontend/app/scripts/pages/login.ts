import { useAuth, type AuthUser } from '~/scripts/auth'

export function useLoginForm() {
  const config = useRuntimeConfig()
  const { user } = useAuth()

  const email = ref('')
  const password = ref('')
  const error = ref('')
  const pending = ref(false)

  async function submit() {
    error.value = ''
    pending.value = true
    try {
      user.value = await $fetch<AuthUser>(`${config.public.apiBase}/login`, {
        method: 'POST',
        credentials: 'include',
        body: { email: email.value, password: password.value },
      })
      await navigateTo('/')
    } catch (err: any) {
      error.value = err?.data?.error ?? 'Could not sign in. Please try again.'
    } finally {
      pending.value = false
    }
  }

  return { email, password, error, pending, submit }
}
