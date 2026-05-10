# cache-helper-abstraction — Design

## Context

The three commands each lock `AppState` twice when serving a cached response: once to check the cooldown and once to read the cached JSON value. The boilerplate is copy-pasted verbatim and is approximately 20 lines per command. Any future change to caching semantics must be applied three times consistently.

## Goals / Non-Goals

**Goals:**

- Eliminate the repeated boilerplate with a single authoritative implementation
- Make future changes to caching behaviour a single-site edit
- Reduce the risk of one copy diverging from the others

**Non-Goals:**

- Changing the caching strategy itself (in-memory HashMap, serde_json values, etc.)
- Addressing the double-locking issue (covered by `app-reduce-lock-contention`)

## Decisions

**Generic async closure helper**: The helper signature is roughly:

```Fussballergebnisse/app/src-tauri/src/lib.rs
async fn with_cache<T>(
    state: &Mutex<AppState>,
    cache_key: &str,
    cooldown_secs: u64,
    fetch: impl Future<Output = Result<T, String>>,
) -> Result<CachedResponse<T>, String>
where
    T: Serialize + DeserializeOwned,
```

The `fetch` closure is only awaited when not on cooldown, keeping the lazy-fetch semantics of the current per-command implementations.

**Inline `with_cache` in the same file**: The helper doesn't warrant a separate module. A well-placed section comment is sufficient.

## Risks / Trade-offs

- The generic closure form requires `T: Serialize + DeserializeOwned` because the cache stores and retrieves `serde_json::Value`. This bound was already implicitly required by the current code; making it explicit is an improvement.

## Open Questions

- Should `get_team_detail` also be refactored to use `with_cache`? Its logic is slightly different (it also checks the on-disk team cache). It can be adapted later once the core helper exists.
