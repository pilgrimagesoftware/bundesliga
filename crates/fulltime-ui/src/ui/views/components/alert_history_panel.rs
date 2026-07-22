//! Alert history panel: the popover anchored to the status bar's alerts
//! button, listing only `Failed` activity entries newest first.

use gpui::prelude::*;
use gpui::{div, px};
use rust_i18n::t;

use crate::data::theme::ColorTokens;
use crate::ui::activity::{ActivityEntry, Status};

/// Renders the alert history, newest first, or an empty-state message if no
/// `Failed` entry has been recorded yet this session.
pub fn render_alert_history_panel(colors: &ColorTokens, entries: &[ActivityEntry])
                                  -> impl IntoElement + use<> {
    if entries.is_empty() {
        return div().w(px(220.0))
                    .text_size(px(12.0))
                    .text_color(colors.text_tertiary)
                    .child(t!("alert_history_panel.empty").to_string())
                    .into_any_element();
    }

    div().id("alert-history-panel-entries")
         .flex()
         .flex_col()
         .gap(px(8.0))
         .w(px(220.0))
         .max_h(px(320.0))
         .overflow_y_scroll()
         .children(entries.iter().map(|entry| render_alert_row(colors, entry)))
         .into_any_element()
}

fn render_alert_row(colors: &ColorTokens, entry: &ActivityEntry) -> impl IntoElement {
    let message = match &entry.status {
        Status::Failed(message) => message.clone(),
        // `render_alert_history_panel` is only ever given `Failed` entries
        // (see `ActivityController::alert_history_snapshot`); this arm is
        // unreachable in practice but avoids a partial match.
        Status::InProgress | Status::Complete => String::new(),
    };

    div().flex()
         .flex_col()
         .gap(px(2.0))
         .child(div().text_size(px(12.0))
                     .text_color(colors.text_primary)
                     .child(entry.label.clone()))
         .child(div().text_size(px(11.0))
                     .text_color(colors.error)
                     .child(message))
}
