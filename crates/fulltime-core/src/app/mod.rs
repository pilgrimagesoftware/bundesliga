//! Application shell modules for the Rust desktop frontend.

mod assets;
#[cfg(feature = "plugin-host")]
mod plugin_manager;

use fulltime_ui::ui::app::setup;
use gpui::*;

/// Boots the GPUI application shell. No service factories are wired up
/// yet — this is the scaffolding pass only; league/match/team data fetching
/// gets wired in once the OpenLigaDB/TheSportsDB port lands. The plugin
/// host is the exception (see `plugin_manager`): it's built and installed
/// as a global before `setup` opens the main window, so the Plugins screen
/// has real data on first render.
pub fn run() {
    gpui_platform::application().with_assets(assets::Assets::new("assets/icons"))
                                .with_quit_mode(QuitMode::LastWindowClosed)
                                .run(|cx| {
                                    // Installed unconditionally, and ahead of
                                    // `plugin_manager::build`, so its startup
                                    // load outcomes have somewhere to record
                                    // into (see `plugin_manager::build`'s
                                    // doc comment). Only bound to a variable
                                    // in the `not(plugin-host)` build, which
                                    // is the only one that records through
                                    // it directly (see below) rather than via
                                    // `plugin_manager`'s own global lookup.
                                    #[cfg(not(feature = "plugin-host"))]
                                    let activity_controller = fulltime_ui::ui::activity::install(cx);
                                    #[cfg(feature = "plugin-host")]
                                    fulltime_ui::ui::activity::install(cx);

                                    #[cfg(feature = "plugin-host")]
                                    match plugin_manager::build(cx) {
                                        Ok(handle) => cx.set_global(handle),
                                        Err(error) => {
                                            tracing::error!(%error, "failed to initialize plugin host")
                                        }
                                    }

                                    // No plugin_manager module exists to record its own
                                    // outcomes in this build, so the absence itself is
                                    // recorded directly — otherwise the Plugins
                                    // screen/league selector's "not available" text is
                                    // the only place this build constraint shows up.
                                    #[cfg(not(feature = "plugin-host"))]
                                    activity_controller.update(cx, |controller, cx| {
                                        controller.record(
                                            "Plugin host unavailable in this build",
                                            fulltime_ui::ui::activity::Status::Failed(
                                                "compiled without the plugin-host feature".to_owned(),
                                            ),
                                            cx,
                                        );
                                    });

                                    setup(cx);
                                });
}
