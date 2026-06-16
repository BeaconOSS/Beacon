export interface ProjectSummary {
  id: string;
  slug: string;
  title: string;
  summary: string;
  project_type: string;
  download_count: number;
  icon_url?: string | null;
  owner: string;
  categories: CategoryTag[];
  created_at: string;
  updated_at?: string;
}

export interface ProjectDetail extends ProjectSummary {
  description: string;
  visibility?: ProjectVisibility;
  status?: ProjectStatus;
  heart_count?: number;
  viewer_hearted?: boolean;
  viewer_saved?: boolean;
  website_url?: string;
  source_url?: string;
  issues_url?: string;
  wiki_url?: string;
  discord_url?: string;
}

export type ProjectVisibility = "public" | "unlisted" | "private";

export type ProjectStatus =
  | "draft"
  | "in_review"
  | "changes_requested"
  | "approved"
  | "rejected";

export type ReviewAction = "approve" | "reject" | "request_changes";

export interface ProjectReview {
  action: ReviewAction;
  notes: string;
}

export interface ProjectSettings {
  id: string;
  slug: string;
  title: string;
  summary: string;
  description: string;
  project_type: string;
  visibility: ProjectVisibility;
  status: ProjectStatus;
  license: string;
  download_count: number;
  monetization_enabled: boolean;
  creator_share: number;
  owner: string;
  icon_url: string | null;
  website_url: string;
  source_url: string;
  issues_url: string;
  wiki_url: string;
  discord_url: string;
  categories: CategoryTag[];
  review: ProjectReview | null;
}

export interface CategoryTag {
  slug: string;
  name: string;
}

export interface Category {
  id: string;
  slug: string;
  name: string;
  project_type: string;
}

export interface VersionFile {
  filename: string;
  size: number;
  sha256: string;
}

export interface Version {
  id: string;
  version_number: string;
  name: string;
  changelog: string;
  channel: string;
  download_count: number;
  created_at: string;
  file: VersionFile | null;
}

export interface GalleryImage {
  id: string;
  caption: string;
  url: string;
}
