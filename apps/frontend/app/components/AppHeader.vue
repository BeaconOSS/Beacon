<script setup lang="ts">
import { BarChart3, Bell, Boxes, Building2, FolderGit2, LogOut, Plus, Settings, ShieldCheck, User } from "@lucide/vue";
import { computed } from "vue";

import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger } from "@/components/ui/dropdown-menu";
import { useAuth } from "~/scripts/auth";

const { user, isModerator, fetchUser, logout } = useAuth();

onMounted(fetchUser);

const navLinks = [
	{ label: "Discover", to: "/projects" },
	{ label: "About", to: "/about" },
	{ label: "Wiki", to: "/wiki" },
];

const initials = computed(() => (user.value?.username ?? "?").slice(0, 2).toUpperCase());
</script>

<template>
	<header class="border-border/60 bg-background/80 sticky top-0 z-40 border-b backdrop-blur">
		<div class="mx-auto flex h-14 max-w-7xl items-center gap-6 px-4 sm:px-6 lg:px-8">
			<NuxtLink to="/" class="group flex items-center gap-2 leading-none" aria-label="Beacon home">
				<span class="icon-chip size-7 transition-transform group-hover:scale-105">
					<Boxes class="size-4" :stroke-width="2.25" />
				</span>
				<span class="font-heading text-foreground text-lg font-bold tracking-tight"> Beacon </span>
			</NuxtLink>

			<nav class="hidden items-center gap-1 md:flex">
				<NuxtLink
					v-for="link in navLinks"
					:key="link.to"
					:to="link.to"
					class="text-muted-foreground hover:text-foreground hover:bg-accent rounded-md px-3 py-1.5 text-sm font-medium transition-colors"
					active-class="text-foreground"
				>
					{{ link.label }}
				</NuxtLink>
			</nav>

			<div class="ml-auto flex items-center gap-2">
				<template v-if="user">
					<Button as-child size="sm">
						<NuxtLink to="/projects/new">
							<Plus />
							Publish
						</NuxtLink>
					</Button>

					<DropdownMenu>
						<DropdownMenuTrigger class="focus-visible:ring-ring rounded-full outline-none focus-visible:ring-2 focus-visible:ring-offset-2" aria-label="Account menu">
							<Avatar>
								<AvatarFallback>{{ initials }}</AvatarFallback>
							</Avatar>
						</DropdownMenuTrigger>
						<DropdownMenuContent align="end" class="w-52">
							<DropdownMenuGroup>
								<DropdownMenuItem as-child>
									<NuxtLink to="/profile">
										<User />
										Profile
									</NuxtLink>
								</DropdownMenuItem>
								<DropdownMenuItem as-child>
									<NuxtLink to="/notifications">
										<Bell />
										Notifications
									</NuxtLink>
								</DropdownMenuItem>
								<DropdownMenuItem as-child>
									<NuxtLink to="/settings">
										<Settings />
										Settings
									</NuxtLink>
								</DropdownMenuItem>
							</DropdownMenuGroup>
							<DropdownMenuSeparator />
							<DropdownMenuGroup>
								<DropdownMenuItem as-child>
									<NuxtLink to="/projects">
										<FolderGit2 />
										Projects
									</NuxtLink>
								</DropdownMenuItem>
								<DropdownMenuItem as-child>
									<NuxtLink to="/organizations">
										<Building2 />
										Organizations
									</NuxtLink>
								</DropdownMenuItem>
								<DropdownMenuItem as-child>
									<NuxtLink to="/analytics">
										<BarChart3 />
										Analytics
									</NuxtLink>
								</DropdownMenuItem>
							</DropdownMenuGroup>
							<template v-if="isModerator">
								<DropdownMenuSeparator />
								<DropdownMenuItem as-child>
									<NuxtLink to="/moderation">
										<ShieldCheck />
										Review queue
									</NuxtLink>
								</DropdownMenuItem>
							</template>
							<DropdownMenuSeparator />
							<DropdownMenuItem variant="destructive" @select="logout">
								<LogOut />
								Sign out
							</DropdownMenuItem>
						</DropdownMenuContent>
					</DropdownMenu>
				</template>

				<template v-else>
					<Button as-child variant="ghost" size="icon-sm" aria-label="Settings">
						<NuxtLink to="/settings">
							<Settings />
						</NuxtLink>
					</Button>
					<Button as-child size="sm">
						<NuxtLink to="/login">Sign in</NuxtLink>
					</Button>
				</template>
			</div>
		</div>
	</header>
</template>
