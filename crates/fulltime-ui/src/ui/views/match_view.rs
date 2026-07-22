//! Match screen: back button + score header shell + Summary/Lineups/Stats
//! tabs. Empty-state only — tab bodies are static placeholders, no live
//! match data exists yet.

use gpui::prelude::*;
use gpui::{Context, Hsla, div, px};
use gpui_component::IconName;
use gpui_component::Sizable;
use gpui_component::avatar::Avatar;
use gpui_component::button::{Button, ButtonVariants as _};

use crate::data::theme::ColorTokens;
use crate::ui::app_state::{AppScreen, MatchTab};
use crate::ui::views::components::status_pill::{MatchStatus, render_status_pill};
use crate::ui::views::components::tab_bar::render_tab_bar;
use crate::ui::views::root_view::RootView;

pub fn render_match_screen(colors: &ColorTokens, active_tab: MatchTab,
                           cx: &mut Context<RootView>)
                           -> impl IntoElement {
    div().flex()
         .flex_col()
         .gap(px(20.0))
         .child(Button::new("match-back").ghost()
                                        .compact()
                                        .small()
                                        .icon(IconName::ArrowLeft)
                                        .label("Standings")
                                        .text_size(px(13.0))
                                        .gap(px(4.0))
                                        .on_click(cx.listener(|this, _event, _window, cx| {
                                              this.set_screen(AppScreen::Standings, cx);
                                          })))
         .child(render_score_header(colors, cx))
         .child(render_match_tabs(active_tab, cx))
         .child(render_tab_body(colors, active_tab))
}

fn render_score_header(colors: &ColorTokens, cx: &Context<RootView>) -> impl IntoElement {
    let accent = colors.accent;

    div().flex()
         .items_center()
         .justify_center()
         .gap(px(24.0))
         .p(px(20.0))
         .rounded(px(16.0))
         .bg(colors.surface_alt)
         .child(div().flex()
                     .flex_col()
                     .items_center()
                     .gap(px(6.0))
                     .child(Avatar::new().name("HM")
                                        .with_size(px(48.0))
                                        .bg(Hsla { a: 0.18, ..accent })
                                        .text_color(colors.text_primary)
                                        .text_size(px(48.0 * 0.38))
                                        .border_0())
                     .child("Home Placeholder"))
         .child(div().flex()
                     .flex_col()
                     .items_center()
                     .gap(px(6.0))
                     .child(render_status_pill(MatchStatus::Live, "LIVE", cx))
                     .child(div().text_size(px(44.0))
                                 .font_weight(gpui::FontWeight::BOLD)
                                 .child("2 - 1"))
                     .child(div().text_size(px(11.0))
                                 .text_color(colors.text_tertiary)
                                 .child("Matchday 12 · Sample Arena")))
         .child(div().flex()
                     .flex_col()
                     .items_center()
                     .gap(px(6.0))
                     .child(Avatar::new().name("AW")
                                        .with_size(px(48.0))
                                        .bg(Hsla { a: 0.18, ..accent })
                                        .text_color(colors.text_primary)
                                        .text_size(px(48.0 * 0.38))
                                        .border_0())
                     .child("Away Placeholder"))
}

fn render_match_tabs(active_tab: MatchTab, cx: &mut Context<RootView>) -> impl IntoElement {
    let selected_index = MatchTab::ALL.iter()
                                      .position(|t| *t == active_tab)
                                      .unwrap_or(0);
    let labels: Vec<String> = MatchTab::ALL.iter().map(|t| t.label()).collect();

    render_tab_bar("match-detail-tabs",
                   labels,
                   selected_index,
                   cx.listener(|this, ix: &usize, _window, cx| {
                         this.set_match_tab(MatchTab::ALL[*ix], cx);
                     }))
}

fn render_tab_body(colors: &ColorTokens, active_tab: MatchTab) -> impl IntoElement {
    let body_text = match active_tab {
        MatchTab::Summary => "Goal/card/substitution timeline goes here.",
        MatchTab::Lineups => "Formation and substitutes go here.",
        MatchTab::Stats => "Possession, shots, and other match stats go here.",
    };

    div().p(px(20.0))
         .rounded(px(16.0))
         .bg(colors.surface)
         .border_1()
         .border_color(colors.border)
         .text_size(px(13.0))
         .text_color(colors.text_secondary)
         .child(body_text)
}
