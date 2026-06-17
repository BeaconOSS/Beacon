import { useApi, apiErrorMessage } from "~/scripts/api";
import type { ProjectMember } from "./types";

export function useProjectMembers(slug: string) {
  const api = useApi();

  const members = ref<ProjectMember[]>([]);
  const error = ref("");
  const pending = ref(false);

  const username = ref("");
  const adding = ref(false);
  const addError = ref("");

  async function load() {
    error.value = "";
    pending.value = true;
    try {
      const data = await api<{ members: ProjectMember[] }>(
        `/projects/${slug}/members`,
      );
      members.value = data.members;
    } catch (err) {
      error.value = apiErrorMessage(err, {
        fallback: "Could not load members. Please try again.",
        status: {
          401: "Please sign in to manage members.",
          403: "You do not have permission to manage members.",
        },
      });
    } finally {
      pending.value = false;
    }
  }

  async function add(): Promise<boolean> {
    const name = username.value.trim();
    if (!name) {
      addError.value = "Enter a username to invite.";
      return false;
    }
    addError.value = "";
    adding.value = true;
    try {
      await api(`/projects/${slug}/members`, {
        method: "POST",
        body: { username: name },
      });
      username.value = "";
      await load();
      return true;
    } catch (err) {
      addError.value = apiErrorMessage(err, {
        fallback: "Could not add that member. Please try again.",
        status: {
          401: "Please sign in to manage members.",
          403: "You do not have permission to manage members.",
          404: "No user with that username.",
          409: "That person is already a member.",
        },
      });
      return false;
    } finally {
      adding.value = false;
    }
  }

  async function remove(userId: string): Promise<boolean> {
    try {
      await api(`/projects/${slug}/members/${userId}`, {
        method: "DELETE",
      });
      members.value = members.value.filter((m) => m.user_id !== userId);
      return true;
    } catch {
      return false;
    }
  }

  return {
    members,
    error,
    pending,
    username,
    adding,
    addError,
    load,
    add,
    remove,
  };
}
