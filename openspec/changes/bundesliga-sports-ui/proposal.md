## Why

The app is being rebuilt from a broken SvelteKit + Tauri proof-of-concept into a native GPUI desktop app (`crates/fulltime-core` + `crates/fulltime-ui`, see the GPUI scaffolding change already landed). The GPUI shell currently opens an empty themed window with no navigation, no data, and no screens. This change defines and builds the actual sports app: standings table, matchday browsing, match detail, and team browsing/detail — functional, fast, and visually consistent with how Bundesliga content is typically presented.

## What Changes

- All league/season/matchday data fetching becomes a single in-process async data layer in `fulltime-ui` — no IPC boundary, no frontend/backend split, since GPUI has no separate frontend process.
- Add sidebar navigation: Table | Matches | Teams.
- Add a toolbar with league and season pickers, a refresh control, and a live-match indicator.
- Add a Table view: league standings with promotion/European/relegation zone accents.
- Add a Matches view: matchday list with prev/next navigation, match cards with scores.
- Add a Match Detail view: goal timeline with scorer, minute, penalty/OG flags.
- Add a Teams view: team grid with click-through to team detail.
- Add a Team Detail view: baseline table position, season stats, and recent matches from OpenLigaDB, with squad/staff enrichment when TheSportsDB data is available.
- Integrate TheSportsDB for squad and staff data (team detail only), with local JSON cache (30-day TTL) under the platform app-data directory already established by `fulltime-core::logging::platform_log_dir` (`com.pilgrimagesoftware.fulltime`).
- Add rate limiting with per-category cooldowns; disable the refresh control during cooldown.
- Add persistent last-viewed state: restore on open if less than 2 days elapsed, else default to the current matchday.
- Derive the season list from the current year (current + last 3); no OpenLigaDB endpoint exists for this.

## Capabilities

### New Capabilities

- `navigation`: Sidebar nav + in-window view state machine; toolbar with league/season pickers.
- `table-view`: League standings table with visual styling.
- `matches-view`: Matchday browsing with match cards and prev/next navigation.
- `match-detail`: Goal timeline drill-down for a single match.
- `teams-view`: Team grid and team detail (squad, staff, stats, recent matches).
- `team-data-cache`: TheSportsDB integration with local JSON cache and TTL invalidation.
- `app-state-persistence`: Last-viewed state saved to disk; restore-or-default-to-current logic.
- `rate-limiting`: Per-category refresh cooldowns; UI feedback on cooldown state.

### Modified Capabilities

*(none — no existing specs)*

## Impact

- `crates/fulltime-ui/src/data/`: New modules for league/season/table/match/team data models and the async fetch+cache layer (OpenLigaDB via the `openligadb` crate, TheSportsDB via `reqwest`).
- `crates/fulltime-ui/src/ui/views/`: `table_view.rs`, `matches_view.rs`, `match_detail_view.rs`, `teams_view.rs`, `team_detail_view.rs` — replace the placeholder content in `root_view.rs`'s main-content area.
- `crates/fulltime-ui/src/ui/views/sidebar.rs`, `toolbar.rs`: Populate with real nav items and league/season pickers, replacing the current empty placeholders.
- `crates/fulltime-ui/src/ui/app/mod.rs`: `open_main_window`'s bounds grow to 1200×800 (resizable, min 960×640), matching the original Tauri window-size intent.
- `crates/fulltime-core/src/app/mod.rs`: No service-factory wiring needed (unlike Libri) — OpenLigaDB/TheSportsDB access lives entirely in `fulltime-ui`.
- `Cargo.toml` (workspace): Add `openligadb`, `strsim` (fuzzy name matching) to `[workspace.dependencies]`.

## Follow-up: superseded by `ui-skeleton`

The `ui-skeleton` change (implemented) removed `crates/fulltime-ui/src/ui/views/sidebar.rs` and
`toolbar.rs`, replacing them with a single persistent header (`header.rs`) and per-screen content
views. This proposal's references to `sidebar.rs`/`toolbar.rs` above are stale and need revision
against the new header-based shell (`AppScreen` enum, `header.rs`, `views/components/`) before
implementation starts.
