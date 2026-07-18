## ADDED Requirements

### Requirement: Stats nav item
The sidebar SHALL include a "Stats" navigation item that activates the League Stats view. The item SHALL be visually consistent with the existing Table, Matches, and Teams nav items, using a bar-chart icon. The item SHALL appear highlighted when the current view is `stats`.

#### Scenario: Stats nav item appears in sidebar
- **WHEN** the app is rendered
- **THEN** a "Stats" nav item is visible in the sidebar below the Teams item

#### Scenario: Clicking Stats navigates to the league stats view
- **WHEN** the user clicks the Stats nav item
- **THEN** the view state transitions to `{ screen: 'stats' }` and the League Stats view is rendered

#### Scenario: Stats nav item is active when Stats view is shown
- **WHEN** the current screen is `stats`
- **THEN** the Stats nav item displays the active highlight (Bundesliga red accent and background)

---

### Requirement: League Stats view — Season Overview
The League Stats view SHALL display an overview card for the selected league and season showing total goals scored, goals per match (to 2 decimal places), matches played vs. total, and a proportional home/away goal split bar.

#### Scenario: Season overview shows correct totals
- **WHEN** the Stats view loads with data for bl1/2024
- **THEN** the Season Overview card shows total goals, goals/match, and match count that are consistent with the data returned by `get_league_stats`

#### Scenario: Home/away split bar reflects actual proportion
- **WHEN** `home_goals = 400` and `away_goals = 320` in the response
- **THEN** the home portion of the split bar is approximately 55.6% wide and the away portion is approximately 44.4% wide

#### Scenario: Season overview shows zero state gracefully
- **WHEN** `total_matches_played === 0`
- **THEN** the view shows "No data for this season" and no further sections are rendered

---

### Requirement: League Stats view — Goal Timing chart
The League Stats view SHALL display a goal timing distribution chart with nine labeled minute buckets (`1-10`, `11-20`, `21-30`, `31-45`, `46-60`, `61-70`, `71-80`, `81-90`, `90+`). Each bucket SHALL be rendered as a horizontal bar scaled relative to the highest-count bucket. Goal counts SHALL be shown on each bar. Goals with an unknown minute SHALL be counted separately and reported in a footnote beneath the chart.

#### Scenario: Bars are proportionally scaled
- **WHEN** bucket `81-90` has 120 goals and all other buckets have fewer
- **THEN** the `81-90` bar renders at 100% width and all other bars render at proportionally shorter widths

#### Scenario: Unknown-minute footnote shown when applicable
- **WHEN** `goals_unknown_minute > 0`
- **THEN** a footnote reads "X goals with unknown minute not shown" beneath the chart

#### Scenario: Unknown-minute footnote hidden when not applicable
- **WHEN** `goals_unknown_minute === 0`
- **THEN** no footnote is displayed beneath the chart

---

### Requirement: League Stats view — Goal Types row
The League Stats view SHALL display penalty goal count and own goal count, each as a percentage of total goals.

#### Scenario: Goal types row shows percentage values
- **WHEN** `total_goals = 500`, `penalty_goals = 60`, `own_goals = 12`
- **THEN** the row shows "Penalties: 60 (12.0%)" and "Own goals: 12 (2.4%)"

---

### Requirement: League Stats view — Top Scorers
The League Stats view SHALL display the top 10 scorers for the selected league and season. Each entry SHALL show the scorer's name, goal count, and a relative goal bar scaled to the leading scorer's count. The list SHALL be sorted by goals descending.

#### Scenario: Top 10 scorers displayed
- **WHEN** the Stats view loads with scorer data
- **THEN** at most 10 scorer entries are shown, sorted by goals descending

#### Scenario: Top scorer's bar is at full width
- **WHEN** the leading scorer has 20 goals
- **THEN** the leading scorer's bar renders at 100% relative width

---

### Requirement: League Stats view — Records (Biggest Win, Highest-Scoring Match, Best Attendance)
The League Stats view SHALL display three record badge cards for the current season: Biggest Win (largest goal-difference margin), Highest-Scoring Match (most total goals), and Best-Attended Match (most viewers). Each badge SHALL show both team names with logos, the final score, and the match date. Clicking a badge SHALL navigate to the `match_detail` view for that match.

#### Scenario: Records badges show correct match data
- **WHEN** the Stats view loads with match data
- **THEN** each badge reflects the correct match, with team names, logos, score, and date

#### Scenario: Clicking a records badge opens match detail
- **WHEN** the user clicks a records badge card
- **THEN** the view state transitions to `{ screen: 'match_detail', matchId: <id>, fromMatchday: <matchday> }`

#### Scenario: Records badges are hidden when no finished matches exist
- **WHEN** `total_matches_played === 0`
- **THEN** the Records section is not rendered

---

### Requirement: League Stats view — Longest Unbeaten Run
The League Stats view SHALL display a badge showing the team with the longest consecutive unbeaten run (W+D streak) in the current season. The badge SHALL show the team logo, name, and run length. Clicking the badge SHALL navigate to `team_detail` for that team. A footnote SHALL read "Based on match result data".

#### Scenario: Unbeaten run badge shows team and streak length
- **WHEN** Team X has the longest unbeaten run of 12 matches
- **THEN** the badge shows Team X's logo, name, and "12 matches unbeaten"

