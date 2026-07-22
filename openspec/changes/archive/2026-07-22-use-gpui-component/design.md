## Context

`fulltime-ui`'s theme bridge is already in place: `ui/app/mod.rs::setup` copies
`FullTimeTheme`'s active palette into `gpui_component::Theme` via
`data::theme::apply_theme_colors` on startup, so any `gpui-component` widget that reads colors
through `cx.theme()` (the `ActiveTheme` trait) already renders in FullTime's palette without
extra wiring. That mapping is incomplete, though — it sets `background`, `foreground`, `border`,
`muted`, `primary`, `secondary`, and `danger`, but never `success`, `warning`, or `info`. Those
three fall back to `gpui-component`'s built-in defaults, which is a real risk for any migrated
widget that uses those variants (see Decisions, `status-pill`).

The pinned `gpui-component` revision (`be4c5d30`) ships, among others: `avatar` (circular
image/initials), `group_box` (bordered/rounded container, `Normal`/`Fill`/`Outline` variants),
`tag` (colored label, `Primary`/`Secondary`/`Danger`/`Success`/`Warning`/`Info`/`Color`/`Custom`
variants), and `button` (already used by `back_button`'s sibling call sites elsewhere in the
codebase). It has no widget for: a row of small status dots, a colored-dot-plus-label legend
row, a big-number stat grid, or a page-title hero band — these are FullTime-specific
compositions, not generic UI primitives.

Separately, the workspace `Cargo.toml` pins `gpui` and `gpui_platform` directly against
`zed-industries/zed` (rev `1d217ee3`) — these are not just a transitive pull-through of
`gpui-component`. `gpui-ce` (github.com/gpui-ce/gpui-ce) is a community fork of `gpui` that
tracks upstream Zed but accepts features Zed's own roadmap rejects (custom shaders, tray
support, Wayland touch event translation, per longbridge/gpui-component discussion #1856). The
pinned `gpui-component` revision's own `Cargo.toml` still depends on `gpui`/`gpui_platform`
from `zed-industries/zed` directly, with no `gpui-ce`-specific code path — but that doesn't
block the swap. `gpui-ce`'s own README documents a workspace-level Cargo `[patch]` override for
exactly this "I depend on something that depends on Zed's gpui" case: `[patch]` redirects any
resolution of a package by name and source, regardless of which `Cargo.toml` declared the
dependency, so a patch declared once in fulltime-ui's top-level `Cargo.toml` redirects
`gpui-component`'s internal `gpui`/`gpui_platform` dependency too, without touching
`gpui-component`'s own manifest:

```toml
[patch."https://github.com/zed-industries/zed"]
gpui = { git = "https://github.com/gpui-ce/gpui-ce" }
gpui_platform = { git = "https://github.com/gpui-ce/gpui-ce" }
```

`gpui-ce`'s repo is itself a workspace with matching `gpui`/`gpui_platform`/`gpui_macros`
members, so both package names resolve there. The remaining risk isn't a hard incompatibility —
it's API/behavior drift: `gpui-ce` "tracks upstream" but isn't guaranteed to be at the exact
commit fulltime-ui currently pins (`1d217ee3`), and `gpui-ce` itself frames the patch as working
"provided upstream hasn't introduced breaking changes that dependent libraries haven't yet
adopted." That makes this a build-and-verify task (`cargo build`/`clippy`/`test` plus a full
visual pass), not a wait-for-upstream-merge task.

## Goals / Non-Goals

**Goals:**
- Replace hand-rolled components that duplicate an existing `gpui-component` widget, with no
  visible change to the rendered UI.
- Close the `success`/`warning`/`info` gap in `apply_theme_colors` before any migrated widget
  depends on those variants, so the migration doesn't ship a color regression.
- Leave a clear, file-level record of which custom components were evaluated and kept, so this
  doesn't get re-audited from scratch later.
- Move fulltime-ui's `gpui`/`gpui_platform` dependency from `zed-industries/zed` to `gpui-ce`
  via a workspace-level Cargo `[patch]` override, verified by a clean build, lint, test, and
  full visual pass — not gated on `gpui-component` adopting `gpui-ce` upstream.

