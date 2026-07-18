//! GPUI application bootstrap: window/menu setup and theme wiring.
//!
//! This is scaffolding only — no login/data services are wired in yet (see
//! `crate` docs). [`setup`] opens a themed, empty main window so the shell
//! is visibly running.

use gpui::*;
use gpui_component::{Root, init};
use rust_i18n::t;

use crate::data::theme::FullTimeTheme;
use crate::ui::actions::*;
use crate::ui::views::root_view::RootView;
use crate::util::init::init_globals;

/// Initializes the GPUI application and opens the main window.
pub fn setup(cx: &mut App) {
    init(cx);
    init_globals(cx);

    // Apply the active FullTime theme's font/colors into gpui-component's
    // theme; otherwise buttons, inputs, popovers, scrollbars, the sidebar,
    // etc. all render with gpui-component's default light colors regardless
    // of which FullTime theme is active.
    let initial_theme = cx.global::<FullTimeTheme>().clone();
    cx.update_global::<gpui_component::Theme, _>(|theme, _cx| {
          theme.font_family = initial_theme.fonts.body_font.clone();
          crate::data::theme::apply_theme_colors(theme, &initial_theme.colors);
      });

    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None),
                  KeyBinding::new("cmd-h", HideApplication, None),
                  KeyBinding::new("alt-cmd-h", HideOthers, None),
                  KeyBinding::new("cmd-m", Minimize, None),
                  KeyBinding::new("ctrl-cmd-f", ToggleFullscreen, None)]);

    cx.on_action::<Quit>(|_, cx| cx.quit());
    cx.on_action::<HideApplication>(|_, cx| cx.hide());
    cx.on_action::<HideOthers>(|_, cx| cx.hide_other_apps());
    cx.on_action::<About>(|_, _cx| {});
    cx.on_action::<Minimize>(|_, cx| {
          if let Some(win) = cx.active_window() {
              win.update(cx, |_, window, _| window.minimize_window()).ok();
          }
      });
    cx.on_action::<Zoom>(|_, cx| {
          if let Some(win) = cx.active_window() {
              win.update(cx, |_, window, _| window.zoom_window()).ok();
          }
      });
    cx.on_action::<ToggleFullscreen>(|_, cx| {
          if let Some(win) = cx.active_window() {
              win.update(cx, |_, window, _| window.toggle_fullscreen())
                 .ok();
          }
      });

    cx.set_menus(build_menus());

    open_main_window(cx);
    cx.activate(true);
}

/// Opens the main FullTime window.
///
/// # Panics
///
/// Panics if the window cannot be opened.
#[allow(clippy::expect_used)]
fn open_main_window(cx: &mut App) {
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(t!("sidebar.app_name").to_string().into()),
                appears_transparent: true,
                // Vertically centers the traffic lights within
                // `title_bar::render_title_bar`'s 44px-tall row.
                traffic_light_position: Some(point(px(12.0), px(14.0))),
            }),
            window_bounds: Some(WindowBounds::centered(Size { width: px(1200.), height: px(800.) }, cx)),
            ..Default::default()
        },
        |window, cx| {
            let view = cx.new(|cx| RootView::new(window, cx));
            cx.new(|cx| Root::new(view, window, cx).bordered(false))
        },
    )
    .expect("failed to open main window");
}

/// Builds the native menu bar.
fn build_menus() -> Vec<Menu> {
    vec![
        Menu::new(t!("sidebar.app_name").to_string()).items([
            MenuItem::action(t!("menu.app_about").to_string(), About),
            MenuItem::separator(),
            MenuItem::os_submenu(t!("menu.app_services").to_string(), SystemMenuType::Services),
            MenuItem::separator(),
            MenuItem::action(t!("menu.app_hide").to_string(), HideApplication),
            MenuItem::action(t!("menu.app_hide_others").to_string(), HideOthers),
            MenuItem::action(t!("menu.app_show_all").to_string(), ShowAll),
            MenuItem::separator(),
            MenuItem::action(t!("menu.app_quit").to_string(), Quit),
        ]),
        Menu::new(t!("menu.window_title").to_string()).items([
            MenuItem::action(t!("menu.window_minimize").to_string(), Minimize),
            MenuItem::action(t!("menu.window_zoom").to_string(), Zoom),
        ]),
        Menu::new(t!("menu.help_title").to_string())
            .items([MenuItem::action(t!("menu.app_about").to_string(), About)]),
    ]
}
