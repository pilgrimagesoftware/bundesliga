//! Standings screen: hero band + two-column body (standings table, matchday
//! rail shell, top-scorers shell). The standings table renders real
//! Bundesliga data when the plugin host fetched it successfully at startup
//! (see `ui::plugin_manager::PluginManagerHandle::standings`), falling back
//! to its mockup rows otherwise (feature off, plugin not loaded, or the
//! fetch failed) — every other league, and the matchday rail/top-scorers
//! panels, remain mockup-only; no data source exists for those yet. The
//! results grid is a CSS-grid-based table (structured rows/columns, not
//! one-off flexbox rows) rather than a virtualized `DataTable`, which proved
//! unreliable for this small, static row set.

use gpui::prelude::*;
use gpui::{App, Hsla, div, px};

use crate::data::theme::{ColorTokens, League, ZoneColors, league_accent, zone_colors};
use crate::ui::plugin_manager::{PluginManagerHandle, StandingsRowSnapshot};
use crate::ui::views::components::badge::render_badge;
use crate::ui::views::components::card::render_card;
use crate::ui::views::components::form_dots::{FormResult, render_form_dots};
use crate::ui::views::components::hero::render_hero;
use crate::ui::views::components::legend::render_legend_item;
use crate::ui::views::components::status_pill::{MatchStatus, render_status_pill};

/// One rendered standings row, from either the mockup table or a real
/// [`StandingsRowSnapshot`]. `form` is `None` for real rows: the canonical
/// schema has no "recent form" concept, so there is nothing to show in the
/// Form column for real data.
struct DisplayRow {
    rank: u16,
    club: String,
    p:    u16,
    w:    u16,
    d:    u16,
    l:    u16,
    gf:   u16,
    ga:   u16,
    pts:  u16,
    form: Option<[FormResult; 5]>,
}

impl From<StandingsRowSnapshot> for DisplayRow {
    fn from(row: StandingsRowSnapshot) -> Self {
        Self { rank: row.rank,
               club: row.team_name,
               p:    row.played,
               w:    row.won,
               d:    row.drawn,
               l:    row.lost,
               gf:   row.goals_for,
               ga:   row.goals_against,
               pts:  row.points,
               form: None, }
    }
}

fn placeholder_table() -> Vec<DisplayRow> {
    use FormResult::{Draw, Loss, Win};
    vec![DisplayRow { rank: 1,
                      club: "FC Placeholder".to_owned(),
                      p:    12,
                      w:    9,
                      d:    2,
                      l:    1,
                      gf:   28,
                      ga:   10,
                      pts:  29,
                      form: Some([Win, Win, Draw, Win, Win]), },
         DisplayRow { rank: 2,
                      club: "Athletic Sample".to_owned(),
                      p:    12,
                      w:    8,
                      d:    3,
                      l:    1,
                      gf:   24,
                      ga:   12,
                      pts:  27,
                      form: Some([Win, Draw, Win, Win, Loss]), },
         DisplayRow { rank: 3,
                      club: "Union Mockup".to_owned(),
                      p:    12,
                      w:    7,
                      d:    2,
                      l:    3,
                      gf:   20,
                      ga:   15,
                      pts:  23,
                      form: Some([Loss, Win, Win, Draw, Win]), },
         DisplayRow { rank: 4,
                      club: "SC Fixture".to_owned(),
                      p:    12,
                      w:    5,
                      d:    4,
                      l:    3,
                      gf:   18,
                      ga:   16,
                      pts:  19,
                      form: Some([Draw, Draw, Win, Loss, Win]), },
         DisplayRow { rank: 5,
                      club: "VfL Dataless".to_owned(),
                      p:    12,
                      w:    2,
                      d:    3,
                      l:    7,
                      gf:   11,
                      ga:   24,
                      pts:  9,
                      form: Some([Loss, Loss, Draw, Loss, Win]), },]
}

const STANDINGS_HEADERS: &[&str] =
    &["#", "Club", "P", "W", "D", "L", "GF", "GA", "GD", "Pts", "Form"];

