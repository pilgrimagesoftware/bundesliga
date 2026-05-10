# app-reduce-lock-contention

## Why

In `get_table`, `get_matchdays`, and `get_matches_for_matchday`, the `Mutex<AppState>` is locked twice in sequence when the response is already cached:

```Fussballergebnisse/app/src-tauri/src/lib.rs
let (on_cooldown, next_refresh_at) = {
    let s = state.lock().unwrap();   // lock #1: released here
    s.check_cooldown(...)
};
if on_cooldown {
    let cached = state.lock().unwrap()  // lock #2: separate acquisition
        .get_cached_response(...)
        .cloned();
```

Between the two lock acquisitions, another task could in principle mutate `AppState`. More practically, the two-lock pattern is inefficient and makes the control flow harder to reason about. Both pieces of information (cooldown status and cached value) should be read in a single lock scope.

## What Changes

- Merge the cooldown check and cached-value read into a single `AppState` lock scope that returns `(bool, Option<u64>, Option<serde_json::Value>)` — i.e., `(on_cooldown, next_refresh_at, maybe_cached_value)` — for the three affected commands

## Capabilities

No observable behaviour change — correctness and efficiency improvement only.

## Impact

- `app/src-tauri/src/lib.rs`: refactor the double-lock pattern in `get_table`, `get_matchdays`, and `get_matches_for_matchday` (or in the `with_cache` helper if change 14 is applied first)
