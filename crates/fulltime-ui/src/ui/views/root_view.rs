//! Root view: composes the header, league/competition selector, and the
//! active screen's content area.
//!
//! Owns the app-level nav state (active screen, active match tab, open
//! history rows, and the league/competition selection) so the header,
//! selector, and content views can read and mutate it via `cx.listener`/
//! `Entity::update`. Only the Standings screen consumes the league
//! selection's fetched data; every other screen still renders its
//! empty-state layout from the mockup.

use std::collections::HashSet;

use gpui::prelude::*;
use gpui::{Context, Entity, Render, SharedString, Subscription, Window, div, px};
use gpui_component::combobox::{ComboboxEvent, ComboboxState};
use gpui_component::searchable_list::SearchableVec;
use rust_i18n::t;

use crate::data::theme::{FullTimeTheme, ThemeKey};
use crate::ui::app_state::{AppScreen, MatchTab};
use crate::ui::plugin_manager::{PluginManagerHandle, StandingsSnapshot};
use crate::ui::views::components::league_selector::{
    CompetitionComboboxState, CompetitionItem, LeagueComboboxState, LeagueItem, competition_index,
    league_index, render_league_selector,
};
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
    active_screen:                      AppScreen,
    active_match_tab:                   MatchTab,
    history_open_rows:                  HashSet<usize>,
    /// The screen to return to when the Plugins screen (a status-bar
    /// utility screen, not one of the header's primary nav tabs) is closed.
    /// Only updated when *entering* Plugins from elsewhere — see
    /// [`Self::toggle_plugins_screen`].
    pre_plugins_screen:                 AppScreen,
    selected_plugin_id:                 Option<String>,
    selected_competition_id:            Option<String>,
    /// The selected league/competition's fetched standings, cached here so
    /// re-rendering doesn't re-fetch over the network every frame. `None`
    /// until a league is selected, or if the fetch failed.
    current_standings:                  Option<StandingsSnapshot>,
    league_combobox:                    Entity<LeagueComboboxState>,
    competition_combobox:               Entity<CompetitionComboboxState>,
    /// Set once at construction if the plugin host is unavailable or no
    /// leagues are loaded; `render_league_selector` shows this in place of
    /// the comboboxes. Leagues are only enumerated at startup - see
    /// `docs/openspec.md`'s scope note on live plugin enable/disable not yet
    /// refreshing this list.
    league_selector_fallback:           Option<SharedString>,
    _league_combobox_subscription:      Subscription,
    _competition_combobox_subscription: Subscription,
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (league_items, league_selector_fallback) = if !cx.has_global::<PluginManagerHandle>() {
            (Vec::new(), Some(SharedString::from(t!("league_selector.unavailable").to_string())))
        }
        else {
            let leagues: Vec<LeagueItem> = cx.global::<PluginManagerHandle>()
                                             .0
                                             .available_leagues()
                                             .into_iter()
                                             .map(LeagueItem::from)
                                             .collect();
            if leagues.is_empty() {
                (leagues, Some(SharedString::from(t!("league_selector.none_loaded").to_string())))
            }
            else {
                (leagues, None)
            }
        };

        let league_combobox = cx.new(|cx| {
                                    ComboboxState::new(SearchableVec::new(league_items.clone()),
                                                       Vec::new(),
                                                       window,
                                                       cx).searchable(true)
                                });
        let competition_combobox =
            cx.new(|cx| {
                  ComboboxState::new(SearchableVec::new(Vec::<CompetitionItem>::new()),
                                     Vec::new(),
                                     window,
                                     cx).searchable(true)
              });

        let league_combobox_subscription =
            cx.subscribe_in(&league_combobox,
                            window,
                            |view, _emitter, event, window, cx| {
                                if let ComboboxEvent::Confirm(values) = event
                                   && let Some(plugin_id) = values.first().cloned()
                                {
                                    view.select_league(plugin_id, window, cx);
                                }
                            });
        let competition_combobox_subscription =
            cx.subscribe_in(&competition_combobox,
                            window,
                            |view, _emitter, event, _window, cx| {
                                if let ComboboxEvent::Confirm(values) = event
                                   && let Some(competition_id) = values.first().cloned()
                                {
                                    view.select_competition(competition_id, cx);
                                }
                            });

        let mut view = Self { active_screen: AppScreen::Standings,
                              active_match_tab: MatchTab::Summary,
                              history_open_rows: HashSet::new(),
                              pre_plugins_screen: AppScreen::Standings,
                              selected_plugin_id: None,
                              selected_competition_id: None,
                              current_standings: None,
                              league_combobox,
                              competition_combobox,
                              league_selector_fallback,
                              _league_combobox_subscription: league_combobox_subscription,
                              _competition_combobox_subscription:
                                  competition_combobox_subscription };

        // Auto-select the first available league/competition, if any, so
        // the Standings screen shows real data immediately rather than
        // requiring the user to open the selector first.
        if let Some(league) = league_items.into_iter().next() {
            view.select_league(league.plugin_id, window, cx);
        }

        view
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

    /// Selects `plugin_id` as the active league, auto-selecting its most
    /// recent competition, and fetches that competition's standings.
    pub fn select_league(&mut self, plugin_id: String, window: &mut Window,
                         cx: &mut Context<Self>) {
        let leagues: Vec<LeagueItem> = if cx.has_global::<PluginManagerHandle>() {
            cx.global::<PluginManagerHandle>()
              .0
              .available_leagues()
              .into_iter()
              .map(LeagueItem::from)
              .collect()
        }
        else {
            Vec::new()
        };
        let competitions: Vec<CompetitionItem> = if cx.has_global::<PluginManagerHandle>() {
            cx.global_mut::<PluginManagerHandle>()
              .0
              .competitions(&plugin_id)
              .into_iter()
              .map(CompetitionItem::from)
              .collect()
        }
        else {
            Vec::new()
        };

        let league_ix = league_index(&leagues, &plugin_id);
        let competition_id = competitions.first()
                                         .map(|competition| competition.id.clone());
        let competition_ix = competition_id.as_deref()
                                           .and_then(|id| competition_index(&competitions, id));

        self.league_combobox.update(cx, |state, cx| {
                                state.set_selected_indices(league_ix, window, cx)
                            });
        self.competition_combobox.update(cx, |state, cx| {
                                     state.set_items(SearchableVec::new(competitions), window, cx);
                                     state.set_selected_indices(competition_ix, window, cx);
                                 });

        self.selected_plugin_id = Some(plugin_id);
        self.selected_competition_id = competition_id;
        self.refetch_standings(cx);
    }

    /// Selects `competition_id` within the currently-selected league and
    /// fetches its standings.
    pub fn select_competition(&mut self, competition_id: String, cx: &mut Context<Self>) {
        self.selected_competition_id = Some(competition_id);
        self.refetch_standings(cx);
    }

    fn refetch_standings(&mut self, cx: &mut Context<Self>) {
        self.current_standings = match (&self.selected_plugin_id, &self.selected_competition_id) {
            (Some(plugin_id), Some(competition_id)) if cx.has_global::<PluginManagerHandle>() => {
                cx.global_mut::<PluginManagerHandle>()
                  .0
                  .fetch_standings(plugin_id, competition_id)
            }
            _ => None,
        };
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
        let active_match_tab = self.active_match_tab;
        let history_open_rows = self.history_open_rows.clone();
        let current_standings = self.current_standings.clone();
        let show_competition = self.selected_plugin_id.is_some();

        div().flex()
             .flex_col()
             .size_full()
             .bg(colors.desktop_bg)
             .child(render_header(&colors, active_screen, theme.key, cx))
             .child(render_league_selector(&colors,
                                           &self.league_combobox,
                                           &self.competition_combobox,
                                           show_competition,
                                           self.league_selector_fallback.clone()))
             .child(div().id("content-area")
                         .flex()
                         .flex_col()
                         .flex_1()
                         .min_h(px(0.0))
                         .overflow_y_scroll()
                         .p(px(20.0))
                         .gap(px(20.0))
                         .child(match active_screen {
                             AppScreen::Standings => render_standings_screen(&colors, current_standings, cx).into_any_element(),
                             AppScreen::Match => render_match_screen(&colors, active_match_tab, cx).into_any_element(),
                             AppScreen::History => render_history_screen(&colors, &history_open_rows, cx).into_any_element(),
                             AppScreen::Player => render_player_screen(&colors, cx).into_any_element(),
                             AppScreen::Team => render_team_screen(&colors, cx).into_any_element(),
                             AppScreen::Plugins => render_plugins_screen(&colors, cx).into_any_element(),
                         }))
             .child(render_status_bar(&colors, active_screen, cx))
    }
}
