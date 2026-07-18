## 1. Workspace Setup

- [ ] 1.1 Add `openligadb` and `strsim` to `[workspace.dependencies]` in the root `Cargo.toml`
- [ ] 1.2 Add `openligadb` and `strsim` to `crates/fulltime-ui/Cargo.toml`'s `[dependencies]`
- [ ] 1.3 Grow the main window bounds in `crates/fulltime-ui/src/ui/app/mod.rs::open_main_window` to 1200×800, resizable, min 960×640
- [ ] 1.4 Extend `ColorTokens` in `crates/fulltime-ui/src/data/theme.rs` with `zone_champions_league`, `zone_europa_league`, `zone_relegation`, `live_indicator` fields for both `pitch_colors` and `pitch_night_colors`

## 2. Data Layer — Models and OpenLigaDB Fetch

- [ ] 2.1 Create `crates/fulltime-ui/src/data/models.rs`: `League`, `Season`, `TableTeam`, `Group` (matchday), `Match`, `Goal` types (re-export or wrap the `openligadb` crate's equivalents as needed)
- [ ] 2.2 Create `crates/fulltime-ui/src/data/leagues.rs`: `fn available_leagues() -> Vec<League>`
- [ ] 2.3 Create `crates/fulltime-ui/src/data/seasons.rs`: `fn available_seasons() -> Vec<i32>` using `chrono::Local::now().year()` (current year and last 3)
- [ ] 2.4 Create `crates/fulltime-ui/src/data/table.rs`: `async fn fetch_table(league, season) -> Result<Vec<TableTeam>>` via `openligadb::TableTeam::get_bl_table`
- [ ] 2.5 Create `crates/fulltime-ui/src/data/matchdays.rs`: `async fn fetch_matchdays(league, season)`, `async fn fetch_current_matchday(league)` via `openligadb::Group`
- [ ] 2.6 Create `crates/fulltime-ui/src/data/matches.rs`: `async fn fetch_matches_for_matchday(league, season, group_order_id)`, `async fn fetch_match_detail(match_id)` via `openligadb::Match`
- [ ] 2.7 Create `crates/fulltime-ui/src/data/teams.rs`: `async fn fetch_teams(league, season)` via `openligadb::Team`

## 3. Navigation State

- [ ] 3.1 Define `NavScreen` enum in `crates/fulltime-ui/src/data/nav.rs`: `Table`, `Matches { matchday }`, `MatchDetail { match_id, from_matchday }`, `Teams`, `TeamDetail { team_id }`
- [ ] 3.2 Define `NavState` (current screen, current league, current season) as a GPUI global or a `RootView` field
- [ ] 3.3 Add a `navigate(screen: NavScreen)` method that updates `NavState` and triggers a re-render

## 4. Sidebar and Toolbar

- [ ] 4.1 Populate `crates/fulltime-ui/src/ui/views/sidebar.rs` with three nav items (Table, Matches, Teams), active-state highlight using `colors.accent`/`accent_soft`
- [ ] 4.2 Populate `crates/fulltime-ui/src/ui/views/toolbar.rs` with a league picker, season picker (placeholder select until `season-picker-ux` lands), a refresh control, and a live-match badge slot
- [ ] 4.3 Wire sidebar item clicks to `navigate(...)`

## 5. Table View

- [ ] 5.1 Create `crates/fulltime-ui/src/ui/views/table_view.rs`: fetch table on mount and on league/season change via `cx.spawn`
- [ ] 5.2 Render standings columns: #, logo, name, P, W, D, L, GF, GA, GD, Pts
- [ ] 5.3 Apply zone-accent left borders using the new `ColorTokens` fields (top 2 rows: Champions League; rows 3-4: Europa League; bottom 2: relegation)
- [ ] 5.4 Make each team row clickable — calls `navigate(NavScreen::TeamDetail { team_id })`
- [ ] 5.5 Replace `root_view.rs`'s placeholder toolbar child with the real screen router (Table/Matches/Teams/detail views based on `NavState`)

## 6. Matches Flow

- [ ] 6.1 Create `crates/fulltime-ui/src/ui/views/matches_view.rs`: accept current matchday, fetch matchday list and matches
- [ ] 6.2 Render matchday heading and prev/next navigation (disable at boundaries)
- [ ] 6.3 Render match cards: home/away logos + names, score, status label (kick-off time / FT / live minute)
- [ ] 6.4 Implement live detection: `!is_finished && now in [when_utc, when_utc + 2h]`
- [ ] 6.5 Add a 30-second auto-refresh timer (`cx.spawn` loop with `Timer::after`) while the Matches or Match Detail view is active; skip silently if within cooldown
- [ ] 6.6 Create `crates/fulltime-ui/src/ui/views/match_detail_view.rs`: fetch match detail on mount for `match_id`
- [ ] 6.7 Render match header: both team logos, names, score side-by-side
- [ ] 6.8 Render goal timeline: sorted by minute, running score, scorer name, PEN/OG badges; empty state when no goals
- [ ] 6.9 Render match metadata: venue, viewer count (formatted), match date/time
- [ ] 6.10 Back control calls `navigate(NavScreen::Matches { matchday: from_matchday })`

## 7. App State Persistence

- [ ] 7.1 Define `AppViewState` serde struct: `last_opened`, `league`, `season`, `nav` (serialized `NavScreen`), `selected_team_id` (Option)
- [ ] 7.2 Add a `data_dir()` helper alongside `platform_log_dir` in `fulltime-core` (or a new shared module) resolving `<app-data>/state`
- [ ] 7.3 Implement `load_last_viewed() -> Option<AppViewState>`: read `<data_dir>/app_state.json`
- [ ] 7.4 Implement `save_last_viewed(state: &AppViewState)`: write JSON to `<data_dir>/app_state.json`
- [ ] 7.5 On startup (`ui::app::setup`), load last-viewed state; if `last_opened` is within 48h, restore; else default to the current matchday of the default league
- [ ] 7.6 Persist on every navigation and league/season change

## 8. Teams Flow Baseline

- [ ] 8.1 Create `crates/fulltime-ui/src/ui/views/teams_view.rs`: fetch teams on mount; render grid of team cards
- [ ] 8.2 Team card click calls `navigate(NavScreen::TeamDetail { team_id })`
- [ ] 8.3 Create `crates/fulltime-ui/src/data/team_detail.rs`: `async fn fetch_team_detail_baseline(team_id, league, season)` using OpenLigaDB data only (identity, table row, recent matches)
- [ ] 8.4 Create `crates/fulltime-ui/src/ui/views/team_detail_view.rs`: fetch baseline team detail on mount
- [ ] 8.5 Render identity section: logo, name, founded year, stadium/capacity (hide if unavailable)
- [ ] 8.6 Render season stats row: P, W, D, L, GF, GA, GD, Pts, league position
- [ ] 8.7 Render recent matches: up to 5 finished matches + next upcoming match; opponent, date, score, H/A
- [ ] 8.8 Show a graceful "details unavailable" fallback when enrichment data is absent

## 9. Rate Limiting

- [ ] 9.1 Create `crates/fulltime-ui/src/data/cache.rs`: `DataCache` struct with `cooldown_tracker: HashMap<&'static str, Instant>` and `last_responses: HashMap<String, serde_json::Value>`, held as a GPUI global
- [ ] 9.2 Define a `Cached<T>` wrapper: `{ data: T, cached: bool, next_refresh_at: Option<Instant> }`
- [ ] 9.3 Add a `with_cache` helper (see `cache-helper-abstraction` change) and use it to wrap `fetch_table` (60s), `fetch_matches_for_matchday` (30s), `fetch_matchdays` (5m), `fetch_team_detail` (5m)

## 10. TheSportsDB Integration and Team Enrichment

- [ ] 10.1 Define `TheSportsDbTeam`, `TheSportsDbPlayer`, `TheSportsDbStaff` serde structs (see `staff-implementation` change for the staff field)
- [ ] 10.2 Expand the team detail model to include TheSportsDB fields + squad + staff
- [ ] 10.3 Implement `search_thesportsdb_team(name)` async fn: GET `searchteams.php?t=<name>`, return best match with Jaro-Winkler score via `strsim`
- [ ] 10.4 Implement `fetch_thesportsdb_players(tsdb_team_id)` async fn: GET `lookup_all_players.php?id=<id>`
- [ ] 10.5 Implement `read_team_cache`/`write_team_cache(team_id, app_data_dir)`: JSON file per team under `<data_dir>/team_cache/`, `cached_at` TTL check (30 days)
- [ ] 10.6 Upgrade `fetch_team_detail`: check cache → if miss, search TSDB → fetch players/staff → assemble enriched detail → write cache → return with source/fallback metadata
- [ ] 10.7 Render squad grouped by position (GK, DF, MF, FW): name, nationality, DOB/age
- [ ] 10.8 Render staff section: name and role list; hide section if no staff data
- [ ] 10.9 Show "Squad data unavailable" placeholder when TheSportsDB match failed

## 11. Refresh and Cooldown UX

- [ ] 11.1 Derive `is_on_cooldown`/`last_updated_label` from the active view's `Cached<T>` metadata in the toolbar
- [ ] 11.2 Disable the refresh control and show "last updated X ago" during cooldown
- [ ] 11.3 Tick the "X ago" label every second while on cooldown (`cx.spawn` loop with `Timer::after(Duration::from_secs(1))`)
- [ ] 11.4 Re-enable the refresh control when cooldown expires, no user action needed
