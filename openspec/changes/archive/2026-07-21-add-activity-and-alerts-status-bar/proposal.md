## Why

The status bar currently has a disclaimer label on the left and a single Plugins button on the
right (added ad hoc alongside `plugin-host-runtime`, ahead of any formal spec for it). There's
nowhere for the user to see background activity (plugin loads, future data fetches) or surfaced
problems (a plugin failing to load, a fetch failure) without digging through logs. `dtrpg-app.rs`
already solved this with a status-bar activity indicator and an alerts/notification button, each
opening an anchored popover panel — this change ports that same pattern into FullTime.

## What Changes

- Add an `ActivityController`-style state owner (in `fulltime-ui`) that records discrete
  activity entries (e.g. "plugin X loaded", "plugin X failed to load") and alert entries (e.g.
  "plugin X failed to load: <reason>"), each with an in-progress/completed distinction for
  activity and a read/unread distinction for alerts.
- Add a status-bar activity button (progress-circle style, matching `dtrpg-app.rs`) that opens an
  anchored popover listing recent activity, and an alerts/bell button with an unread-count badge
  that opens an anchored popover listing alert history.
- Wire the one real event source that exists today: plugin load/enable/disable outcomes from
  `fulltime-core`'s `PluginManager` bridge (`app::plugin_manager`) — a load failure becomes both
  an activity entry and an alert; a successful load becomes an activity entry only. No other
  producer exists yet (no live league-data fetching until `plugin-host-runtime` task group 5), so
  this ships with exactly one integration point, designed to accept more later without a redesign.
- **Modified**: the status bar's `view-shell-layouts` "Footer disclaimer text" requirement is
  extended to also describe the right-aligned utility button row (Plugins, Activity, Alerts),
  since the current spec only describes the disclaimer text.

## Capabilities

### New Capabilities

- `activity-and-alerts`: the activity/alert data model (entries, in-progress/read state), the
  status-bar activity and alerts buttons, and their anchored popover panels.

### Modified Capabilities

- `view-shell-layouts`: the "Footer disclaimer text" requirement is widened to also cover the
  status bar's right-aligned utility button row (Plugins, Activity, Alerts), which exists in code
  today (Plugins) but was never formally specified.

## Impact

- **`fulltime-ui`**: new `ui/activity.rs` (or similar) module for the controller/state and
  snapshot types; new `ui/views/components/activity_panel.rs` and `alert_history_panel.rs`; changes
  to `ui/views/status_bar.rs` to add the two new buttons alongside the existing Plugins button.
  New dependency on `gpui_component`'s `Popover`/`ProgressCircle` (already used transitively via
  `gpui-component`, no new crate dependency).
  Also new i18n keys.
- **`fulltime-core`**: `app::plugin_manager` (from `plugin-host-runtime`) gains calls into the new
  activity/alert recording API at its existing load/enable/disable call sites — no new module,
  just a few new calls where outcomes are already known.
- No change to `fulltime-plugin-api` or any other repo.
