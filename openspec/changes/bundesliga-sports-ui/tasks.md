## 1. Project Setup

- [x] 1.1 Install Tailwind v4: `pnpm add -D tailwindcss @tailwindcss/vite`
- [x] 1.2 Add `@tailwindcss/vite` plugin to `vite.config.js`
- [x] 1.3 Create `src/app.css` with `@import "tailwindcss"` and `@theme` block (dark bg, bundesliga red, surface colors, border color)
- [x] 1.4 Import `app.css` in `src/routes/+layout.svelte` (create if needed)
- [x] 1.5 Update `tauri.conf.json`: window width 1200, height 800, minWidth 960, minHeight 640, resizable true
- [x] 1.6 Add Rust dependencies to `src-tauri/Cargo.toml`: `chrono` (with serde feature), `strsim`

## 2. Rust Backend — Core Fixes and Stateless Commands

- [x] 2.1 Remove `BundesligaState` struct and all references to it
- [x] 2.2 Create `AppState` struct with `cooldown_tracker: HashMap<&'static str, Instant>` and `app_data_dir: PathBuf`; register with `tauri::Builder::manage()`
- [x] 2.3 Fix `get_table`: accept `league: String, season: i32` params; remove state dependency
- [x] 2.4 Fix `get_seasons`: implement using `chrono::Local::now().year()` to return current year and last 3
- [x] 2.5 Fix `get_leagues`: remove unused state param
- [x] 2.6 Add `get_matchdays(league, season)` command using `Group::available()`
- [x] 2.7 Add `get_current_matchday(league)` command using `Group::current()`
- [x] 2.8 Add `get_matches_for_matchday(league, season, group_order_id)` command using `Match::by_league_group()`
- [x] 2.9 Add `get_match_detail(match_id)` command using `Match::get()`
- [x] 2.10 Add `get_teams(league, season)` command using `Team::available()`
- [x] 2.11 Add `get_top_scorers(league, season)` command using `GoalGetter::list()`
- [x] 2.12 Register all new commands in `tauri::generate_handler![]`

## 3. Rust Backend — App State Persistence

- [x] 3.1 Define `AppViewState` serde struct: `last_opened`, `league`, `season`, `view`, `matchday` (Option), `selected_team_id` (Option)
- [x] 3.2 Add `get_last_viewed()` command: read `<app_data_dir>/app_state.json`; return None if missing
- [x] 3.3 Add `save_last_viewed(state: AppViewState)` command: write JSON to `<app_data_dir>/app_state.json`
- [x] 3.4 Ensure `app_data_dir` is created on first run (use `std::fs::create_dir_all`)

## 4. Rust Backend — Rate Limiting

- [x] 4.1 Add `check_cooldown(category, min_secs)` helper in AppState: returns `(bool, Option<u64>)` — (is_fresh, next_refresh_ms)
- [x] 4.2 Wrap `get_table` with cooldown check (60s); return cached payload envelope if within window
- [x] 4.3 Wrap `get_matches_for_matchday` with cooldown check (30s)
- [x] 4.4 Wrap `get_matchdays` with cooldown check (5m)
- [x] 4.5 Define a `CachedResponse<T>` wrapper struct: `{ data: T, cached: bool, next_refresh_at: Option<u64> }`; use as return type for rate-limited commands
- [x] 4.6 Store last successful response per category in AppState to serve during cooldown

## 5. Rust Backend — TheSportsDB Integration

- [x] 5.1 Define `TheSportsDbTeam` serde struct (team fields: founded, stadium, capacity, description, country)
- [x] 5.2 Define `TheSportsDbPlayer` serde struct (name, position, nationality, date_of_birth, photo_url)
- [x] 5.3 Define `TheSportsDbStaff` serde struct (name, role)
- [x] 5.4 Define `TeamDetail` serde struct combining openligadb Team + TheSportsDB fields + squad + staff
- [x] 5.5 Implement `search_thesportsdb_team(name)` async fn: GET `searchteams.php?t=<name>`, return best match with Jaro-Winkler score using `strsim`
- [x] 5.6 Implement `fetch_thesportsdb_players(tsdb_team_id)` async fn: GET `lookup_all_players.php?id=<id>`
- [x] 5.7 Implement `read_team_cache(team_id, app_data_dir)` fn: read JSON file, check `cached_at` TTL (30 days), return None if missing or stale
- [x] 5.8 Implement `write_team_cache(team_id, data, app_data_dir)` fn: write JSON with `cached_at` timestamp
- [x] 5.9 Add `get_team_detail(team_id, team_name, league, season)` command: check cache → if miss, search TSDB → fetch players → assemble TeamDetail → write cache → return with rate-limit envelope (5m cooldown)

## 6. TypeScript Types

