## Why

The `fulltime-ui` GPUI crate currently renders only an empty themed window: a title bar, a blank 200px sidebar, an empty toolbar, and an empty status bar, all using placeholder color tokens. The Claude Design mockup ("Football Scores and tracking app") already defines the real visual language and screen structure — header-based navigation, six views (header chrome plus Standings, Match, History, Player, Team), and a full light/dark color and typography system — but none of it has been ported into the GPUI shell. Later feature changes (`bundesliga-sports-ui`, `stats-view`, `season-picker-ux`) need real layouts and design tokens to build against instead of an empty pane.

## What Changes

- **BREAKING**: Remove the left sidebar (`render_sidebar`, `crates/fulltime-ui/src/ui/views/sidebar.rs`) and its 200px reservation in `RootView`. The mockup uses a single top header for all navigation, superseding the sidebar nav concept assumed by the (not-yet-implemented) `bundesliga-sports-ui` proposal.
- Replace the placeholder `ColorTokens` in `crates/fulltime-ui/src/data/theme.rs` with the full token set extracted from the mockup's Style A / light+dark OKLCH palette: background, surface, text (primary/muted), border, per-league accent colors, zone-highlight colors (UCL/UEL/relegation), and form-indicator colors (win/draw/loss).
- Add typography tokens for Style A: heading font (`Sora`), body font (`Manrope`), and the mockup's type scale (brand, hero title, section title, score, body).
- Add radius/spacing tokens matching Style A (`radius: 16`, pill-shaped controls).
- Rebuild the header (`render_title_bar` → generalized header view) to contain: brand mark + wordmark, horizontal league tab bar (5 leagues, no live data — static labels), and a light/dark theme toggle button. Drop the Style A/B segmented control (Style A is the only shipped variant per this change).
- Add app-level view-switching state (`AppScreen` or similar enum: Standings, Match, History, Player, Team) and wire the header's screen navigation control to it, replacing the toolbar's current empty shell.
- Add empty-state (no real data, static/placeholder content) layout structure for each of the 5 views, matching the mockup's composition:
  - Standings: hero band + two-column grid (standings table shell, matchday rail shell, top-scorers shell).
  - Match: back button + score header shell + 3-tab row (Summary/Lineups/Stats) with empty tab bodies.
  - History: back button + hero + accordion list shell (no expand data).
  - Player: back button + detail hero + 3-column stat grid shell.
  - Team: back button + detail hero + 3-column stat grid shell + form-dots row shell.
- Add shared reusable shell components used across views: hero banner, card, team badge/avatar, form-indicator dots, status pill, tab bar, back button, stat cell grid, legend item.
- Do not port the mockup's unused `modalBackdrop`/`modalCard`/`modalClose` tokens (dead code in the source mockup).
- No real league/match/team/stats data, no API integration, no persistence beyond theme selection — this change is layout and visual scaffolding only.

## Capabilities

### New Capabilities
- `design-tokens`: Color (light/dark, per-league accent, zone highlights, form colors), typography, and radius/spacing tokens matching the Claude Design mockup's Style A variant.
- `app-header-nav`: Persistent header chrome — brand, league tabs, screen navigation, and light/dark theme toggle — replacing the sidebar/toolbar/title-bar scaffold.
- `view-shell-layouts`: Empty-state structural layouts for the five app views (Standings, Match, History, Player, Team) and the shared shell components they're built from.

### Modified Capabilities
(none — no existing `openspec/specs/` capabilities predate this change)

## Impact

- `crates/fulltime-ui/src/data/theme.rs`: token set rewritten.
- `crates/fulltime-ui/src/ui/views/sidebar.rs`: removed.
- `crates/fulltime-ui/src/ui/views/title_bar.rs`, `toolbar.rs`: superseded by a new header view; status bar unaffected.
- `crates/fulltime-ui/src/ui/views/root_view.rs`: layout recomposed around header + view-switch content area, no sidebar.
- `crates/fulltime-ui/src/ui/views/`: five new view modules (standings, match, history, player, team) plus a `components/` module for shared shell pieces.
- `crates/fulltime-ui/i18n/en.yaml`: new keys for header/nav/view labels.
- Downstream: `bundesliga-sports-ui`, `stats-view`, `season-picker-ux`, `response-errors-ui` proposals assume a sidebar-based nav and will need their designs revisited against this header-based shell before implementation.
