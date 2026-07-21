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
                                    #[cfg(feature = "plugin-host")]
                                    match plugin_manager::build() {
                                        Ok(handle) => cx.set_global(handle),
                                        Err(error) => {
                                            tracing::error!(%error, "failed to initialize plugin host")
                                        }
                                    }

                                    setup(cx);
                                });
}
