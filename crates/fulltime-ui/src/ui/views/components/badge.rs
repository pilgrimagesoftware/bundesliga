//! Team badge/avatar component: a circular, initials-based badge used for
//! team and player identity across the Standings, Match, Player, and Team
//! screens.

use gpui::prelude::*;
use gpui::{App, Hsla, SharedString, div, px};

use crate::data::theme::FullTimeTheme;

/// Renders a circular badge with `initials` on an `accent`-tinted
/// background.
pub fn render_badge(initials: impl Into<SharedString>, accent: Hsla, size: f32, cx: &App)
                    -> impl IntoElement {
    let colors = cx.global::<FullTimeTheme>().colors.clone();

    div().flex()
         .items_center()
         .justify_center()
         .size(px(size))
         .rounded_full()
         .bg(Hsla { a: 0.18, ..accent })
         .text_color(colors.text_primary)
         .text_size(px(size * 0.38))
         .font_weight(gpui::FontWeight::BOLD)
         .child(initials.into())
}
