# design-tokens Specification

## Purpose

TBD - created by archiving change ui-skeleton. Update Purpose after archive.

## Requirements

### Requirement: Light and dark color tokens
The system SHALL provide a complete `ColorTokens` set for both a light theme (`Pitch`) and a dark theme (`PitchNight`), covering background, surface, primary/secondary/tertiary text, border, border-strong, accent, accent-soft, accent-on, and error colors, derived from the Claude Design mockup's Style A OKLCH palette.

#### Scenario: Theme provides all required tokens
- **WHEN** `FullTimeTheme` is constructed for either `ThemeKey::Pitch` or `ThemeKey::PitchNight`
- **THEN** the resulting `ColorTokens` has non-default, mockup-derived values for every field (background, surface, text tiers, border tiers, accent tiers, error)

#### Scenario: Dark mode uses the mockup's dark canvas colors
- **WHEN** `ThemeKey::PitchNight` is active
- **THEN** `desktop_bg` matches the mockup's dark-mode canvas color and `surface`/`text_primary` follow the mockup's dark-mode surface/text values

### Requirement: Per-league accent colors
The system SHALL provide an accent color for each of the five leagues shown in the header (EPL, LaLiga, Serie A, Bundesliga, Ligue 1), matching the mockup's per-league OKLCH hue values, available independently of the active light/dark theme's generic accent token.

#### Scenario: Each league resolves a distinct accent
- **WHEN** a league accent color is requested for any of the five supported leagues
- **THEN** the returned color is distinct from the other four leagues' accent colors and matches the mockup's hue for that league

### Requirement: Zone-highlight colors
The system SHALL provide background highlight colors for the three standings qualification zones shown in the mockup: UEFA Champions League, UEFA Europa League, and relegation, with separate light-mode and dark-mode variants.

#### Scenario: Zone colors differ by mode
- **WHEN** a zone-highlight color (UCL, UEL, or relegation) is resolved for light mode versus dark mode
- **THEN** the light-mode and dark-mode colors differ in lightness consistent with the mockup's `93%`/`28%` lightness split

### Requirement: Form-indicator colors
The system SHALL provide three form-indicator colors - win (green), draw (neutral gray), loss (red) - matching the mockup's form-dot palette, independent of theme mode.

#### Scenario: Form colors are stable across themes
- **WHEN** the win/draw/loss form colors are resolved under `Pitch` and under `PitchNight`
- **THEN** each of the three colors keeps the same hue identity (green/gray/red) in both themes

### Requirement: Style A typography tokens
The system SHALL provide Style A typography tokens: heading font family (`Sora`), body font family (`Manrope`), and a type scale covering brand text, hero title, section title, score display, and body text sizes, matching the mockup's Style A values.

#### Scenario: Font selections resolve to Style A families
- **WHEN** `FontSelections` is constructed
- **THEN** the heading font family is `Sora` and the body font family is `Manrope`

#### Scenario: Type scale covers all named sizes
- **WHEN** the type scale is queried for brand, hero-title, section-title, score, and body sizes
- **THEN** each returns a distinct pixel size consistent with the mockup's Style A scale (body text in the 12.5-13.5px range, score at 44px, hero title at 34px)

### Requirement: Radius and spacing tokens
The system SHALL provide a base radius token matching the mockup's Style A value (`16px`) and derived radius variants (smaller and larger than base) usable by pill-shaped and card-shaped elements.

#### Scenario: Base radius matches Style A
- **WHEN** the base radius token is read
- **THEN** its value is `16px`

### Requirement: No Style B tokens
The system SHALL NOT expose a Style B token set or a runtime style-variant switch; only Style A tokens are defined.

#### Scenario: No variant toggle exists
- **WHEN** the theme/token API is inspected
- **THEN** there is no enum, field, or function that selects between "Style A" and "Style B"
