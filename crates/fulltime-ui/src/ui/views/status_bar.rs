//! Status bar view: the persistent footer disclaimer, matching the
//! mockup's "Prototype data is illustrative, not live sports data." text
//! shown beneath the content area on every screen.

use gpui::prelude::*;
use gpui::{App, div, px};
use rust_i18n::t;

use crate::data::theme::ColorTokens;

/// Renders the footer disclaimer row.
pub fn render_status_bar(colors: &ColorTokens, _cx: &App) -> impl IntoElement + 'static + use<> {
    div().h(px(26.0))
         .flex_none()
         .flex()
         .items_center()
         .justify_center()
         .px(px(12.0))
         .border_t_1()
         .border_color(colors.border)
         .bg(colors.surface_alt)
         .text_size(px(11.0))
         .text_color(colors.text_tertiary)
         .child(t!("status_bar.disclaimer").to_string())
}
