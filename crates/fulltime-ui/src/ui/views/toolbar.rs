//! Toolbar view: per-screen controls above the main content area (league
//! picker, season picker, view-mode switch, etc.).
//!
//! Placeholder shell — populated once a real screen exists to host controls
//! for (see the `season-picker-ux` openspec proposal).

use gpui::prelude::*;
use gpui::{App, div, px};

use crate::data::theme::ColorTokens;

/// Renders the toolbar row above the main content area.
pub fn render_toolbar(colors: &ColorTokens, _cx: &App) -> impl IntoElement + 'static + use<> {
    div().h(px(48.0))
         .flex_none()
         .flex()
         .items_center()
         .px(px(16.0))
         .border_b_1()
         .border_color(colors.border)
         .bg(colors.surface)
}
