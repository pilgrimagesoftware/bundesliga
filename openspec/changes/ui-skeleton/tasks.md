## 1. Design tokens

- [x] 1.1 Convert the mockup's Style A OKLCH light/dark palette to `Hsla` constants (background, surface, text tiers, border tiers, accent, accent-soft, accent-on, error) and replace the placeholder values in `crates/fulltime-ui/src/data/theme.rs`.
- [x] 1.2 Add per-league accent colors (EPL, LaLiga, Serie A, Bundesliga, Ligue 1) as a lookup independent of the active theme.
- [x] 1.3 Add zone-highlight colors (UCL, UEL, relegation) for light and dark mode.
- [x] 1.4 Add form-indicator colors (win/draw/loss), stable across themes.
- [x] 1.5 Add Style A typography tokens: `Sora` heading font, `Manrope` body font, and the type scale (brand, hero title, section title, score, body).
- [x] 1.6 Add radius token (`16px` base) and derived variants.
- [x] 1.7 Spot-check rendered light/dark colors against the mockup screenshots for each token category.

## 2. Header and navigation shell

- [x] 2.1 Remove `crates/fulltime-ui/src/ui/views/sidebar.rs` and its 200px reservation in `root_view.rs`.
- [x] 2.2 Remove/replace `title_bar.rs` and `toolbar.rs` with a single new header view module.
- [x] 2.3 Implement the brand block (circular mark + "Fulltime" wordmark) in the new header.
- [x] 2.4 Implement the league tab bar (5 static leagues) with active/inactive state.
- [x] 2.5 Implement the screen navigation control (Standings/Match/History/Player/Team) wired to `AppScreen` state.
- [x] 2.6 Implement the light/dark theme toggle button, wired to existing theme persistence.
- [x] 2.7 Update `root_view.rs` to compose header + content area + status bar (no sidebar).
- [x] 2.8 Add/update i18n keys for header, nav, and league labels in `crates/fulltime-ui/i18n/en.yaml`.

## 3. Shared shell components

- [x] 3.1 Create `crates/fulltime-ui/src/ui/views/components/` module.
- [x] 3.2 Implement hero banner component.
- [x] 3.3 Implement card component.
- [x] 3.4 Implement team badge/avatar component (circular, initials-based).
- [x] 3.5 Implement form-indicator dots component (5-dot strip).
- [x] 3.6 Implement status pill component (live/FT/scheduled variants).
- [x] 3.7 Implement tab bar component (reused by header screen nav and Match view tabs).
- [x] 3.8 Implement back button component.
- [x] 3.9 Implement stat cell grid component (3-column).
- [x] 3.10 Implement legend item component (colored dot + label).

## 4. View layouts

- [x] 4.1 Define `AppScreen` enum (Standings, Match, History, Player, Team) and app-level active-screen state, defaulting to Standings.
- [x] 4.2 Implement Standings screen: hero band, standings table shell, matchday rail shell, top-scorers shell.
- [x] 4.3 Implement Match screen: back button, score header shell, 3-tab row (Summary/Lineups/Stats) with placeholder tab bodies.
- [x] 4.4 Implement History screen: back button, hero, accordion list shell with working expand/collapse.
- [x] 4.5 Implement Player screen: back button, detail hero, 3-column stat grid.
- [x] 4.6 Implement Team screen: back button, detail hero, 3-column stat grid, form-dots row.
- [x] 4.7 Wire content-area rendering to switch between the five screens based on `AppScreen` state.
- [x] 4.8 Add persistent footer disclaimer text beneath the content area.

## 5. Verification and follow-up

- [x] 5.1 Run `cargo build` and `cargo clippy` for `fulltime-ui`; fix warnings.
- [x] 5.2 Manually run the app and visually verify each of the five screens in both light and dark mode against the mockup.
- [x] 5.3 Verify no `modalBackdrop`/`modalCard`/`modalClose` tokens or modal-based Player/Team presentation exist in the ported code.
- [x] 5.4 File follow-up notes on `bundesliga-sports-ui`, `stats-view`, `season-picker-ux`, and `response-errors-ui` proposals flagging that their sidebar-based nav assumptions need revision against this header-based shell before their implementation starts.
