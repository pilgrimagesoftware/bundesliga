## Context

The app is a native GPUI desktop app (`crates/fulltime-core` + `crates/fulltime-ui`) with three planned views: Table, Matches, and Teams (see `bundesliga-sports-ui`). The data layer fetches from OpenLigaDB, whose match responses embed full `goals: Goal[]` arrays directly in each match object. A full-season fetch returns all ~306 Bundesliga matches with goals inline — the primary raw source for any derived statistic.

The fetch graph already provides (or will provide, per `bundesliga-sports-ui`):
- `fetch_table` → `Vec<TableTeam>` (W, D, L, GF, GA, GD, Pts per team)
- `fetch_matches_for_matchday` → matches per matchday (with embedded goals)
- top scorers via `openligadb::GoalGetter`
- `fetch_team_matches` (or equivalent) → all matches for a given team and season

Stats must be computed on top of this data, not by adding new external data sources.

## Goals / Non-Goals

**Goals:**
- Provide league-wide aggregated stats (goal totals, tempo, distribution by time and type)
- Provide derived team stats (form, home/away split, clean sheets, points per game)
- Provide projection scenarios (title race math, relegation math, European qualification)
- Surface records and accomplishments (biggest win, highest-scoring match, longest unbeaten run) with navigation to the relevant match or team
- Reuse the existing fetch graph and caching pattern (`Cached<T>`, per-category cooldowns, disk file cache — see `bundesliga-sports-ui`/`cache-helper-abstraction`)
- Keep all visualizations as plain GPUI elements (proportional-width `div`s) — no charting crate

**Non-Goals:**
- Player-level career or cross-season statistics
- Live match statistics (possession %, shots, xG) — OpenLigaDB does not provide these
- Statistical modeling beyond deterministic projection math
- Cross-league comparison
- Exporting or printing stats

## Decisions

### 1. Data-layer-computed aggregation via `fetch_league_stats`

**Decision**: All aggregated league statistics are computed in a new `fetch_league_stats(league, season)` async function in `crates/fulltime-ui/src/data/stats.rs`. It fetches all season matches once, iterates them in a single pass, and returns a `Cached<LeagueStats>`. `LeagueStats` is also persisted to disk at `<data_dir>/league_stats_{league}_{season}.json` with a 30-minute TTL.

**Rationale**: Iterating 306 match objects — each with an embedded goals array — belongs in the data layer, not scattered across view render calls. A single-pass computation keeps the view code focused on rendering. The disk cache means repeat loads within 30 minutes return instantly; the in-memory cooldown (5 minutes, same `DataCache` mechanism as `bundesliga-sports-ui`) prevents re-computation during a session.

**Alternative considered**: Recomputing stats inline in `stats_view.rs`'s render function on every frame. Rejected — GPUI views re-render frequently (any state change, hover, etc.); computing over 306 matches per render would be wasteful and is unnecessary since the underlying match data only changes on refresh.

### 2. Team Statistics as a section in the team detail view, not a new screen

**Decision**: Per-team statistics live as a new collapsible "Statistics" section within `crates/fulltime-ui/src/ui/views/team_detail_view.rs`, not as a separate `NavScreen::TeamStats` variant.

**Rationale**: The data required (table entry and full team match list) is already fetched by the team detail view. A separate screen would duplicate those fetches. Co-locating stats with the team identity, squad, and recent matches keeps related information in one place and avoids an extra navigation step.

**Implementation note**: The section is computed from data the team detail view already holds in its entity state — no new fetch is triggered when the section is expanded.

**Alternative considered**: `NavScreen::TeamStats { team_id }` variant. Rejected — the only thing it would add over `TeamDetail` is the derivation step, which belongs in the same view.

### 3. Projection scenarios use remaining-matches math only

