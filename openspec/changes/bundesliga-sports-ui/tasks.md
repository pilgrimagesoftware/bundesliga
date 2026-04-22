## 1. Project Setup

- [ ] 1.1 Install Tailwind v4: `pnpm add -D tailwindcss @tailwindcss/vite`
- [ ] 1.2 Add `@tailwindcss/vite` plugin to `vite.config.js`
- [ ] 1.3 Create `src/app.css` with `@import "tailwindcss"` and `@theme` block (dark bg, bundesliga red, surface colors, border color)
- [ ] 1.4 Import `app.css` in `src/routes/+layout.svelte` (create if needed)
- [ ] 1.5 Update `tauri.conf.json`: window width 1200, height 800, minWidth 960, minHeight 640, resizable true
- [ ] 1.6 Add Rust dependencies to `src-tauri/Cargo.toml`: `chrono` (with serde feature), `strsim`

## 2. Rust Backend — Core Fixes and Stateless Commands

- [ ] 2.1 Remove `BundesligaState` struct and all references to it
- [ ] 2.2 Create `AppState` struct with `cooldown_tracker: HashMap<&'static str, Instant>` and `app_data_dir: PathBuf`; register with `tauri::Builder::manage()`
- [ ] 2.3 Fix `get_leagues`: remove unused state param
- [ ] 2.4 Fix `get_seasons`: implement using `chrono::Local::now().year()` to return current year and last 3
- [ ] 2.5 Fix `get_table`: accept `league: String, season: i32` params; remove state dependency
- [ ] 2.6 Add `get_matchdays(league, season)` command using `Group::available()`
- [ ] 2.7 Add `get_current_matchday(league)` command using `Group::current()`
- [ ] 2.8 Add `get_matches_for_matchday(league, season, group_order_id)` command using `Match::by_league_group()`
- [ ] 2.9 Add `get_match_detail(match_id)` command using `Match::get()`
- [ ] 2.10 Add `get_teams(league, season)` command using `Team::available()`
- [ ] 2.11 Add `get_top_scorers(league, season)` command using `GoalGetter::list()`
- [ ] 2.12 Register all new commands in `tauri::generate_handler![]`

## 3. TypeScript Types and Frontend State Model

- [ ] 3.1 Add `Match.ts` type: id, when_utc, is_finished, team1, team2, results, goals, location, group, number_of_viewers
- [ ] 3.2 Add `Goal.ts` type: id, score_team1, score_team2, match_minute, goal_getter_name, is_penalty, is_own_goal, is_overtime
- [ ] 3.3 Add `Group.ts` type: id, name, order_id
- [ ] 3.4 Add `Player.ts` type: name, position, nationality, date_of_birth, photo_url
- [ ] 3.5 Add `TeamDetail.ts` type: id, name, short_name, icon_url, founded, stadium, capacity, description, squad (Player[]), staff (Staff[])
- [ ] 3.6 Add `AppViewState.ts` type mirroring the Rust struct
- [ ] 3.7 Add `CachedResponse<T>.ts` generic wrapper type
- [ ] 3.8 Add frontend `AppView` type for the in-page state machine
- [ ] 3.9 Update `TableTeam.ts` with any missing fields (goals, opponentGoals already present — verify)
- [ ] 3.10 Create `src/lib/stores/view.svelte.ts`: export `$state` view object of type `AppView`; export `navigate(view)` helper
- [ ] 3.11 Create `src/lib/stores/context.svelte.ts`: export `$state` for `league`, `season`; export `setLeague`, `setSeason` helpers

## 4. Frontend — App Shell and Table Vertical Slice

- [ ] 4.1 Rewrite `src/routes/+page.svelte`: app shell with sidebar + header + content pane layout; no data fetching here
- [ ] 4.2 Create `src/lib/components/Sidebar.svelte`: three nav items (Table, Matches, Teams) with active highlight using Bundesliga red
- [ ] 4.3 Create `src/lib/components/Header.svelte`: app name, league picker, season picker, live badge, refresh button shell
- [ ] 4.4 Create `src/lib/views/TableView.svelte`: fetch table on mount and on league/season change using `$effect`
- [ ] 4.5 Implement standings table with columns: #, logo, name, P, W, D, L, GF, GA, GD, Pts
- [ ] 4.6 Add colored left-border zone indicators (top 2: CL blue, rows 3-4: EL orange, bottom 2: red)
- [ ] 4.7 Make each team row clickable — navigates to `team_detail`

## 5. Frontend — Matches Flow

- [ ] 5.1 Create `src/lib/views/MatchesView.svelte`: accept `matchday` prop; fetch matchday list and matches
- [ ] 5.2 Implement matchday heading and prev/next navigation buttons (disable at boundaries)
- [ ] 5.3 Create `src/lib/components/MatchCard.svelte`: home/away logos + names, score, status label (kick-off time / FT / live minute)
- [ ] 5.4 Implement live detection logic: `!is_finished && new Date(when_utc) <= now <= new Date(when_utc) + 2h`
- [ ] 5.5 Add 30-second auto-refresh using `$effect` with `setInterval`; clean up on component destroy
- [ ] 5.6 Create `src/lib/views/MatchDetailView.svelte`: accept `matchId` and `fromMatchday` props; fetch match detail on mount
- [ ] 5.7 Implement match header: both team logos, names, and score side-by-side
- [ ] 5.8 Implement goal timeline list: sorted by minute, running score, scorer name, PEN/OG badges
- [ ] 5.9 Implement match metadata footer: venue, viewer count (formatted), match date/time
- [ ] 5.10 Back button navigates to `{ screen: 'matches', matchday: fromMatchday }`

