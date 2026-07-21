//! Header view: the single persistent nav chrome shown on every screen,
//! replacing the prior sidebar/toolbar/title-bar scaffold. Carries the
//! brand mark, the screen navigation control, and the light/dark theme
//! toggle. The league tab bar this used to show moved to a plugin-driven
//! selector row under the header — see
//! `ui::views::components::league_selector` — since leagues are no longer a
//! fixed list of five.

use gpui::prelude::*;
use gpui::{Context, MouseButton, div, px};
use rust_i18n::t;

use crate::data::theme::{ColorTokens, FullTimeTheme, ThemeKey};
use crate::ui::app_state::AppScreen;
use crate::ui::views::components::tab_bar::render_tab_bar;
use crate::ui::views::root_view::RootView;

/// Renders the header row: brand, screen nav, theme toggle.
pub fn render_header(colors: &ColorTokens, active_screen: AppScreen, theme_key: ThemeKey,
                     cx: &mut Context<RootView>)
                     -> impl IntoElement {
    let leading_inset = if cfg!(target_os = "macos") {
        px(78.0)
    }
    else {
        px(12.0)
    };

    div().h(px(56.0))
         .flex_none()
         .flex()
         .items_center()
         .gap(px(16.0))
         .pl(leading_inset)
         .pr(px(16.0))
         .border_b_1()
         .border_color(colors.border)
         .bg(colors.surface)
         .child(render_brand(colors, cx))
         .child(render_screen_nav(active_screen, cx))
         .child(div().flex_1())
         .child(render_theme_toggle(theme_key, colors, cx))
}

fn render_brand(colors: &ColorTokens, cx: &mut Context<RootView>) -> impl IntoElement {
    let theme = cx.global::<FullTimeTheme>();
    let accent = theme.colors.accent;

    div().flex()
         .items_center()
         .gap(px(8.0))
         .child(div().size(px(22.0)).rounded_full().bg(accent))
         .child(div().text_size(theme.type_scale.brand)
                     .font_weight(gpui::FontWeight::EXTRA_BOLD)
                     .text_color(colors.text_primary)
                     .child(t!("sidebar.app_name").to_string()))
}

fn render_screen_nav(active_screen: AppScreen, cx: &mut Context<RootView>) -> impl IntoElement {
    let selected_index = AppScreen::PRIMARY_NAV.iter()
                                               .position(|s| *s == active_screen)
                                               .unwrap_or(0);
    let labels: Vec<String> = AppScreen::PRIMARY_NAV.iter().map(|s| s.label()).collect();

    render_tab_bar("header-screen-nav",
                   labels,
                   selected_index,
                   cx.listener(|this, ix: &usize, _window, cx| {
                         this.set_screen(AppScreen::PRIMARY_NAV[*ix], cx);
                     }))
}

fn render_theme_toggle(theme_key: ThemeKey, colors: &ColorTokens, cx: &mut Context<RootView>)
                       -> impl IntoElement {
    let icon = match theme_key {
        ThemeKey::Pitch => "☀",
        ThemeKey::PitchNight => "☾",
    };

    div().id("theme-toggle")
         .flex()
         .items_center()
         .justify_center()
         .size(px(32.0))
         .rounded_full()
         .bg(colors.surface_alt)
         .text_color(colors.text_primary)
         .cursor_pointer()
         .on_mouse_up(MouseButton::Left,
                      cx.listener(|this, _event, _window, cx| {
                            this.toggle_theme(cx);
                        }))
         .child(icon)
}
