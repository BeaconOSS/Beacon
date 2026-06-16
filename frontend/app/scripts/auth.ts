import { useApi } from "~/scripts/api";

export interface AuthUser {
  id: string;
  username: string;
  email: string;
}

export function useAuth() {
  const api = useApi();
  const user = useState<AuthUser | null>("auth-user", () => null);

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

  return { user, fetchUser, logout };
}
