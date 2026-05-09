## 1. TypeScript Types

- [ ] 1.1 Add `src/types/LeagueStats.ts`: export `FormResult` (`'W' | 'D' | 'L' | 'U'`), `TeamCleanSheets` (`{ team_id: number; team_name: string | null; team_icon_url: string | null; count: number }`), `MatchSummary` (`{ match_id: number; when_utc: string | null; team1_id: number; team1_name: string | null; team1_icon_url: string | null; team2_id: number; team2_name: string | null; team2_icon_url: string | null; score_team1: number; score_team2: number }`), `TeamForm` (`{ team_id: number; team_name: string | null; team_icon_url: string | null; results: FormResult[]; points: number }`), and the top-level `LeagueStats` interface
- [ ] 1.2 `LeagueStats` fields: `total_goals: number`, `total_matches_played: number`, `total_matches_remaining: number`, `goals_per_match: number`, `home_goals: number`, `away_goals: number`, `penalty_goals: number`, `own_goals: number`, `goals_unknown_minute: number`, `goals_by_minute_bucket: number[]` (9 elements, index 0-8 = 1-10, 11-20, 21-30, 31-45, 46-60, 61-70, 71-80, 81-90, 90+), `top_scorers: GoalGetter[]`, `clean_sheets: TeamCleanSheets[]`, `biggest_win: MatchSummary | null`, `highest_scoring_match: MatchSummary | null`, `highest_attendance_match: MatchSummary | null`, `longest_unbeaten: { team_id: number; team_name: string | null; team_icon_url: string | null; run: number } | null`, `form_table: TeamForm[]`
- [ ] 1.3 Update `src/lib/stores/view.svelte.ts`: add `| { screen: 'stats' }` to the `AppView` union type

## 2. Rust Backend — LeagueStats Structs and Helpers

- [ ] 2.1 Add `FormResult` enum in `lib.rs` with variants `Win`, `Draw`, `Loss`, `Unknown`; derive `Serialize, Deserialize, Clone, Copy`; serialize as strings `"W"`, `"D"`, `"L"`, `"U"` using `#[serde(rename)]` or a custom serializer
- [ ] 2.2 Add `TeamCleanSheets` struct in `lib.rs`: `team_id: i32`, `team_name: Option<String>`, `team_icon_url: Option<String>`, `count: u32`; derive `Serialize, Deserialize, Clone`
- [ ] 2.3 Add `MatchSummary` struct in `lib.rs`: `match_id: i32`, `when_utc: Option<String>`, `team1_id: i32`, `team1_name: Option<String>`, `team1_icon_url: Option<String>`, `team2_id: i32`, `team2_name: Option<String>`, `team2_icon_url: Option<String>`, `score_team1: i32`, `score_team2: i32`; derive `Serialize, Deserialize, Clone`
- [ ] 2.4 Add `TeamForm` struct in `lib.rs`: `team_id: i32`, `team_name: Option<String>`, `team_icon_url: Option<String>`, `results: Vec<FormResult>`, `points: i32`; derive `Serialize, Deserialize, Clone`
- [ ] 2.5 Add `UnbeatenRecord` struct in `lib.rs`: `team_id: i32`, `team_name: Option<String>`, `team_icon_url: Option<String>`, `run: u32`; derive `Serialize, Deserialize, Clone`
- [ ] 2.6 Add `LeagueStats` struct in `lib.rs`: all fields matching the TypeScript type in task 1.2; `top_scorers: Vec<serde_json::Value>` (reuse serialized `GoalGetter` array), `clean_sheets: Vec<TeamCleanSheets>`, `biggest_win: Option<MatchSummary>`, `highest_scoring_match: Option<MatchSummary>`, `highest_attendance_match: Option<MatchSummary>`, `longest_unbeaten: Option<UnbeatenRecord>`, `form_table: Vec<TeamForm>`; derive `Serialize, Deserialize, Clone`

## 3. Rust Backend — `compute_league_stats` Function

- [ ] 3.1 Implement `fn compute_league_stats(matches: &[Match], top_scorers: Vec<serde_json::Value>) -> LeagueStats` as a pure function (no I/O, no async)
- [ ] 3.2 Single-pass loop: for each finished match, extract final score from `results` (type 2 first, fall back to type 1); skip unfinished matches for all goal/result stats but count them toward `total_matches_remaining`
- [ ] 3.3 Accumulate: `total_goals`, `home_goals` (team1 goals), `away_goals` (team2 goals), `penalty_goals`, `own_goals`, `goals_unknown_minute` from embedded `goals` arrays; classify each goal minute into the correct bucket index (1-10→0, 11-20→1, 21-30→2, 31-45→3, 46-60→4, 61-70→5, 71-80→6, 81-90→7, null or 90+→8; use `is_overtime` flag to put extra-time goals in a separate accumulator that is included in bucket 8 per the "90+" label)
- [ ] 3.4 Track `biggest_win` as the match with the largest absolute score difference; `highest_scoring_match` as the match with the largest total goals; `highest_attendance_match` as the match with the largest `number_of_viewers`; build `MatchSummary` for each
- [ ] 3.5 Build a per-team match list (keyed by `team_id`) during the single pass: for each finished match, append a `(date, result_for_team: FormResult)` entry for both `team1` and `team2`
- [ ] 3.6 After the pass, compute `clean_sheets`: for each team, count finished matches where the team conceded 0 goals; sort descending by count; build `Vec<TeamCleanSheets>` carrying the team name and icon from the match data
- [ ] 3.7 After the pass, compute `form_table`: for each team, sort their match list by date descending, take the last 5 entries, derive W/D/L per entry, compute last-5 points, build `TeamForm`; sort the form table by `points` descending
- [ ] 3.8 After the pass, compute `longest_unbeaten`: for each team's match list sorted by date ascending, find the longest consecutive W+D streak; return the team with the longest streak as `UnbeatenRecord`
- [ ] 3.9 Compute `goals_per_match`: `total_goals as f64 / total_matches_played as f64`; store as `f64`, serialized to 2dp precision

