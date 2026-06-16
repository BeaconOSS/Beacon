import { useApi, apiErrorMessage } from "~/scripts/api";

export interface DayStat {
  day: string;
  views: number;
  downloads: number;
}

export interface ProjectAnalytics {
  range_days: number;
  total_views: number;
  total_downloads: number;
  all_time_downloads: number;
  series: DayStat[];
}

export function useProjectAnalytics(slug: string) {
  const api = useApi();

  const data = ref<ProjectAnalytics | null>(null);
  const error = ref("");
  const pending = ref(false);

  async function load() {
    error.value = "";
    pending.value = true;
    try {
      data.value = await api<ProjectAnalytics>(`/projects/${slug}/analytics`);
    } catch (err) {
      error.value = apiErrorMessage(err, {
        fallback: "Could not load analytics. Please try again.",
        status: {
          401: "Please sign in to view analytics.",
          403: "You do not have permission to view analytics.",
        },
      });
    } finally {
      pending.value = false;
    }
  }

  return { data, error, pending, load };
}
