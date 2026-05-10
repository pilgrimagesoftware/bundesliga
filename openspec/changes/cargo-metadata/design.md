# Design: cargo-metadata

## Context

The `authors` field in a Cargo manifest is surfaced in `cargo metadata` output, crate documentation, and audit tooling. Leaving it as `"you"` is a scaffold artefact. The `openligadb` version pin at `0.0.8` means the app doesn't receive any changes shipped in `0.0.9`.

## Goals / Non-Goals

**Goals:**
- Accurate crate metadata
- Latest patch version of the `openligadb` dependency

**Non-Goals:**
- Publishing this crate to crates.io (it's a Tauri app binary, not a library)

## Decisions

**Update both in the same commit**: They are both trivial metadata changes with no code impact.

## Risks / Trade-offs

- `openligadb 0.0.9` may introduce new APIs or breaking changes. Review the `openligadb` CHANGELOG before updating.

## Open Questions

- None.
