<script setup lang="ts">
import { useAuth } from '~/scripts/auth';
const { user, fetchUser, logout } = useAuth()

onMounted(fetchUser)
</script>

<template>
  <header class="header">
    <NuxtLink class="logo" to="/">Beacon</NuxtLink>
    <div v-if="user" class="account">
      <span class="username">{{ user.username }}</span>
      <button class="sign-out" type="button" @click="logout">Sign out</button>
    </div>
    <NuxtLink v-else class="sign-in" to="/login">Sign in</NuxtLink>
  </header>
</template>