## 6. Rust Backend — App State Persistence

- [ ] 6.1 Define `AppViewState` serde struct: `last_opened`, `league`, `season`, `view`, `matchday` (Option), `selected_team_id` (Option)
- [ ] 6.2 Ensure `app_data_dir` is created on first run (use `std::fs::create_dir_all`)
- [ ] 6.3 Add `get_last_viewed()` command: read `<app_data_dir>/app_state.json`; return None if missing
- [ ] 6.4 Add `save_last_viewed(state: AppViewState)` command: write JSON to `<app_data_dir>/app_state.json`
- [ ] 6.5 Implement startup logic in `+page.svelte`: invoke `get_last_viewed`, check 48h threshold, navigate accordingly
- [ ] 6.6 Update `navigate(view)` and context setters to persist on navigation and league/season changes

## 7. Frontend — Teams Flow Baseline

- [ ] 7.1 Create `src/lib/views/TeamsView.svelte`: fetch teams on mount; render grid of TeamCard components
- [ ] 7.2 Create `src/lib/components/TeamCard.svelte`: team logo + name card; click navigates to team detail
- [ ] 7.3 Add baseline `get_team_detail(team_id, league, season)` command using OpenLigaDB data only (identity, table row, recent matches)
- [ ] 7.4 Create `src/lib/views/TeamDetailView.svelte`: accept `teamId` prop; invoke `get_team_detail` on mount
- [ ] 7.5 Implement identity section: logo, name, founded year, stadium/capacity (hide if unavailable)
- [ ] 7.6 Implement season stats row: P, W, D, L, GF, GA, GD, Pts, league position
- [ ] 7.7 Implement recent matches section: up to 5 finished matches + next upcoming match; show opponent, date, score, H/A
- [ ] 7.8 Show graceful "details unavailable" fallback when enrichment data is absent

## 8. Rust Backend — Rate Limiting

- [ ] 8.1 Add `check_cooldown(category, min_secs)` helper in AppState: returns `(bool, Option<u64>)` — (is_fresh, next_refresh_ms)
- [ ] 8.2 Define a `CachedResponse<T>` wrapper struct: `{ data: T, cached: bool, next_refresh_at: Option<u64> }`; use as return type for rate-limited commands
- [ ] 8.3 Store last successful response per category in AppState to serve during cooldown
- [ ] 8.4 Wrap `get_table` with cooldown check (60s); return cached payload envelope if within window
- [ ] 8.5 Wrap `get_matches_for_matchday` with cooldown check (30s)
- [ ] 8.6 Wrap `get_matchdays` with cooldown check (5m)
- [ ] 8.7 Wrap `get_team_detail` with cooldown check (5m)

## 9. Rust Backend — TheSportsDB Integration and Team Enrichment

- [ ] 9.1 Define `TheSportsDbTeam` serde struct (team fields: founded, stadium, capacity, description, country)
- [ ] 9.2 Define `TheSportsDbPlayer` serde struct (name, position, nationality, date_of_birth, photo_url)
- [ ] 9.3 Define `TheSportsDbStaff` serde struct (name, role)
- [ ] 9.4 Expand `TeamDetail` serde struct to include TheSportsDB fields + squad + staff
- [ ] 9.5 Implement `search_thesportsdb_team(name)` async fn: GET `searchteams.php?t=<name>`, return best match with Jaro-Winkler score using `strsim`
- [ ] 9.6 Implement `fetch_thesportsdb_players(tsdb_team_id)` async fn: GET `lookup_all_players.php?id=<id>`
- [ ] 9.7 Implement `read_team_cache(team_id, app_data_dir)` fn: read JSON file, check `cached_at` TTL (30 days), return None if missing or stale
- [ ] 9.8 Implement `write_team_cache(team_id, data, app_data_dir)` fn: write JSON with `cached_at` timestamp
- [ ] 9.9 Upgrade `get_team_detail(...)`: check cache → if miss, search TSDB → fetch players/staff → assemble enriched `TeamDetail` → write cache → return with source/fallback metadata
- [ ] 9.10 Implement squad section: grouped by position (GK, DF, MF, FW); show name, nationality, DOB/age
- [ ] 9.11 Implement staff section: name and role list; hide section if no staff data
- [ ] 9.12 Show "Squad data unavailable" placeholder when TheSportsDB match failed

## 10. Frontend — Refresh and Cooldown UX

- [ ] 10.1 Implement cooldown timer in Header: derive `isOnCooldown` and `lastUpdatedLabel` from `CachedResponse` metadata using `$derived`
- [ ] 10.2 Disable refresh button and show "last updated X ago" label during cooldown
- [ ] 10.3 Use `$effect` with `setInterval(1000)` to update the "X ago" label each second while on cooldown
- [ ] 10.4 Re-enable button when cooldown expires (reactive, no user action needed)
