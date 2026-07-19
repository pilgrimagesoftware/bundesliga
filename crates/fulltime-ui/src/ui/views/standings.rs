//! Standings screen: hero band + two-column body (standings table, matchday
//! rail shell, top-scorers shell). Empty-state only — no live league/match
//! data exists yet. The results grid uses `gpui_component`'s `DataTable`
//! rather than a hand-rolled row layout.

use gpui::prelude::*;
use gpui::{App, Context, Div, Entity, Hsla, Stateful, Window, div, px};
use gpui_component::table::{Column, DataTable, TableDelegate, TableState};

use crate::data::theme::{ColorTokens, League, ZoneColors, league_accent, zone_colors};
use crate::ui::views::components::badge::render_badge;
use crate::ui::views::components::card::render_card;
use crate::ui::views::components::form_dots::{FormResult, render_form_dots};
use crate::ui::views::components::hero::render_hero;
use crate::ui::views::components::legend::render_legend_item;
use crate::ui::views::components::status_pill::{MatchStatus, render_status_pill};
use crate::ui::views::root_view::RootView;

struct PlaceholderRow {
    rank: u8,
    club: &'static str,
    p:    u8,
    w:    u8,
    d:    u8,
    l:    u8,
    gf:   u8,
    ga:   u8,
    pts:  u8,
    form: [FormResult; 5],
}

fn placeholder_table() -> Vec<PlaceholderRow> {
    use FormResult::{Draw, Loss, Win};
    vec![PlaceholderRow { rank: 1,
                          club: "FC Placeholder",
                          p:    12,
                          w:    9,
                          d:    2,
                          l:    1,
                          gf:   28,
                          ga:   10,
                          pts:  29,
                          form: [Win, Win, Draw, Win, Win], },
         PlaceholderRow { rank: 2,
                          club: "Athletic Sample",
                          p:    12,
                          w:    8,
                          d:    3,
                          l:    1,
                          gf:   24,
                          ga:   12,
                          pts:  27,
                          form: [Win, Draw, Win, Win, Loss], },
         PlaceholderRow { rank: 3,
                          club: "Union Mockup",
                          p:    12,
                          w:    7,
                          d:    2,
                          l:    3,
                          gf:   20,
                          ga:   15,
                          pts:  23,
                          form: [Loss, Win, Win, Draw, Win], },
         PlaceholderRow { rank: 4,
                          club: "SC Fixture",
                          p:    12,
                          w:    5,
                          d:    4,
                          l:    3,
                          gf:   18,
                          ga:   16,
                          pts:  19,
                          form: [Draw, Draw, Win, Loss, Win], },
         PlaceholderRow { rank: 5,
                          club: "VfL Dataless",
                          p:    12,
                          w:    2,
                          d:    3,
                          l:    7,
                          gf:   11,
                          ga:   24,
                          pts:  9,
                          form: [Loss, Loss, Draw, Loss, Win], },]
}

const STANDINGS_COLUMNS: &[(&str, &str)] = &[("rank", "#"),
                                             ("club", "Club"),
                                             ("p", "P"),
                                             ("w", "W"),
                                             ("d", "D"),
                                             ("l", "L"),
                                             ("gf", "GF"),
                                             ("ga", "GA"),
                                             ("gd", "GD"),
                                             ("pts", "Pts"),
                                             ("form", "Form")];

/// `TableDelegate` for the Standings results grid. Holds the placeholder
/// rows plus the currently active league's accent color and the active
/// theme's zone-highlight colors, both updated in place (via
/// `delegate_mut()` + `refresh()`) when the league or theme changes,
/// rather than recreating the table entity.
pub struct StandingsTableDelegate {
    rows:   Vec<PlaceholderRow>,
    accent: Hsla,
    zones:  ZoneColors,
}

impl StandingsTableDelegate {
    pub fn new(accent: Hsla, zones: ZoneColors) -> Self {
        Self { rows: placeholder_table(),
               accent,
               zones }
    }

    pub fn set_accent(&mut self, accent: Hsla) {
        self.accent = accent;
    }

    pub fn set_zones(&mut self, zones: ZoneColors) {
        self.zones = zones;
    }

    fn zone_bg(&self, rank: u8) -> Option<Hsla> {
        match rank {
            1..=4 => Some(self.zones.ucl),
            5 => Some(self.zones.uel),
            16..=20 => Some(self.zones.relegation),
            _ => None,
        }
    }
}

impl TableDelegate for StandingsTableDelegate {
    fn columns_count(&self, _cx: &App) -> usize {
        STANDINGS_COLUMNS.len()
    }

    fn rows_count(&self, _cx: &App) -> usize {
        self.rows.len()
    }

    fn column(&self, col_ix: usize, _cx: &App) -> Column {
        let (key, name) = STANDINGS_COLUMNS[col_ix];
        Column::new(key, name)
    }

    fn render_tr(&mut self, row_ix: usize, _window: &mut Window,
                 _cx: &mut Context<TableState<Self>>)
                 -> Stateful<Div> {
        let mut row_div = div().id(("standings-row", row_ix));
        if let Some(row) = self.rows.get(row_ix)
           && let Some(bg) = self.zone_bg(row.rank)
        {
            row_div = row_div.bg(bg);
        }
        row_div
    }