## 4. Rust Backend — `get_league_stats` Command

- [ ] 4.1 Add `read_league_stats_cache(league: &str, season: i32, app_data_dir: &Path) -> Option<LeagueStats>` fn: read `league_stats_{league}_{season}.json` from `app_data_dir`, check file mtime against 30-minute TTL using `std::fs::metadata().modified()`, deserialize and return if fresh
- [ ] 4.2 Add `write_league_stats_cache(league: &str, season: i32, data: &LeagueStats, app_data_dir: &Path)` fn: serialize to JSON and write to `league_stats_{league}_{season}.json`
- [ ] 4.3 Implement `get_league_stats(league: String, season: i32, state: State<Mutex<AppState>>) -> Result<CachedResponse<LeagueStats>, String>` command:
  1. Clone `app_data_dir` from locked state
  2. Check disk cache via `read_league_stats_cache`; if hit, return `CachedResponse { data, cached: true, next_refresh_at: None }`
  3. Check in-memory cooldown (5 minutes) via `state.check_cooldown`; if on cooldown, deserialize last cached `serde_json::Value` from `AppState.last_responses` and return as cached
  4. Fetch `Match::by_league(&league, season).await` and `GoalGetter::list(&league, season).await`; map errors with `.map_err(|e| e.to_string())`
  5. Serialize `top_scorers` to `Vec<serde_json::Value>` for inclusion in `LeagueStats`
  6. Call `compute_league_stats(&matches, top_scorers_json)`
  7. Write disk cache via `write_league_stats_cache`
  8. Serialize the full `LeagueStats` to `serde_json::Value`, store in `AppState.last_responses` under key `"league_stats:{league}:{season}"`
  9. Update in-memory cooldown via `state.update_cooldown`
  10. Return `CachedResponse { data, cached: false, next_refresh_at: None }`
- [ ] 4.4 Register `get_league_stats` in `tauri::generate_handler![]`

## 5. Frontend — League Stats View (`StatsView.svelte`)

- [ ] 5.1 Create `src/lib/views/StatsView.svelte`: fetch `get_table` (reuse — already cached by TableView) and `get_league_stats` on mount via `$effect`; propagate `onCooldownChange` from the `get_league_stats` response; expose `refresh()` method
- [ ] 5.2 Implement **Season Overview** card: total goals, goals/match (to 2dp), matches played / total (e.g., "22 / 34"), home goals vs. away goals with a proportional split bar (home = Bundesliga red, away = muted)
- [ ] 5.3 Implement **Goal Timing** chart: nine labeled buckets as horizontal bars; bar width = `(bucket_count / max_bucket_count) * 100%`; Bundesliga-red fill; bucket label left-aligned, count right-aligned; footnote "X goals with unknown minute not shown" when `goals_unknown_minute > 0`
- [ ] 5.4 Implement **Goal Types** inline stats row: penalties (count + % of total), own goals (count + % of total)
- [ ] 5.5 Implement **Top Scorers** section: top 10 from `league_stats.top_scorers`; each row shows rank, scorer name, goal count, and a relative bar scaled to the leader's count
- [ ] 5.6 Implement **Records** section: three badge cards in a horizontal scroll row — Biggest Win, Highest-Scoring Match, Best-Attended Match; each card shows score, team names, team logos, and date; clicking navigates to `match_detail` for that `match_id`
- [ ] 5.7 Implement **Longest Unbeaten Run** badge: team logo, name, run length (e.g., "12 matches unbeaten"); clicking navigates to `team_detail`; footnote "Based on match result data"
- [ ] 5.8 Implement **Clean Sheets** section: ranked list (team logo, name, count); clicking a row navigates to `team_detail`
- [ ] 5.9 Implement **Form Table** section: table with columns — team logo, team name, five form dots (W/D/L colored circles), last-5 points total; sorted by last-5 points descending; clicking a row navigates to `team_detail`
- [ ] 5.10 Implement **Title Race Scenarios** section: for each team in the table (sorted by position), show team name, current points, max possible points (`current + 3 × remaining`), PPG-projected points, and a status badge: "Champion" (if current leader and all rivals are eliminated), "In contention", or "Eliminated" (if `max_points < leader_current_points`); show early-season caveat note when `matches_played < 6`
- [ ] 5.11 Implement **Relegation Scenarios** section: for rows 13-18 (bottom 6), show team name, current points, gap to safety (row 15 boundary), "Safe" / "In danger" / "Relegated (math)" badge; use Bundesliga zone constants (direct relegation: 17-18, playoff: 16)
- [ ] 5.12 Implement **European Qualification Scenarios** section: for rows 1-7, show team name, current points, gap to next threshold (CL at 2, EL at 4, Conf at 5), status badge; use zone constants
- [ ] 5.13 Add loading skeleton (shimmer placeholder blocks) while data is fetching
- [ ] 5.14 Add "No data for this season" empty state when `total_matches_played === 0`

