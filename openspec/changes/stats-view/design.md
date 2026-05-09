## Context

The app is a Tauri v2 desktop app (Rust backend, SvelteKit/Svelte 5 frontend) with three existing views: Table, Matches, and Teams. The backend fetches data from OpenLigaDB, whose match responses embed full `goals: Goal[]` arrays directly in each `Match` object. A full-season fetch via `Match::by_league(league, season)` returns all ~306 Bundesliga matches with goals inline — the primary raw source for any derived statistic.

The existing fetch graph already retrieves:
- `get_table` → `TableTeam[]` (W, D, L, GF, GA, GD, Pts per team)
- `get_matches_for_matchday` → `Match[]` per matchday (with embedded goals)
- `get_top_scorers` → `GoalGetter[]`
- `get_team_matches` → all `Match[]` for a given team and season

Stats must be computed on top of this data, not by adding new external data sources.

## Goals / Non-Goals

**Goals:**
- Provide league-wide aggregated stats (goal totals, tempo, distribution by time and type)
- Provide derived team stats (form, home/away split, clean sheets, points per game)
- Provide projection scenarios (title race math, relegation math, European qualification)
- Surface records and accomplishments (biggest win, highest-scoring match, longest unbeaten run) with navigation to the relevant match or team
- Reuse existing fetch graph and caching patterns (`CachedResponse`, `AppState` cooldowns, disk file cache)
- Keep all visualizations in pure CSS/HTML — no external charting library

**Non-Goals:**
- Player-level career or cross-season statistics
- Live match statistics (possession %, shots, xG) — OpenLigaDB does not provide these
- Statistical modeling beyond deterministic projection math
- Cross-league comparison
- Exporting or printing stats

## Decisions

### 1. Backend-computed aggregation via `get_league_stats`

**Decision**: All aggregated league statistics are computed in a new `get_league_stats(league, season)` Rust command. The command fetches all season matches once via `Match::by_league`, iterates them in a single pass, and returns a `CachedResponse<LeagueStats>`. The `LeagueStats` struct is also persisted to disk at `<app_data_dir>/league_stats_{league}_{season}.json` with a 30-minute TTL.

**Rationale**: Iterating 306 match objects — each with an embedded goals array — is non-trivial JavaScript. A single-pass Rust computation is faster and keeps the frontend component logic clean. The disk cache means repeat loads within 30 minutes return instantly without a network round-trip. The in-memory `AppState` cooldown (5 minutes) prevents re-computation during a session.

**Alternative considered**: Fetching all matches in the frontend and computing with `$derived`. Rejected — no existing `invoke("get_all_matches")` command exposes this, it would add heavy JS computation to the component, and the full match list is already iterating in Rust for other commands.

### 2. Team Statistics as a section in TeamDetailView, not a new screen

**Decision**: Per-team statistics live as a new collapsible "Statistics" section within the existing `TeamDetailView.svelte`, not as a separate `{ screen: 'team_stats' }` AppView state.

**Rationale**: The data required (table entry and full team match list) is already fetched by `TeamDetailView` via `get_table` and `get_team_matches`. A separate screen would duplicate those fetches. Co-locating stats with the team identity, squad, and recent matches keeps related information in one place and avoids an extra navigation step.

**Implementation note**: The section is computed entirely on the frontend from already-fetched data using `$derived`. No new Tauri invoke calls are required for the team stats section.

**Alternative considered**: `{ screen: 'team_stats'; teamId: number }` AppView variant. Rejected — the only data it would add over `team_detail` is the derivation step, which belongs in the same component.

### 3. Projection scenarios use remaining-matches math only

