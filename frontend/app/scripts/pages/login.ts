import { useAuth, type AuthUser } from '~/scripts/auth'

const OAUTH_ERRORS: Record<string, string> = {
  github_denied: 'GitHub sign-in was cancelled.',
  github_email: 'Your GitHub account has no verified email we can use.',
}

export function useLoginForm() {
  const config = useRuntimeConfig()
  const route = useRoute()
  const { user } = useAuth()

  const email = ref('')
  const password = ref('')
  const error = ref('')
  const pending = ref(false)

  const githubUrl = `${config.public.apiBase}/auth/github`

  const oauthError = computed(() => {
    const code = route.query.error
    if (typeof code !== 'string') return ''
    return OAUTH_ERRORS[code] ?? 'Could not sign in with GitHub. Please try again.'
  })

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

  return { email, password, error, pending, submit, githubUrl, oauthError }
}
