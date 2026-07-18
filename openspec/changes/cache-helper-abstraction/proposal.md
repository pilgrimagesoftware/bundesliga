# app-cache-helper-abstraction

## Why

`bundesliga-sports-ui`'s rate-limiting capability plans an identical cooldown/caching pattern across three data-layer fetches — table, matchdays, and matches-for-matchday (see that change's task 9). Writing that pattern three times invites the same problem the original Tauri implementation hit: any change to the caching strategy (e.g., adding logging, changing error handling, switching cache stores) would need to be applied in three separate places, and each copy is a potential site for divergence bugs. This change specifies a single `with_cache` helper for `bundesliga-sports-ui` to use from the start, rather than letting the duplication happen and refactoring it out later.

## What Changes

- Introduce a generic async helper function `with_cache<T>` in `crates/fulltime-ui/src/data/cache.rs` that encapsulates the full cooldown/cache pattern.
- The helper accepts a cache key, cooldown duration, a reference to the `DataCache` global (see `bundesliga-sports-ui` decision 7), and an async fetch closure.
- `bundesliga-sports-ui`'s table, matchdays, and matches-for-matchday fetches (task 9) are implemented in terms of `with_cache` from the start, not refactored into it afterward.

## Capabilities

No behaviour change — pure implementation pattern, folded into `bundesliga-sports-ui`'s rate-limiting capability rather than introducing a new one.

## Impact

- `crates/fulltime-ui/src/data/cache.rs`: add the `with_cache` helper.
- `bundesliga-sports-ui` task 9 (rate limiting): implemented against `with_cache` directly.
