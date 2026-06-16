// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from "@tailwindcss/vite";

export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",
  modules: ["@nuxt/eslint", "shadcn-nuxt"],
  devtools: { enabled: true },
  devServer: { port: 3001 },
  css: ["~/assets/css/tailwind.css", "~/assets/css/main.css"],
  app: {
    head: {
      htmlAttrs: { class: "dark" },
    },
  },
  shadcn: {
    prefix: "",
    componentDir: "@/components/ui",
  },
  runtimeConfig: {
    public: {
      apiBase: "http://localhost:3000",
      turnstileSiteKey: "",
    },
  },
  vite: {
    plugins: [tailwindcss()],
    optimizeDeps: {
      include: ["@vue/devtools-core", "@vue/devtools-kit"],
    },
  },
});
