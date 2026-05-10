# app-reduce-lock-contention — Tasks

1. [ ] 15.1 Add a method `fn check_cooldown_and_get_cache(&self, cache_key: &str, min_secs: u64) -> (bool, Option<u64>, Option<serde_json::Value>)` to `AppState` in `app/src-tauri/src/lib.rs`
2. [ ] 15.2 Implement the method: call `check_cooldown` internally and immediately call `get_cached_response` to return both results from the same borrow of `self`
3. [ ] 15.3 Update `get_table` to call the new combined method and remove the second `.lock()` acquisition
4. [ ] 15.4 Update `get_matchdays` similarly
5. [ ] 15.5 Update `get_matches_for_matchday` similarly
6. [ ] 15.6 Run `cargo build` to confirm no compilation errors
7. [ ] 15.7 Note: if `app-cache-helper-abstraction` (change 14) is implemented first, apply this fix inside `with_cache` instead of the three individual commands
