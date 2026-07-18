//! Title bar view: app name and a drag region.
//!
//! Sits above the sidebar/content split. This is a placeholder shell —
//! account/window-menu controls land once there's a signed-in concept or a
//! settings window to open.

use gpui::prelude::*;
use gpui::{App, MouseButton, div, px};
use rust_i18n::t;

use crate::data::theme::ColorTokens;

/// Renders the title bar: app name on the left, a drag region filling the
/// rest of the row.
pub fn render_title_bar(colors: &ColorTokens, _cx: &App) -> impl IntoElement + 'static + use<> {
    let surface = colors.surface;
    let border = colors.border;
    let text_primary = colors.text_primary;

    // Reserve clearance for the macOS traffic light buttons, which overlay the
    // top-left of the window when the titlebar renders with
    // `appears_transparent: true`.
    let leading_inset = if cfg!(target_os = "macos") {
        px(78.0)
    }
    else {
        px(12.0)
    };

    div().h(px(44.0))
         .flex_none()
         .flex()
         .items_center()
         .gap(px(8.0))
         .pl(leading_inset)
         .pr(px(12.0))
         .border_b_1()
         .border_color(border)
         .bg(surface)
         .child(div().text_color(text_primary)
                     .font_weight(gpui::FontWeight::SEMIBOLD)
                     .child(t!("sidebar.app_name").to_string()))
         .child(div().id("title-bar-drag-region")
                     .flex_1()
                     .h_full()
                     .on_mouse_down(MouseButton::Left, |_, window, _| {
                         window.start_window_move();
                     }))
}
