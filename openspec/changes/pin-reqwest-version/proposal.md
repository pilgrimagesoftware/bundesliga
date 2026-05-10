# Proposal: app-pin-reqwest-version

## Why

`app/src-tauri/Cargo.toml` declares `reqwest = { version = "*", features = ["json"] }`. Using a wildcard version (`*`) means `cargo update` can silently upgrade `reqwest` to any future release, including one with breaking changes. `reqwest` has a history of significant breaking changes between major versions (e.g., 0.11 → 0.12 changed the body API; 0.12 → 0.13 changed async execution requirements). A wildcard version makes the build non-reproducible across environments that have different cached registry states.

## What Changes

- Replace `reqwest = { version = "*", features = ["json"] }` with a pinned major version `reqwest = { version = "0.12", features = ["json"] }` (matching the version pulled transitively by the `openligadb` library, which uses `0.13` — see note below)

## Capabilities

No behaviour change — dependency version pinning only.

## Impact

- `app/src-tauri/Cargo.toml`: update `reqwest` version specifier from `"*"` to `"0.12"` (or `"0.13"` to align with the `openligadb` library crate)
