//! Form-indicator dots component: a row of small colored dots (win/draw/
//! loss) summarizing recent results, used in the Standings table and the
//! Team screen.
//!
//! Evaluated against `gpui-component` during the `use-gpui-component`
//! migration and kept custom: no widget in the pinned revision represents
//! a multi-dot result strip.

use gpui::prelude::*;
use gpui::{div, px};

use crate::data::theme::form_colors;

/// One recent-match result, oldest to most recent as passed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormResult {
    Win,
    Draw,
    Loss,
}

/// Renders a horizontal strip of form-indicator dots.
pub fn render_form_dots(results: &[FormResult]) -> impl IntoElement {
    let form = form_colors();

    div().flex()
         .items_center()
         .gap(px(4.0))
         .children(results.iter().map(|result| {
                                     let color = match result {
                                         FormResult::Win => form.win,
                                         FormResult::Draw => form.draw,
                                         FormResult::Loss => form.loss,
                                     };
                                     div().size(px(8.0)).rounded_full().bg(color)
                                 }))
}