#### Scenario: Clicking unbeaten run badge opens team detail
- **WHEN** the user clicks the unbeaten run badge
- **THEN** the view state transitions to `{ screen: 'team_detail', teamId: <id> }`

---

### Requirement: League Stats view — Clean Sheets
The League Stats view SHALL display a ranked list of teams sorted by clean sheet count (descending). Each entry SHALL show the team logo, team name, and clean sheet count. Clicking an entry SHALL navigate to `team_detail` for that team.

#### Scenario: Clean sheet list is sorted descending
- **WHEN** Team A has 10 clean sheets and Team B has 7
- **THEN** Team A appears above Team B in the clean sheets list

#### Scenario: Clicking a clean sheet entry opens team detail
- **WHEN** the user clicks a clean sheet entry
- **THEN** the view state transitions to `{ screen: 'team_detail', teamId: <id> }`

---

### Requirement: League Stats view — Form Table
The League Stats view SHALL display a form table showing each team's last 5 match results as colored indicators (W=green, D=gray, L=red) plus the total points earned in those 5 matches. The form table SHALL be sorted by last-5 points descending. Clicking a row SHALL navigate to `team_detail`.

#### Scenario: Form table sorted by recent points
- **WHEN** Team A has 12 points in last 5 and Team B has 9
- **THEN** Team A appears above Team B in the form table

#### Scenario: Form dot colors match results
- **WHEN** a team's last 5 results are W, D, L, W, W
- **THEN** dots are rendered green, gray, red, green, green from oldest to most recent (left to right)

#### Scenario: Clicking a form table row opens team detail
- **WHEN** the user clicks a team row in the form table
- **THEN** the view state transitions to `{ screen: 'team_detail', teamId: <id> }`

---

### Requirement: League Stats view — Title Race Scenarios
The League Stats view SHALL display a title race section showing, for each team, their current points, maximum possible points, PPG-projected final points, and a status badge indicating whether they are "In contention" or "Eliminated" from the title race. The section SHALL show an early-season note when fewer than 6 matchdays have been completed.

#### Scenario: Eliminated team shows "Eliminated" badge
- **WHEN** a team's `max_points` is less than the leader's current points
- **THEN** that team's row shows an "Eliminated" badge

#### Scenario: Leader shows "In contention" badge
- **WHEN** no team can be confirmed as champion yet
- **THEN** the leader shows "In contention"

#### Scenario: Early-season note shown
- **WHEN** fewer than 6 matchdays have been completed (any team has `matches_played < 6`)
- **THEN** the section displays "Season is in early stages — projection ranges are wide"

---

### Requirement: League Stats view — Relegation Scenarios
The League Stats view SHALL display a relegation section for the bottom 6 teams (positions 13-18), showing current points, gap to safety (15th-place boundary for Bundesliga), and a status badge: "Safe", "In danger", or "Relegated (math)" when a team is mathematically certain to be relegated. League zone constants SHALL use Bundesliga defaults (bl1): direct relegation positions 17-18, relegation playoff position 16.

#### Scenario: Team with gap to safety shows "In danger"
- **WHEN** a team is within 3 points of the relegation playoff line
- **THEN** the team's row displays an "In danger" badge

#### Scenario: Mathematically relegated team shows correct badge
- **WHEN** a team's `max_points` is less than or equal to the 16th-place team's current points
- **THEN** the team's row displays "Relegated (math)"

---

### Requirement: League Stats view — European Qualification Scenarios
The League Stats view SHALL display a European qualification section for the top 7 teams, showing current points, gap to the next qualification threshold (Champions League top 2, Europa League rows 3-4, Conference League row 5), and a qualification status badge. Zone constants SHALL use Bundesliga defaults for bl1; other leagues show generic top/bottom zones.

#### Scenario: Top 2 teams show CL qualification status
- **WHEN** a team is in position 1 or 2
- **THEN** their row shows a "Champions League" zone indicator

#### Scenario: Team at row 3-4 shows EL status
- **WHEN** a team is in position 3 or 4
- **THEN** their row shows a "Europa League" zone indicator

---

### Requirement: League Stats view — loading and refresh
The Stats view SHALL display a loading skeleton while league stats data is being fetched. The view SHALL support a refresh action that re-fetches league stats and updates the toolbar's cooldown indicator.

#### Scenario: Loading skeleton shown during fetch
- **WHEN** the Stats view mounts and data has not yet loaded
- **THEN** placeholder loading blocks are shown instead of stat sections

#### Scenario: Refresh re-fetches and updates cooldown state
- **WHEN** the toolbar refresh control is activated while the Stats view is active
- **THEN** league stats are re-fetched and the cooldown state is updated

---

### Requirement: Stats view state persisted and restored
Navigating to the Stats view SHALL persist `view: 'stats'` to `AppViewState`. When the app is reopened within 48 hours of the last session, and the last viewed screen was `stats`, the app SHALL restore the Stats view on startup.

#### Scenario: Stats view saved to app state
- **WHEN** the user navigates to the Stats view
- **THEN** `save_last_viewed` is called with `view: 'stats'`, `matchday: null`, `selected_team_id: null`

#### Scenario: Stats view restored on startup
- **WHEN** the app is reopened within 48 hours and `saved.view === 'stats'`
- **THEN** the app navigates to `{ screen: 'stats' }` on startup
