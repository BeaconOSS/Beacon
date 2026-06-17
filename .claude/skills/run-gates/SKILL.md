---
name: run-gates
description: >-
  Run Beacon's quality gates after editing code, on Windows PowerShell. USE WHEN
  you have changed Rust files under backend/ (run fmt + clippy) and/or Nuxt
  files under frontend/ (run prettier, typecheck, lint, format:check) and need
  to confirm everything passes before reporting done. Covers the PowerShell
  clippy-stderr workaround and the required command ordering. DO NOT USE FOR
  deploying (that is automated in CI) or for general "does this compile"
  questions unrelated to the gate sequence.
---

# Run the Beacon quality gates

Run the gates that match what you touched. Backend and frontend are
independent — only run the side you changed. All commands assume Windows
PowerShell run from the repository root (the folder containing `backend/`,
`frontend/`, and `docker-compose.yml`). Paths below are relative to that root —
do not hardcode an absolute path, since each clone lives somewhere different.

## Why the `cmd /c "... > log 2>&1"` pattern

`cargo clippy`, `npm run typecheck`, and `npm run lint` write progress to
**stderr**, which PowerShell surfaces as a failure even on success. Redirect to
a log file inside `cmd /c` and inspect `$LASTEXITCODE` (the real exit code)
plus the log contents, instead of trusting PowerShell's error stream.

## Backend gate (run when any `backend/**` Rust file changed)

Run from the repository root:

```powershell
Get-Process beacon-backend -ErrorAction SilentlyContinue | Stop-Process -Force
cargo fmt --manifest-path backend/Cargo.toml
cmd /c "cargo clippy --manifest-path backend/Cargo.toml --all-targets -- -D warnings > clippy.log 2>&1"
Write-Output "CLIPPY=$LASTEXITCODE"
```

- Pass condition: `CLIPPY=0`.
- Confirm the log ends cleanly:
  ```powershell
  Select-String -Path clippy.log -Pattern "Finished|error\[|warning:" | Select-Object -Last 3
  ```
  Expect a `Finished` line and **no** `error[` / `warning:` lines.
- Stop the running dev binary first (the `Stop-Process` line) so the build is
  not blocked by a locked executable.

## Frontend gate (run when any `frontend/**` Nuxt/Vue/TS file changed)

Run these **in order** — prettier first so formatting churn does not fail the
later checks. The first three run from `frontend/`; `format:check` runs from the
repository root.

```powershell
cd frontend
npx prettier --write "app/pages/.../changed.vue" "app/scripts/.../changed.ts"
cmd /c "npm run typecheck > tc.log 2>&1"; Write-Output "TC=$LASTEXITCODE"
cmd /c "npm run lint > lint.log 2>&1"; Write-Output "LINT=$LASTEXITCODE"
cd ..
cmd /c "npm run format:check > fmt.log 2>&1"; Write-Output "FMT=$LASTEXITCODE"
```

- Pass conditions: `TC=0`, `LINT=0` (0 **errors**; pre-existing warnings are
  acceptable), `FMT=0`.
- `npm run typecheck` (vue-tsc) is the **only** gate that catches
  undefined-components-in-template. The editor/language server does not. Always
  run it for `.vue` changes.
- Lint warnings: the `vue/html-self-closing` rule warns on every `<img />` /
  void element, but prettier forces the self-closing slash — so each new `<img>`
  permanently adds one warning. That is expected; only **errors** fail the gate.
- If lint reports an error, inspect it:
  ```powershell
  Select-String -Path lint.log -Pattern "error" -Context 0,2
  ```

## Reporting

State the gate results explicitly (e.g. `CLIPPY=0`, `TC=0`, `LINT=0 (0 errors)`,
`FMT=0`) when reporting a change done. If a gate fails, fix the cause and rerun
that gate — do not report done with a failing gate.
