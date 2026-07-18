## 1. Navigation

- [ ] 1.1 Add a `Stats` variant to `NavScreen` (`crates/fulltime-ui/src/data/nav.rs`, see `bundesliga-sports-ui`).
- [ ] 1.2 Ensure `AppViewState` persistence round-trips the `Stats` screen with `matchday: None`, `selected_team_id: None`.

## 2. Data Layer — `LeagueStats` Types and Helpers

- [ ] 2.1 Add `FormResult` enum in `crates/fulltime-ui/src/data/stats.rs` with variants `Win`, `Draw`, `Loss`, `Unknown`; derive `Serialize, Deserialize, Clone, Copy`.
- [ ] 2.2 Add `TeamCleanSheets` struct: `team_id`, `team_name: Option<String>`, `team_icon_url: Option<String>`, `count: u32`.
- [ ] 2.3 Add `MatchSummary` struct: `match_id`, `when_utc: Option<String>`, `team1_id`, `team1_name`, `team1_icon_url`, `team2_id`, `team2_name`, `team2_icon_url`, `score_team1`, `score_team2`.
- [ ] 2.4 Add `TeamForm` struct: `team_id`, `team_name`, `team_icon_url`, `results: Vec<FormResult>`, `points: i32`.
- [ ] 2.5 Add `UnbeatenRecord` struct: `team_id`, `team_name`, `team_icon_url`, `run: u32`.
- [ ] 2.6 Add `LeagueStats` struct aggregating: `total_goals`, `total_matches_played`, `total_matches_remaining`, `goals_per_match`, `home_goals`, `away_goals`, `penalty_goals`, `own_goals`, `goals_unknown_minute`, `goals_by_minute_bucket: [u32; 9]`, `top_scorers`, `clean_sheets: Vec<TeamCleanSheets>`, `biggest_win: Option<MatchSummary>`, `highest_scoring_match: Option<MatchSummary>`, `highest_attendance_match: Option<MatchSummary>`, `longest_unbeaten: Option<UnbeatenRecord>`, `form_table: Vec<TeamForm>`.

## 3. Data Layer — `compute_league_stats`

- [ ] 3.1 Implement `fn compute_league_stats(matches: &[Match], top_scorers: Vec<GoalGetter>) -> LeagueStats` as a pure function (no I/O, no async).
- [ ] 3.2 Single-pass loop: for each finished match, extract the final score; skip unfinished matches for goal/result stats but count them toward `total_matches_remaining`.
- [ ] 3.3 Accumulate `total_goals`, `home_goals`, `away_goals`, `penalty_goals`, `own_goals`, `goals_unknown_minute` from embedded goals; classify each goal minute into the correct bucket index (1-10→0 … 81-90→7, unknown or 90+→8).
- [ ] 3.4 Track `biggest_win` (largest absolute score difference), `highest_scoring_match` (largest total goals), `highest_attendance_match` (largest viewer count); build a `MatchSummary` for each.
- [ ] 3.5 Build a per-team match list during the single pass: for each finished match, append a `(date, FormResult)` entry for both teams.
- [ ] 3.6 Compute `clean_sheets`: for each team, count finished matches with 0 goals conceded; sort descending by count.
- [ ] 3.7 Compute `form_table`: for each team, sort their match list by date descending, take the last 5, derive W/D/L, compute last-5 points; sort the form table by points descending.
- [ ] 3.8 Compute `longest_unbeaten`: for each team's match list sorted by date ascending, find the longest consecutive W+D streak; return the team with the longest streak.
- [ ] 3.9 Compute `goals_per_match = total_goals as f64 / total_matches_played as f64`.

## 4. Data Layer — `fetch_league_stats`

- [ ] 4.1 Add `read_league_stats_cache(league, season, data_dir) -> Option<LeagueStats>`: read `league_stats_{league}_{season}.json`, check mtime against 30-minute TTL, deserialize if fresh.
- [ ] 4.2 Add `write_league_stats_cache(league, season, data, data_dir)`: serialize and write the JSON file.
- [ ] 4.3 Implement `async fn fetch_league_stats(league, season) -> Cached<LeagueStats>`:
  1. Check disk cache; if hit, return `Cached { data, cached: true, next_refresh_at: None }`.
  2. Check the in-memory cooldown (5 minutes, `DataCache`, see `bundesliga-sports-ui`); if on cooldown, return the last cached value.
  3. Fetch all season matches and top scorers.
  4. Call `compute_league_stats`.
  5. Write the disk cache, update the in-memory cooldown, and store the last response.
  6. Return `Cached { data, cached: false, next_refresh_at: None }`.

