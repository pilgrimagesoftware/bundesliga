## ADDED Requirements

### Requirement: Sandboxed Plugin Execution
`fulltime-core` SHALL execute data-provider plugins inside a WASM sandbox with no ambient
filesystem, process, or network access.

#### Scenario: Plugin attempts unauthorized filesystem access
- **WHEN** a loaded plugin attempts to read or write a file on the host filesystem
- **THEN** the WASM runtime denies the operation and the plugin call fails with a
  capability error, without crashing the host

#### Scenario: Plugin attempts direct network access
- **WHEN** a loaded plugin attempts to open a raw socket or make an HTTP call without
  going through the host-provided fetch capability
- **THEN** the operation is unavailable to the plugin because no such import is granted by
  the runtime

### Requirement: Host-Provided HTTP Capability
The host SHALL expose a constrained HTTP fetch function that plugins call to reach their
upstream data source, scoped to the hosts declared in the plugin's manifest.

#### Scenario: Plugin fetches from a declared host
- **WHEN** a plugin calls the host fetch capability with a request to a hostname listed in
  its manifest's declared network capabilities
- **THEN** the host performs the HTTP request and returns the response to the plugin

#### Scenario: Plugin fetches from an undeclared host
- **WHEN** a plugin calls the host fetch capability with a request to a hostname not
  listed in its manifest
- **THEN** the host rejects the request without making the network call and returns a
  capability error to the plugin

### Requirement: Plugin Lifecycle Management
The host SHALL support loading, initializing, invoking, and unloading a plugin instance
without requiring an app restart.

#### Scenario: Plugin loads successfully
- **WHEN** the host loads a plugin whose manifest declares a schema and interface version
  compatible with the host's current `fulltime-plugin-api` versions
- **THEN** the plugin is instantiated and becomes available for data-provider calls

#### Scenario: Plugin fails to load due to incompatible version
- **WHEN** the host loads a plugin whose manifest declares a schema or interface version
  the host does not support
- **THEN** the host refuses to instantiate the plugin and reports an incompatibility error
  identifying the plugin and the mismatched version

#### Scenario: Plugin is unloaded and reloaded without an app restart
- **WHEN** a user disables then re-enables a plugin, or a plugin is updated on disk
- **THEN** the host drops the existing instance and re-instantiates it on the next call
  without requiring the app to restart

### Requirement: Fault Isolation
A failure or panic inside a plugin SHALL NOT crash the host application or affect other
loaded plugins.

#### Scenario: Plugin call panics
- **WHEN** a plugin's data-provider call panics or traps during execution
- **THEN** the host catches the trap, marks the call as failed, and continues operating
  normally including serving other loaded plugins