**Non-Goals:**
- No visual redesign. If a `gpui-component` widget's default look doesn't match the current
  design, prefer `TagVariant::Custom`/explicit styling over accepting a different look.
- No migration of components with no `gpui-component` equivalent (`form_dots`, `legend`,
  `stat_grid`, `hero`, `widgets::small_caps_text`). These stay as hand-rolled `gpui` code.
- No changes to `alert_history_panel`/`activity_panel`: their row status indicator is plain
  colored text driven by a `Status` enum (`InProgress`/`Complete`/`Failed(message)`) distinct
  from `status_pill.rs`'s `MatchStatus`, not a pill/tag shape — out of scope for this migration.
- No changes to `gpui-component`'s own `Cargo.toml` or a fork of `gpui-component`. The `[patch]`
  override achieves the swap without needing `gpui-component` to change.

## Decisions

**`badge.rs` → `gpui_component::avatar::Avatar`.** `render_badge` (circular, initials-based,
accent-tinted) is exactly what `Avatar` renders. Note the naming trap: `gpui-component` also has
a `badge` module (`gpui_component::badge::Badge`, already used in `status_bar.rs`), which is a
notification-count/dot overlay, not an initials avatar — the custom `badge.rs` maps to `Avatar`,
not to that module.

**`card.rs` → `gpui_component::group_box::GroupBox`.** `render_card` (bordered, rounded,
`surface`-backed container) matches `GroupBox`'s `Outline` or default variant. Confirm padding/
gap defaults against the current `px(12.0)` gap / `px(16.0)` padding before deleting the custom
version; override via `StyleRefinement` if `GroupBox`'s defaults differ.

**`status_pill.rs` → `gpui_component::tag::Tag`, with a theme fix first.** `Tag`'s variant enum
lines up with the three `MatchStatus` states (`Live` → `Success` or `Danger` depending on the
current mapping, `FullTime` → `Secondary`, `Scheduled` → `Info`/`Warning`) — but only once
`apply_theme_colors` also sets `theme.colors.success`, `.warning`, and `.info` (and their
`_foreground`/`_active`/`_hover` counterparts, following the existing `primary`/`secondary`/
`danger` pattern). Do this mapping fix as the first task, independent of the `Tag` migration
itself, so any other future `gpui-component` usage of those variants benefits too. `Tag` has no
built-in pulse animation; keep the existing `AnimationExt`-based opacity loop wrapping the `Tag`
element for the `Live` state rather than dropping it.

**`back_button.rs` → `gpui_component::button::Button`.** A text/ghost-variant `Button` with a
leading arrow `Icon` reproduces the arrow-prefixed text button; use `ButtonVariants::ghost()` or
equivalent (`status_bar.rs` already demonstrates the `Button`/`ButtonVariants` import pattern).

**Keep custom, no equivalent exists:** `form_dots.rs` (win/draw/loss dot row — no comparable
"result strip" widget), `legend.rs` (colored dot + label — closest is `Tag`, but a solid dot
reads differently from a filled pill and the two are used side-by-side with pills elsewhere, so
swapping would blur that visual distinction), `stat_grid.rs` and `hero.rs` (page-level layout
compositions, not discrete widgets), `widgets.rs::small_caps_text` (a string transform, not a
render function — out of scope for a "UI component" migration).

**`alert_history_panel.rs`/`activity_panel.rs`:** out of scope entirely. Their status indicator
renders `Status::InProgress`/`Complete`/`Failed(message)` as plain colored text, not a pill —
a different data model from `status_pill.rs`'s `MatchStatus`, and not a `Tag` fit as-is.

**`gpui` → `gpui-ce`, via a workspace `[patch]`, not a per-crate swap.** A `[patch]` override
applies once, workspace-wide — there's no way (and no need) to migrate `fulltime-ui`'s own
`gpui`/`gpui_platform` usage to `gpui-ce` on one commit while `gpui-component` still resolves
Zed's `gpui`; the patch redirects both simultaneously, keeping `Window`/`App`/`Div` as the same
concrete type across app code and `gpui-component` widgets throughout. Do the patch, dependency
audit (step 8.2), and verification pass together as one unit of work, not incrementally.

## Risks / Trade-offs

