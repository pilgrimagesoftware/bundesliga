# Design: reduce-lock-contention

## Context

`std::sync::Mutex` in Rust is not reentrant. Each `.lock().unwrap()` call is a new lock acquisition. Holding two separate short-lived locks in sequence — rather than one lock covering both reads — means there is a window between the two acquisitions during which another thread or async task could call `update_cooldown` or `store_response`. In practice this race is benign (the worst outcome is re-fetching data unnecessarily), but it is an unnecessary complexity.

## Goals / Non-Goals

**Goals:**

- Read cooldown status and cached value in a single atomic lock acquisition
- Simplify the control flow in each affected command
- Remove the window for a benign but unnecessary TOCTOU race

**Non-Goals:**

- Switching from `std::sync::Mutex` to `tokio::sync::Mutex` (a larger change)
- Changing the cache storage format

## Decisions

**Return a tuple from a single lock scope**: A small helper method on `AppState` — `fn check_cooldown_with_cache(&self, key, min_secs) -> (bool, Option<u64>, Option<serde_json::Value>)` — reads both pieces of data while holding a single lock, then returns them to the caller.

**Apply before or alongside `app-cache-helper-abstraction`**: If change 14 is implemented first, this fix belongs inside the `with_cache` helper. Otherwise, apply it directly to the three command bodies.

## Risks / Trade-offs

- The return type is slightly more complex (a 3-tuple). Named return struct or destructuring with comments should make it clear.

## Open Questions

- None.
