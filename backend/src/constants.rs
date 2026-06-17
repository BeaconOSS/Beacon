//! Enumerated string values shared across routes.
//!
//! Single Rust-side source of truth for role, project status, review action,
//! visibility, channel, and project-type values. Values embedded directly in
//! SQL string literals stay inline because `concat!` cannot interpolate `const`
//! items.

pub const ROLE_MODERATOR: &str = "moderator";
pub const ROLE_ADMIN: &str = "admin";

pub const STATUS_DRAFT: &str = "draft";
pub const STATUS_IN_REVIEW: &str = "in_review";
pub const STATUS_CHANGES_REQUESTED: &str = "changes_requested";
pub const STATUS_APPROVED: &str = "approved";
pub const STATUS_REJECTED: &str = "rejected";

pub const REVIEW_ACTION_APPROVE: &str = "approve";
pub const REVIEW_ACTION_REJECT: &str = "reject";
pub const REVIEW_ACTION_REQUEST_CHANGES: &str = "request_changes";
pub const REVIEW_ACTIONS: [&str; 3] = [
    REVIEW_ACTION_APPROVE,
    REVIEW_ACTION_REJECT,
    REVIEW_ACTION_REQUEST_CHANGES,
];

pub const VISIBILITY_PUBLIC: &str = "public";
pub const VISIBILITY_UNLISTED: &str = "unlisted";
pub const VISIBILITY_PRIVATE: &str = "private";
pub const VISIBILITIES: [&str; 3] = [VISIBILITY_PUBLIC, VISIBILITY_UNLISTED, VISIBILITY_PRIVATE];

pub const CHANNEL_RELEASE: &str = "release";
pub const CHANNEL_BETA: &str = "beta";
pub const CHANNEL_ALPHA: &str = "alpha";
pub const VERSION_CHANNELS: [&str; 3] = [CHANNEL_RELEASE, CHANNEL_BETA, CHANNEL_ALPHA];

pub const PROJECT_TYPE_ADDON: &str = "addon";
pub const PROJECT_TYPE_WORLD: &str = "world";
pub const PROJECT_TYPE_RESOURCE_PACK: &str = "resource_pack";
pub const PROJECT_TYPE_SKIN_PACK: &str = "skin_pack";
pub const PROJECT_TYPES: [&str; 4] = [
    PROJECT_TYPE_ADDON,
    PROJECT_TYPE_WORLD,
    PROJECT_TYPE_RESOURCE_PACK,
    PROJECT_TYPE_SKIN_PACK,
];