**Decision**: Scenario projections use simple deterministic formulas:
- `max_points = current_points + 3 × matches_remaining`
- `min_points = current_points` (hypothetical all-loss outcome)
- `projected_points = round(ppg × total_matchdays)` where `ppg = current_points / matches_played`
- Title eliminated: team's `max_points < leader's current_points` (leader cannot be caught)
- Relegation safe: team's `min_points > 17th_place max_points` (cannot be caught from below)
- Qualification threshold constants for Bundesliga (CL: top 2, EL: 3-4, Conf: 5, Relegation playoff: 16, Direct relegation: 17-18) are defined as named constants in `data/stats.rs`

**Rationale**: Deterministic math is transparent, verifiable by fans, and matches how mainstream sports apps present qualification/relegation outlook. Adding probabilistic modeling would require historical cross-season data unavailable from OpenLigaDB and would imply a precision the data cannot support.

**Early-season caveat**: If fewer than 6 matchdays have been played (`matches_played < 6` for any team in the table), the Scenarios section shows a note: "Season is in early stages — projection ranges are wide."

**Alternative considered**: Elo-based win probability. Rejected — requires cross-season historical data not available from the API.

### 4. Goal timing distribution uses nine fixed 10-minute buckets

**Decision**: Goal timing uses nine buckets: `1-10`, `11-20`, `21-30`, `31-45`, `46-60`, `61-70`, `71-80`, `81-90`, `90+`. Bucket `31-45` captures first-half stoppage time and bucket `90+` captures full-time stoppage and overtime goals. Rendered as a horizontal bar chart using `div` width fractions (`gpui::relative(fraction)` or an equivalent `px()` computed from the bucket with the most goals).

**Rationale**: Ten-minute bands are the standard football analytics convention. Nine buckets fit cleanly in the toolbar/content width at the app's 1200px default. Separating `31-45` and `90+` from plain 10-minute bands handles the well-known stoppage-time goal clustering. Plain proportional-width elements give a clean, on-theme visualization without a charting dependency.

**Implementation note**: `match_minute` values can be absent. Minute-less goals are counted in a separate "minute unknown" total displayed beneath the chart but not in the bars.

**Alternative considered**: A dedicated charting crate. Rejected — adds a dependency with no material benefit at this data density; GPUI's own layout primitives are sufficient.

### 5. Records and accomplishments as badge cards with drill-down navigation

**Decision**: Records (biggest win, highest-scoring match, most viewers) appear as a horizontal row of badge cards in the League Stats view. Each badge shows the headline value, a label, and the two team names + logos. Clicking a badge calls `navigate(NavScreen::MatchDetail { .. })` (or `TeamDetail` for team-level records like longest unbeaten run).

**Rationale**: Badge cards provide at-a-glance scanability consistent with the card-based visual language established by the match/team cards in `bundesliga-sports-ui`. Drill-down links tie the stat to its concrete source without adding new navigation states.

**Alternative considered**: A flat text list of records. Rejected — lacks the visual emphasis and actionability appropriate for "achievement" data.

### 6. Form Table in League Stats view uses last 5 finished matches per team

**Decision**: `TeamForm` (returned by `fetch_league_stats`) contains each team's last 5 results (W/D/L) derived from the full season match list sorted by date. The view renders each result as a colored dot (green=W, gray=D, red=L). The form table is sorted by last-5 points descending, providing a "current momentum" ranking distinct from the standings.

**Rationale**: Form over the last 5 matches is the universally understood "in form / out of form" signal in European football. Computing it in the data layer avoids re-deriving it per render. The separate sort order from standings gives the section its own analytical value.

**Alternative considered**: Last 10 matches. Rejected — last 5 is the standard and fits the available card width with dot indicators.

### 7. Home/Away split computed from already-fetched team match data

**Decision**: The home/away split section in the team stats panel is computed from the team match list the team detail view already holds in its entity state (all of the team's season matches, not just the 5 shown in "Recent Matches"). No new data-layer fetch is needed.

**Rationale**: The team detail view already fetches the full season match list for the team. It has all the data it needs to compute home/away records and goals. A dedicated fetch for this would be redundant.

## Risks / Trade-offs

**Full season fetch latency (first load)**: A full-season fetch returns ~306 matches with embedded goals — typically 200-400 KB of JSON. First load may take 1-3 seconds depending on connection. A loading skeleton is shown. The 30-minute disk cache means subsequent loads within the session are instant.

**Streak calculations may diverge from official records**: OpenLigaDB only provides results and goals; it does not reflect VAR reversals, retroactive point deductions, or administrative results. All streak/record labels carry a "Based on match result data" footnote.

**Wide projection ranges early in season**: In the first 5 matchdays, max-points projections are near 102 for any team. A contextual note is shown when fewer than 6 matchdays have been completed.

**Zone thresholds assume Bundesliga rules**: CL/EL/relegation position thresholds are hardcoded for an 18-team, 34-matchday league (bl1). For other leagues in the OpenLigaDB dataset (bl2, bl3) the thresholds differ. The spec shows Bundesliga-tuned constants as the primary case; a generic top-3/bottom-3 fallback is used for non-bl1 leagues.

**Team match lookups by name, not ID**: If the underlying data-layer function for a team's full match list filters by team name rather than ID (see `bundesliga-sports-ui`'s equivalent), name collisions are rare in Bundesliga but possible. The stats derivation should guard against matches where neither team's ID matches the known `team_id`, treating them as unexpected data rather than panicking.

## Open Questions

- Should the form guide (5-dot strip) also appear as a column in the main Table view, or remain exclusive to the Stats and Team Detail views? (Suggested: Stats/Team Detail only — keeps the Table view uncluttered.)
- Should the "Longest Unbeaten Run" record in the League Stats view navigate to the team's detail, or simply display the team name inline? (Suggested: navigate to team detail — consistent with table row click behavior.)
- For the `90+` minute bucket, should extra-time goals (overtime matches) be separated from regular stoppage time goals? The goal data's overtime flag enables this. (Suggested: separate them with a distinct `ET` bucket so penalty-shootout-adjacent stats are not inflated in the chart.)
