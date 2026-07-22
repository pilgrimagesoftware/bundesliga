//! Legend item component: a colored dot plus a label, reused for the
//! standings zone legend and (later) chart legends.
//!
//! Evaluated against `gpui-component` during the `use-gpui-component`
//! migration and kept custom: the closest widget, `Tag`, renders a filled
//! pill rather than a solid dot, and this component appears side-by-side
//! with `Tag`-based status pills elsewhere, so swapping it in would blur
//! a visual distinction the design relies on.

use gpui::prelude::*;
use gpui::{App, Hsla, SharedString, div, px};

use crate::data::theme::FullTimeTheme;

/// Renders one legend row: a colored dot and a label.
pub fn render_legend_item(color: Hsla, label: impl Into<SharedString>, cx: &App)
                          -> impl IntoElement {
    let colors = cx.global::<FullTimeTheme>().colors.clone();

    div().flex()
         .items_center()
         .gap(px(6.0))
         .child(div().size(px(8.0)).rounded_full().bg(color))
         .child(div().text_size(px(11.0))
                     .text_color(colors.text_secondary)
                     .child(label.into()))
}
