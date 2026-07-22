//! Activity panel: the popover anchored to the status bar's activity
//! button, listing the activity log newest first.

use gpui::prelude::*;
use gpui::{div, px};
use rust_i18n::t;

use crate::data::theme::ColorTokens;
use crate::ui::activity::{ActivityEntry, Status};

/// Renders the activity log, newest first, or an empty-state message if
/// nothing has been recorded yet this session.
pub fn render_activity_panel(colors: &ColorTokens, entries: &[ActivityEntry])
                             -> impl IntoElement + use<> {
    if entries.is_empty() {
        return div().w(px(220.0))
                    .text_size(px(12.0))
                    .text_color(colors.text_tertiary)
                    .child(t!("activity_panel.empty").to_string())
                    .into_any_element();
    }

    div().id("activity-panel-entries")
         .flex()
         .flex_col()
         .gap(px(8.0))
         .w(px(220.0))
         .max_h(px(320.0))
         .overflow_y_scroll()
         .children(entries.iter().map(|entry| render_entry_row(colors, entry)))
         .into_any_element()
}

fn render_entry_row(colors: &ColorTokens, entry: &ActivityEntry) -> impl IntoElement {
    let (status_label, status_color) = match &entry.status {
        Status::InProgress => {
            (t!("activity_panel.status_in_progress").to_string(), colors.text_tertiary)
        }
        Status::Complete => {
            (t!("activity_panel.status_complete").to_string(), colors.text_secondary)
        }
        Status::Failed(message) => (message.clone(), colors.error),
    };

    div().flex()
         .flex_col()
         .gap(px(2.0))
         .child(div().text_size(px(12.0))
                     .text_color(colors.text_primary)
                     .child(entry.label.clone()))
         .child(div().text_size(px(11.0))
                     .text_color(status_color)
                     .child(status_label))
}
