//! Match screen: back button + score header shell + Summary/Lineups/Stats
//! tabs. Empty-state only — tab bodies are static placeholders, no live
//! match data exists yet.

use gpui::prelude::*;
use gpui::{AnimationExt, AnyElement, Context, Hsla, SharedString, div, px};
use gpui_component::IconName;
use gpui_component::Sizable;
use gpui_component::avatar::Avatar;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::tag::{Tag, TagVariant};

use crate::data::theme::ColorTokens;
use crate::ui::app_state::{AppScreen, MatchStatus, MatchTab};
use crate::ui::views::components::tab_bar::render_tab_bar;
use crate::ui::views::root_view::RootView;

/// Renders a match status indicator via `gpui_component::tag::Tag`,
/// mapping `MatchStatus` to the `Tag` variant whose theme color matches its
/// previous hand-rolled color: `Live` used the theme's accent (now
/// `Danger`, since `Live` reads as this app's "urgent" state), `FullTime`
/// used a neutral surface tint (now `Secondary`), and `Scheduled` used a
/// dimmer neutral tint (now `Warning`, for "upcoming"). The `Live` state's
/// looping opacity pulse wraps the `Tag` rather than being reimplemented.
fn render_match_status(status: MatchStatus, label: impl Into<SharedString>) -> AnyElement {
    let variant = match status {
        MatchStatus::Live => TagVariant::Danger,
        MatchStatus::FullTime => TagVariant::Secondary,
        MatchStatus::Scheduled => TagVariant::Warning,
    };

    let tag = Tag::new().with_variant(variant)
                        .rounded_full()
                        .px(px(8.0))
                        .py(px(2.0))
                        .text_size(px(11.0))
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .child(label.into());

    if status == MatchStatus::Live {
        tag.with_animation("live-status-pulse",
                           gpui::Animation::new(std::time::Duration::from_millis(1600)).repeat(),
                           |this, delta| {
                               let opacity =
                                   1.0 - (delta * std::f32::consts::PI).sin().abs() * 0.65;
                               this.opacity(opacity)
                           })
           .into_any_element()
    }
    else {
        tag.into_any_element()
    }
}

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
         .child(render_score_header(colors))
         .child(render_match_tabs(active_tab, cx))
         .child(render_tab_body(colors, active_tab))
}

fn render_score_header(colors: &ColorTokens) -> impl IntoElement {
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
                     .child(render_match_status(MatchStatus::Live, "LIVE"))
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
