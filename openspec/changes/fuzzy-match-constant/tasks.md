# app-fuzzy-match-constant — Tasks

1. [ ] 18.1 Add `const MIN_TEAM_NAME_MATCH_SCORE: f64 = 0.75;` in the TheSportsDB helpers section of `app/src-tauri/src/lib.rs`
2. [ ] 18.2 Add an inline comment explaining the threshold: e.g., `// Jaro-Winkler threshold; accepts near-matches like "Bayern Munich" <-> "FC Bayern Munchen" while rejecting false positives`
3. [ ] 18.3 Replace the inline `0.75` literal in `search_thesportsdb_team` with `MIN_TEAM_NAME_MATCH_SCORE`
4. [ ] 18.4 Run `cargo build` to confirm no compilation errors
