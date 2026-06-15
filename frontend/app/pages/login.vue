<script setup lang="ts">
import { useLoginForm } from '~/scripts/pages/login';
const { email, password, error, pending, submit, githubUrl, discordUrl, oauthError } = useLoginForm()
</script>

<template>
  <section class="login">
    <h1>Sign in</h1>
    <p v-if="oauthError" class="form-error">{{ oauthError }}</p>
    <div class="oauth-buttons">
      <a class="oauth-github" :href="githubUrl">Sign in with GitHub</a>
      <a class="oauth-discord" :href="discordUrl">Sign in with Discord</a>
    </div>
    <div class="login-divider"><span>or</span></div>
    <form class="login-form" @submit.prevent="submit">
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
          autocomplete="current-password"
        />
      </label>
      <p v-if="error" class="form-error">{{ error }}</p>
      <button class="submit" type="submit" :disabled="pending">
        {{ pending ? 'Signing in…' : 'Sign in' }}
      </button>
    </form>
    <p class="login-alt">
      Don't have an account?
      <NuxtLink to="/register">Create one</NuxtLink>
    </p>
  </section>
</template>

