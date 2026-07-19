//! Status pill component: a small rounded label used for match status
//! (live/full-time/scheduled) on fixture cards and the Match screen's
//! score header.

use gpui::prelude::*;
use gpui::{AnimationExt, App, SharedString, div, px};

use crate::data::theme::FullTimeTheme;

/// A fixture's display status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchStatus {
    Live,
    FullTime,
    Scheduled,
}

/// Renders a status pill; `Live` uses the accent fill and pulses via a
/// looping opacity animation, matching the mockup's `livePulse` keyframes.
pub fn render_status_pill(status: MatchStatus, label: impl Into<SharedString>, cx: &App)
                          -> impl IntoElement {
    let theme = cx.global::<FullTimeTheme>();
    let colors = theme.colors.clone();

    let (bg, fg) = match status {
        MatchStatus::Live => (colors.accent, colors.accent_on),
        MatchStatus::FullTime => (colors.surface_alt, colors.text_secondary),
        MatchStatus::Scheduled => (colors.surface_alt, colors.text_tertiary),
    };

    let pill = div().flex()
                    .items_center()
                    .px(px(8.0))
                    .py(px(2.0))
                    .rounded_full()
                    .bg(bg)
                    .text_size(px(11.0))
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(label.into());

    if status == MatchStatus::Live {
        pill.with_animation("live-status-pulse",
                            gpui::Animation::new(std::time::Duration::from_millis(1600)).repeat(),
                            |this, delta| {
                                let opacity =
                                    1.0 - (delta * std::f32::consts::PI).sin().abs() * 0.65;
                                this.opacity(opacity)
                            })
            .into_any_element()
    }
    else {
        pill.into_any_element()
    }
}
