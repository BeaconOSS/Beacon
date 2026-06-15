<script setup lang="ts">
import { useRegisterForm } from '~/scripts/pages/register';
const { username, email, password, error, pending, submit, siteKey, widget, mountTurnstile, unmountTurnstile } = useRegisterForm()

onMounted(mountTurnstile)
onBeforeUnmount(unmountTurnstile)
</script>

<template>
  <section class="login">
    <h1>Create account</h1>
    <form class="login-form" @submit.prevent="submit">
      <label class="field">
        <span>Username</span>
        <input v-model="username" name="username" autocomplete="username" />
      </label>
      <label class="field">
        <span>Email</span>
        <input v-model="email" type="email" name="email" autocomplete="email" />
      </label>
      <label class="field">
        <span>Password</span>
        <input
          v-model="password"
          type="password"
          name="password"
          autocomplete="new-password"
        />
      </label>
      <div v-show="siteKey" ref="widget" class="turnstile"></div>
      <p v-if="error" class="form-error">{{ error }}</p>
      <button class="submit" type="submit" :disabled="pending">
        {{ pending ? 'Creating account…' : 'Create account' }}
      </button>
    </form>
    <p class="login-alt">
      Already have an account?
      <NuxtLink to="/login">Sign in</NuxtLink>
    </p>
  </section>
</template>
