import { useSettings } from "~/scripts/settings";

export default defineNuxtPlugin(() => {
  const { resolvedThemeClass, systemDark } = useSettings();

  useHead({
    htmlAttrs: {
      class: resolvedThemeClass,
    },
  });

  if (import.meta.client) {
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    systemDark.value = media.matches;
    media.addEventListener("change", (event) => {
      systemDark.value = event.matches;
    });
  }
});
