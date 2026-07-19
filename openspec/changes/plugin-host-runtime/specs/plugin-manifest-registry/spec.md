## ADDED Requirements

### Requirement: Plugin Discovery
The host SHALL discover bundled first-party plugins and user-installed plugins from
designated plugin directories at startup without requiring manual configuration for the
bundled set.

#### Scenario: Bundled plugin is discovered at startup
- **WHEN** the app starts with a first-party plugin present in the bundled plugin
  directory
- **THEN** the host registers the plugin and it becomes available without user action

#### Scenario: User-installed plugin is discovered
- **WHEN** a plugin package is placed in the user plugin directory
- **THEN** the host discovers it on next startup and registers it alongside bundled
  plugins

### Requirement: Manifest Validation at Registration
The registry SHALL parse each discovered plugin's manifest using `fulltime-plugin-api`'s
manifest schema and refuse to register a plugin whose manifest is malformed or missing
required fields.

#### Scenario: Plugin with a malformed manifest is skipped
- **WHEN** the host discovers a plugin directory whose manifest fails
  `fulltime-plugin-api`'s parse validation
- **THEN** the host does not register that plugin, logs the validation error, and
  continues discovering other plugins

### Requirement: Plugin Enable/Disable State
The host SHALL allow a registered plugin to be individually enabled or disabled without
uninstalling it, and disabled plugins SHALL NOT be loaded or invoked. This state SHALL
persist across restarts, separate from the plugin's own manifest file.

#### Scenario: User disables a plugin
- **WHEN** a user disables a registered plugin
- **THEN** the host stops invoking that plugin for data requests while keeping it
  registered for later re-enabling

#### Scenario: Disabled plugin is not loaded on startup
- **WHEN** the app starts and a registered plugin is marked disabled
- **THEN** the host does not instantiate that plugin's WASM module

### Requirement: Plugin Version Tracking
The registry SHALL track the installed version of each plugin and detect when a newer
version is available for plugins that declare an update source.

#### Scenario: Plugin update is available
- **WHEN** a plugin declares an update source and a newer version exists there than the
  installed version
- **THEN** the registry surfaces that an update is available for that plugin
