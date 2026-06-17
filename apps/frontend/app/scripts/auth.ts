import type { UserRole } from "~/scripts/constants";

import { useApi } from "~/scripts/api";
import { STAFF_ROLES } from "~/scripts/constants";

export interface AuthUser {
	id: string;
	username: string;
	email: string;
	role: UserRole;
}

export function useAuth() {
	const api = useApi();
	const user = useState<AuthUser | null>("auth-user", () => null);

	const isModerator = computed(() => (user.value ? STAFF_ROLES.includes(user.value.role) : false));

	async function fetchUser() {
		try {
			user.value = await api<AuthUser>("/me");
		} catch {
			user.value = null;
		}
	}

	async function logout() {
		try {
			await api("/logout", { method: "POST" });
		} finally {
			user.value = null;
			await navigateTo("/");
		}
	}

	return { user, isModerator, fetchUser, logout };
}