- [`Tag`'s default padding/font size doesn't match `status_pill`'s current compact pill] →
  Compare screenshots before/after; override via `StyleRefinement` if needed rather than
  accepting a size change.
- [Extending `apply_theme_colors` for `success`/`warning`/`info` changes colors for every
  existing `gpui-component` widget that already reads those fields (if any do today)] → Audit
  current widget usage (`Combobox`, `TabBar`, `Badge`, `Popover`, `Button`, `StatusBar`) for
  reliance on those three fields before changing them; none currently appear to use
  status-semantic variants, but confirm by grep before merging.
- [`GroupBox` rendering differs subtly from `card.rs` for nested cards (fixture rail entries)]
  → Migrate `card.rs`'s call sites one screen at a time, starting with the lowest-risk screen
  (fewest nested/overlapping cards), and visually diff each before moving to the next.
- [Deleting a custom module before all its call sites are migrated leaves a dangling import] →
  Migrate all call sites of a given component in the same commit as the module deletion; never
  leave a partially-migrated component across a commit boundary.
- [`gpui-ce`'s tracked commit has diverged from the pinned Zed `gpui` revision (`1d217ee3`) in
  an API fulltime-ui or `gpui-component` relies on, so the patch fails to compile] → Run
  `cargo build`/`clippy` immediately after adding the patch, before touching any widget code;
  a compile failure here is expected-case, not exceptional, and should be triaged (patch to a
  specific `gpui-ce` rev close to `1d217ee3` if `main` has drifted too far, or report/track the
  gap) before proceeding.
- [`gpui-ce` compiles fine but a rendering/layout/input-handling behavior differs subtly from
  Zed's `gpui` at runtime, without a compile error to flag it] → The full visual pass (task 8.5)
  covers layout/paint; add a manual interaction check (window resize, input focus, keyboard
  navigation) per screen, since those aren't caught by a static screenshot diff.

## Migration Plan

1. Extend `apply_theme_colors` to map `success`/`warning`/`info` (and their
   `_foreground`/`_active`/`_hover` variants) from `ColorTokens`, verified against every existing
   `gpui-component` widget usage first (see Risks).
2. Migrate `badge.rs` → `Avatar` (lowest risk: single visual property — circle + initials).
3. Migrate `card.rs` → `GroupBox`, one screen at a time.
4. Migrate `back_button.rs` → `Button`.
5. Migrate `status_pill.rs` → `Tag` (depends on step 1), keeping the pulse animation wrapper.
   Call sites: `match_view.rs`, `standings.rs`.
6. Delete each custom module in the same commit that removes its last call site.
7. No rollback mechanism beyond `git revert` per-commit — each component migration is an
   independent, revertable commit, not a single big-bang change.
8. Add the `[patch."https://github.com/zed-industries/zed"]` override (see Context) to the
   top-level `Cargo.toml`, redirecting `gpui` and `gpui_platform` to `gpui-ce/gpui-ce`.
   `gpui-component`'s own `Cargo.toml` is untouched — the patch applies transitively.
9. `cargo build` and `cargo clippy --all-targets --all-features -- -D warnings` against the
   patched dependency graph. Treat any compile failure as an API-drift gap to triage (see
   Risks), not a reason to abandon the patch approach.
10. `cargo test --workspace`, then a full manual visual and interaction pass across every
    screen (task 7.3's list) in both theme variants, since a rendering-backend swap can shift
    layout or input handling even where no widget code changed.
11. If `gpui-ce`'s tracked commit has drifted enough from `1d217ee3` to cause compile or
    behavioral gaps, pin the patch to a specific `gpui-ce` git `rev` close to that point in its
    history rather than floating on its default branch, and note the chosen rev's reasoning in
    the commit message.

## Open Questions

- Does `GroupBox`'s `Fill` variant read close enough to `card.rs`'s current `surface`-tinted
  background to use directly, or does `card.rs` need `StyleRefinement` overrides per call site?
  Resolve during step 3 by comparing rendered output, not by inspection alone.
- Does floating the `[patch]` on `gpui-ce`'s default branch (no `rev` pin) risk unannounced
  breakage on a future `cargo update`, given the rest of the workspace pins exact revs for
  `gpui`/`gpui-component`? Default to pinning a `rev` on the patch too, consistent with the
  rest of the workspace's dependency style, unless there's a specific reason to float.
