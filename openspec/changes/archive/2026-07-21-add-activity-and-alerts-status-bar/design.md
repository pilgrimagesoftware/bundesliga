## Context

`dtrpg-app.rs`'s status bar (`crates/dtrpg-ui/src/ui/views/status_bar_view.rs`) has an
`ActivityController` tracking in-progress/recently-completed background operations (downloads,
each with progress and a cancel callback) plus a durable alert log, both surfaced via status-bar
buttons that open `Popover`-anchored panels. FullTime's status bar currently has only the
disclaimer text and the Plugins button added ad hoc alongside `plugin-host-runtime`.

FullTime has exactly one real event source today: `fulltime-core`'s `app::plugin_manager` bridge,
which loads/enables/disables plugins via `PluginHost`/`PluginRegistry` — all synchronous, blocking
calls with a known outcome (success or a `PluginHostError`/`RegistryError`) by the time control
returns to the caller. There is no genuinely asynchronous, long-running, cancellable background
operation anywhere in the app yet (that arrives with `plugin-host-runtime` task group 5's live
data fetching). This is a materially simpler starting point than `dtrpg-app.rs`'s downloads.

## Goals / Non-Goals

**Goals:**
- Add a status-bar activity button and an alerts button, each opening an anchored popover panel,
  visually matching `dtrpg-app.rs`'s pattern (ghost/compact `Button`, `Popover` anchored to the
  trigger, unread-dot badge on the alerts button).
- Give the rest of the app (starting with `plugin_manager`) a simple API to record an activity
  entry and, for failures, an alert entry — designed so a later async/long-running producer (a
  live data fetch) can report an in-progress state through the same model without a redesign.

**Non-Goals:**
- Progress fractions, cancel callbacks, or item expiry timers. `dtrpg-app.rs` needs these for
  long-running, user-cancellable downloads; nothing in FullTime today is long-running or
  cancellable. Adding this machinery now with no real caller would be speculative.
- Persisting activity/alerts across app restarts. `dtrpg-app.rs`'s alert log is session-only
  (capped, in-memory); FullTime's is the same — a restart clears it, matching the "durable for
  the session" framing, not "durable forever."
- Wiring any producer besides `plugin_manager`. No other part of the app has anything to report
  yet.

## Decisions

**One `ActivityEntry` model with a `Status` enum (`InProgress`, `Complete`, `Failed(String)`),
not two separate item/alert types joined ad hoc.** `dtrpg-app.rs` keeps `ActivityItem` (transient,
expiring) and `AlertEntry` (durable) as separate types constructed from the same originating
event. For FullTime's single current producer — an already-resolved plugin load/toggle outcome —
that split adds no value yet: every entry is immediately terminal (`Complete` or `Failed`), so
there's nothing to "promote" from one type to the other. A single `ActivityEntry` list serves the
activity panel directly; the alerts panel is a filtered view over the same list (only `Failed`
entries), computed on render rather than duplicated into a second store. Alternative considered:
port `dtrpg-app.rs`'s two-type split now, rejected as premature — reintroduce it if/when a real
in-progress producer needs the transient/durable distinction dtrpg's split exists for.

**`fulltime-ui` owns the state (an `Entity<ActivityController>`-style controller, matching
`dtrpg-app.rs`), `fulltime-core` only calls a recording method.** Mirrors the
`PluginManager`/`PluginManagerHandle` split from `plugin-host-runtime`: the UI crate owns the
shape and rendering, the app crate feeds it real events. Concretely, `plugin_manager::build()` and
`FulltimePluginManager::set_enabled` gain calls to record an activity (and, on failure, that same
entry is what the alerts panel's filter surfaces) — no new trait needed on the `fulltime-ui` side
beyond exposing the controller as a `gpui::Global` (or an `Entity` reachable from one), since
unlike `PluginManager` there's no fulltime-core-side data to abstract over.

**Reuse `gpui_component`'s `ProgressCircle` for the activity button exactly as `dtrpg-app.rs`
does, including its zero-in-progress idle rendering.** No FullTime-specific "idle" icon needed —
`ProgressCircle::new(...).value(0.0)` already renders as a static ring when nothing is in
progress, which is the only state this change's single producer ever reaches (synchronous calls
resolve before the UI can render an in-progress frame). This is not scope creep: it's using an
already-idle-safe shared component rather than adding a new one.

## Risks / Trade-offs

- [Single-list design (entry with a `Status`) may need revisiting once a real in-progress
  producer exists] → Acceptable: the `Status::InProgress` variant already exists in the enum, so a
  future producer can report it through the same model; only the *panel's* filtering/grouping
  logic would need extending, not the data model itself.
- [No persistence means alerts vanish on restart, so a plugin load failure a user doesn't notice
  before quitting is lost] → Matches `dtrpg-app.rs`'s own scope (session-only); revisit only if
  this becomes a real reported problem.

## Migration Plan

1. Add the `ActivityEntry`/`Status` model and `ActivityController` (or equivalent) to
   `fulltime-ui`, with a capped in-memory log (same cap-and-evict pattern as `dtrpg-app.rs`'s
   `ALERT_LOG_CAP`).
2. Add the activity and alerts status-bar buttons + anchored popover panels, wired to the
   controller.
3. Call the recording API from `fulltime-core`'s `app::plugin_manager` at its existing
   load/enable/disable call sites (already logging via `tracing::warn!`; add the UI-facing record
   alongside, not instead of, that logging).
4. No flag or staged rollout needed — this is additive UI with one already-existing, low-risk
   producer.

## Open Questions

- Should the activity log's cap be a fixed constant (matching `dtrpg-app.rs`'s
  `ALERT_LOG_CAP`) or configurable? Default to a fixed constant; revisit only if a real need for
  configurability shows up.