## 5. League Stats View

- [ ] 5.1 Create `crates/fulltime-ui/src/ui/views/stats_view.rs`: fetch the table (reuse the cached table fetch) and league stats on mount via `cx.spawn`.
- [ ] 5.2 Render **Season Overview**: total goals, goals/match (2dp), matches played / total, home/away goal split bar.
- [ ] 5.3 Render **Goal Timing** chart: nine labeled buckets as proportional-width bars; footnote for unknown-minute goals when present.
- [ ] 5.4 Render **Goal Types** row: penalties and own goals, each with count and percentage of total.
- [ ] 5.5 Render **Top Scorers**: top 10, each with rank, name, goal count, and a relative bar scaled to the leader's count.
- [ ] 5.6 Render **Records**: badge cards for Biggest Win, Highest-Scoring Match, Best-Attended Match; clicking navigates to `NavScreen::MatchDetail`.
- [ ] 5.7 Render **Longest Unbeaten Run** badge; clicking navigates to `NavScreen::TeamDetail`; footnote "Based on match result data".
- [ ] 5.8 Render **Clean Sheets**: ranked list; clicking a row navigates to `NavScreen::TeamDetail`.
- [ ] 5.9 Render **Form Table**: team, five form dots, last-5 points; sorted by points descending; clicking a row navigates to `NavScreen::TeamDetail`.
- [ ] 5.10 Render **Title Race Scenarios**: per team, current points, max possible points, PPG-projected points, status badge ("Champion"/"In contention"/"Eliminated"); early-season caveat when `matches_played < 6`.
- [ ] 5.11 Render **Relegation Scenarios**: bottom 6 rows, gap to safety, status badge ("Safe"/"In danger"/"Relegated (math)"); Bundesliga zone constants (direct relegation 17-18, playoff 16).
- [ ] 5.12 Render **European Qualification Scenarios**: top 7 rows, gap to next threshold (CL at 2, EL at 4, Conf at 5), status badge.
- [ ] 5.13 Add a loading skeleton while data is fetching.
- [ ] 5.14 Add a "No data for this season" empty state when `total_matches_played == 0`.
- [ ] 5.15 Wire `Stats` into the `root_view.rs` screen router and sidebar (see `bundesliga-sports-ui`).

## 6. Team Statistics Section

- [ ] 6.1 In `team_detail_view.rs`, compute `TeamStats` from the team's full match list and table entry: home/away splits (played, W/D/L, GF/GA per side), clean sheets (home/away/total), last-5 form, longest unbeaten run this season, best/worst result.
- [ ] 6.2 Compute projections from the table entry: PPG, matches remaining, max/min/projected points.
- [ ] 6.3 Guard against matches where neither team ID matches the known `team_id`; guard against `matches_played == 0` for PPG.
- [ ] 6.4 Render a collapsible **Statistics** section between "Season Stats" and "Recent Matches", collapsed by default.
- [ ] 6.5 Render **Home / Away Split** as a 2-column comparison (P, W, D, L, GF, GA, GD).
- [ ] 6.6 Render **Goal Rates** row: goals/game scored/conceded (2dp), points/game (2dp), clean sheets count.
- [ ] 6.7 Render **Form Guide** strip: five colored dots, oldest to most recent, labeled "Last 5 results".
- [ ] 6.8 Render **Records** row: longest unbeaten run, best result, worst result; footnote "Based on match result data".
- [ ] 6.9 Render **Projection** bar: min → projected → max points, with an early-season note if `matches_played < 6`.
- [ ] 6.10 Render **Qualification Scenarios** for the team's current position, using the same zone constants as the league stats view; hide for non-bl1 leagues unless a generic threshold applies.
- [ ] 6.11 Show a "Not enough data" placeholder when `matches_played < 3`.

## 7. Verification

- [ ] 7.1 Add unit tests for `compute_league_stats` covering bucket classification, records, form table ordering, and unbeaten-streak detection.
- [ ] 7.2 Run `cargo clippy --workspace`.
- [ ] 7.3 Run `cargo +nightly fmt --check`.
- [ ] 7.4 Run `cargo run -p fulltime-core` and visually verify the Stats view sections and the Team Detail Statistics section, including empty/early-season states.
