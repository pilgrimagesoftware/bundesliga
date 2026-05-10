# Design: pin-reqwest-version

## Context

The app's `Cargo.toml` uses `reqwest = { version = "*" }`. The `openligadb` library dependency (which the app consumes) specifies `reqwest = { version = "0.13" }`. When both are resolved by Cargo, the wildcard in the app will resolve to the same version as the library (`0.13`), but this is incidental — not guaranteed. If the registry later receives `reqwest 1.0`, a `cargo update` in a fresh environment will upgrade the app's `reqwest` to `1.0` while the library stays at `0.13`, resulting in two different versions of `reqwest` in the same binary.

## Goals / Non-Goals

**Goals:**
- Make the `reqwest` version constraint explicit and intentional
- Ensure `cargo update` cannot silently introduce a breaking version change
- Align the app's `reqwest` version with the version used by the `openligadb` library

**Non-Goals:**
- Upgrading `reqwest` to a new major version (separate concern)

## Decisions

**Pin to `"0.12"` or `"0.13"`**: Inspect the `Cargo.lock` to determine the currently resolved version and pin to that major version series. If the library uses `0.13`, the app should also declare `"0.13"` to make the intent to share a single version explicit.

## Risks / Trade-offs

- None. This is a purely additive constraint that can only make the build more predictable.

## Open Questions

- Should the app use `reqwest` directly at all, or delegate all HTTP calls to the `openligadb` library? If the app's direct `reqwest` usage is only for the TheSportsDB integration, consider whether that belongs in a separate library crate. (Deferred decision.)
