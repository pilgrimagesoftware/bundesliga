## Why

The app currently surfaces raw standings and match results but offers no synthesized statistical perspective. A user wanting to understand a team's trajectory, evaluate the state of the title race, or spot historical highpoints within a season must mentally derive everything from the raw table and score data. This change adds a dedicated League Stats view for season-wide insights, and enhances the Team Detail view with per-team derived statistics, projection scenarios, and recognition of records and accomplishments — all computed from OpenLigaDB data already in the fetch graph (see `bundesliga-sports-ui`).

## What Changes

- **Add a Stats nav item** to the sidebar (5th nav item, below Teams).
- **Add a League Stats view**: aggregated, derived, and scenario statistics for the entire selected league season.
- **Add a Team Statistics section** inside the existing Team Detail view: per-team derived stats, projection scenarios, and records — no new top-level screen.
- **Add a `fetch_league_stats(league, season)` data-layer function**: fetches all season matches once, computes aggregated stats, caches on disk (30-minute TTL) and in-memory (5-minute cooldown), matching the `bundesliga-sports-ui` cache pattern.
- **Extend `NavScreen`** with a `Stats` variant and handle it in view persistence.
- **Add `LeagueStats` Rust types** and related nested types in `fulltime-ui`'s data layer.
- No new external API integrations; all stats are derived from OpenLigaDB data.

## Capabilities

### New Capabilities

- `league-stats-view`: League-level aggregated, derived, and scenario statistics in a dedicated nav-level view.
- `team-stats-view`: Per-team derived statistics, projection scenarios, and records/accomplishments within the team detail view.

### Modified Capabilities

- `navigation`: Sidebar gains a 5th "Stats" nav item; `NavScreen` and `AppViewState` persistence extended with the `Stats` screen.
- `teams-view`: Team Detail view gains a collapsible "Statistics" section.

## Impact

- `crates/fulltime-ui/src/data/stats.rs` (new): `FormResult`, `TeamCleanSheets`, `MatchSummary`, `TeamForm`, `LeagueStats` structs; `compute_league_stats` pure function; `fetch_league_stats` with disk+memory cache.
- `crates/fulltime-ui/src/ui/views/stats_view.rs` (new): League Stats screen.
- `crates/fulltime-ui/src/ui/views/team_detail_view.rs`: Add a computed "Statistics" section (home/away split, clean sheets, PPG, form strip, projection scenarios, records).
- `crates/fulltime-ui/src/data/nav.rs`: Add `Stats` variant to `NavScreen` (see `bundesliga-sports-ui`).
- `crates/fulltime-ui/src/ui/views/sidebar.rs`: Add Stats nav item and active-state mapping.
- `crates/fulltime-ui/src/ui/views/root_view.rs`: Wire the Stats view into the screen router; handle `Stats` in startup restore.

## Follow-up: superseded by `ui-skeleton`

The `ui-skeleton` change (implemented) removed `crates/fulltime-ui/src/ui/views/sidebar.rs` and
`toolbar.rs`, replacing them with a single persistent header (`header.rs`) and per-screen content
views. This proposal's references to `sidebar.rs`/`toolbar.rs` above are stale and need revision
against the new header-based shell (`AppScreen` enum, `header.rs`, `views/components/`) before
implementation starts.
