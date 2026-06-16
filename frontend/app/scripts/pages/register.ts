import { useAuth, type AuthUser } from '~/scripts/auth'
import { useApi } from '~/scripts/api'

interface TurnstileApi {
  render: (el: HTMLElement, options: Record<string, unknown>) => string
  reset: (id?: string) => void
  remove: (id?: string) => void
}

declare global {
  interface Window {
    turnstile?: TurnstileApi
  }
}

const SCRIPT_SRC = 'https://challenges.cloudflare.com/turnstile/v0/api.js'

function loadTurnstileScript(): Promise<void> {
  return new Promise((resolve, reject) => {
    if (window.turnstile) {
      resolve()
      return
    }
    const existing = document.querySelector<HTMLScriptElement>(`script[src="${SCRIPT_SRC}"]`)
    if (existing) {
      existing.addEventListener('load', () => resolve())
      existing.addEventListener('error', () => reject(new Error('turnstile failed to load')))
      return
    }
    const script = document.createElement('script')
    script.src = SCRIPT_SRC
    script.async = true
    script.defer = true
    script.onload = () => resolve()
    script.onerror = () => reject(new Error('turnstile failed to load'))
    document.head.appendChild(script)
  })
}

export function useRegisterForm() {
  const config = useRuntimeConfig()
  const api = useApi()
  const { user } = useAuth()

  const username = ref('')
  const email = ref('')
  const password = ref('')
  const error = ref('')
  const pending = ref(false)

  const siteKey = config.public.turnstileSiteKey
  const turnstileToken = ref('')
  const widget = ref<HTMLElement | null>(null)
  let widgetId: string | undefined

  async function mountTurnstile() {
    if (!siteKey || !widget.value) return
    try {
      await loadTurnstileScript()
      widgetId = window.turnstile?.render(widget.value, {
        sitekey: siteKey,
        callback: (token: string) => {
          turnstileToken.value = token
        },
        'expired-callback': () => {
          turnstileToken.value = ''
        },
        'error-callback': () => {
          turnstileToken.value = ''
        },
      })
    } catch {
      error.value = 'Could not load the captcha. Please refresh and try again.'
    }
  }

  function unmountTurnstile() {
    if (widgetId) {
      window.turnstile?.remove(widgetId)
      widgetId = undefined
    }
  }

  async function submit() {
    error.value = ''
    if (siteKey && !turnstileToken.value) {
      error.value = 'Please complete the captcha.'
      return
    }
    pending.value = true
    try {
      user.value = await api<AuthUser>('/register', {
        method: 'POST',
        body: {
          username: username.value,
          email: email.value,
          password: password.value,
          turnstile_token: turnstileToken.value || undefined,
        },
      })
      await navigateTo('/')
    } catch (err: any) {
      error.value = err?.data?.error ?? 'Could not create your account. Please try again.'
      turnstileToken.value = ''
      window.turnstile?.reset(widgetId)
    } finally {
      pending.value = false
    }
  }

  return { username, email, password, error, pending, submit, siteKey, widget, mountTurnstile, unmountTurnstile }
}
