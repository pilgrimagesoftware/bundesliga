//! Back button component: a text-style, arrow-prefixed button used at the
//! top of the Match, History, Player, and Team screens to return to the
//! previous screen.

use gpui::prelude::*;
use gpui::{App, ClickEvent, ElementId, Window, div, px};

use crate::data::theme::FullTimeTheme;

/// Renders a back button. `on_click` is invoked on left-click.
pub fn render_back_button(id: impl Into<ElementId>, label: &str,
                          on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
                          cx: &App)
                          -> impl IntoElement {
    let colors = cx.global::<FullTimeTheme>().colors.clone();

    div().id(id.into())
         .flex()
         .items_center()
         .gap(px(4.0))
         .text_size(px(13.0))
         .text_color(colors.text_secondary)
         .cursor_pointer()
         .on_click(on_click)
         .child("←")
         .child(label.to_string())
}
