## 1. Activity/Alert Data Model

- [ ] 1.1 Add `ActivityEntry` (`id`, `label`, `status`, `occurred_at`) and `Status`
  (`InProgress`, `Complete`, `Failed(String)`) to `fulltime-ui`
- [ ] 1.2 Add an `ActivityController` (or equivalent `Entity`-backed owner) holding a
  capped, oldest-evicted-first `Vec<ActivityEntry>`, with a `record(label, status)` method
- [ ] 1.3 Add snapshot types (`ActivitySnapshot`, `AlertHistorySnapshot` or a combined
  equivalent) the status bar/panels render from, following `dtrpg-app.rs`'s
  `data/activity.rs` shape minus progress/cancel fields (see design.md)
- [ ] 1.4 Track panel-open state and "has unread alert since last open" on the controller,
  matching `dtrpg-app.rs`'s `AlertHistorySnapshot.has_unread`

## 2. Status Bar Buttons

- [ ] 2.1 Add the activity button (ghost/compact `Button` + `gpui_component::progress::ProgressCircle`,
  idle state when no entry is `InProgress`) to `ui/views/status_bar.rs`, alongside the existing
  Plugins button
- [ ] 2.2 Add the alerts button (ghost/compact `Button` with a bell icon and an unread-dot
  overlay) to `ui/views/status_bar.rs`
- [ ] 2.3 Add i18n keys for both buttons' tooltips

## 3. Panels

- [ ] 3.1 Add `ui/views/components/activity_panel.rs` rendering the activity log (newest
  first), anchored via `gpui_component::popover::Popover` to the activity button
- [ ] 3.2 Add `ui/views/components/alert_history_panel.rs` rendering only `Failed` entries
  (newest first), anchored via `Popover` to the alerts button
- [ ] 3.3 Wire `Popover::on_open_change` for both panels to the controller's open-state and
  unread-clearing methods

## 4. Wiring the One Real Producer

- [ ] 4.1 In `fulltime-core`'s `app::plugin_manager::build()`, record an activity entry for
  each plugin load outcome (`Complete` on success, `Failed(message)` on error) alongside the
  existing `tracing::warn!` call
- [ ] 4.2 In `FulltimePluginManager::set_enabled`, record an activity entry for the
  enable/disable outcome alongside the existing `tracing::warn!` call
- [ ] 4.3 Confirm the activity controller is reachable from `app::plugin_manager` (as a
  `gpui::Global`/`Entity`, consistent with how `PluginManagerHandle` is installed)

## 5. Tests

- [ ] 5.1 Unit test: recording entries beyond the log's capacity evicts the oldest first
- [ ] 5.2 Unit test: the alerts panel's filtered view includes only `Failed` entries
- [ ] 5.3 Unit test: opening the alerts panel clears the unread indicator
- [ ] 5.4 Manually verify (run the app): a plugin load failure (e.g. temporarily rename
  `assets/plugins/bundesliga/plugin.wasm`) shows up in both the activity panel and the alerts
  panel with the unread indicator set
