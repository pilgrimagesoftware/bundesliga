#!/usr/bin/env bash
set -euo pipefail

# Builds the Bundesliga reference plugin (git submodule at plugins/bundesliga)
# to wasm32-wasip2 and copies its manifest + component into
# assets/plugins/bundesliga/, where rust-embed picks it up as a bundled
# first-party plugin (see crates/fulltime-core/src/plugin_host/bundled.rs).
#
# Debug builds of fulltime-core read assets/plugins/ from disk at runtime, so
# no rebuild is needed after running this for local dev. Release/packaged
# builds embed the files at compile time, so this must run before packaging
# (see crates/fulltime-core/Cargo.toml's before-packaging-command).
#
# Usage: scripts/vendor-bundesliga-plugin.sh [--release]

die() {
  echo "Error: ${1}" >&2
  exit 1
}

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
plugin_src="${repo_root}/plugins/bundesliga"
dest_dir="${repo_root}/assets/plugins/bundesliga"

[[ -f "${plugin_src}/Cargo.toml" ]] || die "plugins/bundesliga submodule not checked out; run: git submodule update --init"

profile="dev"
target_subdir="debug"
if [[ "${1:-}" == "--release" ]]; then
  profile="release"
  target_subdir="release"
fi

command -v cargo >/dev/null 2>&1 || die "cargo is required"

echo "Building Bundesliga plugin (--profile ${profile}, target wasm32-wasip2)..." >&2
(
  cd "${plugin_src}"
  cargo build --profile "${profile}" --target wasm32-wasip2
)

wasm_path="${plugin_src}/target/wasm32-wasip2/${target_subdir}/fulltime_plugin_bundesliga.wasm"
manifest_path="${plugin_src}/manifest.toml"

[[ -f "${wasm_path}" ]] || die "expected component not found at ${wasm_path}"
[[ -f "${manifest_path}" ]] || die "expected manifest not found at ${manifest_path}"

mkdir -p "${dest_dir}"
cp "${manifest_path}" "${dest_dir}/manifest.toml"
cp "${wasm_path}" "${dest_dir}/plugin.wasm"

echo "Vendored Bundesliga plugin into ${dest_dir}" >&2
