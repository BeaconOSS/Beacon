import { useAuth, type AuthUser } from '~/scripts/auth'

export function useRegisterForm() {
  const config = useRuntimeConfig()
  const { user } = useAuth()

  const username = ref('')
  const email = ref('')
  const password = ref('')
  const error = ref('')
  const pending = ref(false)

  async function submit() {
    error.value = ''
    pending.value = true
    try {
      user.value = await $fetch<AuthUser>(`${config.public.apiBase}/register`, {
        method: 'POST',
        credentials: 'include',
        body: { username: username.value, email: email.value, password: password.value },
      })
      await navigateTo('/')
    } catch (err: any) {
      error.value = err?.data?.error ?? 'Could not create your account. Please try again.'
    } finally {
      pending.value = false
    }
  }

  return { username, email, password, error, pending, submit }
}
