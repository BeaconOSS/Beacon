export interface ProjectSummary {
  id: string;
  slug: string;
  title: string;
  summary: string;
  project_type: string;
  download_count: number;
  owner: string;
  categories: CategoryTag[];
  created_at: string;
  updated_at?: string;
}

export interface ProjectDetail extends ProjectSummary {
  description: string;
}

export type ProjectVisibility = "public" | "unlisted" | "private";

export interface ProjectSettings {
  id: string;
  slug: string;
  title: string;
  summary: string;
  description: string;
  project_type: string;
  visibility: ProjectVisibility;
  published: boolean;
  download_count: number;
  monetization_enabled: boolean;
  creator_share: number;
  owner: string;
  icon_url: string | null;
  categories: CategoryTag[];
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
