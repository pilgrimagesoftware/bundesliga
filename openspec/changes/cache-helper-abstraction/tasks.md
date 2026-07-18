# app-cache-helper-abstraction — Tasks

1. [ ] 1.1 Define `async fn with_cache<T>(cx, cache_key, cooldown, fetch) -> Result<Cached<T>, FetchError>` in `crates/fulltime-ui/src/data/cache.rs`, where `T: Clone + Serialize + DeserializeOwned`.
2. [ ] 1.2 Implement the helper body: check cooldown via `DataCache`, serve the cached value if on cooldown, else await `fetch`, update the cooldown and stored response, return the fresh data.
3. [ ] 1.3 Implement `bundesliga-sports-ui` task 9's table, matchdays, and matches-for-matchday fetches directly against `with_cache` (no separate inline implementation to later refactor).
4. [ ] 1.4 Run `cargo build --workspace` to confirm the fetches compile against the helper.
5. [ ] 1.5 Add unit tests for `with_cache`: fresh fetch on first call, cached response within cooldown, fresh fetch again after cooldown expires.
