---
name: standardize-cleanup
description: >-
    Refactor Beacon for consistency and de-duplication without changing behavior.
    USE WHEN asked to clean up, standardize, de-duplicate, centralize a helper,
    remove magic strings, or split a large "god file" into focused
    components/modules. Covers the find-duplication -> centralize -> behavior-
    preserving -> split -> run-gates playbook, the known shared homes for helpers
    on each side, the god-file splitting pattern (orchestrator + section
    components, types in a sibling types.ts), and the Vue defineModel lint trap.
    DO NOT USE FOR feature work, bug fixes, or new endpoints (see add-backend-route
    for those); this skill is strictly behavior-preserving refactoring.
---

# Standardize & clean up Beacon

This is a **behavior-preserving** refactor playbook. The app must look and behave
identically afterward — you are moving and de-duplicating code, not changing what
it does. Follow the Standardization rules in `AGENTS.md`; this skill is the
repeatable workflow for applying them.

## The playbook

1. **Find the duplication / god file.** Confirm the scope before editing — grep
   for the repeated logic or the magic string across both `apps/backend/` and
   `apps/frontend/`. Know every call site before you move anything.
2. **Centralize in the known shared home** (below) — do not invent a new
   abstraction or a new file when an established home exists.
3. **Replace every copy** with the shared version. Leave zero stragglers; a
   half-done de-dupe is worse than none.
4. **Keep it behavior-preserving.** Preserve exact markup, exact conditionals,
   exact SQL semantics. Do not "simplify" a condition into a similar-looking one
   while extracting — copy it verbatim.
5. **Run the gates** (`run-gates` skill) and report results before declaring
   done. Backend `CLIPPY=0`; frontend `TC=0`, `LINT=0 errors`, `FMT=0`.

## Known shared homes (reuse before you write)

- **Frontend**
    - `scripts/api.ts` — `useApi`, `apiErrorMessage`
    - `scripts/auth.ts` — `useAuth`
    - `scripts/formatters.ts` — `formatBytes`, `formatDate`, `relativeTime` (never
      re-implement a formatter inline)
    - `scripts/constants.ts` — status / role / action / channel / visibility
      constants (never retype the string literals)
    - Page logic: `scripts/pages/<feature>/...` composables; **types in a sibling
      `types.ts`**, not mixed into the composable file
- **Backend**
    - `routes/owner.rs` — `require_project_owner`
    - `routes/sql.rs` — `created_at_utc!` (always use the macro for the ISO
      timestamp cast; never re-type the `to_char(... at time zone 'utc', ...)`)
    - `utils.rs` — cross-cutting helpers (`hex_encode`, filename sanitizing,
      multipart field parsing). New cross-feature helpers go here, not in a route
      file.

## Splitting a god file (the proven pattern)

Treat ~250+ lines as the signal to split; aim for ~200-line focused files.

1. **Move logic out of the SFC first.** Page logic becomes an exported composable
   in `scripts/pages/<feature>/index.ts`; its types go in a sibling `types.ts`;
   static display maps / option lists / icon tables go in a `meta.ts`. The page
   keeps only orchestration (data loading, toast-wrapped handlers, the section
   switch).
2. **One section = one component**, mirrored under `app/components/<feature>/`.
   When a section itself is large, add an **orchestrator** component that composes
   smaller cards (e.g. `SettingsGeneral` -> `GeneralInfoCard` + `MonetizationCard`
    - `DangerZoneCard`). Component CSS mirrors the structure under
      `assets/css/components/`.
3. **Explicit imports.** `scripts/` is not auto-imported, and these section
   components are explicitly imported by the page.
4. **Preserve markup verbatim.** Copy each `v-if/v-else-if` section's template
   exactly. Re-verify lucide icon import lists after edits — they drop out of
   import blocks between edits.

### The `vue/no-mutating-props` trap (must avoid)

`vue/no-mutating-props` is an **error** (shallowOnly: false). A child must never
receive the reactive `form` as a prop and then `v-model` a field of it.

- **Form-bearing children** use `defineModel` **per field**; the page binds
  `v-model:field="form.field"`. `form` is a page-local const from the composable
  (not a page prop), so mutating it is safe.
- For ref-bearing upload forms the page binds `v-model:x="versionForm.x.value"`;
  the child uses `defineModel`. Pass `onFileChange` as a prop function (calling is
  fine) and `file` / `error` / `pending` as read-only props.

## Gotchas

- **`vue-tsc` is the only gate** that catches an undefined component used in a
  template — always run `typecheck` for `.vue` changes.
- **Magic strings:** status/role/action/channel/visibility values must come from
  the single declared constant set on each side. Replacing them is a pure
  de-dupe — the wire stays `snake_case`; convert at usage, not by renaming the
  payload.
- **sqlx runs SQL at runtime.** A bad cast compiles fine and 500s in prod — keep
  the timestamp cast going through `created_at_utc!`.
- **Do not commit, push, or deploy.** The user commits; CI deploys on push to
  `main`.