/// Renders the Standings screen: no interactivity in this skeleton (the
/// matchday stepper/season pill are static placeholders). Real data is only
/// available for [`League::Bundesliga`] (see the module doc comment); every
/// other league renders the mockup table.
pub fn render_standings_screen(colors: &ColorTokens, active_league: League, cx: &App)
                               -> impl IntoElement {
    let accent = league_accent(active_league);
    let zones = zone_colors(cx.global::<crate::data::theme::FullTimeTheme>().key);

    let real_standings = (active_league == League::Bundesliga).then(|| {
                             cx.try_global::<PluginManagerHandle>()
                               .and_then(|handle| handle.0.standings())
                         })
                         .flatten();

    let (season_label, rows): (String, Vec<DisplayRow>) = match real_standings {
        Some(standings) => {
            (standings.competition_name, standings.rows.into_iter().map(DisplayRow::from).collect())
        }
        None => ("2025/26".to_owned(), placeholder_table()),
    };
    let club_count = rows.len();

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
                                             .child(season_label))
                                 .child(div().px(px(10.0))
                                             .py(px(4.0))
                                             .rounded_full()
                                             .bg(colors.surface)
                                             .border_1()
                                             .border_color(colors.border)
                                             .text_size(px(12.0))
                                             .child(format!("{club_count} clubs"))),
                            cx))
         .child(div().flex()
                     .gap(px(20.0))
                     .child(render_standings_table_panel(colors, accent, &zones, rows, cx))
                     .child(div().flex()
                                 .flex_col()
                                 .gap(px(20.0))
                                 .w(px(300.0))
                                 .flex_none()
                                 .child(render_matchday_rail(colors, cx))
                                 .child(render_top_scorers(colors, cx))))
}

fn render_standings_table_panel(colors: &ColorTokens, accent: Hsla, zones: &ZoneColors,
                                rows: Vec<DisplayRow>, cx: &App)
                                -> impl IntoElement {
    div().flex_1()
         .flex()
         .flex_col()
         .gap(px(12.0))
         .child(render_standings_grid(colors, accent, zones, rows, cx))
         .child(div().flex()
                     .gap(px(16.0))
                     .child(render_legend_item(zones.ucl, "Champions League", cx))
                     .child(render_legend_item(zones.uel, "Europa League", cx))
                     .child(render_legend_item(zones.relegation, "Relegation", cx)))
}

fn render_standings_grid(colors: &ColorTokens, accent: Hsla, zones: &ZoneColors,
                         rows: Vec<DisplayRow>, cx: &App)
                         -> impl IntoElement {
    let header_cells = STANDINGS_HEADERS.iter().map(|label| {
                                                   div().px(px(8.0))
                                                        .py(px(6.0))
                                                        .bg(colors.surface_alt)
                                                        .text_size(px(11.0))
                                                        .text_color(colors.text_tertiary)
                                                        .child(*label)
                                                        .into_any_element()
                                               });

    let mut body_cells = Vec::with_capacity(rows.len() * STANDINGS_HEADERS.len());
    for row in rows {
        let zone_bg = match row.rank {
            1..=4 => Some(zones.ucl),
            5 => Some(zones.uel),
            16..=20 => Some(zones.relegation),
            _ => None,
        };
        let bg = zone_bg.unwrap_or(colors.surface);
        let gd = format!("{:+}", row.gf as i32 - row.ga as i32);

        body_cells.push(grid_cell(bg, colors.text_primary, row.rank.to_string()));
        body_cells.push(div().flex()
                             .items_center()
                             .gap(px(6.0))
                             .px(px(8.0))
                             .py(px(6.0))
                             .bg(bg)
                             .text_color(colors.text_primary)
                             .text_size(px(13.0))
                             .child(render_badge(row.club.chars().take(2).collect::<String>(),
                                                 accent,
                                                 20.0,
                                                 cx))
                             .child(row.club)
                             .into_any_element());
        body_cells.push(grid_cell(bg, colors.text_primary, row.p.to_string()));
        body_cells.push(grid_cell(bg, colors.text_primary, row.w.to_string()));
        body_cells.push(grid_cell(bg, colors.text_primary, row.d.to_string()));
        body_cells.push(grid_cell(bg, colors.text_primary, row.l.to_string()));
        body_cells.push(grid_cell(bg, colors.text_primary, row.gf.to_string()));
        body_cells.push(grid_cell(bg, colors.text_primary, row.ga.to_string()));
        body_cells.push(grid_cell(bg, colors.text_primary, gd));
        body_cells.push(grid_cell(bg, colors.text_primary, row.pts.to_string()));
        body_cells.push(div().px(px(8.0))
                             .py(px(6.0))
                             .bg(bg)
                             .child(render_form_dots(row.form
                                                        .as_ref()
                                                        .map_or(&[][..], |form| form)))
                             .into_any_element());
    }

    div().grid()
         .grid_cols(STANDINGS_HEADERS.len() as u16)
         .rounded(px(16.0))
         .border_1()
         .border_color(colors.border)
         .overflow_hidden()
         .children(header_cells)
         .children(body_cells)
}

fn grid_cell(bg: Hsla, text_color: Hsla, value: String) -> gpui::AnyElement {
    div().px(px(8.0))
         .py(px(6.0))
         .bg(bg)
         .text_color(text_color)
         .text_size(px(13.0))
         .child(value)
         .into_any_element()
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
