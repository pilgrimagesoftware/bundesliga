//! Shared test-only helpers for `plugin_host`'s submodule tests.

use std::path::PathBuf;

/// Compiles `fulltime-plugin-fixture` to `wasm32-wasip2` (a real component,
/// not just a core wasm module — see that crate's own doc comment) and
/// returns its manifest and component paths.
pub(super) fn build_fixture() -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
                                                                  .canonicalize()?;
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());

    let status = std::process::Command::new(cargo).args(["build",
                                                         "--target",
                                                         "wasm32-wasip2",
                                                         "--package",
                                                         "fulltime-plugin-fixture"])
                                                  .current_dir(&workspace_root)
                                                  .status()?;
    if !status.success() {
        return Err(format!("building fulltime-plugin-fixture failed: {status}").into());
    }

    let manifest_path = workspace_root.join("crates/fulltime-plugin-fixture/manifest.toml");
    let wasm_path = workspace_root.join("target/wasm32-wasip2/debug/fulltime_plugin_fixture.wasm");
    Ok((manifest_path, wasm_path))
}
