---
name: add-backend-route
description: >-
    Add a new HTTP route or endpoint to the Beacon Rust/axum backend. USE WHEN
    creating a new API handler, adding an endpoint to an existing feature module,
    or scaffolding a new per-feature route module under apps/backend/src/routes/.
    Covers the module pattern (pub fn routes() -> Router<AppState>, private
    handlers), registration in routes/mod.rs, the sqlx-runs-SQL-at-runtime trap,
    timestamp handling, and the created_at_utc! macro. DO NOT USE FOR frontend
    pages, database migrations (see add-migration if present), or non-HTTP backend
    changes.
---

# Add a backend route to Beacon

The backend is axum + sqlx (PostgreSQL). Routes are split into per-feature
modules under `apps/backend/src/routes/`. Each module owns its handlers, structs, and
SQL, and exposes a single `pub fn routes() -> Router<AppState>`.

## Where the code goes

- **Extending an existing feature** (e.g. another projects endpoint): add the
  handler to that feature's module/directory and add a `.route(...)` line to its
  `routes()` function. Larger features are directories (`projects/`, `auth/`,
  `versions/`, `gallery/`) with a `mod.rs` that declares submodules and builds
  the router; smaller ones are a single file (`categories.rs`, `health.rs`).
- **A brand-new feature**: create `apps/backend/src/routes/<feature>.rs` (or a
  directory with `mod.rs`), then register it in `apps/backend/src/routes/mod.rs` —
  add `mod <feature>;` and `.merge(<feature>::routes())` in `router(...)`.

## Module skeleton

```rust
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Router, extract::State, http::StatusCode};

use crate::error::AppError;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/things/{slug}", get(get_thing))
}

async fn get_thing(
    State(pool): State<sqlx::PgPool>,
    // ModeratorUser / SessionUser extractor here if the route needs auth
) -> Result<Response, AppError> {
    // ...
}
```

- Keep handlers **private** (`async fn`, not `pub`). Only `routes()` is public.
- `AppError` methods take `impl Into<String>`. Return `Result<Response, AppError>`
  and map branches with `.into_response()`.
- Auth: gate handlers with the existing extractors (`SessionUser`, or
  `ModeratorUser(SessionUser)` for moderator/admin) rather than re-checking
  cookies by hand.
- Path params use axum 0.8 brace syntax: `/projects/{slug}/versions/{version}`.

## sqlx gotchas (the expensive ones)

- **sqlx runs the SQL at runtime.** A bad `::cast`, a typo, or a wrong column
  compiles fine and only fails as a 500 in production. The masked client error
  is generic (`{"error":"something went wrong"}`) — the real classification is
  in `docker logs beacon-backend-1`. Read your SQL carefully.
- **`sqlx::query()` requires `&'static str`.** You cannot pass a `format!`
  string (it triggers an `AssertSqlSafe` injection-audit error). Build dynamic
  SQL with compile-time `concat!`, and keep user input in `.bind(...)`
  parameters (`$1`, `$2`), never interpolated.
- **Timestamps: there is no sqlx timestamp decoder configured.** Never select a
  `timestamptz` column directly into Rust. Cast it to an ISO string in SQL using
  the shared macro:
  `rust
    sqlx::query(concat!(
        "select id::text as id, ",
        crate::routes::sql::created_at_utc!("t.created_at"),
        " from things t where t.slug = $1",
    ))
    `
  `created_at_utc!("col")` expands to a `to_char(... at time zone 'utc', ...) as
created_at` fragment. It must receive a string **literal**. If more columns
  follow the macro in the select list, remember the comma placement.
- Cast UUIDs and numerics to text in SQL (`id::text`) unless you have a decoder
  for the concrete type.

## After editing

Run the backend gate (see the `run-gates` skill): `cargo fmt` then
`cargo clippy --all-targets -- -D warnings` must be clean. Because migrations and
SQL are validated at runtime, also smoke-test the endpoint against the running
dev server when practical — clippy cannot catch a bad query.

Do not commit, push, or deploy — the user handles commits and CI auto-deploys on
push to `main`.
