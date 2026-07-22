//! Status bar view: the persistent footer, matching the mockup's
//! disclaimer text on the left, plus a right-aligned row of icon buttons
//! for cross-screen utilities — Plugins, Activity, and Alerts — matching
//! the status-bar convention in `dtrpg-app.rs`.

use gpui::prelude::*;
use gpui::{Anchor, Context, div, px};
use gpui_component::badge::Badge;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::popover::Popover;
use gpui_component::progress::ProgressCircle;
use gpui_component::status_bar::StatusBar;
use gpui_component::{Icon, Selectable as _, Sizable as _, Size};
use rust_i18n::t;

use crate::data::theme::ColorTokens;
use crate::ui::activity::ActivityControllerHandle;
use crate::ui::app_state::AppScreen;
use crate::ui::views::components::activity_panel::render_activity_panel;
use crate::ui::views::components::alert_history_panel::render_alert_history_panel;
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

    let (in_progress_count, has_unread) = cx.try_global::<ActivityControllerHandle>()
                                            .map(|handle| {
                                                let controller = handle.0.read(cx);
                                                (controller.activity_snapshot().in_progress_count,
                                                 controller.alert_history_snapshot().has_unread)
                                            })
                                            .unwrap_or_default();

    let utility_buttons = div().flex()
                               .items_center()
                               .gap(px(4.0))
                               .child(plugins_button)
                               .child(render_activity_button(colors, in_progress_count > 0))
                               .child(render_alerts_button(colors, has_unread));

    div().h(px(26.0))
         .flex_none()
         .border_t_1()
         .border_color(colors.border)
         .bg(colors.surface_alt)
         .px(px(12.0))
         .child(StatusBar::new().left(disclaimer).right(utility_buttons))
}

/// The activity button: a ghost/compact `Button` wrapping a
/// `ProgressCircle` that idles as a static ring when nothing is
/// `InProgress`, and opens a popover listing the activity log (newest
/// first) when clicked.
fn render_activity_button(colors: &ColorTokens, is_in_progress: bool) -> impl IntoElement {
    let colors = colors.clone();

    Popover::new("status-bar-activity-popover")
        .anchor(Anchor::BottomRight)
        .trigger(Button::new("status-bar-activity").ghost()
                                                    .compact()
                                                    .tooltip(t!("status_bar.activity_tooltip").to_string())
                                                    .child(ProgressCircle::new("status-bar-activity-progress").with_size(Size::XSmall)
                                                                                                               .value(0.0)
                                                                                                               .loading(is_in_progress)))
        .content(move |_state, _window, cx| {
            let entries = cx.try_global::<ActivityControllerHandle>()
                            .map(|handle| handle.0.read(cx).activity_snapshot().entries)
                            .unwrap_or_default();
            render_activity_panel(&colors, &entries)
        })
}

/// The alerts button: a ghost/compact `Button` with a bell icon and an
/// unread-dot overlay, opening a popover listing only `Failed` entries
/// (newest first) when clicked. Opening the popover clears the unread
/// indicator via [`ActivityController::set_alerts_panel_open`].
fn render_alerts_button(colors: &ColorTokens, has_unread: bool) -> impl IntoElement {
    let colors_for_content = colors.clone();

    let trigger =
        Button::new("status-bar-alerts").ghost()
                                        .compact()
                                        .icon(Icon::default().path("icons/bell.svg"))
                                        .tooltip(t!("status_bar.alerts_tooltip").to_string());

    let popover =
        Popover::new("status-bar-alerts-popover")
            .anchor(Anchor::BottomRight)
            .trigger(trigger)
            .on_open_change(|is_open, _window, cx| {
                if let Some(handle) = cx.try_global::<ActivityControllerHandle>() {
                    let controller = handle.0.clone();
                    controller.update(cx, |controller, cx| {
                                  controller.set_alerts_panel_open(*is_open, cx);
                              });
                }
            })
            .content(move |_state, _window, cx| {
                let entries = cx.try_global::<ActivityControllerHandle>()
                                .map(|handle| handle.0.read(cx).alert_history_snapshot().entries)
                                .unwrap_or_default();
                render_alert_history_panel(&colors_for_content, &entries)
            });

    if has_unread {
        Badge::new().dot().child(popover).into_any_element()
    }
    else {
        popover.into_any_element()
    }
}
