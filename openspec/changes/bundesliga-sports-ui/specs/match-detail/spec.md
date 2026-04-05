## ADDED Requirements

### Requirement: Match detail header
The match detail view SHALL display both team logos, full team names, and the final or current score prominently at the top. A back button SHALL return the user to the matchday that was previously viewed.

#### Scenario: Back button returns to originating matchday
- **WHEN** user presses the back button on the match detail view
- **THEN** the view state transitions to `{ screen: 'matches', matchday: <fromMatchday> }`

#### Scenario: Match header shows both teams and score
- **WHEN** match detail loads
- **THEN** team1 logo, name, score, separator, score, team2 logo, and name are all visible in the header area

### Requirement: Goal timeline
The match detail view SHALL display an ordered list of all goals scored, showing: the minute the goal was scored, the scorer's name, the running score at that point, and badges for penalty and own-goal events.

#### Scenario: Goals listed in chronological order
- **WHEN** match detail loads and the match has goals
- **THEN** goals are displayed in ascending minute order

#### Scenario: Penalty goal shows badge
- **WHEN** a goal has `is_penalty: true`
- **THEN** a "PEN" badge is displayed alongside the goal entry

#### Scenario: Own goal shows badge
- **WHEN** a goal has `is_own_goal: true`
- **THEN** an "OG" badge is displayed alongside the goal entry and the goal is attributed to the conceding team's column

#### Scenario: No goals shows empty state
- **WHEN** a finished match has zero goals
- **THEN** the goal timeline shows a "No goals scored" message

### Requirement: Match metadata
The match detail view SHALL display supplementary match information: venue/stadium name, number of viewers (if available), and match date/time.

#### Scenario: Venue displayed when available
- **WHEN** match location data is present
- **THEN** the stadium name is shown below the goal timeline

#### Scenario: Viewer count displayed when available
- **WHEN** `number_of_viewers` is non-null
- **THEN** the viewer count is formatted with thousands separators and displayed
