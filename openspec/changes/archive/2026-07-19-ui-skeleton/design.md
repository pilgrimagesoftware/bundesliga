## Context

`fulltime-ui` is a GPUI desktop crate that currently renders only an empty themed window (`RootView` → title bar + blank sidebar + empty toolbar + empty status bar). A Claude Design mockup, "Football Scores and tracking app" (`Football Scores.dc.html`, 1143 lines, exported as static HTML/CSS/JS with a `dc-props` config surface), defines the intended visual language and screen structure. It ships two visual variants (Style A: rounded/pill, `Sora`+`Manrope`; Style B: sharp/square, `Space Grotesk`+`IBM Plex Sans`) toggled live via a segmented control, and six view states (header chrome + Standings/Match/History/Player/Team) switched by a `view` state variable — no router, no URL, no sidebar.

The already-authored `bundesliga-sports-ui` proposal assumes a left sidebar for screen navigation (Table/Matches/Teams/Stats). That proposal is unimplemented (tasks all pending). Per product decision, this change follows the mockup's header-based nav instead, which means `bundesliga-sports-ui` (and `stats-view`, `season-picker-ux`, `response-errors-ui`) will need their nav-related sections revisited before their own implementation — tracked as follow-up, not blocking this change.

## Goals / Non-Goals

**Goals:**
- Port the mockup's Style A design tokens (color, typography, radius) into `ColorTokens`/`FullTimeTheme` for both light and dark modes.
- Replace the sidebar/toolbar/title-bar scaffold with a single persistent header (brand, league tabs, screen nav, theme toggle).
- Add view-switching state and empty-state (no data) structural layouts for all five mockup views.
- Extract the mockup's reusable visual components (badge, card, stat grid, tab bar, status pill, form dots, etc.) into shared GPUI helpers other changes can build on.

**Non-Goals:**
- No real data fetching, no OpenLigaDB/TheSportsDB integration (owned by `bundesliga-sports-ui` and friends).
- No Style B support or runtime style-variant toggle — Style A only.
- No interactive behavior beyond view-switching and the theme toggle (no working matchday stepper, no accordion expand, no tab content beyond static placeholders).
- No persistence of selected view/league beyond what the existing theme-persistence mechanism already covers.
- No revision of `bundesliga-sports-ui`/`stats-view`/`season-picker-ux`/`response-errors-ui` proposal text — flagged as follow-up only.

## Decisions

**Header-based nav replaces the sidebar.** The mockup has no sidebar; every view shares one sticky header. Keeping a sidebar (from `bundesliga-sports-ui`) would fork the app's navigation model in two directions before either is built. Removing it now, while `bundesliga-sports-ui` is still unimplemented, costs nothing beyond editing one already-superseded proposal. Alternative considered: keep both (header + sidebar) — rejected as it isn't what the mockup shows and adds a redundant nav surface with no current use.

**View-switching via a plain Rust enum + GPUI entity state, not a router.** The mockup itself uses a single `view` state variable with `sc-if`-gated view blocks — no routing library, no history stack shown. Mirror that directly: `enum AppScreen { Standings, Match, History, Player, Team }` held in `RootView`'s (or a new `AppState`) entity, switched by header nav clicks, rendered via a `match` in the content area. Alternative considered: a small router/history abstraction for back/forward — rejected as premature; the mockup's own back buttons are simple "return to previous screen" actions, not history-stack navigation, and there's no data yet to navigate between.

**Style A tokens only, hardcoded, no variant enum.** Style B roughly doubles every token (radius, two font families, shadow vs. border cards). Since the product decision is to ship one variant, encoding an unused Style B enum now is speculative work with no consumer. If Style B is wanted later, it's an additive change to `ColorTokens`/`FontSelections`, not a rewrite.

**Shared shell components live in a new `ui/views/components/` module.** The mockup's per-view compositions (hero, card, badge, stat grid, tab bar, status pill, form dots, legend item) recur across 3+ views each. Extracting them now avoids five near-duplicate implementations and gives `bundesliga-sports-ui`/`stats-view` real building blocks instead of ad hoc `div()` trees. Alternative considered: inline everything per-view and refactor later — rejected per the "leave code cleaner than you found it" bar, since the duplication is visible from the mockup summary before any code is written.

**League tabs and screen nav render statically (no live league list, no fetch).** Five hardcoded league labels (EPL, LaLiga, Serie A, Bundesliga, Ligue 1) matching the mockup, matching `bundesliga-sports-ui`'s eventual league source. No API call — this change has no data layer.

**Color tokens computed from the mockup's OKLCH formulas, not copied as literal hex.** The mockup derives league accents as `oklch(62% 0.17 <hue>)` per league and zone highlights as `oklch(<93%|28%> <chroma> <hue>)` depending on mode. GPUI's `Hsla` doesn't support OKLCH directly, so token values are pre-computed (converted to HSLA) once at design time and stored as constants — not computed at runtime. Alternative considered: runtime OKLCH→HSLA conversion — rejected as unnecessary complexity for a static, small token set.

## Risks / Trade-offs

- [Dropping the sidebar contradicts the authored `bundesliga-sports-ui`/`stats-view` proposals] → Those proposals are unimplemented; this change's proposal.md documents the conflict under Impact. A follow-up task updates their nav-related text before implementation starts (tracked in tasks.md, not blocking this change's own completion).
- [Empty-state layouts built without real data risk not matching actual data shapes once `bundesliga-sports-ui` lands] → Layouts are built directly from the mockup's static HTML structure (known column sets, known field counts), which is a stronger source of truth than guessing; any mismatch is a small follow-up adjustment, not a rebuild.
- [OKLCH→HSLA manual conversion introduces rounding/hue-mapping error versus the mockup] → Values are visually spot-checked against rendered mockup screenshots during implementation (per view, light and dark) rather than trusted from formula alone.
- [Removing `title_bar.rs`/`toolbar.rs`/`sidebar.rs` is a breaking internal API change] → No external consumers exist yet (crate is UI-only scaffold, unshipped); safe to remove outright rather than deprecate.

## Migration Plan

1. Land design tokens (`theme.rs`) first — additive-only until the header/views switch over, so intermediate commits stay buildable.
2. Build the new header view and `AppScreen` state, wire into `RootView`, remove `sidebar.rs`/old `title_bar.rs`/`toolbar.rs` in the same pass (avoids a half-migrated shell rendering incorrectly).
3. Add the five view modules and shared components incrementally, each independently buildable/previewable.
4. No data migration, no persisted-state schema change beyond existing theme key — no rollback complexity beyond reverting the commits.

## Open Questions

- None blocking — style-variant support (Style B) and real routing/history are explicitly deferred per Non-Goals above, not left ambiguous.
