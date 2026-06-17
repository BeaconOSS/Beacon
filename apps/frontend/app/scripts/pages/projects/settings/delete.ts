import { useApi, apiErrorMessage } from "~/scripts/api";

export function useSettingsDelete(slug: string) {
  const api = useApi();

  const deleting = ref(false);
  const deleteError = ref("");

  async function deleteProject(): Promise<boolean> {
    deleteError.value = "";
    deleting.value = true;
    try {
      await api(`/projects/${slug}`, { method: "DELETE" });
      return true;
    } catch (err) {
      deleteError.value = apiErrorMessage(err, {
        fallback: "Could not delete this project. Please try again.",
        status: {
          401: "Please sign in to delete this project.",
          403: "You do not have permission to delete this project.",
          404: "This project no longer exists.",
        },
      });
      return false;
    } finally {
      deleting.value = false;
    }
  }

  return { deleting, deleteError, deleteProject };
}
