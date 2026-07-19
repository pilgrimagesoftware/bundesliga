//! Player screen: back button + detail hero + 3-column stat grid. Empty-
//! state only — no real player data exists yet.

use gpui::prelude::*;
use gpui::{App, Context, div, px};

use crate::data::theme::ColorTokens;
use crate::ui::app_state::AppScreen;
use crate::ui::views::components::back_button::render_back_button;
use crate::ui::views::components::badge::render_badge;
use crate::ui::views::components::stat_grid::{StatCell, render_stat_grid};
use crate::ui::views::root_view::RootView;

pub fn render_player_screen(colors: &ColorTokens, cx: &mut Context<RootView>) -> impl IntoElement {
    div().flex()
         .flex_col()
         .gap(px(20.0))
         .child(render_back_button("player-back",
                                   "Team",
                                   cx.listener(|this, _event, _window, cx| {
                                         this.set_screen(AppScreen::Team, cx);
                                     }),
                                   cx))
         .child(render_detail_hero(colors,
                                   "Player Placeholder",
                                   "Midfielder · #10 · Sample FC",
                                   cx))
         .child(render_stat_grid(&[StatCell::new("18", "Appearances"),
                                   StatCell::new("6", "Goals"),
                                   StatCell::new("4", "Assists"),
                                   StatCell::new("1,420", "Minutes"),
                                   StatCell::new("2", "Yellow cards"),
                                   StatCell::new("0", "Red cards")],
                                 cx))
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
