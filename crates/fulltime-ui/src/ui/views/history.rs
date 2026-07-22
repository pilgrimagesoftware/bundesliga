//! History screen: back button + hero + an accordion list of matchdays.
//! Empty-state only — fixture lists inside each expanded row are static
//! placeholders, no live match data exists yet.

use std::collections::HashSet;

use gpui::prelude::*;
use gpui::{AnyElement, Context, div, px};
use gpui_component::IconName;
use gpui_component::Sizable;
use gpui_component::button::{Button, ButtonVariants as _};

use crate::data::theme::ColorTokens;
use crate::ui::app_state::AppScreen;
use crate::ui::views::components::hero::render_hero;
use crate::ui::views::root_view::RootView;

const MATCHDAY_COUNT: usize = 6;

pub fn render_history_screen(colors: &ColorTokens, open_rows: &HashSet<usize>,
                             cx: &mut Context<RootView>)
                             -> impl IntoElement {
    let mut rows: Vec<AnyElement> = Vec::with_capacity(MATCHDAY_COUNT);
    for ix in 0..MATCHDAY_COUNT {
        let is_open = open_rows.contains(&ix);
        rows.push(render_matchday_row(colors, ix, is_open, cx).into_any_element());
    }

    let back_button =
        Button::new("history-back").ghost()
                                   .compact()
                                   .small()
                                   .icon(IconName::ArrowLeft)
                                   .label("Standings")
                                   .text_size(px(13.0))
                                   .gap(px(4.0))
                                   .on_click(cx.listener(|this, _event, _window, cx| {
                                                   this.set_screen(AppScreen::Standings, cx);
                                               }));
    let hero = render_hero("Bundesliga",
                           format!("{MATCHDAY_COUNT} Matchdays"),
                           div(),
                           cx);

    div().flex()
         .flex_col()
         .gap(px(16.0))
         .child(back_button)
         .child(hero)
         .child(div().flex().flex_col().gap(px(8.0)).children(rows))
}

fn render_matchday_row(colors: &ColorTokens, ix: usize, is_open: bool,
                       cx: &mut Context<RootView>)
                       -> impl IntoElement {
    let chevron = if is_open { "▾" } else { "▸" };

    div().flex()
         .flex_col()
         .rounded(px(12.0))
         .border_1()
         .border_color(colors.border)
         .overflow_hidden()
         .child(div().id(("history-row", ix))
                     .flex()
                     .items_center()
                     .justify_between()
                     .px(px(14.0))
                     .py(px(10.0))
                     .bg(colors.surface_alt)
                     .cursor_pointer()
                     .on_click(cx.listener(move |this, _event, _window, cx| {
                                     this.toggle_history_row(ix, cx);
                                 }))
                     .child(div().flex()
                                 .items_center()
                                 .gap(px(8.0))
                                 .child(chevron)
                                 .child(format!("Matchday {}", ix + 1)))
                     .child(div().text_size(px(12.0))
                                 .text_color(colors.accent)
                                 .child("View standings →")))
         .when(is_open, |this| {
             this.child(div().flex()
                             .flex_col()
                             .gap(px(6.0))
                             .p(px(14.0))
                             .children((1..=4).map(|i| {
                                                  div().flex()
                                                       .items_center()
                                                       .justify_between()
                                                       .text_size(px(12.5))
                                                       .text_color(colors.text_secondary)
                                                       .child(format!("Home {i}"))
                                                       .child("2 - 1")
                                                       .child(format!("Away {i}"))
                                              })))
         })
}
