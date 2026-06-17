<script setup lang="ts">
import { useRegisterForm } from "~/scripts/pages/register";

const { email, password, error, pending, submit, siteKey, widget, mountTurnstile, unmountTurnstile, githubUrl, discordUrl, oauthError } = useRegisterForm();

onMounted(mountTurnstile);
onBeforeUnmount(unmountTurnstile);
</script>

<template>
	<section class="login">
		<h1>Create account</h1>
		<p v-if="oauthError" class="form-error">{{ oauthError }}</p>
		<div class="oauth-buttons">
			<a class="oauth-github" :href="githubUrl">Sign up with GitHub</a>
			<a class="oauth-discord" :href="discordUrl">Sign up with Discord</a>
		</div>
		<div class="login-divider"><span>or</span></div>
		<form class="login-form" @submit.prevent="submit">
			<label class="field">
				<span>Email</span>
				<input v-model="email" type="email" name="email" autocomplete="email" />
			</label>
			<label class="field">
				<span>Password</span>
				<input v-model="password" type="password" name="password" autocomplete="new-password" />
			</label>
			<div v-show="siteKey" ref="widget" class="turnstile" />
			<p v-if="error" class="form-error">{{ error }}</p>
			<button class="submit" type="submit" :disabled="pending">
				{{ pending ? "Creating account…" : "Create account" }}
			</button>
		</form>
		<p class="login-alt">
			Already have an account?
			<NuxtLink to="/login">Sign in</NuxtLink>
		</p>
	</section>
</template>