## 6. Frontend — Team Statistics Section in TeamDetailView

- [ ] 6.1 In `TeamDetailView.svelte`, compute `$derived` `teamStats` object from `recentMatches` (the full season match list) and `tableEntry`:
  - `home_played`, `home_wins`, `home_draws`, `home_losses`
  - `away_played`, `away_wins`, `away_draws`, `away_losses`
  - `goals_scored_home`, `goals_conceded_home`, `goals_scored_away`, `goals_conceded_away`
  - `clean_sheets_home`, `clean_sheets_away`, `clean_sheets_total`
  - `last_5_form: FormResult[]` (last 5 finished matches, most recent first)
  - `longest_unbeaten_run: number` (longest W+D streak this season)
  - `best_result: { opponent: string; score: string; date: string } | null` (largest positive GD)
  - `worst_result: { opponent: string; score: string; date: string } | null` (largest negative GD)
- [ ] 6.2 Compute projections from `tableEntry` in the same `$derived`: `ppg`, `matches_remaining`, `max_pts`, `min_pts`, `projected_pts`
- [ ] 6.3 Guard against matches where neither `team1.id` nor `team2.id` matches `teamId` (log warning, skip); guard against `matches_played === 0` (avoid division by zero for PPG)
- [ ] 6.4 Render **Statistics** collapsible section in `TeamDetailView.svelte`, positioned between "Season Stats" and "Recent Matches"; collapsed by default, expanded by clicking the section header; use a `$state(expanded)` toggle
- [ ] 6.5 Within the Statistics section, render **Home / Away Split** as a 2-column comparison table: rows for P, W, D, L, GF, GA, GD; left column = Home, right column = Away
- [ ] 6.6 Render **Goal Rates** inline row: Goals/game scored (2dp), Goals/game conceded (2dp), Points/game (2dp), Clean sheets (count)
- [ ] 6.7 Render **Form Guide** strip: five colored dots (W=green, D=`--color-text-muted`, L=Bundesliga red) with the most recent result on the right; label "Last 5 results"
- [ ] 6.8 Render **Records** row: Longest unbeaten run (number + "matches"), Best result (score vs. team name), Worst result (score vs. team name); footnote "Based on match result data"
- [ ] 6.9 Render **Projection** bar: a horizontal range bar showing `min_pts` → `projected_pts` → `max_pts`; label each anchor; show early-season note if `matches_played < 6`
- [ ] 6.10 Render **Qualification Scenarios** for the team's current position: which European spots are mathematically possible, which are eliminated, relegation status; use the same zone constants as the league stats view; hide section entirely for non-bl1 leagues unless a generic threshold can be computed
- [ ] 6.11 Show "Not enough data" placeholder in the Statistics section when `tableEntry.matches < 3`

## 7. Navigation Integration

- [ ] 7.1 Add "Stats" nav item to `Sidebar.svelte` after the Teams item; icon: a bar chart SVG (three ascending vertical bars)
- [ ] 7.2 Update `isActive()` in `Sidebar.svelte`: `screen === 'stats'` returns true for the Stats nav item
- [ ] 7.3 Update `go()` in `Sidebar.svelte`: Stats item navigates to `{ screen: 'stats' }`
- [ ] 7.4 Add `<StatsView>` branch in the `+page.svelte` main content router for `view.screen === 'stats'`
- [ ] 7.5 Add `statsViewRef` binding in `+page.svelte`; wire `handleRefresh` to call `statsViewRef?.refresh()` when the current view is `'stats'`
- [ ] 7.6 Update startup logic in `+page.svelte`: handle restored `'stats'` view by calling `navigate({ screen: 'stats' })`

## 8. App State and Persistence

- [ ] 8.1 Confirm `AppViewState.view` accepts `'stats'` — it is stored as a plain string so no Rust struct change is needed; verify `get_last_viewed` and `save_last_viewed` round-trip correctly
- [ ] 8.2 Update `saveCurrentView()` in `view.svelte.ts`: when `view.screen === 'stats'`, set `matchday: null` and `selected_team_id: null` in the persisted `AppViewState`
