//! Root view: composes the header and the active screen's content area.
//!
//! Owns the app-level nav state (active screen, active league, active
//! match tab, open history rows) so the header and content views can read
//! and mutate it via `cx.listener`. No league/match/team data exists yet —
//! every screen renders its empty-state layout from the mockup.

use std::collections::HashSet;

use gpui::prelude::*;
use gpui::{Context, Render, Window, div, px};

use crate::data::theme::{FullTimeTheme, League, ThemeKey};
use crate::ui::app_state::{AppScreen, MatchTab};
use crate::ui::views::header::render_header;
use crate::ui::views::history::render_history_screen;
use crate::ui::views::match_view::render_match_screen;
use crate::ui::views::player::render_player_screen;
use crate::ui::views::plugins::render_plugins_screen;
use crate::ui::views::standings::render_standings_screen;
use crate::ui::views::status_bar::render_status_bar;
use crate::ui::views::team::render_team_screen;

/// Top-level GPUI view for the FullTime main window.
pub struct RootView {
    active_screen:      AppScreen,
    active_league:      League,
    active_match_tab:   MatchTab,
    history_open_rows:  HashSet<usize>,
    /// The screen to return to when the Plugins screen (a status-bar
    /// utility screen, not one of the header's primary nav tabs) is closed.
    /// Only updated when *entering* Plugins from elsewhere — see
    /// [`Self::toggle_plugins_screen`].
    pre_plugins_screen: AppScreen,
}

impl RootView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { active_screen:      AppScreen::Standings,
               active_league:      League::Bundesliga,
               active_match_tab:   MatchTab::Summary,
               history_open_rows:  HashSet::new(),
               pre_plugins_screen: AppScreen::Standings, }
    }

    pub fn set_screen(&mut self, screen: AppScreen, cx: &mut Context<Self>) {
        self.active_screen = screen;
        cx.notify();
    }

    /// Opens the Plugins screen (remembering the screen to return to), or
    /// closes it back to whichever screen was active before, if it's
    /// already open. Used by both the status bar's Plugins button and the
    /// Plugins screen's own close button, so the two stay in sync.
    pub fn toggle_plugins_screen(&mut self, cx: &mut Context<Self>) {
        if self.active_screen == AppScreen::Plugins {
            self.active_screen = self.pre_plugins_screen;
        }
        else {
            self.pre_plugins_screen = self.active_screen;
            self.active_screen = AppScreen::Plugins;
        }
        cx.notify();
    }

    pub fn set_league(&mut self, league: League, cx: &mut Context<Self>) {
        self.active_league = league;
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

        div().flex()
             .flex_col()
             .size_full()
             .bg(colors.desktop_bg)
             .child(render_header(&colors, active_screen, active_league, theme.key, cx))
             .child(div().id("content-area")
                         .flex()
                         .flex_col()
                         .flex_1()
                         .min_h(px(0.0))
                         .overflow_y_scroll()
                         .p(px(20.0))
                         .gap(px(20.0))
                         .child(match active_screen {
                             AppScreen::Standings => render_standings_screen(&colors, active_league, cx).into_any_element(),
                             AppScreen::Match => render_match_screen(&colors, active_match_tab, cx).into_any_element(),
                             AppScreen::History => render_history_screen(&colors, &history_open_rows, cx).into_any_element(),
                             AppScreen::Player => render_player_screen(&colors, cx).into_any_element(),
                             AppScreen::Team => render_team_screen(&colors, cx).into_any_element(),
                             AppScreen::Plugins => render_plugins_screen(&colors, cx).into_any_element(),
                         }))
             .child(render_status_bar(&colors, active_screen, cx))
    }
}
