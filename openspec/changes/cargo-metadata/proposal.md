# Proposal: app-cargo-metadata

## Why

`app/src-tauri/Cargo.toml` has two placeholder values that should be resolved before the crate is considered production-ready:

1. `authors = ["you"]` — the default Tauri scaffold placeholder; no actual author is listed
2. `openligadb = "0.0.8"` — the `openligadb` library crate is at version `0.0.9` (its own `Cargo.toml` declares `version = "0.0.9"`); the app is pinned one patch behind and will not receive the latest fixes

## What Changes

- Update `authors` to the actual author name and email
- Update `openligadb = "0.0.8"` to `openligadb = "0.0.9"`

## Capabilities

No behaviour change — metadata and dependency version update only.

## Impact

- `app/src-tauri/Cargo.toml`: update `authors` and `openligadb` version fields
