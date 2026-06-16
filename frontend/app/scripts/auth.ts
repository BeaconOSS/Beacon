import { useApi } from "~/scripts/api";

export interface AuthUser {
  id: string;
  username: string;
  email: string;
  role: "user" | "moderator" | "admin";
}

export function useAuth() {
  const api = useApi();
  const user = useState<AuthUser | null>("auth-user", () => null);

  const isModerator = computed(
    () => user.value?.role === "moderator" || user.value?.role === "admin",
  );

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
