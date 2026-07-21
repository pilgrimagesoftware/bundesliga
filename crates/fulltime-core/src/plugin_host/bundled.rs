//! First-party plugins embedded into the binary from `assets/plugins/`, one
//! subdirectory per plugin (`assets/plugins/<id>/manifest.toml` +
//! `assets/plugins/<id>/plugin.wasm`). See `assets/plugins/README.md`.

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../assets/plugins"]
#[include = "*/manifest.toml"]
#[include = "*/plugin.wasm"]
struct BundledPlugins;

/// A bundled plugin's embedded manifest source and component bytes, found
/// under `assets/plugins/<id>/`.
pub(super) struct BundledPlugin {
    /// The subdirectory name under `assets/plugins/`, used only as an error
    /// message label (the plugin's real `id` is not known until its
    /// manifest is parsed).
    pub(super) label:           String,
    pub(super) manifest_source: String,
    pub(super) component_bytes: Vec<u8>,
}

/// Enumerates every bundled plugin embedded at compile time.
///
/// A subdirectory missing either `manifest.toml` or `plugin.wasm` is
/// skipped — this reads the embedded file table, it doesn't validate
/// manifests (see `registry`'s discovery, which does).
pub(super) fn bundled_plugins() -> Vec<BundledPlugin> {
    let mut labels: Vec<String> = BundledPlugins::iter().filter_map(|path| {
                                                            path.strip_suffix("/manifest.toml")
                                                                .map(str::to_owned)
                                                        })
                                                        .collect();
    labels.sort();

    labels.into_iter()
          .filter_map(|label| {
              let manifest_source = BundledPlugins::get(&format!("{label}/manifest.toml"))
                .and_then(|file| String::from_utf8(file.data.into_owned()).ok())?;
              let component_bytes =
                  BundledPlugins::get(&format!("{label}/plugin.wasm")).map(|file| {
                                                                          file.data.into_owned()
                                                                      })?;
              Some(BundledPlugin { label,
                                   manifest_source,
                                   component_bytes })
          })
          .collect()
}
