//! Plugins screen: lists installed data-provider plugins with an
//! enable/disable switch per plugin. Data comes from the
//! [`PluginManagerHandle`](crate::ui::plugin_manager::PluginManagerHandle)
//! global; see that module for why `fulltime-ui` doesn't own this state
//! directly.

use gpui::prelude::*;
use gpui::{Context, SharedString, div, px};
use gpui_component::IconName;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::switch::Switch;
use rust_i18n::t;

use crate::data::theme::ColorTokens;
use crate::ui::plugin_manager::{PluginManagerHandle, PluginSummary};
use crate::ui::views::components::card::render_card;
use crate::ui::views::root_view::RootView;

pub fn render_plugins_screen(colors: &ColorTokens, cx: &mut Context<RootView>) -> impl IntoElement {
    let title = div().flex()
                     .items_center()
                     .justify_between()
                     .child(div().text_size(px(20.0))
                                 .font_weight(gpui::FontWeight::BOLD)
                                 .text_color(colors.text_primary)
                                 .child(t!("screen.plugins").to_string()))
                     .child(Button::new("plugins-close").ghost()
                                                        .compact()
                                                        .icon(IconName::Close)
                                                        .tooltip(t!("plugins.close_tooltip").to_string())
                                                        .on_click(cx.listener(
        |this, _event, _window, cx| {
            this.toggle_plugins_screen(cx);
        },
    )));

    let Some(handle) = cx.try_global::<PluginManagerHandle>()
    else {
        return div().flex()
                    .flex_col()
                    .gap(px(20.0))
                    .child(title)
                    .child(div().text_color(colors.text_tertiary)
                                .child(t!("plugins.unavailable").to_string()))
                    .into_any_element();
    };

    let mut plugins = handle.0.list();
    plugins.sort_by(|a, b| a.id.cmp(&b.id));

    if plugins.is_empty() {
        return div().flex()
                    .flex_col()
                    .gap(px(20.0))
                    .child(title)
                    .child(div().text_color(colors.text_tertiary)
                                .child(t!("plugins.none_installed").to_string()))
                    .into_any_element();
    }

    let mut rows = Vec::with_capacity(plugins.len());
    for plugin in plugins {
        rows.push(render_plugin_row(colors, plugin, cx).into_any_element());
    }

    div().flex()
         .flex_col()
         .gap(px(20.0))
         .child(title)
         .child(div().flex().flex_col().gap(px(12.0)).children(rows))
         .into_any_element()
}

fn render_plugin_row(colors: &ColorTokens, plugin: PluginSummary, cx: &mut Context<RootView>)
                     -> impl IntoElement {
    let id = plugin.id.clone();
    let switch_id: SharedString = format!("plugin-toggle-{}", plugin.id).into();

    render_card(cx).flex_row()
                   .items_center()
                   .justify_between()
                   .child(div().flex()
                               .flex_col()
                               .gap(px(2.0))
                               .child(div().font_weight(gpui::FontWeight::SEMIBOLD)
                                           .child(plugin.id.clone()))
                               .child(div().text_size(px(12.0))
                                           .text_color(colors.text_tertiary)
                                           .child(format!("v{}", plugin.version))))
                   .child(Switch::new(switch_id).checked(plugin.enabled)
                                                .on_click(move |checked, _window, cx| {
                                                    if cx.has_global::<PluginManagerHandle>() {
                                                        cx.update_global::<PluginManagerHandle, _>(|handle, cx| {
                                                              handle.0.set_enabled(&id, *checked, cx);
                                                          });
                                                    }
                                                    cx.refresh_windows();
                                                }))
}
