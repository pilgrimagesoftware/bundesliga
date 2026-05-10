# app-path-over-pathbuf

## Why

`read_team_cache` and `write_team_cache` in `app/src-tauri/src/lib.rs` both take `app_data_dir: &PathBuf` as a parameter. The idiomatic Rust convention is for functions that only read from (or traverse) a path to accept `&Path` rather than `&PathBuf`. `&PathBuf` coerces automatically to `&Path`, so changing the parameter type is backward-compatible for all callers. The `&Path` type is more general: it accepts string slices, `PathBuf` references, and other path-like types, making the functions more reusable.

## What Changes

- Change the `app_data_dir: &PathBuf` parameter to `app_data_dir: &Path` in both `read_team_cache` and `write_team_cache`
- Add `use std::path::Path;` import if not already present

## Capabilities

No behaviour change — signature generality improvement only.

## Impact

- `app/src-tauri/src/lib.rs`: update two function signatures and imports; no call-site changes required (coercion is automatic)
