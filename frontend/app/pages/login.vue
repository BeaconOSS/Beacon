<template>
  <section class="login">
    <h1>Sign in</h1>
    <form class="login-form" @submit.prevent>
      <label class="field">
        <span>Email</span>
        <input type="email" name="email" autocomplete="email" />
      </label>
      <label class="field">
        <span>Password</span>
        <input type="password" name="password" autocomplete="current-password" />
      </label>
      <button class="submit" type="submit">Sign in</button>
    </form>
  </section>
</template>
