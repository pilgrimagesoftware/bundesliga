# view-shell-layouts Specification

## Purpose

TBD - created by archiving change ui-skeleton. Update Purpose after archive.

## Requirements

### Requirement: App screen state
The system SHALL define an `AppScreen` enum with five variants - Standings, Match, History, Player, Team - and hold the currently active screen in application state, defaulting to Standings.

#### Scenario: Default screen is Standings
- **WHEN** the app window first opens
- **THEN** the active screen is Standings and the Standings layout renders in the content area

### Requirement: Standings view empty-state layout
The system SHALL render a Standings screen layout consisting of a hero band (league name, kicker text, season pill, matchday stepper, club-count chip) above a two-column content area: a standings table shell (rank, club, P/W/D/L/GF/GA/GD/Pts columns, a 5-dot form strip per row, and a zone-legend row) on the left, and a matchday-fixture-rail shell plus top-scorers-list shell on the right - all populated with static placeholder content, no live data.

#### Scenario: Standings layout renders both columns
- **WHEN** the active screen is Standings
- **THEN** the content area shows the hero band, a left column containing the standings table shell, and a right column containing the matchday rail shell and top-scorers shell

### Requirement: Match view empty-state layout
The system SHALL render a Match screen layout consisting of a back button, a score header shell (two team badges, center status pill, score, venue/matchday text), and a three-tab row (Summary, Lineups, Stats) where each tab body is an empty placeholder.

#### Scenario: Match layout renders three tabs
- **WHEN** the active screen is Match
- **THEN** the content area shows a back button, a score header shell, and a tab bar with Summary, Lineups, and Stats tabs, exactly one of which is active at a time

#### Scenario: Switching match tabs changes the visible tab body
- **WHEN** a user selects a different tab within the Match screen
- **THEN** the previously active tab's body is hidden and the newly selected tab's placeholder body is shown

### Requirement: History view empty-state layout
The system SHALL render a History screen layout consisting of a back button, a hero (league name and season), and an accordion list shell of matchdays where each row can be expanded or collapsed.

#### Scenario: History accordion rows toggle
- **WHEN** a user activates a collapsed matchday row in the History screen
- **THEN** that row expands to show its (placeholder) fixture list, and activating it again collapses it

### Requirement: Player view empty-state layout
The system SHALL render a Player screen layout consisting of a back button, a detail hero (avatar, name, meta line), and a three-column stat grid shell.

#### Scenario: Player layout renders hero and stat grid
- **WHEN** the active screen is Player
- **THEN** the content area shows a back button, a detail hero, and a three-column stat grid

### Requirement: Team view empty-state layout
The system SHALL render a Team screen layout consisting of a back button, a detail hero (badge, name, meta line), a three-column stat grid shell, and a form-indicator dots row.

#### Scenario: Team layout renders hero, stat grid, and form dots
- **WHEN** the active screen is Team
- **THEN** the content area shows a back button, a detail hero, a three-column stat grid, and a row of form-indicator dots

### Requirement: Shared shell components
The system SHALL provide reusable shell components - hero banner, card, team badge/avatar, form-indicator dots, status pill, tab bar, back button, stat cell grid, and legend item - used by two or more of the five view layouts, rather than each view reimplementing its own copies.

#### Scenario: Shared component reused across views
- **WHEN** the detail-hero component is rendered on both the Player screen and the Team screen
- **THEN** both usages render through the same shared component implementation

### Requirement: Footer disclaimer text
The system SHALL render a persistent status bar beneath the content area on every screen,
with the disclaimer text ("Prototype data is illustrative, not live sports data.") on the left
and a right-aligned row of utility buttons — Plugins, Activity, and Alerts — matching the
mockup's disclaimer placement plus the `dtrpg-app.rs`-style status bar button row.

#### Scenario: Disclaimer present on every screen
- **WHEN** any screen is active
- **THEN** the disclaimer text is visible in the status bar's left region

#### Scenario: Utility buttons present on every screen
- **WHEN** any screen is active
- **THEN** the Plugins, Activity, and Alerts buttons are visible in the status bar's right region

### Requirement: No legacy modal tokens ported
The system SHALL NOT include the mockup's unused `modalBackdrop`/`modalCard`/`modalClose` tokens or any modal-based presentation for Player/Team detail views, since the mockup itself uses full-view navigation, not modals, for those screens.

#### Scenario: No modal component exists for detail views
- **WHEN** a user navigates to the Player or Team screen
- **THEN** the screen replaces the content area (full-view navigation) rather than opening as a modal/overlay
