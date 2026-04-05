## ADDED Requirements

### Requirement: Matchday list with navigation
The app SHALL display all matches for the selected matchday as a list of match cards. The view SHALL include prev/next navigation buttons to move between matchdays. The current matchday name (e.g., "26. Spieltag") SHALL be displayed as a heading.

#### Scenario: Prev/next navigation loads adjacent matchday
- **WHEN** user clicks the next arrow on matchday 10
- **THEN** the view loads and displays the matches for matchday 11

#### Scenario: Prev button disabled on first matchday
- **WHEN** the selected matchday is the first of the season
- **THEN** the prev navigation button is disabled

#### Scenario: Next button disabled on last available matchday
- **WHEN** the selected matchday is the last available matchday for the season
- **THEN** the next navigation button is disabled

### Requirement: Match card displays teams and score
Each match card SHALL display: home team logo and name, away team logo and name, the current or final score, and a status indicator (kick-off time for future matches, "FT" for finished matches, live minute for in-progress matches).

#### Scenario: Finished match card shows final score and FT
- **WHEN** a match is finished
- **THEN** the match card shows the final score and "FT" status label

#### Scenario: Future match card shows kick-off time
- **WHEN** a match has not yet started
- **THEN** the match card shows the scheduled kick-off time in the user's local timezone

#### Scenario: In-progress match card shows live minute
- **WHEN** a match is in progress (not finished, within 2h of kick-off)
- **THEN** the match card shows a pulsing indicator and the current approximate minute

### Requirement: Match card navigates to match detail
Clicking a match card SHALL navigate to the match detail view for that match.

#### Scenario: Click match card opens match detail
- **WHEN** user clicks a match card
- **THEN** the view state transitions to `{ screen: 'match_detail', matchId: <id>, fromMatchday: <n> }`
