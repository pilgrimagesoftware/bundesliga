//! League/competition selector row, rendered under the header on every
//! screen. Replaces the header's old static 5-league tab bar: leagues are
//! now whatever plugins are actually loaded, not a fixed list - see
//! `ui::plugin_manager::PluginManager::available_leagues`.
//!
//! Two searchable comboboxes, left to right: League (one entry per loaded
//! plugin) and Competition (that plugin's competitions, most recent first).
//! Selecting either updates `RootView`'s selection and triggers a live
//! standings fetch - see `RootView::select_league`/`select_competition`.

use gpui::prelude::*;
use gpui::{AnyElement, Entity, SharedString, div, px};
use gpui_component::IndexPath;
use gpui_component::combobox::{Combobox, ComboboxState};
use gpui_component::searchable_list::{SearchableListItem, SearchableVec};
use rust_i18n::t;

use crate::data::theme::ColorTokens;
use crate::ui::plugin_manager::{CompetitionSummary, LeagueSummary};

/// A league combobox row: one entry per loaded plugin.
#[derive(Debug, Clone, PartialEq)]
pub struct LeagueItem {
    pub plugin_id:    String,
    pub display_name: SharedString,
}

impl SearchableListItem for LeagueItem {
    type Value = String;

    fn title(&self) -> SharedString {
        self.display_name.clone()
    }

    fn value(&self) -> &Self::Value {
        &self.plugin_id
    }
}

impl From<LeagueSummary> for LeagueItem {
    fn from(league: LeagueSummary) -> Self {
        Self { plugin_id:    league.plugin_id,
               display_name: league.display_name.into(), }
    }
}

/// A competition combobox row within the currently selected league.
#[derive(Debug, Clone, PartialEq)]
pub struct CompetitionItem {
    pub id:   String,
    pub name: SharedString,
}

impl SearchableListItem for CompetitionItem {
    type Value = String;

    fn title(&self) -> SharedString {
        self.name.clone()
    }

    fn value(&self) -> &Self::Value {
        &self.id
    }
}

impl From<CompetitionSummary> for CompetitionItem {
    fn from(competition: CompetitionSummary) -> Self {
        Self { id:   competition.id,
               name: competition.name.into(), }
    }
}

/// Combobox state type for the league selector.
pub type LeagueComboboxState = ComboboxState<SearchableVec<LeagueItem>>;
/// Combobox state type for the competition selector.
pub type CompetitionComboboxState = ComboboxState<SearchableVec<CompetitionItem>>;

/// Finds `plugin_id`'s index among `items`, for seeding a league combobox's
/// selection.
pub fn league_index(items: &[LeagueItem], plugin_id: &str) -> Option<IndexPath> {
    items.iter()
         .position(|item| item.plugin_id == plugin_id)
         .map(IndexPath::new)
}

/// Finds `competition_id`'s index among `items`, for seeding a competition
/// combobox's selection.
pub fn competition_index(items: &[CompetitionItem], competition_id: &str) -> Option<IndexPath> {
    items.iter()
         .position(|item| item.id == competition_id)
         .map(IndexPath::new)
}

pub fn render_league_selector(colors: &ColorTokens,
                              league_combobox: &Entity<LeagueComboboxState>,
                              competition_combobox: &Entity<CompetitionComboboxState>,
                              show_competition: bool, fallback_message: Option<SharedString>)
                              -> AnyElement {
    let content: AnyElement = if let Some(message) = fallback_message {
        div().text_size(px(12.0))
             .text_color(colors.text_tertiary)
             .child(message)
             .into_any_element()
    }
    else {
        div().flex()
             .items_center()
             .gap(px(12.0))
             .child(div().w(px(220.0)).child(
                 Combobox::new(league_combobox).placeholder(t!("league_selector.select_league").to_string())
                                               .search_placeholder(t!("league_selector.search").to_string()),
             ))
             .when(show_competition, |this| {
                 this.child(div().w(px(280.0)).child(
                     Combobox::new(competition_combobox).placeholder(t!("league_selector.select_competition")
                                                                          .to_string())
                                                        .search_placeholder(t!("league_selector.search").to_string()),
                 ))
             })
             .into_any_element()
    };

    div().flex()
         .items_center()
         .px(px(20.0))
         .py(px(12.0))
         .border_b_1()
         .border_color(colors.border)
         .bg(colors.surface)
         .child(content)
         .into_any_element()
}