**Decision**: Scenario projections use simple deterministic formulas:
- `max_points = current_points + 3 × matches_remaining`
- `min_points = current_points` (hypothetical all-loss outcome)
- `projected_points = round(ppg × total_matchdays)` where `ppg = current_points / matches_played`
- Title eliminated: team's `max_points < leader's current_points` (leader cannot be caught)
- Relegation safe: team's `min_points > 17th_place max_points` (cannot be caught from below)
- Qualification threshold constants for Bundesliga (CL: top 2, EL: 3-4, Conf: 5, Relegation playoff: 16, Direct relegation: 17-18) are defined as named constants

**Rationale**: Deterministic math is transparent, verifiable by fans, and matches how mainstream sports apps (Sky Sports, Kicker, Bundesliga.com) present qualification/relegation outlook. Adding probabilistic modeling would require historical cross-season data unavailable from OpenLigaDB and would imply a precision the data cannot support.

**Early-season caveat**: If fewer than 6 matchdays have been played (i.e., `matches_played < 6` for any team in the table), the Scenarios section shows a note: "Season is in early stages — projection ranges are wide."

**Alternative considered**: Elo-based win probability. Rejected — requires cross-season historical data not available from the API.

### 4. Goal timing distribution uses nine fixed 10-minute buckets

**Decision**: Goal timing uses nine buckets: `1-10`, `11-20`, `21-30`, `31-45`, `46-60`, `61-70`, `71-80`, `81-90`, `90+`. Bucket `31-45` captures first-half stoppage time and bucket `90+` captures full-time stoppage and overtime goals. Rendered as a horizontal bar chart using CSS `width` percentages relative to the bucket with the most goals.

**Rationale**: Ten-minute bands are the standard football analytics convention. Nine buckets fit cleanly in the available screen width at 1200px. Separating `31-45` and `90+` from plain 10-minute bands handles the well-known stoppage-time goal clustering. No external charting library is needed — percentage-width `div` elements with a Bundesliga-red fill give a clean, on-theme visualization.

**Implementation note**: `match_minute` values in the `Goal` struct can be `null`. Null-minute goals are counted in a separate "minute unknown" total displayed beneath the chart but not in the bars.

**Alternative considered**: SVG bar chart with axis labels. Rejected — adds complexity with no material benefit at this data density.

### 5. Records and accomplishments as badge cards with drill-down navigation

**Decision**: Records (biggest win, highest-scoring match, most viewers) appear as a horizontal row of badge cards in the League Stats view. Each badge shows the headline value, a label, and the two team names + logos. Clicking a badge navigates to `match_detail` for that match (or `team_detail` for team-level records like longest unbeaten run).

**Rationale**: Badge cards provide at-a-glance scanability consistent with the card-based visual language established by `MatchCard` and `TeamCard`. Drill-down links tie the stat to its concrete source without adding new navigation states.

**Alternative considered**: A flat text list of records. Rejected — lacks the visual emphasis and actionability appropriate for "achievement" data.

### 6. Form Table in League Stats view uses last 5 finished matches per team

**Decision**: The `TeamForm` data returned by `get_league_stats` contains each team's last 5 results (W/D/L) derived from the full season match list sorted by date. The frontend renders each result as a colored dot (green=W, gray=D, red=L, dark=unplayed). The form table is sorted by last-5 points descending, providing a "current momentum" ranking distinct from the standings.

**Rationale**: Form over the last 5 matches is the universally understood "in form / out of form" signal in European football. Computing it in the backend avoids passing all match data to the frontend. The separate sort order from standings gives the section its own analytical value.

**Alternative considered**: Last 10 matches. Rejected — last 5 is the Bundesliga.com standard and fits in the available card width with dot indicators.

### 7. Home/Away split computed entirely on the frontend

**Decision**: The home/away split section in the team stats panel is computed on the frontend from the `recentMatches` array already held in `TeamDetailView` state (which contains all of the team's season matches, not just the 5 shown in "Recent Matches"). No new backend command is needed.

**Rationale**: `get_team_matches` already returns the full season match list for the team. The frontend has all the data it needs to compute home/away records and goals. A backend command for this would be redundant.

**Implementation note**: The `recentMatches` variable in `TeamDetailView` is currently used only for the "Recent Matches" display section. The stats derivation reuses the same array without re-fetching.

## Risks / Trade-offs

**Full season fetch latency (first load)**: `Match::by_league` returns ~306 matches with embedded goals — typically 200-400 KB of JSON. First load may take 1-3 seconds depending on connection. A loading skeleton is shown. The 30-minute disk cache means subsequent loads within the session are instant.

**Streak calculations may diverge from official records**: OpenLigaDB only provides results and goals; it does not reflect VAR reversals, retroactive point deductions, or administrative results. All streak/record labels carry a "Based on match result data" footnote.

**Wide projection ranges early in season**: In the first 5 matchdays, max-points projections are near 102 for any team. A contextual note is shown when fewer than 6 matchdays have been completed.

**Zone thresholds assume Bundesliga rules**: CL/EL/relegation position thresholds are hardcoded for an 18-team, 34-matchday league (Bundesliga bl1). For other leagues in the OpenLigaDB dataset (bl2, bl3) the thresholds differ. The spec shows Bundesliga-tuned constants as the primary case; an `OTHER_LEAGUE_ZONES` fallback using generic top-3/bottom-3 is used for non-bl1 leagues.

**`get_team_matches` returns all matches for a name filter, not team ID**: The existing backend command uses `Match::by_league_team(league, season, team_name)` which is a text filter. Name collisions are rare in Bundesliga but possible. The stats derivation should guard against matches where neither `team1.id` nor `team2.id` matches the known `teamId`, treating them as unexpected data rather than crashing.

## Open Questions

- Should the form guide (5-dot strip) also appear as a column in the main Table view, or remain exclusive to the Stats and Team Detail views? (Suggested: Stats/Team Detail only — keeps the Table view uncluttered.)
- Should the "Longest Unbeaten Run" record in the League Stats view navigate to the team's detail, or simply display the team name inline? (Suggested: navigate to `team_detail` — consistent with Table row click behavior.)
- For the `90+` minute bucket, should extra-time goals (overtime matches) be separated from regular stoppage time goals? The `Goal.is_overtime` field enables this. (Suggested: separate them with a distinct `ET` bucket so penalty-shootout-adjacent stats are not inflated in the chart.)