    fn render_td(&mut self, row_ix: usize, col_ix: usize, _window: &mut Window,
                 cx: &mut Context<TableState<Self>>)
                 -> impl IntoElement {
        let Some(row) = self.rows.get(row_ix)
        else {
            return div().into_any_element();
        };
        let accent = self.accent;

        match STANDINGS_COLUMNS[col_ix].0 {
            "rank" => row.rank.to_string().into_any_element(),
            "club" => div().flex()
                           .items_center()
                           .gap(px(6.0))
                           .child(render_badge(row.club.chars().take(2).collect::<String>(),
                                               accent,
                                               20.0,
                                               cx))
                           .child(row.club)
                           .into_any_element(),
            "p" => row.p.to_string().into_any_element(),
            "w" => row.w.to_string().into_any_element(),
            "d" => row.d.to_string().into_any_element(),
            "l" => row.l.to_string().into_any_element(),
            "gf" => row.gf.to_string().into_any_element(),
            "ga" => row.ga.to_string().into_any_element(),
            "gd" => format!("{:+}", row.gf as i16 - row.ga as i16).into_any_element(),
            "pts" => row.pts.to_string().into_any_element(),
            "form" => render_form_dots(&row.form).into_any_element(),
            _ => div().into_any_element(),
        }
    }
}

/// Constructs the Standings results table entity. Called once in
/// `RootView::new`.
pub fn new_standings_table(window: &mut Window, cx: &mut Context<RootView>)
                           -> Entity<TableState<StandingsTableDelegate>> {
    let delegate = StandingsTableDelegate::new(league_accent(League::Bundesliga),
                                               zone_colors(crate::data::theme::ThemeKey::Pitch));
    cx.new(|cx| TableState::new(delegate, window, cx))
}

/// Renders the Standings screen: the results grid is interactive
/// (`DataTable`); the hero controls and rail/scorers panels are static
/// placeholders in this skeleton.
pub fn render_standings_screen(colors: &ColorTokens, active_league: League,
                               table: &Entity<TableState<StandingsTableDelegate>>, cx: &App)
                               -> impl IntoElement {
    let zones = zone_colors(cx.global::<crate::data::theme::FullTimeTheme>().key);

    div().flex()
         .flex_col()
         .gap(px(20.0))
         .child(render_hero(active_league.label(),
                            "Standings",
                            div().flex()
                                 .items_center()
                                 .gap(px(12.0))
                                 .child(div().text_size(px(12.0))
                                             .text_color(colors.text_tertiary)
                                             .child("2025/26"))
                                 .child(div().px(px(10.0))
                                             .py(px(4.0))
                                             .rounded_full()
                                             .bg(colors.surface)
                                             .border_1()
                                             .border_color(colors.border)
                                             .text_size(px(12.0))
                                             .child("18 clubs")),
                            cx))
         .child(div().flex()
                     .gap(px(20.0))
                     .child(render_standings_table_panel(colors, table, &zones, cx))
                     .child(div().flex()
                                 .flex_col()
                                 .gap(px(20.0))
                                 .w(px(300.0))
                                 .flex_none()
                                 .child(render_matchday_rail(colors, cx))
                                 .child(render_top_scorers(colors, cx))))
}

fn render_standings_table_panel(colors: &ColorTokens,
                                table: &Entity<TableState<StandingsTableDelegate>>,
                                zones: &ZoneColors, cx: &App)
                                -> impl IntoElement {
    div().flex_1()
         .flex()
         .flex_col()
         .gap(px(12.0))
         .child(div().h(px(320.0))
                     .flex()
                     .flex_col()
                     .rounded(px(16.0))
                     .border_1()
                     .border_color(colors.border)
                     .overflow_hidden()
                     .child(DataTable::new(table).stripe(true).bordered(false)))
         .child(div().flex()
                     .gap(px(16.0))
                     .child(render_legend_item(zones.ucl, "Champions League", cx))
                     .child(render_legend_item(zones.uel, "Europa League", cx))
                     .child(render_legend_item(zones.relegation, "Relegation", cx)))
}

fn render_matchday_rail(colors: &ColorTokens, cx: &App) -> impl IntoElement {
    render_card(cx).child(div().text_size(px(14.5))
                               .font_weight(gpui::FontWeight::BOLD)
                               .child("Matchday"))
                   .children((1..=4).map(|i| render_matchday_fixture(i, cx)))
                   .child(div().text_size(px(12.0))
                               .text_color(colors.accent)
                               .child("Full schedule →"))
}

fn render_matchday_fixture(i: u8, cx: &App) -> impl IntoElement {
    let status = match i {
        1 => MatchStatus::Live,
        2 => MatchStatus::FullTime,
        _ => MatchStatus::Scheduled,
    };
    let label = match status {
        MatchStatus::Live => "LIVE",
        MatchStatus::FullTime => "FT",
        MatchStatus::Scheduled => "20:30",
    };

    div().flex()
         .items_center()
         .justify_between()
         .py(px(6.0))
         .child(div().text_size(px(12.0)).child(format!("Fixture {i}")))
         .child(render_status_pill(status, label, cx))
}

fn render_top_scorers(colors: &ColorTokens, cx: &App) -> impl IntoElement {
    render_card(cx).child(div().text_size(px(14.5))
                               .font_weight(gpui::FontWeight::BOLD)
                               .child("Top Scorers"))
                   .children((1..=5).map(|i| render_top_scorer_row(colors, i)))
}

fn render_top_scorer_row(colors: &ColorTokens, i: u8) -> impl IntoElement {
    div().flex()
         .items_center()
         .justify_between()
         .py(px(4.0))
         .text_size(px(12.0))
         .text_color(colors.text_secondary)
         .child(format!("{i}. Player {i}"))
         .child(format!("{} goals", 20 - i))
}
