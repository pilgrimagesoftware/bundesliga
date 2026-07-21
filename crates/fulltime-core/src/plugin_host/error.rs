//! Errors surfaced by the plugin host runtime.

use fulltime_plugin_api::{ManifestError, Version};

use super::bindings::fulltime::plugin_api::errors::ProviderError;

/// Everything that can go wrong loading or calling a plugin.
///
/// None of these variants indicate the host itself crashed: a call failure
/// (including a plugin trap, see [`Self::Call`]) is reported here rather than
/// propagated as a panic, per the fault-isolation requirement in
/// `openspec/changes/plugin-host-runtime/specs/plugin-host-runtime/spec.md`.
#[derive(Debug, thiserror::Error)]
pub enum PluginHostError {
    /// The plugin's manifest file could not be read from disk.
    #[error("failed to read manifest for plugin at {location}: {source}")]
    ManifestIo {
        /// Path to the manifest file that could not be read.
        location: String,
        /// Underlying I/O error.
        #[source]
        source:   std::io::Error,
    },

    /// The plugin's manifest is malformed or missing required fields.
    #[error("plugin manifest at {location} is invalid: {source}")]
    InvalidManifest {
        /// Path to the invalid manifest file, or a descriptive label if the
        /// manifest came from an embedded source rather than a real file.
        location: String,
        /// Underlying validation error.
        #[source]
        source:   ManifestError,
    },

    /// The plugin declares a schema or interface version this host does not
    /// support.
    #[error(
            "plugin {plugin_id:?} targets schema {plugin_schema} / interface {plugin_interface}, \
         incompatible with this host's schema {host_schema} / interface {host_interface}"
    )]
    IncompatibleVersion {
        /// The plugin's manifest-declared `id`.
        plugin_id:        String,
        /// The plugin's manifest-declared schema version.
        plugin_schema:    Version,
        /// The plugin's manifest-declared interface version.
        plugin_interface: Version,
        /// The schema version this host supports.
        host_schema:      Version,
        /// The interface version this host supports.
        host_interface:   Version,
    },

    /// The plugin's `.wasm` component bytes could not be read from disk.
    #[error("failed to read component bytes at {location}: {source}")]
    ComponentIo {
        /// Path to the component file that could not be read.
        location: String,
        /// Underlying I/O error.
        #[source]
        source:   std::io::Error,
    },

    /// The plugin's `.wasm` bytes failed to compile as a valid component.
    #[error("failed to compile plugin {plugin_id:?} as a component: {source}")]
    Compile {
        /// The plugin's manifest-declared `id`.
        plugin_id: String,
        /// Underlying compile error.
        #[source]
        source:    anyhow::Error,
    },

    /// No plugin with this ID is currently loaded.
    #[error("plugin {plugin_id:?} is not loaded")]
    NotLoaded {
        /// The plugin ID that was requested.
        plugin_id: String,
    },

    /// A data-provider call failed at the host/instantiation level: an
    /// invocation error, or the plugin itself panicked or trapped during
    /// execution. Distinct from [`Self::ProviderError`], which is a
    /// structured failure the plugin itself reported.
    #[error("call to plugin {plugin_id:?} failed: {source}")]
    Call {
        /// The plugin's manifest-declared `id`.
        plugin_id: String,
        /// Underlying error, which may wrap a caught WASM trap.
        #[source]
        source:    anyhow::Error,
    },

    /// The plugin's data-provider call completed but reported a structured
    /// failure (e.g. its upstream source was unreachable or rate-limited).
    #[error("plugin {plugin_id:?} reported a provider error: {source:?}")]
    ProviderError {
        /// The plugin's manifest-declared `id`.
        plugin_id: String,
        /// The structured error the plugin returned.
        source:    ProviderError,
    },
}
