## ADDED Requirements

### Requirement: Embedded font assets
The system SHALL embed the Sora and Manrope font files as compiled-in binary assets and register them with GPUI's text system during app startup, before any window is created, so the `heading_font` (`Sora`) and `body_font` (`Manrope`) tokens in `FontSelections` render using the vendored fonts rather than depending on fonts installed on the host OS.

#### Scenario: Fonts render without OS-level installation
- **WHEN** the app starts on a machine with no Sora or Manrope fonts installed at the OS level
- **THEN** text rendered with the `heading_font` or `body_font` family uses the app's embedded Sora/Manrope glyphs, not a substituted system font

#### Scenario: Font registration happens before first render
- **WHEN** the app's startup sequence runs
- **THEN** the embedded fonts are registered with the text system before the first window's first frame is drawn

#### Scenario: Font registration failure is not silent
- **WHEN** an embedded font file fails to register with the text system
- **THEN** app startup fails loudly (returns/propagates the error) rather than continuing with a mismatched fallback font
