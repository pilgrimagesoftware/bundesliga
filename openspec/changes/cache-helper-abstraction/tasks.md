# app-cache-helper-abstraction — Tasks

1. [ ] 14.1 Define `async fn with_cache<T>(state, cache_key, cooldown_secs, fetch_fn) -> Result<CachedResponse<T>, String>` in `app/src-tauri/src/lib.rs` where `T: Serialize + DeserializeOwned`
2. [ ] 14.2 Implement the helper body: check cooldown (single lock), serve cached value if on cooldown, else call fetch_fn, update cooldown and store response (single lock), return fresh data
3. [ ] 14.3 Refactor `get_table` to use `with_cache`
4. [ ] 14.4 Refactor `get_matchdays` to use `with_cache`
5. [ ] 14.5 Refactor `get_matches_for_matchday` to use `with_cache`
6. [ ] 14.6 Run `cargo build` in `app/src-tauri/` to confirm the refactored commands still compile
7. [ ] 14.7 Manually test each command from the frontend to confirm cached and uncached responses both return correctly
