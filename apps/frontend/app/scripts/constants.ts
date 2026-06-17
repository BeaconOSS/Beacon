/**
 * Enumerated value constants shared across the app.
 *
 * Single source of truth for role / status / action / visibility / channel
 * values that cross the API boundary (snake_case on the wire). Union types are
 * derived from these objects so the runtime set and the type can never drift.
 */

export const USER_ROLE = {
  USER: "user",
  MODERATOR: "moderator",
  ADMIN: "admin",
} as const;
export type UserRole = (typeof USER_ROLE)[keyof typeof USER_ROLE];

/** Roles that grant moderator privileges. */
export const STAFF_ROLES: readonly UserRole[] = [
  USER_ROLE.MODERATOR,
  USER_ROLE.ADMIN,
];

export const PROJECT_STATUS = {
  DRAFT: "draft",
  IN_REVIEW: "in_review",
  CHANGES_REQUESTED: "changes_requested",
  APPROVED: "approved",
  REJECTED: "rejected",
} as const;
export type ProjectStatus =
  (typeof PROJECT_STATUS)[keyof typeof PROJECT_STATUS];

export const REVIEW_ACTION = {
  APPROVE: "approve",
  REJECT: "reject",
  REQUEST_CHANGES: "request_changes",
} as const;
export type ReviewAction = (typeof REVIEW_ACTION)[keyof typeof REVIEW_ACTION];

export const PROJECT_VISIBILITY = {
  PUBLIC: "public",
  UNLISTED: "unlisted",
  PRIVATE: "private",
} as const;
export type ProjectVisibility =
  (typeof PROJECT_VISIBILITY)[keyof typeof PROJECT_VISIBILITY];

export const VERSION_CHANNEL = {
  RELEASE: "release",
  BETA: "beta",
  ALPHA: "alpha",
} as const;
export type VersionChannel =
  (typeof VERSION_CHANNEL)[keyof typeof VERSION_CHANNEL];
