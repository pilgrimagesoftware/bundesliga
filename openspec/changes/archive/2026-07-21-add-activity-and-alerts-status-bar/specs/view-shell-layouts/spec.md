## MODIFIED Requirements

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
