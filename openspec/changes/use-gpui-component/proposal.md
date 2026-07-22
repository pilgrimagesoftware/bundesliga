## Why

`fulltime-ui` already depends on `gpui-component` and uses it for several widgets
(`Combobox`, `TabBar`, `Badge`, `Popover`, `Button`, `StatusBar`), but a handful of visual
primitives were hand-rolled in raw `gpui` (`div()`/`px()`) before or alongside that adoption:
`widgets.rs` (`small_caps_text`) and the `views/components/` files `badge.rs`, `card.rs`,
`status_pill.rs`, `legend.rs`, `form_dots.rs`, `stat_grid.rs`, `hero.rs`, `back_button.rs`,
`alert_history_panel.rs`, and `activity_panel.rs`. Some of these duplicate functionality
`gpui-component` already ships (e.g. a custom circular badge alongside
`gpui_component::badge::Badge`), which is exactly the redundancy `docs/rust.md`'s "prefer
existing UI components over custom UI code" rule exists to avoid. Duplicated rendering logic
means theme/spacing/animation fixes have to be applied twice and drift independently.

## What Changes

- Audit every hand-rolled component under `crates/fulltime-ui/src/ui/widgets.rs` and
  `crates/fulltime-ui/src/ui/views/components/` against the widget set exposed by the pinned
  `gpui-component` revision.
- For each hand-rolled component with a directly equivalent `gpui-component` widget, replace
  the custom `render_*` function and its call sites with the `gpui-component` widget,
  preserving the existing visual result (colors, spacing, radii sourced from
  `FullTimeTheme`/`design-tokens`) — this is a refactor, not a redesign.
- For hand-rolled components with no equivalent in `gpui-component` (e.g. `form_dots.rs`'s
  match-form indicator dots, `legend.rs`'s domain-specific legend), leave them as-is and record
  why no replacement exists, so a future audit doesn't re-litigate the same file.
- Delete the custom component modules that become dead code once their call sites are
  migrated, and the now-unused theme fields that only existed to style them, if any.

## Capabilities

### New Capabilities
- `shared-ui-components`: which visual primitives fulltime-ui renders via `gpui-component`
  widgets vs. bespoke `gpui` code, and the rule that a new shared primitive must use an
  existing `gpui-component` widget when one covers the need before a custom one is written.

### Modified Capabilities
(none — this changes implementation, not user-observable behavior of the capabilities already
specified in `activity-and-alerts`, `app-header-nav`, `design-tokens`, `view-shell-layouts`)

## Impact

- **Code**: `crates/fulltime-ui/src/ui/widgets.rs` and up to ten files under
  `crates/fulltime-ui/src/ui/views/components/`, plus every call site of their `render_*`
  functions across `views/` (standings, match, team, player, history, header).
- **Dependencies**: no new dependencies — `gpui-component` and `gpui-component-assets` are
  already pinned in the workspace `Cargo.toml`.
- **Risk**: visual regressions if a `gpui-component` widget's default styling doesn't match the
  current hand-rolled look; each migrated component needs a side-by-side visual check (`cargo
  run`, compare against the pre-migration screenshot) before its custom module is deleted.
