## ADDED Requirements

### Requirement: Sidebar navigation with three sections
The app SHALL display a persistent left sidebar with navigation items: Table, Matches, and Teams. The active section SHALL be visually highlighted. Clicking a nav item SHALL update the in-page view state and render the corresponding content pane.

#### Scenario: User switches to Matches view
- **WHEN** user clicks "Matches" in the sidebar
- **THEN** the Matches view is rendered in the content pane and the Matches nav item is highlighted

#### Scenario: Active nav item is highlighted on load
- **WHEN** the app restores last-viewed state to the Teams view
- **THEN** the Teams nav item is highlighted on initial render without any user action

### Requirement: Header with league and season pickers
The app SHALL display a header containing the app name/logo, a league dropdown, and a season dropdown. Changing either picker SHALL reload the current view's data for the new selection.

#### Scenario: User changes league
- **WHEN** user selects a different league from the header dropdown
- **THEN** the current view refreshes with data for the newly selected league and the default season for that league

#### Scenario: Season list is derived from current year
- **WHEN** the season picker is rendered
- **THEN** it SHALL show four options: current year, current year − 1, current year − 2, current year − 3

### Requirement: Header refresh button with cooldown feedback
The app SHALL display a refresh button in the header. During a cooldown period the button SHALL be disabled and display a "last updated X ago" label. After the cooldown expires the button SHALL re-enable.

#### Scenario: Refresh button disabled during cooldown
- **WHEN** the user refreshes data and the cooldown has not elapsed
- **THEN** the refresh button is disabled and shows how long ago the data was last fetched

#### Scenario: Refresh button re-enables after cooldown
- **WHEN** the cooldown period elapses
- **THEN** the refresh button becomes active without requiring any user action

### Requirement: Live match indicator in header
The app SHALL display a visual "LIVE" badge in the header when at least one match in the currently selected league is in progress (not finished and within two hours of kick-off time).

#### Scenario: Live badge shown during active match
- **WHEN** the current time is within two hours after a match's UTC kick-off time and the match is not finished
- **THEN** the header displays a pulsing red "LIVE" badge

#### Scenario: Live badge hidden when no matches are live
- **WHEN** no matches in the selected league meet the live criteria
- **THEN** no live badge is shown

### Requirement: In-page view state machine
The app SHALL manage navigation via an in-page state object — not URL routing. The state SHALL be serializable and include the active screen identifier and any screen-specific parameters (matchday number, match ID, team ID).

#### Scenario: View state encodes screen and parameters
- **WHEN** the user drills into a match detail from matchday 12
- **THEN** the view state is `{ screen: 'match_detail', matchId: <id>, fromMatchday: 12 }`

#### Scenario: Back navigation returns to parent screen
- **WHEN** the user is on match detail and presses the back button
- **THEN** the view state returns to `{ screen: 'matches', matchday: 12 }`
