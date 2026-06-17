import type { Component } from "vue";
import type { ProjectVisibility } from "../types";

export interface SettingsForm {
  title: string;
  urlSlug: string;
  summary: string;
  description: string;
  visibility: ProjectVisibility;
  license: string;
  monetizationEnabled: boolean;
  creatorShare: number;
  websiteUrl: string;
  sourceUrl: string;
  issuesUrl: string;
  wikiUrl: string;
  discordUrl: string;
}

export type SectionId =
  | "publish"
  | "general"
  | "tags"
  | "description"
  | "versions"
  | "license"
  | "gallery"
  | "links"
  | "members"
  | "analytics";

export interface NavItem {
  id: SectionId;
  label: string;
  icon: Component;
}

export type ChecklistLevel = "required" | "warning" | "suggestion";

export interface ChecklistItem {
  level: ChecklistLevel;
  title: string;
  description: string;
  complete: boolean;
}

export interface LevelStyle {
  icon: Component;
  label: string;
  tone: string;
  pill: string;
  accent: string;
}

export interface StatusBanner {
  label: string;
  description: string;
  icon: Component;
  pill: string;
  card: string;
  iconTone: string;
  showNotes: boolean;
}

export interface MarkdownAction {
  icon: Component;
  label: string;
  before: string;
  after?: string;
  placeholder?: string;
  block?: boolean;
}

export interface VisibilityOption {
  value: ProjectVisibility;
  label: string;
  description: string;
  icon: Component;
}

export interface SharePreset {
  value: number;
  label: string;
}

export interface LicenseOption {
  value: string;
  label: string;
}

export interface PendingChangeRow {
  label: string;
  before: string;
  after: string;
  long: boolean;
}
