//! Team screen: back button + detail hero + 3-column stat grid + form-dots
//! row. Empty-state only — no real team data exists yet.

use gpui::prelude::*;
use gpui::{App, Context, div, px};

use crate::data::theme::ColorTokens;
use crate::ui::app_state::AppScreen;
use crate::ui::views::components::back_button::render_back_button;
use crate::ui::views::components::badge::render_badge;
use crate::ui::views::components::form_dots::{FormResult, render_form_dots};
use crate::ui::views::components::stat_grid::{StatCell, render_stat_grid};
use crate::ui::views::root_view::RootView;

pub fn render_team_screen(colors: &ColorTokens, cx: &mut Context<RootView>) -> impl IntoElement {
    div().flex()
         .flex_col()
         .gap(px(20.0))
         .child(render_back_button("team-back",
                                   "Standings",
                                   cx.listener(|this, _event, _window, cx| {
                                         this.set_screen(AppScreen::Standings, cx);
                                     }),
                                   cx))
         .child(render_detail_hero(colors, "Sample FC", "Bundesliga · 3rd place", cx))
         .child(render_stat_grid(&[StatCell::new("12", "Played"),
                                   StatCell::new("7", "Wins"),
                                   StatCell::new("23", "Points"),
                                   StatCell::new("28", "Goals for"),
                                   StatCell::new("15", "Goals against"),
                                   StatCell::new("+13", "Goal difference")],
                                 cx))
         .child(render_form_dots(&[FormResult::Win,
                                   FormResult::Win,
                                   FormResult::Draw,
                                   FormResult::Win,
                                   FormResult::Loss]))
}

fn render_detail_hero(colors: &ColorTokens, name: &str, meta: &str, cx: &App) -> impl IntoElement {
    div().flex()
         .items_center()
         .gap(px(16.0))
         .p(px(20.0))
         .rounded(px(16.0))
         .bg(colors.surface_alt)
         .child(render_badge(name.chars().take(2).collect::<String>(),
                             colors.accent,
                             56.0,
                             cx))
         .child(div().flex()
                     .flex_col()
                     .gap(px(4.0))
                     .child(div().text_size(px(20.0))
                                 .font_weight(gpui::FontWeight::BOLD)
                                 .child(name.to_string()))
                     .child(div().text_size(px(12.0))
                                 .text_color(colors.text_tertiary)
                                 .child(meta.to_string())))
}
