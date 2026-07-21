//! League/competition selector row, rendered under the header on every
//! screen. Replaces the header's old static 5-league tab bar: leagues are
//! now whatever plugins are actually loaded, not a fixed list — see
//! `ui::plugin_manager::PluginManager::available_leagues`.
//!
//! Two dropdowns, left to right: League (one entry per loaded plugin) and
//! Competition (that plugin's competitions, most recent first). Selecting
//! either updates `RootView`'s selection and triggers a live standings
//! fetch — see `RootView::select_league`/`select_competition`.

use gpui::prelude::*;
use gpui::{AnyElement, Context, div, px};
use gpui_component::button::Button;
use gpui_component::menu::{DropdownMenu as _, PopupMenuItem};
use rust_i18n::t;

use crate::data::theme::ColorTokens;
use crate::ui::plugin_manager::PluginManagerHandle;
use crate::ui::views::root_view::RootView;

pub fn render_league_selector(colors: &ColorTokens, selected_plugin_id: Option<&str>,
                              selected_competition_id: Option<&str>, cx: &mut Context<RootView>)
                              -> AnyElement {
    if !cx.has_global::<PluginManagerHandle>() {
        return render_message(colors, t!("league_selector.unavailable").to_string());
    }

    let leagues = cx.global::<PluginManagerHandle>().0.available_leagues();
    if leagues.is_empty() {
        return render_message(colors, t!("league_selector.none_loaded").to_string());
    }

    let league_label =
        selected_plugin_id.and_then(|plugin_id| {
                              leagues.iter().find(|league| league.plugin_id == plugin_id)
                          })
                          .map(|league| league.display_name.clone())
                          .unwrap_or_else(|| t!("league_selector.select_league").to_string());

    let selected_plugin_id_owned = selected_plugin_id.map(str::to_owned);
    let selected_competition_id_owned = selected_competition_id.map(str::to_owned);

    let entity = cx.entity();
    let league_entity = entity.clone();
    let selected_plugin_id_for_check = selected_plugin_id_owned.clone();
    let league_dropdown =
        Button::new("league-selector-league").outline()
                                             .compact()
                                             .label(league_label)
                                             .dropdown_caret(true)
                                             .dropdown_menu(move |menu, _, _| {
                let mut menu = menu;
                for league in &leagues {
                    let plugin_id = league.plugin_id.clone();
                    let checked = selected_plugin_id_for_check.as_deref() == Some(league.plugin_id.as_str());
                    let league_entity = league_entity.clone();
                    menu = menu.item(PopupMenuItem::new(league.display_name.clone()).checked(checked).on_click(
                        move |_, _, cx| {
                            league_entity.update(cx, |view, cx| view.select_league(plugin_id.clone(), cx));
                        },
                    ));
                }
                menu
            },
        );

    let competition_dropdown = selected_plugin_id_owned.map(|plugin_id| {
        let competitions = cx.global_mut::<PluginManagerHandle>().0.competitions(&plugin_id);
        let competition_label = selected_competition_id_owned.as_deref()
                                                              .and_then(|competition_id| {
                                                                  competitions.iter().find(|competition| {
                                                                                          competition.id
                                                                                          == competition_id
                                                                                      })
                                                              })
                                                              .map(|competition| competition.name.clone())
                                                              .unwrap_or_else(|| {
                                                                  t!("league_selector.select_competition")
                                                                      .to_string()
                                                              });

        let competition_entity = entity.clone();
        Button::new("league-selector-competition").outline()
                                                  .compact()
                                                  .label(competition_label)
                                                  .dropdown_caret(true)
                                                  .dropdown_menu(move |menu, _, _| {
            let mut menu = menu;
            for competition in &competitions {
                let competition_id = competition.id.clone();
                let checked = selected_competition_id_owned.as_deref() == Some(competition.id.as_str());
                let competition_entity = competition_entity.clone();
                menu = menu.item(
                    PopupMenuItem::new(competition.name.clone()).checked(checked).on_click(
                        move |_, _, cx| {
                            competition_entity.update(cx, |view, cx| {
                                                   view.select_competition(competition_id.clone(), cx);
                                               });
                        },
                    ),
                );
            }
            menu
        })
    });

    div().flex()
         .items_center()
         .gap(px(12.0))
         .px(px(20.0))
         .py(px(12.0))
         .border_b_1()
         .border_color(colors.border)
         .bg(colors.surface)
         .child(league_dropdown)
         .children(competition_dropdown)
         .into_any_element()
}

fn render_message(colors: &ColorTokens, message: String) -> AnyElement {
    div().flex()
         .items_center()
         .px(px(20.0))
         .py(px(12.0))
         .border_b_1()
         .border_color(colors.border)
         .bg(colors.surface)
         .text_size(px(12.0))
         .text_color(colors.text_tertiary)
         .child(message)
         .into_any_element()
}
