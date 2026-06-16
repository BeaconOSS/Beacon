<script setup lang="ts">
interface FooterLink {
  label: string;
  to?: string;
  href?: string;
}

interface FooterColumn {
  title: string;
  links: FooterLink[];
}

const columns: FooterColumn[] = [
  {
    title: "About",
    links: [
      { label: "News", to: "/news" },
      { label: "Changelog", to: "/changelog" },
      { label: "Careers", to: "/careers" },
      { label: "Licenses", to: "/licenses" },
      { label: "Support", to: "/support" },
    ],
  },
  {
    title: "Community",
    links: [
      { label: "Discord", href: "#" },
      { label: "Reddit", href: "#" },
      { label: "GitHub", href: "#" },
      { label: "X", href: "#" },
      { label: "TikTok", href: "#" },
      { label: "Instagram", href: "#" },
      { label: "YouTube", href: "#" },
    ],
  },
  {
    title: "Resources",
    links: [
      { label: "Wiki", to: "/wiki" },
      { label: "Help Center", to: "/help" },
      { label: "Translate", to: "/translate" },
      { label: "Report issues", href: "#" },
      { label: "API documentation", to: "/docs/api" },
    ],
  },
  {
    title: "Company",
    links: [
      { label: "Content Rules", to: "/legal/content-rules" },
      { label: "Terms & Conditions", to: "/legal/terms" },
      { label: "Privacy Policy", to: "/legal/privacy" },
      { label: "Security Notice", to: "/legal/security" },
      { label: "Copyright Policy & DMCA", to: "/legal/copyright" },
      { label: "Company Info", to: "/legal/company" },
    ],
  },
];

const year = new Date().getFullYear();
</script>

<template>
  <footer class="border-border/60 bg-card/30 mt-16 border-t">
    <div class="mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
      <div class="grid grid-cols-2 gap-8 md:grid-cols-5">
        <div class="col-span-2 md:col-span-1">
          <NuxtLink
            to="/"
            class="font-heading text-primary text-lg tracking-wide"
          >
            Beacon
          </NuxtLink>
          <p class="text-muted-foreground mt-2 text-sm">
            Open-source content platform. TODO: Better text
          </p>
        </div>

        <div v-for="column in columns" :key="column.title">
          <h3 class="text-foreground mb-3 text-sm font-semibold">
            {{ column.title }}
          </h3>
          <ul class="space-y-2">
            <li v-for="link in column.links" :key="link.label">
              <a
                v-if="link.href"
                :href="link.href"
                target="_blank"
                rel="noopener noreferrer"
                class="text-muted-foreground hover:text-foreground text-sm transition-colors"
              >
                {{ link.label }}
              </a>
              <NuxtLink
                v-else
                :to="link.to!"
                class="text-muted-foreground hover:text-foreground text-sm transition-colors"
              >
                {{ link.label }}
              </NuxtLink>
            </li>
          </ul>
        </div>
      </div>

      <div
        class="border-border/60 text-muted-foreground mt-10 border-t pt-6 text-sm"
      >
        © {{ year }} BeaconOSS. All rights reserved.
      </div>
    </div>
  </footer>
</template>
