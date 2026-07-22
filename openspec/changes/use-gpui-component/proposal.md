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

Separately, `Cargo.toml` pins `gpui`/`gpui_platform` directly against
`zed-industries/zed` (rev `1d217ee3`), not just transitively through `gpui-component`. Zed's
`gpui` accepts features on Zed's own roadmap and timeline; features `gpui-component`
contributors have wanted (custom shaders, tray support, Wayland touch event translation) were
rejected upstream and picked up instead by `gpui-ce` (github.com/gpui-ce/gpui-ce), a community
fork that tracks Zed's `gpui` but isn't gated by Zed's own release needs. Moving fulltime-ui's
direct `gpui`/`gpui_platform` dependency to `gpui-ce` reduces exposure to that upstream
rejection risk. `gpui-component` itself still depends directly on `zed-industries/zed`'s `gpui`
(pinned rev `be4c5d30`) and has no `gpui-ce`-specific code path, but `gpui-ce`'s README
documents a workspace-level Cargo `[patch]` override for exactly this case — redirecting every
consumer of the `zed-industries/zed` source (including `gpui-component`, without it needing any
change) to `gpui-ce` instead:

```toml
[patch."https://github.com/zed-industries/zed"]
gpui = { git = "https://github.com/gpui-ce/gpui-ce" }
gpui_platform = { git = "https://github.com/gpui-ce/gpui-ce" }
```

`gpui-ce` positions itself as a drop-in for `gpui`, "provided upstream hasn't introduced
breaking changes that dependent libraries haven't yet adopted" (its own caveat) — so this
migration is a build-and-verify task, not a wait-for-upstream one. See `design.md` for the
verification plan.

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
- Add a workspace-level Cargo `[patch]` override that redirects `gpui`/`gpui_platform` from
  `zed-industries/zed` to `gpui-ce/gpui-ce` for every consumer, including `gpui-component`,
  without changing `gpui-component`'s own declared dependency.
- Verify the patched workspace builds, lints, tests, and renders correctly (full visual pass
  across every screen in section 7 of `tasks.md`) before treating the migration as done; treat
  any compile or behavioral gap between `gpui-ce` and the previously pinned Zed `gpui` revision
  as a blocker to fix or work around, not something to silently paper over.

## Capabilities

### New Capabilities
- `shared-ui-components`: which visual primitives fulltime-ui renders via `gpui-component`
  widgets vs. bespoke `gpui` code, and the rule that a new shared primitive must use an
  existing `gpui-component` widget when one covers the need before a custom one is written.
- `gpui-runtime`: which `gpui` implementation (`zed-industries/zed` vs. `gpui-ce`) fulltime-ui
  builds against, applied via a workspace-level `[patch]` override so `gpui-component` doesn't
  need its own `gpui-ce` support, and verified with a full build/lint/test/visual pass before
  being considered complete.

### Modified Capabilities
(none — this changes implementation, not user-observable behavior of the capabilities already
specified in `activity-and-alerts`, `app-header-nav`, `design-tokens`, `view-shell-layouts`)

## Impact

- **Code**: `crates/fulltime-ui/src/ui/widgets.rs` and up to ten files under
  `crates/fulltime-ui/src/ui/views/components/`, plus every call site of their `render_*`
  functions across `views/` (standings, match, team, player, history, header). The top-level
  `Cargo.toml`, adding a `[patch."https://github.com/zed-industries/zed"]` section.
- **Dependencies**: no new dependencies for the widget migration — `gpui-component` and
  `gpui-component-assets` are already pinned in the workspace `Cargo.toml`. The `gpui-ce`
  migration adds a `[patch]` override that redirects the existing `gpui`/`gpui_platform`
  dependencies (`zed-industries/zed` → `gpui-ce/gpui-ce`); no new crate name is introduced, and
  `gpui-component`'s own `Cargo.toml` is untouched.
- **Risk**: visual regressions if a `gpui-component` widget's default styling doesn't match the
  current hand-rolled look; each migrated component needs a side-by-side visual check (`cargo
  run`, compare against the pre-migration screenshot) before its custom module is deleted.
  Separately, the `gpui-ce` patch risks a build break or subtle behavioral drift if `gpui-ce`'s
  tracked revision has diverged from the pinned Zed `gpui` revision (`1d217ee3`) in an API or
  behavior fulltime-ui or `gpui-component` relies on — mitigated by `cargo build`/`clippy`/
  `test` plus a full visual re-pass immediately after patching, before merging.
