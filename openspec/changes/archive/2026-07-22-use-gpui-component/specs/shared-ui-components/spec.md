## ADDED Requirements

### Requirement: gpui-component widgets are used where one covers the need
`fulltime-ui` SHALL render a shared visual primitive (avatar/initials badge, bordered card
container, status pill/tag, icon-prefixed text button) using the corresponding
`gpui-component` widget rather than a hand-rolled `gpui` implementation, whenever
`gpui-component`'s pinned revision ships a widget that covers the primitive's behavior.

#### Scenario: Team/player badge renders via Avatar
- **WHEN** a screen renders a team or player identity badge (circular, initials-based)
- **THEN** it is rendered using `gpui_component::avatar::Avatar`, not a hand-rolled `div`-based
  implementation

#### Scenario: Bordered content container renders via GroupBox
- **WHEN** a screen groups content in a bordered, rounded surface (fixture rail entry, stat
  panel, tab body)
- **THEN** it is rendered using `gpui_component::group_box::GroupBox`, not a hand-rolled
  `div`-based implementation

#### Scenario: Match status indicator renders via Tag
- **WHEN** a screen renders a match status indicator (live/full-time/scheduled)
- **THEN** it is rendered using `gpui_component::tag::Tag` with a variant driven by
  `FullTimeTheme`'s success/warning/info/danger colors, and the `Live` state's pulse animation
  is preserved by wrapping the `Tag` element, not reimplemented from scratch

#### Scenario: Arrow-prefixed back navigation renders via Button
- **WHEN** a screen renders a back-navigation control
- **THEN** it is rendered using `gpui_component::button::Button` with a leading arrow `Icon`,
  not a hand-rolled `div`-based implementation

### Requirement: Custom components without a gpui-component equivalent remain unmigrated
`fulltime-ui` SHALL keep a hand-rolled `gpui` implementation for a visual primitive when no
`gpui-component` widget in the pinned revision covers its behavior, rather than forcing a
mismatched widget into that role.

#### Scenario: Win/draw/loss form indicator stays custom
- **WHEN** a screen renders the win/draw/loss form-indicator dot row
- **THEN** it continues to be rendered by the existing hand-rolled implementation, because no
  `gpui-component` widget represents a multi-dot result strip

#### Scenario: Colored-dot legend stays custom
- **WHEN** a screen renders a colored-dot-plus-label legend row
- **THEN** it continues to be rendered by the existing hand-rolled implementation, distinct in
  shape from the `Tag`-based status pill so the two remain visually distinguishable where they
  appear together

### Requirement: FullTime theme colors reach every semantic Tag variant
`gpui_component::Theme`'s success, warning, and info color fields SHALL be populated from
`FullTimeTheme`'s `ColorTokens` on theme application, matching the existing treatment of
primary/secondary/danger, so a `Tag` using any semantic variant renders in the active FullTime
palette rather than `gpui-component`'s built-in defaults.

#### Scenario: Theme switch updates a semantic Tag's color
- **WHEN** the active `FullTimeTheme` changes
- **THEN** a `Tag` rendered with `TagVariant::Success`, `Warning`, or `Info` reflects the new
  theme's corresponding color on the next render, the same way `Danger`/`Primary`/`Secondary`
  already do
