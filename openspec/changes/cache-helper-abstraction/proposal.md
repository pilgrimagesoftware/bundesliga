# app-cache-helper-abstraction

## Why

Three Tauri commands in `app/src-tauri/src/lib.rs` — `get_table`, `get_matchdays`, and `get_matches_for_matchday` — share an identical ~20-line cooldown/caching pattern:

1. Lock state, check cooldown, release lock
2. If on cooldown: lock state, fetch cached JSON, release lock, deserialise, return with `cached: true`
3. If not on cooldown: call the API, lock state, update cooldown + store response, release lock, return with `cached: false`

This duplication means any change to the caching strategy (e.g., adding logging, changing error handling, or switching to a different cache store) must be applied to three separate places. The pattern is also subtly tricky (two separate lock acquisitions per request when on cooldown), making each copy a potential site for divergence bugs.

## What Changes

- Introduce a generic async helper function `with_cache<T>` in `lib.rs` that encapsulates the full cooldown/cache pattern
- The helper accepts a cache key, cooldown duration, a reference to the `Mutex<AppState>`, and an async fetch closure
- Update `get_table`, `get_matchdays`, and `get_matches_for_matchday` to delegate to `with_cache`

## Capabilities

No behaviour change — pure refactor.

## Impact

- `app/src-tauri/src/lib.rs`: add `with_cache` helper; simplify the three affected command bodies
