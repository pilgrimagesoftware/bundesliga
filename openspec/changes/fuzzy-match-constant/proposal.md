# app-fuzzy-match-constant

## Why

`search_thesportsdb_team` in `app/src-tauri/src/lib.rs` uses a hard-coded `0.75` threshold to accept or reject a fuzzy-matched team name:

```Fussballergebnisse/app/src-tauri/src/lib.rs
best.and_then(|(score, team)| if score > 0.75 { Some(team) } else { None })
```

This magic number has no accompanying explanation. A reader cannot tell why `0.75` was chosen over `0.70` or `0.80`, whether it was ever tuned, or what the consequences of changing it are. Moving it to a named constant with a doc comment communicates intent and makes future tuning a single-site change.

## What Changes

- Define `const MIN_TEAM_NAME_MATCH_SCORE: f64 = 0.75;` near the top of the TheSportsDB helpers section in `lib.rs`
- Add a brief inline comment explaining the tuning rationale (e.g., "empirically chosen to accept near-matches like 'Bayern Munich' <-> 'FC Bayern Munchen' while rejecting false positives")
- Replace the inline `0.75` literal with the constant

## Capabilities

No behaviour change — value is identical.

## Impact

- `app/src-tauri/src/lib.rs`: add one constant and replace one literal
