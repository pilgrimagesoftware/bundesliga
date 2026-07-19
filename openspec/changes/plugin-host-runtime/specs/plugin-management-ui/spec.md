## ADDED Requirements

### Requirement: Installed Plugin List
`fulltime-ui` SHALL provide a screen listing every registered plugin with its name, version,
and enabled/disabled state.

#### Scenario: User opens the plugin management screen
- **WHEN** a user navigates to plugin management
- **THEN** the screen lists every plugin the registry has discovered, including both
  bundled and user-installed plugins

### Requirement: Enable/Disable Control
The plugin management screen SHALL let a user enable or disable any registered plugin, and
SHALL reflect the change immediately without requiring an app restart.

#### Scenario: User toggles a plugin's enabled state
- **WHEN** a user disables an enabled plugin from the plugin management screen
- **THEN** the host stops invoking that plugin, and the screen reflects the new state
  immediately

### Requirement: Update Availability Indicator
The plugin management screen SHALL show when an update is available for a plugin that
declares an update source.

#### Scenario: Update is available for a plugin
- **WHEN** the registry has detected that a newer version exists for an installed plugin
- **THEN** the plugin management screen displays an update-available indicator next to
  that plugin
