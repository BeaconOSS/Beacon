import type {
  ProjectVisibility,
  ProjectStatus,
  ReviewAction,
} from "~/scripts/constants";

export type { ProjectVisibility, ProjectStatus, ReviewAction };

export interface ProjectSummary {
  id: string;
  slug: string;
  title: string;
  summary: string;
  project_type: string;
  download_count: number;
  heart_count?: number;
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
  license?: string;
  is_published?: boolean;
  has_pending_changes?: boolean;
  heart_count?: number;
  viewer_hearted?: boolean;
  viewer_saved?: boolean;
  website_url?: string;
  source_url?: string;
  issues_url?: string;
  wiki_url?: string;
  discord_url?: string;
  preview?: boolean;
}

export interface ProjectReview {
  action: ReviewAction;
  notes: string;
}

export interface RevisionContent {
  title: string;
  summary: string;
  description: string;
  license: string;
  icon_url: string | null;
  categories: string[];
}

export interface OwnerContext {
  username: string;
  member_since: string | null;
  project_count: number;
  approved_count: number;
  rejected_count: number;
}

export interface ProjectLinks {
  website_url: string;
  source_url: string;
  issues_url: string;
  wiki_url: string;
  discord_url: string;
}

export interface ProjectFacts {
  visibility: string;
  monetization_enabled: boolean;
  creator_share: number;
  heart_count: number;
  download_count: number;
  version_count: number;
  gallery_count: number;
  created_at: string | null;
}

export interface ReviewHistoryEntry {
  action: ReviewAction;
  reviewer: string;
  notes: string;
  created_at: string | null;
}

export interface ModeratorNote {
  id: string;
  author: string;
  body: string;
  created_at: string | null;
}

export interface GalleryItem {
  id: string;
  caption: string;
  url: string;
}

export interface VersionFile {
  filename: string;
  size: number;
  sha256: string;
}

export interface VersionItem {
  version_number: string;
  name: string;
  channel: string;
  changelog: string;
  created_at: string | null;
  file: VersionFile | null;
}

export interface AnalysisCounts {
  errors: number;
  warnings: number;
  recommendations: number;
  testSuccess: number;
  testFail: number;
  testNotApplicable: number;
}

export interface AnalysisInfo {
  capabilities: string[];
  apisUsed: string[];
  behaviorPackManifestCount: number;
  resourcePackManifestCount: number;
  entityTypeManifestCount: number;
  itemTypeManifestCount: number;
  blockTypeManifestCount: number;
  worldCount: number;
  subpackCount: number;
  overallSize: number;
  contentSize: number;
  fileCounts: number;
  folderCounts: number;
  contentFileCounts: number;
  animationCount: number;
  textureCount: number;
  vanillaGameTextureCoverage: number;
  minBehaviorPackMinEngineVersion: number;
  minResourcePackMinEngineVersion: number;
  itemTypes: string[];
}

export interface AnalysisFinding {
  type: string;
  generatorId: string;
  message: string;
}

export interface AnalysisFeature {
  generatorId: string;
  label: string;
  data: unknown;
}

export interface AnalysisReportBody {
  schemaVersion: string;
  mctoolsVersion: string;
  mctoolsVersionRaw: number;
  decision: "pass" | "warn" | "fail";
  counts: AnalysisCounts;
  info: AnalysisInfo;
  findings: AnalysisFinding[];
  features: AnalysisFeature[];
  summaries: {
    error: string;
    warning: string;
    testFail: string;
  };
}

export interface AnalysisReport {
  status: "ready" | "pending" | "error";
  error: string;
  mctools_version: string;
  analyzed_at: string | null;
  report: AnalysisReportBody | null;
}

export type PackDiffStatus = "added" | "removed" | "modified";

export interface PackDiffEntry {
  path: string;
  kind: string;
  status: PackDiffStatus;
  old_size: number | null;
  new_size: number | null;
}

export interface PackDiffKind {
  kind: string;
  added: number;
  removed: number;
  modified: number;
}

export interface PackDiff {
  added: number;
  removed: number;
  modified: number;
  unchanged: number;
  by_kind: PackDiffKind[];
  files: PackDiffEntry[];
  files_truncated: boolean;
}

export interface PendingReview {
  status: ProjectStatus;
  submitted_at: string | null;
  changelog: string;
  is_first_review: boolean;
  icon_changed: boolean;
  published: RevisionContent | null;
  pending: RevisionContent;
  owner: OwnerContext;
  links: ProjectLinks;
  facts: ProjectFacts;
  history: ReviewHistoryEntry[];
  gallery: GalleryItem[];
  versions: VersionItem[];
  analysis: AnalysisReport | null;
  pack_diff: PackDiff | null;
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
  pending_changelog: string;
  website_url: string;
  source_url: string;
  issues_url: string;
  wiki_url: string;
  discord_url: string;
  categories: CategoryTag[];
  review: ProjectReview | null;
  is_published: boolean;
  has_pending_changes: boolean;
  icon_changed: boolean;
  published: RevisionContent | null;
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
