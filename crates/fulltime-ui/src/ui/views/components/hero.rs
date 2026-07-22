//! Hero banner component: the title band used at the top of the Standings
//! and History screens (league name, kicker text, and optional trailing
//! controls/stat chips).
//!
//! Evaluated against `gpui-component` during the `use-gpui-component`
//! migration and kept custom: this is a page-level layout composition,
//! not a discrete widget the pinned revision has an equivalent for.

use gpui::prelude::*;
use gpui::{App, SharedString, div, px};

use crate::data::theme::FullTimeTheme;

/// Renders a hero banner: a kicker line, a large title, and a trailing
/// slot for extra controls (season pill, matchday stepper, stat chip, etc.).
pub fn render_hero(kicker: impl Into<SharedString>, title: impl Into<SharedString>,
                   trailing: impl IntoElement, cx: &App)
                   -> impl IntoElement {
    let theme = cx.global::<FullTimeTheme>();
    let colors = theme.colors.clone();
    let type_scale = theme.type_scale.clone();

    div().flex()
         .items_end()
         .justify_between()
         .gap(px(16.0))
         .p(px(20.0))
         .rounded(theme.radius.base)
         .bg(colors.surface_alt)
         .border_1()
         .border_color(colors.border)
         .child(div().flex()
                     .flex_col()
                     .gap(px(4.0))
                     .child(div().text_size(px(12.0))
                                 .text_color(colors.text_tertiary)
                                 .child(kicker.into()))
                     .child(div().text_size(type_scale.hero_title)
                                 .font_weight(gpui::FontWeight::BOLD)
                                 .text_color(colors.text_primary)
                                 .child(title.into())))
         .child(trailing)
}
