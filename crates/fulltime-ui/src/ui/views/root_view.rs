//! Root view: composes the header and the active screen's content area.
//!
//! Owns the app-level nav state (active screen, active league, active
//! match tab, open history rows) so the header and content views can read
//! and mutate it via `cx.listener`. No league/match/team data exists yet —
//! every screen renders its empty-state layout from the mockup.

use std::collections::HashSet;

use gpui::prelude::*;
use gpui::{Context, Entity, Render, Window, div, px};
use gpui_component::table::TableState;

use crate::data::theme::{FullTimeTheme, League, ThemeKey, league_accent, zone_colors};
use crate::ui::app_state::{AppScreen, MatchTab};
use crate::ui::views::header::render_header;
use crate::ui::views::history::render_history_screen;
use crate::ui::views::match_view::render_match_screen;
use crate::ui::views::player::render_player_screen;
use crate::ui::views::standings::{
    StandingsTableDelegate, new_standings_table, render_standings_screen,
};
use crate::ui::views::status_bar::render_status_bar;
use crate::ui::views::team::render_team_screen;

/// Top-level GPUI view for the FullTime main window.
pub struct RootView {
    active_screen:     AppScreen,
    active_league:     League,
    active_match_tab:  MatchTab,
    history_open_rows: HashSet<usize>,
    standings_table:   Entity<TableState<StandingsTableDelegate>>,
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let view = Self { active_screen:     AppScreen::Standings,
                          active_league:     League::Bundesliga,
                          active_match_tab:  MatchTab::Summary,
                          history_open_rows: HashSet::new(),
                          standings_table:   new_standings_table(window, cx), };

        // The standings `DataTable` measures its scroll-container bounds
        // from the completed window layout; those bounds are still zero on
        // the very first paint (before this view's first full layout
        // pass), so its virtualized rows render empty until something
        // triggers a second render. Force that second render explicitly
        // rather than relying on the user resizing the window.
        let entity = cx.entity();
        window.on_next_frame(move |_window, cx| {
                  entity.update(cx, |_, cx| cx.notify());
              });

        view
    }

    pub fn set_screen(&mut self, screen: AppScreen, cx: &mut Context<Self>) {
        self.active_screen = screen;
        cx.notify();
    }

    pub fn set_league(&mut self, league: League, cx: &mut Context<Self>) {
        self.active_league = league;
        self.standings_table.update(cx, |table, cx| {
                                table.delegate_mut().set_accent(league_accent(league));
                                table.refresh(cx);
                            });
        cx.notify();
    }

    pub fn set_match_tab(&mut self, tab: MatchTab, cx: &mut Context<Self>) {
        self.active_match_tab = tab;
        cx.notify();
    }

    pub fn toggle_history_row(&mut self, ix: usize, cx: &mut Context<Self>) {
        if !self.history_open_rows.remove(&ix) {
            self.history_open_rows.insert(ix);
        }
        cx.notify();
    }

    pub fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        let current = cx.global::<FullTimeTheme>().clone();
        let next_key = match current.key {
            ThemeKey::Pitch => ThemeKey::PitchNight,
            ThemeKey::PitchNight => ThemeKey::Pitch,
        };
        let next_theme = FullTimeTheme::new(next_key, current.fonts.clone());
        cx.update_global::<gpui_component::Theme, _>(|theme, _cx| {
              theme.font_family = next_theme.fonts.body_font.clone();
              crate::data::theme::apply_theme_colors(theme, &next_theme.colors);
          });
        cx.set_global(next_theme);
        self.standings_table.update(cx, |table, cx| {
                                table.delegate_mut().set_zones(zone_colors(next_key));
                                table.refresh(cx);
                            });
        cx.notify();
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<FullTimeTheme>().clone();
        let colors = theme.colors.clone();
        let active_screen = self.active_screen;
        let active_league = self.active_league;
        let active_match_tab = self.active_match_tab;
        let history_open_rows = self.history_open_rows.clone();
        let standings_table = self.standings_table.clone();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(colors.desktop_bg)
            .child(render_header(
                &colors,
                active_screen,
                active_league,
                theme.key,
                cx,
            ))
            .child(
                div()
                    .id("content-area")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_h(px(0.0))
                    .overflow_y_scroll()
                    .p(px(20.0))
                    .gap(px(20.0))
                    .child(match active_screen {
                        AppScreen::Standings => {
                            render_standings_screen(&colors, active_league, &standings_table, cx)
                                .into_any_element()
                        }
                        AppScreen::Match => {
                            render_match_screen(&colors, active_match_tab, cx).into_any_element()
                        }
                        AppScreen::History => {
                            render_history_screen(&colors, &history_open_rows, cx)
                                .into_any_element()
                        }
                        AppScreen::Player => render_player_screen(&colors, cx).into_any_element(),
                        AppScreen::Team => render_team_screen(&colors, cx).into_any_element(),
                    }),
            )
            .child(render_status_bar(&colors, cx))
    }
}
