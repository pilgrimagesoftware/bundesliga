//! Stat cell grid component: a 3-column grid of big-number/label cells,
//! used identically for Player and Team screen season stats.
//!
//! Evaluated against `gpui-component` during the `use-gpui-component`
//! migration and kept custom: this is a page-level layout composition,
//! not a discrete widget the pinned revision has an equivalent for.

use gpui::prelude::*;
use gpui::{App, SharedString, div, px};

use crate::data::theme::FullTimeTheme;

/// One `(value, label)` stat cell.
pub struct StatCell {
    pub value: SharedString,
    pub label: SharedString,
}

impl StatCell {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self { value: value.into(),
               label: label.into(), }
    }
}

/// Renders a 3-column grid of stat cells.
pub fn render_stat_grid(cells: &[StatCell], cx: &App) -> impl IntoElement {
    let colors = cx.global::<FullTimeTheme>().colors.clone();

    div().grid()
         .grid_cols(3)
         .gap(px(12.0))
         .children(cells.iter().map(|cell| {
                                   div().flex()
                                        .flex_col()
                                        .items_center()
                                        .gap(px(2.0))
                                        .p(px(12.0))
                                        .rounded(px(12.0))
                                        .bg(colors.surface_alt)
                                        .child(div().text_size(px(20.0))
                                                    .font_weight(gpui::FontWeight::BOLD)
                                                    .text_color(colors.text_primary)
                                                    .child(cell.value.clone()))
                                        .child(div().text_size(px(11.0))
                                                    .text_color(colors.text_tertiary)
                                                    .child(cell.label.clone()))
                               }))
}
