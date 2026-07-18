## ADDED Requirements

### Requirement: Team detail — Statistics section
The Team Detail view SHALL include a collapsible "Statistics" section positioned between the "Season Stats" row and the "Recent Matches" section. The section SHALL be collapsed by default and expand on click. When the team has fewer than 3 finished matches, the section SHALL show a "Not enough data" placeholder instead of computed content.

#### Scenario: Statistics section is collapsed by default
- **WHEN** the Team Detail view loads
- **THEN** the Statistics section header is visible but the section content is hidden (collapsed state)

#### Scenario: Statistics section expands on click
- **WHEN** the user clicks the Statistics section header
- **THEN** the section content becomes visible

#### Scenario: Statistics section collapses on second click
- **WHEN** the Statistics section is expanded and the user clicks the header again
- **THEN** the section content is hidden again

#### Scenario: Not-enough-data placeholder shown early in season
- **WHEN** `tableEntry.matches < 3`
- **THEN** the Statistics section shows "Not enough data" and no computed stats are rendered

---

### Requirement: Team detail — Home/Away Split table
The Statistics section SHALL display a Home/Away Split comparison table with rows for Played (P), Wins (W), Draws (D), Losses (L), Goals For (GF), Goals Against (GA), and Goal Difference (GD), with separate columns for Home and Away. Values SHALL be computed from the full season match list for the team (`get_team_matches` response).

#### Scenario: Home/Away split values are correct
- **WHEN** the team has played 5 home games (3W, 1D, 1L, 12 GF, 5 GA) and 4 away games (1W, 2D, 1L, 4 GF, 4 GA)
- **THEN** the table shows those exact values under the Home and Away columns respectively

#### Scenario: Home/Away GD computed correctly
- **WHEN** home GF = 12 and home GA = 5
- **THEN** the Home GD cell shows +7

---

### Requirement: Team detail — Goal Rates row
The Statistics section SHALL display an inline row showing Goals/game scored (to 2dp), Goals/game conceded (to 2dp), Points/game (to 2dp), and total Clean sheets. These values SHALL be derived from `tableEntry` data (goals, opponent_goals, points, matches) and the full season match list.

#### Scenario: Goal rates use table entry data
- **WHEN** `tableEntry.goals = 30`, `tableEntry.opponent_goals = 15`, `tableEntry.matches = 12`
- **THEN** Goals/game scored = 2.50, Goals/game conceded = 1.25

#### Scenario: PPG derived from table entry
- **WHEN** `tableEntry.points = 22` and `tableEntry.matches = 10`
- **THEN** Points/game shows 2.20

#### Scenario: Division by zero guarded
- **WHEN** `tableEntry.matches === 0`
- **THEN** all rate fields show "—" without throwing an error

---

### Requirement: Team detail — Form Guide strip
The Statistics section SHALL display a Form Guide strip showing the team's last 5 finished match results as five colored dots, from oldest (left) to most recent (right). Win = green, Draw = muted/gray, Loss = Bundesliga red. The strip SHALL be labeled "Last 5 results".

#### Scenario: Form dots reflect correct results in order
- **WHEN** the last 5 finished matches (oldest to most recent) produced L, W, D, W, W
- **THEN** dots appear in order: red, green, gray, green, green

#### Scenario: Fewer than 5 finished matches renders partial strip
- **WHEN** only 3 finished matches exist
- **THEN** 3 dots are shown with no placeholder for missing games

---

### Requirement: Team detail — Records row
The Statistics section SHALL display a Records row showing: Longest Unbeaten Run (consecutive W+D count), Best Result (largest positive goal difference in a single match, shown as "X-Y vs Opponent"), and Worst Result (largest negative goal difference in a single match). All records SHALL carry a "Based on match result data" footnote.

#### Scenario: Longest unbeaten run is correct
- **WHEN** the team won 3 then drew 2 then lost 1 then won 4 (all consecutive from season start)
- **THEN** Longest Unbeaten Run shows 5 (the 3W+2D streak before the loss; or 4 if a later streak is longer)

#### Scenario: Best result shows correct score and opponent
- **WHEN** the team's biggest win was 5-0 away at Opponent X
- **THEN** Best Result shows "5-0 vs Opponent X" (with the team's goals first regardless of home/away)

#### Scenario: Records row shown only when sufficient matches played
- **WHEN** `tableEntry.matches >= 3`
- **THEN** the Records row is visible with computed values

---

### Requirement: Team detail — Projection bar
The Statistics section SHALL display a Projection bar showing the team's minimum achievable points (current points), PPG-projected final points, and maximum achievable points for the season. The bar SHALL visually represent this range. An early-season caveat note SHALL be shown if fewer than 6 matches have been played.

#### Scenario: Projection values are computed correctly
- **WHEN** `tableEntry.points = 24`, `tableEntry.matches = 12`, and total matchdays = 34
- **THEN** min = 24, projected = round(24/12 × 34) = 68, max = 24 + 3×22 = 90

#### Scenario: Total matchdays default to 34 for Bundesliga
- **WHEN** the selected league is bl1
- **THEN** `matches_remaining = 34 - tableEntry.matches` is used for max_pts and projected_pts

#### Scenario: Early-season note shown for projections
- **WHEN** `tableEntry.matches < 6`
- **THEN** the Projection section shows "Season is in early stages — projection ranges are wide"

---

### Requirement: Team detail — Qualification Scenarios
The Statistics section SHALL display a Qualification Scenarios section for Bundesliga (bl1) teams. The section SHALL show the team's current qualification outlook: CL qualification (top 2), EL qualification (rows 3-4), Conf. League (row 5), relegation playoff (row 16), and direct relegation (rows 17-18). Each zone SHALL show whether the team is "Currently in", "In contention", or "Mathematically out". For leagues other than bl1, the section SHALL be hidden unless a generic threshold is applicable.

#### Scenario: Team in CL position shows "Currently in" for CL
- **WHEN** the team is in position 1 or 2 in the table
- **THEN** the CL qualification row shows "Currently in" with a blue indicator

#### Scenario: Team with no path to a zone shows "Mathematically out"
- **WHEN** a team's `max_points` cannot reach the minimum points of the target zone's current occupant
- **THEN** that zone row shows "Mathematically out"

#### Scenario: Team within reach of a zone shows "In contention"
- **WHEN** a team could still mathematically reach a European zone
- **THEN** that zone row shows "In contention" with the points gap displayed

#### Scenario: Qualification section hidden for non-bl1 leagues
- **WHEN** the selected league is not bl1
- **THEN** the Qualification Scenarios section is not rendered in the Statistics section

---

### Requirement: Team detail — Statistics section uses existing fetched data
The Statistics section SHALL derive all its values from data already fetched by the team detail view (the table entry and the full match list for the team). No additional data-layer fetches SHALL be made specifically for the Statistics section.

#### Scenario: No additional network requests made for stats
- **WHEN** the Statistics section is expanded
- **THEN** no new fetches are triggered; all values are computed from already-loaded state
