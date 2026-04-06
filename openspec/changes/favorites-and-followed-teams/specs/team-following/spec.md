## ADDED Requirements

### Requirement: User can follow multiple teams
The system SHALL allow the user to follow any number of teams. Followed teams are a set distinct from the favorite. The favorite team SHALL NOT also appear in the followed set. The followed team ids SHALL be persisted across app restarts.

#### Scenario: Follow a team from the table
- **WHEN** user clicks the heart icon on a non-favorite team row in the league table
- **THEN** that team is added to the followed set and the heart icon fills amber

#### Scenario: Unfollow a team
- **WHEN** user clicks the filled heart icon on an already-followed team
- **THEN** that team is removed from the followed set and the heart icon becomes unfilled

#### Scenario: Follow a team from team card
- **WHEN** user clicks the heart icon on a TeamCard
- **THEN** that team is added to or removed from the followed set

#### Scenario: Follow from team detail
- **WHEN** user clicks the follow/unfollow button in the team detail identity section
- **THEN** the team is added to or removed from the followed set

#### Scenario: Cannot follow the favorite team
- **WHEN** user attempts to follow the current favorite team
- **THEN** the action is silently ignored and the heart icon is not shown for the favorite (star takes precedence)

### Requirement: Followed teams are visually highlighted everywhere
Followed team rows, cards, and detail headers SHALL be visually distinguished with an amber left border (`#f59e0b`) and a filled heart icon (🧡) in all views where teams appear.

#### Scenario: Highlighted in league table
- **WHEN** one or more followed teams are set and the league table is displayed
- **THEN** each followed team's row has an amber left border and a 🧡 icon

#### Scenario: Highlighted in team grid
- **WHEN** one or more followed teams are set and the teams grid is displayed
- **THEN** each followed team's card has an amber border and 🧡 icon

#### Scenario: Highlighted in team detail
- **WHEN** viewing the team detail of a followed team
- **THEN** the identity section shows a filled 🧡 button in amber

### Requirement: Followed teams are pinned below the favorite in the league table
The system SHALL render all followed teams immediately after the favorite (if any), before the rest of the table, separated by a visual divider.

#### Scenario: Followed rows pinned below favorite
- **WHEN** a favorite and one or more followed teams are set
- **THEN** the favorite appears first, followed by all followed teams (in their natural league order), then a divider, then all remaining teams

#### Scenario: Followed rows pinned at top when no favorite
- **WHEN** followed teams are set but no favorite is set
- **THEN** the followed teams appear at the top of the table (in their natural league order), then a divider, then remaining teams

### Requirement: Matches view shows a "My Teams" filter tab
The matches view SHALL include a "My Teams" tab that, when selected, filters match cards to only show matches involving the user's favorite or followed teams.

#### Scenario: My Teams tab shows relevant matches
- **WHEN** user selects the "My Teams" tab in the matches view
- **THEN** only match cards where `team1` or `team2` is the favorite or a followed team are displayed for the current matchday

#### Scenario: Empty state when no teams selected
- **WHEN** user selects "My Teams" tab and no favorite or followed teams are set
- **THEN** an empty state message "No followed teams yet — star or follow teams to track them here" is displayed

#### Scenario: Empty state when no relevant matches on matchday
- **WHEN** user selects "My Teams" tab and the current matchday has no matches involving followed/favorite teams
- **THEN** an empty state message "None of your teams play this matchday" is displayed

#### Scenario: Tab persists during matchday navigation
- **WHEN** user is on the "My Teams" tab and navigates to a different matchday
- **THEN** the filter stays active and the new matchday's matches are filtered accordingly

### Requirement: Followed team preferences persist across restarts
The followed team ids SHALL be saved to `app_state.json` whenever they change and restored on next app launch.

#### Scenario: Restored on startup
- **WHEN** the app launches and `app_state.json` contains `followed_team_ids`
- **THEN** those teams are immediately treated as followed in all views without user action