- [x] 6.1 Add `Match.ts` type: id, when_utc, is_finished, team1, team2, results, goals, location, group, number_of_viewers
- [x] 6.2 Add `Goal.ts` type: id, score_team1, score_team2, match_minute, goal_getter_name, is_penalty, is_own_goal, is_overtime
- [x] 6.3 Add `Group.ts` type: id, name, order_id
- [x] 6.4 Add `TeamDetail.ts` type: id, name, short_name, icon_url, founded, stadium, capacity, description, squad (Player[]), staff (Staff[])
- [x] 6.5 Add `Player.ts` type: name, position, nationality, date_of_birth, photo_url
- [x] 6.6 Add `AppViewState.ts` type mirroring the Rust struct
- [x] 6.7 Add `CachedResponse<T>.ts` generic wrapper type
- [x] 6.8 Update `TableTeam.ts` with any missing fields (goals, opponentGoals already present — verify)

## 7. Frontend — App Shell and Navigation

- [x] 7.1 Create `src/lib/stores/view.svelte.ts`: export `$state` view object of type `AppView`; export `navigate(view)` helper that also calls `save_last_viewed`
- [x] 7.2 Create `src/lib/stores/context.svelte.ts`: export `$state` for `league`, `season`; export `setLeague`, `setSeason` helpers
- [x] 7.3 Rewrite `src/routes/+page.svelte`: app shell with sidebar + header + content pane layout; no data fetching here
- [x] 7.4 Create `src/lib/components/Sidebar.svelte`: three nav items (Table, Matches, Teams) with active highlight using Bundesliga red
- [x] 7.5 Create `src/lib/components/Header.svelte`: app name, league picker, season picker, live badge, refresh button with cooldown state
- [x] 7.6 Implement startup logic in +page.svelte: invoke `get_last_viewed`, check 48h threshold, navigate accordingly

## 8. Frontend — Table View

- [x] 8.1 Create `src/lib/views/TableView.svelte`: fetch table on mount and on league/season change using `$effect`
- [x] 8.2 Implement standings table with columns: #, logo, name, P, W, D, L, GF, GA, GD, Pts
- [x] 8.3 Add colored left-border zone indicators (top 2: CL blue, rows 3-4: EL orange, bottom 2: red)
- [x] 8.4 Make each team row clickable — navigates to `team_detail`

## 9. Frontend — Matches View

- [x] 9.1 Create `src/lib/views/MatchesView.svelte`: accept `matchday` prop; fetch matchday list and matches
- [x] 9.2 Implement matchday heading and prev/next navigation buttons (disable at boundaries)
- [x] 9.3 Create `src/lib/components/MatchCard.svelte`: home/away logos + names, score, status label (kick-off time / FT / live minute)
- [x] 9.4 Implement live detection logic: `!is_finished && new Date(when_utc) <= now <= new Date(when_utc) + 2h`
- [x] 9.5 Add 30-second auto-refresh using `$effect` with `setInterval`; clean up on component destroy

## 10. Frontend — Match Detail View

- [x] 10.1 Create `src/lib/views/MatchDetailView.svelte`: accept `matchId` and `fromMatchday` props; fetch match detail on mount
- [x] 10.2 Implement match header: both team logos, names, and score side-by-side
- [x] 10.3 Implement goal timeline list: sorted by minute, running score, scorer name, PEN/OG badges
- [x] 10.4 Implement match metadata footer: venue, viewer count (formatted), match date/time
- [x] 10.5 Back button navigates to `{ screen: 'matches', matchday: fromMatchday }`

## 11. Frontend — Teams View

- [x] 11.1 Create `src/lib/views/TeamsView.svelte`: fetch teams on mount; render grid of TeamCard components
- [x] 11.2 Create `src/lib/components/TeamCard.svelte`: team logo + name card; click navigates to team detail
- [x] 11.3 Create `src/lib/views/TeamDetailView.svelte`: accept `teamId` prop; invoke `get_team_detail` on mount
- [x] 11.4 Implement identity section: logo, name, founded year, stadium/capacity (hide if unavailable)
- [x] 11.5 Implement season stats row: P, W, D, L, GF, GA, GD, Pts, league position
- [x] 11.6 Implement squad section: grouped by position (GK, DF, MF, FW); show name, nationality, DOB/age
- [x] 11.7 Implement staff section: name and role list; hide section if no staff data
- [x] 11.8 Implement recent matches section: up to 5 finished matches + next upcoming match; show opponent, date, score, H/A
- [x] 11.9 Show "Squad data unavailable" placeholder when TheSportsDB match failed

## 12. Frontend — Refresh and Cooldown UI

- [x] 12.1 Implement cooldown timer in Header: derive `isOnCooldown` and `lastUpdatedLabel` from `CachedResponse` metadata using `$derived`
- [x] 12.2 Disable refresh button and show "last updated X ago" label during cooldown
- [x] 12.3 Use `$effect` with `setInterval(1000)` to update the "X ago" label each second while on cooldown
- [x] 12.4 Re-enable button when cooldown expires (reactive, no user action needed)
