//! Card component: a bordered, rounded surface used to group content
//! (fixture rail entries, stat panels, tab bodies) across every screen.

use gpui::prelude::*;
use gpui::{App, div, px};

use crate::data::theme::FullTimeTheme;

/// Renders an empty card container; callers add children via `.child()`.
pub fn render_card(cx: &App) -> gpui::Div {
    let theme = cx.global::<FullTimeTheme>();
    let colors = &theme.colors;

    div().flex()
         .flex_col()
         .gap(px(12.0))
         .p(px(16.0))
         .rounded(theme.radius.base)
         .bg(colors.surface)
         .border_1()
         .border_color(colors.border)
}
