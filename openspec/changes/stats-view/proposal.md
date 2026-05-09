## Why

The app currently surfaces raw standings and match results but offers no synthesized statistical perspective. A user wanting to understand a team's trajectory, evaluate the state of the title race, or spot historical highpoints within a season must mentally derive everything from the raw table and score data. This change adds a dedicated League Stats view for season-wide insights, and enhances the Team Detail view with per-team derived statistics, projection scenarios, and recognition of records and accomplishments — all computed from OpenLigaDB data already in the fetch graph.

## What Changes

- **Add a Stats nav item** to the sidebar (5th nav item, below Teams)
- **Add a League Stats view** (`StatsView.svelte`): aggregated, derived, and scenario statistics for the entire selected league season
- **Add a Team Statistics section** inside the existing Team Detail view: per-team derived stats, projection scenarios, and records — no new top-level screen
- **Add `get_league_stats(league, season)` Rust command**: fetches all season matches once, computes aggregated stats, caches on disk (30-minute TTL) and in-memory (5-minute cooldown)
- **Extend `AppView` type** with `{ screen: 'stats' }` and handle in view persistence
- **Add `LeagueStats.ts` TypeScript type** and related nested types
- No new external API integrations; all stats are derived from OpenLigaDB data

## Capabilities

### New Capabilities

- `league-stats-view`: League-level aggregated, derived, and scenario statistics in a dedicated nav-level view
- `team-stats-view`: Per-team derived statistics, projection scenarios, and records/accomplishments within the team detail view

### Modified Capabilities

- `navigation`: Sidebar gains a 5th "Stats" nav item; `AppView` type and `AppViewState` persistence extended with the `'stats'` screen
- `teams-view`: Team Detail view gains a collapsible "Statistics" section

## Impact

- `src-tauri/src/lib.rs`: Add `FormResult`, `TeamCleanSheets`, `MatchSummary`, `TeamForm`, `LeagueStats` structs; implement `compute_league_stats` fn; add `get_league_stats` command with disk+memory cache; register in `generate_handler![]`
- `src/types/LeagueStats.ts`: New TypeScript types mirroring Rust structs (`LeagueStats`, `TeamCleanSheets`, `MatchSummary`, `TeamForm`, `FormResult`)
- `src/lib/views/StatsView.svelte`: New view component for the League Stats screen
- `src/lib/views/TeamDetailView.svelte`: Add frontend-computed "Statistics" section (home/away split, clean sheets, PPG, form strip, projection scenarios, records)
- `src/lib/stores/view.svelte.ts`: Add `{ screen: 'stats' }` variant to `AppView` union
- `src/lib/components/Sidebar.svelte`: Add Stats nav item and active-state mapping
- `src/routes/+page.svelte`: Wire `StatsView` into content router; add `statsViewRef` for refresh delegation; handle `'stats'` in startup restore
