//! Status bar view: the persistent footer, matching the mockup's
//! disclaimer text on the left, plus a right-aligned row of icon buttons
//! for cross-screen utilities — currently just Plugins, with room for an
//! activity indicator and an alerts button (not built yet) to join it
//! later, matching the status-bar convention in `dtrpg-app.rs`.

use gpui::prelude::*;
use gpui::{Context, div, px};
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::status_bar::StatusBar;
use gpui_component::{Icon, Selectable as _};
use rust_i18n::t;

use crate::data::theme::ColorTokens;
use crate::ui::app_state::AppScreen;
use crate::ui::views::root_view::RootView;

/// Renders the footer row: disclaimer on the left, utility buttons on the
/// right. `active_screen` is only used to highlight the Plugins button
/// while that screen is open.
pub fn render_status_bar(colors: &ColorTokens, active_screen: AppScreen,
                         cx: &mut Context<RootView>)
                         -> impl IntoElement {
    let disclaimer = div().text_size(px(11.0))
                          .text_color(colors.text_tertiary)
                          .child(t!("status_bar.disclaimer").to_string());

    let plugins_button =
        Button::new("status-bar-plugins").ghost()
                                         .compact()
                                         .selected(active_screen == AppScreen::Plugins)
                                         .icon(Icon::default().path("icons/plug.svg"))
                                         .tooltip(t!("status_bar.plugins_tooltip").to_string())
                                         .on_click(cx.listener(|this, _event, _window, cx| {
                                                         this.toggle_plugins_screen(cx);
                                                     }));

    div().h(px(26.0))
         .flex_none()
         .border_t_1()
         .border_color(colors.border)
         .bg(colors.surface_alt)
         .px(px(12.0))
         .child(StatusBar::new().left(disclaimer).right(plugins_button))
}
