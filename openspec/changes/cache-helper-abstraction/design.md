# cache-helper-abstraction — Design

## Context

`bundesliga-sports-ui` plans a `DataCache` GPUI global holding a cooldown tracker and last-response cache (decision 7 in that change's design). Writing the cooldown-check-then-fetch-then-store sequence inline at each of the three call sites (table, matchdays, matches-for-matchday) would repeat ~20 lines of boilerplate per site. This change specifies a single helper those three call sites use from the start.

Unlike the original Tauri implementation (`Mutex<AppState>`, accessed from multiple async command invocations that could race), GPUI globals are only ever mutated from the main thread via `cx.update_global`/`cx.global`, serialized through GPUI's single foreground executor. There is no `Mutex` and no double-lock-acquisition hazard to design around — the `reduce-lock-contention` concern from the original design (discarded as an openspec change, since it targeted dead Tauri code) doesn't have an equivalent in the GPUI architecture.

## Goals / Non-Goals

**Goals:**

- One authoritative implementation of the cooldown/cache pattern.
- Make future changes to caching behavior a single-site edit.
- Keep the pattern's lazy-fetch semantics: the fetch closure only runs when not on cooldown.

**Non-Goals:**

- Changing the caching strategy itself (in-memory `HashMap`, JSON-serializable cached values).
- Lock-contention mitigation — not applicable under GPUI's single-threaded global access model.

## Decisions

**Async helper over a `DataCache` global reference**: The helper signature is roughly:

```rust
async fn with_cache<T>(
    cx: &mut AsyncApp,
    cache_key: &str,
    cooldown: Duration,
    fetch: impl Future<Output = Result<T, FetchError>>,
) -> Result<Cached<T>, FetchError>
where
    T: Clone + Serialize + DeserializeOwned,
```

The `fetch` future is only awaited when not on cooldown, keeping lazy-fetch semantics. Cooldown check and cache read/update happen via `cx.update_global::<DataCache, _>(...)` calls, each a single, non-overlapping access — no separate lock-acquisition steps are needed the way the original `Mutex<AppState>` design required.

**Inline `with_cache` in `crates/fulltime-ui/src/data/cache.rs`**: The helper lives alongside the `DataCache` struct it operates on (see `bundesliga-sports-ui` decision 7), rather than in a separate module — the two are tightly coupled and small enough to keep together.

## Risks / Trade-offs

- The generic form requires `T: Clone + Serialize + DeserializeOwned` since the cache stores typed values (or `serde_json::Value`, mirroring the original design) and must return an owned clone to the caller without holding a global borrow across the `.await` point.

## Open Questions

- Should the team-detail fetch (which also checks an on-disk cache, see `bundesliga-sports-ui` decision 6) also use `with_cache`, or does its two-tier disk+memory cache warrant a separate helper? Its logic differs enough (disk TTL vs. in-memory cooldown) that it may be adapted to use `with_cache` for the in-memory tier only, once the core helper exists.
