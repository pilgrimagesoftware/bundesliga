//! Session-only activity/alert log the status bar's Activity and Alerts
//! buttons render from.
//!
//! One [`ActivityEntry`] model with a [`Status`] enum serves both the
//! activity panel (every entry) and the alerts panel (only `Failed`
//! entries, filtered on render) — see
//! `openspec/changes/add-activity-and-alerts-status-bar/design.md` for why
//! this isn't split into two types the way `dtrpg-app.rs` splits
//! `ActivityItem`/`AlertEntry`.

use std::collections::VecDeque;

use chrono::{DateTime, Utc};
use gpui::{App, AppContext as _, Context, Entity, Global};

/// Oldest-evicted-first cap on the in-memory log, matching `dtrpg-app.rs`'s
/// `ALERT_LOG_CAP`. Fixed rather than configurable — see design.md's Open
/// Questions.
const ACTIVITY_LOG_CAP: usize = 50;

/// The final or in-flight state of an [`ActivityEntry`]. `InProgress`
/// exists for a future asynchronous producer (e.g. a live data fetch); no
/// producer today ever reaches it, since plugin load/enable/disable calls
/// are synchronous and resolve before control returns to the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    InProgress,
    Complete,
    Failed(String),
}

/// One recorded activity: a plugin load, a plugin enable/disable, or
/// (later) a data fetch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityEntry {
    pub id:          u64,
    pub label:       String,
    pub status:      Status,
    pub occurred_at: DateTime<Utc>,
}

/// What the status bar's activity button and its popover render from.
#[derive(Debug, Clone)]
pub struct ActivitySnapshot {
    /// Newest first.
    pub entries:           Vec<ActivityEntry>,
    pub in_progress_count: usize,
}

/// What the status bar's alerts button and its popover render from —
/// [`ActivitySnapshot`] filtered to `Failed` entries only.
#[derive(Debug, Clone)]
pub struct AlertHistorySnapshot {
    /// Newest first.
    pub entries:    Vec<ActivityEntry>,
    pub has_unread: bool,
}

/// Owns the activity/alert log. `fulltime-core` calls [`Self::record`] at
/// its plugin load/enable/disable call sites; `fulltime-ui`'s status bar
/// renders from the snapshots.
pub struct ActivityController {
    entries:           VecDeque<ActivityEntry>,
    next_id:           u64,
    alerts_panel_open: bool,
    has_unread_alert:  bool,
}

impl ActivityController {
    pub fn new() -> Self {
        Self { entries:           VecDeque::new(),
               next_id:           0,
               alerts_panel_open: false,
               has_unread_alert:  false, }
    }

    /// Appends a new entry, evicting the oldest if the log is already at
    /// [`ACTIVITY_LOG_CAP`]. A `Failed` entry also marks the alerts button
    /// unread — unless the alerts panel is already open, in which case
    /// there's nothing to mark unread since the user is looking at it.
    pub fn record(&mut self, label: impl Into<String>, status: Status, cx: &mut Context<Self>) {
        if self.entries.len() >= ACTIVITY_LOG_CAP {
            self.entries.pop_front();
        }
        let is_failure = matches!(status, Status::Failed(_));
        self.entries.push_back(ActivityEntry { id: self.next_id,
                                               label: label.into(),
                                               status,
                                               occurred_at: Utc::now() });
        self.next_id += 1;
        if is_failure && !self.alerts_panel_open {
            self.has_unread_alert = true;
        }
        cx.notify();
    }

    /// Tracks the alerts popover's open state; opening it clears the unread
    /// indicator even if no entry was individually read.
    pub fn set_alerts_panel_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.alerts_panel_open = open;
        if open {
            self.has_unread_alert = false;
        }
        cx.notify();
    }

    pub fn activity_snapshot(&self) -> ActivitySnapshot {
        let in_progress_count = self.entries
                                    .iter()
                                    .filter(|entry| matches!(entry.status, Status::InProgress))
                                    .count();
        ActivitySnapshot { entries: self.entries.iter().rev().cloned().collect(),
                           in_progress_count }
    }

    pub fn alert_history_snapshot(&self) -> AlertHistorySnapshot {
        let entries = self.entries
                          .iter()
                          .rev()
                          .filter(|entry| matches!(entry.status, Status::Failed(_)))
                          .cloned()
                          .collect();
        AlertHistorySnapshot { entries,
                               has_unread: self.has_unread_alert }
    }
}

impl Default for ActivityController {
    fn default() -> Self {
        Self::new()
    }
}

/// [`Global`] wrapper so a shared [`ActivityController`] entity can be
/// reached from anywhere with an `App`/`Context`, matching
/// [`crate::ui::plugin_manager::PluginManagerHandle`]'s pattern.
pub struct ActivityControllerHandle(pub Entity<ActivityController>);

impl Global for ActivityControllerHandle {}

/// Installs the activity controller as a global, if not already installed,
/// and returns it.
///
/// Must run before anything records into or renders from it — in
/// particular, before `fulltime-core`'s plugin manager starts recording
/// load outcomes, so callers install this ahead of
/// `app::plugin_manager::build()` rather than waiting for
/// `fulltime-ui::util::init::init_globals`.
pub fn install(cx: &mut App) -> Entity<ActivityController> {
    if !cx.has_global::<ActivityControllerHandle>() {
        let entity = cx.new(|_| ActivityController::new());
        cx.set_global(ActivityControllerHandle(entity));
    }
    cx.global::<ActivityControllerHandle>().0.clone()
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;

    use super::*;

    #[gpui::test]
    fn eviction_removes_oldest_entry_first(cx: &mut TestAppContext) {
        let controller = cx.new(|_| ActivityController::new());
        controller.update(cx, |controller, cx| {
                      for i in 0..ACTIVITY_LOG_CAP {
                          controller.record(format!("entry-{i}"), Status::Complete, cx);
                      }
                      controller.record("overflow", Status::Complete, cx);
                  });

        let snapshot = controller.read_with(cx, |controller, _| controller.activity_snapshot());
        assert_eq!(snapshot.entries.len(), ACTIVITY_LOG_CAP);
        assert!(snapshot.entries
                        .iter()
                        .all(|entry| entry.label != "entry-0"));
        assert_eq!(snapshot.entries.first().map(|entry| entry.label.as_str()),
                   Some("overflow"));
    }

    #[gpui::test]
    fn alert_snapshot_includes_only_failed_entries(cx: &mut TestAppContext) {
        let controller = cx.new(|_| ActivityController::new());
        controller.update(cx, |controller, cx| {
                      controller.record("ok", Status::Complete, cx);
                      controller.record("bad", Status::Failed("boom".into()), cx);
                  });

        let snapshot =
            controller.read_with(cx, |controller, _| controller.alert_history_snapshot());
        assert_eq!(snapshot.entries.len(), 1);
        assert_eq!(snapshot.entries[0].label, "bad");
    }

    #[gpui::test]
    fn opening_alerts_panel_clears_unread(cx: &mut TestAppContext) {
        let controller = cx.new(|_| ActivityController::new());
        controller.update(cx, |controller, cx| {
                      controller.record("bad", Status::Failed("boom".into()), cx);
                  });
        assert!(controller.read_with(cx, |controller, _| {
                              controller.alert_history_snapshot().has_unread
                          }));

        controller.update(cx, |controller, cx| {
                      controller.set_alerts_panel_open(true, cx);
                  });
        assert!(!controller.read_with(cx, |controller, _| {
                               controller.alert_history_snapshot().has_unread
                           }));
    }
}
