
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * This module provides access to environment variables that are injected _statically_ into your bundle at build time and are limited to _private_ access.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Static environment variables are [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env` at build time and then statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * **_Private_ access:**
 * 
 * - This module cannot be imported into client-side code
 * - This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured)
 * 
 * For example, given the following build time environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { ENVIRONMENT, PUBLIC_BASE_URL } from '$env/static/private';
 * 
 * console.log(ENVIRONMENT); // => "production"
 * console.log(PUBLIC_BASE_URL); // => throws error during build
 * ```
 * 
 * The above values will be the same _even if_ different values for `ENVIRONMENT` or `PUBLIC_BASE_URL` are set at runtime, as they are statically replaced in your code with their build time values.
 */
declare module '$env/static/private' {
	export const SHELL: string;
	export const npm_command: string;
	export const LSCOLORS: string;
	export const WINDOWID: string;
	export const COLORTERM: string;
	export const FLUX_GITHUB_TOKEN: string;
	export const PYENV_SHELL: string;
	export const LESS: string;
	export const XPC_FLAGS: string;
	export const TERM_PROGRAM_VERSION: string;
	export const SWIFTLY_TOOLCHAINS_DIR: string;
	export const npm_config_npm_globalconfig: string;
	export const FPATH: string;
	export const _P9K_TTY: string;
	export const NODE: string;
	export const TAURI_ENV_DEBUG: string;
	export const __CFBundleIdentifier: string;
	export const SSH_AUTH_SOCK: string;
	export const ANTHROPIC_API_KEY: string;
	export const npm_config_verify_deps_before_run: string;
	export const MallocNanoZone: string;
	export const P9K_TTY: string;
	export const CURSEFORGE_API_KEY: string;
	export const npm_config__jsr_registry: string;
	export const OSLogRateLimit: string;
	export const NODENV_DIR: string;
	export const HOMEBREW_PREFIX: string;
	export const RBENV_SHELL: string;
	export const NODENV_VERSION: string;
	export const npm_config_globalconfig: string;
	export const EDITOR: string;
	export const JENV_SHELL: string;
	export const MACOSX_DEPLOYMENT_TARGET: string;
	export const GOENV_SHELL: string;
	export const PWD: string;
	export const LOGNAME: string;
	export const TAURI_ENV_PLATFORM: string;
	export const SWIFTLY_BIN_DIR: string;
	export const PNPM_HOME: string;
	export const COMMAND_MODE: string;
	export const NODENV_SHELL: string;
	export const HOME: string;
	export const MCFLY_HISTORY: string;
	export const NODENV_ORIG_PATH: string;
	export const LANG: string;
	export const MCFLY_HISTFILE: string;
	export const LS_COLORS: string;
	export const CARGO_HOME: string;
	export const RUSTUP_TOOLCHAIN: string;
	export const NODENV_ROOT: string;
	export const npm_package_version: string;
	export const VIRTUAL_ENV_DISABLE_PROMPT: string;
	export const TMPDIR: string;
	export const FG_HOME: string;
	export const ANTHROPIC_BASE_URL: string;
	export const pnpm_config_verify_deps_before_run: string;
	export const GOROOT: string;
	export const NODENV_HOOK_PATH: string;
	export const MCFLY_SESSION_ID: string;
	export const INIT_CWD: string;
	export const INFOPATH: string;
	export const npm_lifecycle_script: string;
	export const RUST_RECURSION_COUNT: string;
	export const NVM_DIR: string;
	export const SWIFTLY_HOME_DIR: string;
	export const TAURI_ENV_PLATFORM_VERSION: string;
	export const TAURI_ENV_FAMILY: string;
	export const TERM: string;
	export const npm_package_name: string;
	export const ZSH: string;
	export const RUSTUP_HOME: string;
	export const USER: string;
	export const npm_config_frozen_lockfile: string;
	export const PARADOX_GITHUB_TOKEN: string;
	export const HOMEBREW_CELLAR: string;
	export const MallocSpaceEfficient: string;
	export const MCFLY_HISTORY_FORMAT: string;
	export const TAURI_ENV_TARGET_TRIPLE: string;
	export const npm_lifecycle_event: string;
	export const SHLVL: string;
	export const EXTENSION_KIT_EXTENSION_TYPE: string;
	export const PAGER: string;
	export const GOENV_ROOT: string;
	export const TAURI_CLI_VERBOSITY: string;
	export const HOMEBREW_REPOSITORY: string;
	export const _P9K_SSH_TTY: string;
	export const XPC_SERVICE_NAME: string;
	export const npm_config_user_agent: string;
	export const KUBECONFIG: string;
	export const PNPM_SCRIPT_SRC_DIR: string;
	export const npm_execpath: string;
	export const HOMEBREW_GITHUB_API_TOKEN: string;
	export const SWIFTENV_ROOT: string;
	export const TFENV_ROOT: string;
	export const QLTY_INSTALL: string;
	export const NODE_PATH: string;
	export const PYENV_ROOT: string;
	export const LUAENV_SHELL: string;
	export const npm_package_json: string;
	export const TAURI_ENV_ARCH: string;
	export const P9K_SSH: string;
	export const LUAENV_ROOT: string;
	export const ZED_ENVIRONMENT: string;
	export const DOCKER_GITHUB_TOKEN: string;
	export const PATH: string;
	export const CARGO: string;
	export const RUSTUP_TOOLCHAIN_SOURCE: string;
	export const npm_config_registry: string;
	export const JENV_ROOT: string;
	export const JENV_LOADED: string;
	export const ALACRITTY_WINDOW_ID: string;
	export const RBENV_ROOT: string;
	export const ZED_TERM: string;
	export const npm_node_execpath: string;
	export const OLDPWD: string;
	export const GOPATH: string;
	export const __CF_USER_TEXT_ENCODING: string;
	export const TERM_PROGRAM: string;
	export const NODE_ENV: string;
}

/**
 * This module provides access to environment variables that are injected _statically_ into your bundle at build time and are _publicly_ accessible.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Static environment variables are [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env` at build time and then statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * **_Public_ access:**
 * 
 * - This module _can_ be imported into client-side code
 * - **Only** variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`) are included
 * 
 * For example, given the following build time environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { ENVIRONMENT, PUBLIC_BASE_URL } from '$env/static/public';
 * 
 * console.log(ENVIRONMENT); // => throws error during build
 * console.log(PUBLIC_BASE_URL); // => "http://site.com"
 * ```
 * 
 * The above values will be the same _even if_ different values for `ENVIRONMENT` or `PUBLIC_BASE_URL` are set at runtime, as they are statically replaced in your code with their build time values.
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to environment variables set _dynamically_ at runtime and that are limited to _private_ access.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Dynamic environment variables are defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`.
 * 
 * **_Private_ access:**
 * 
 * - This module cannot be imported into client-side code
 * - This module includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured)
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 * 
 * > [!NOTE] To get correct types, environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * >
 * > ```env
 * > MY_FEATURE_FLAG=
 * > ```
 * >
 * > You can override `.env` values from the command line like so:
 * >
 * > ```sh
 * > MY_FEATURE_FLAG="enabled" npm run dev
 * > ```
 * 
 * For example, given the following runtime environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * 
 * console.log(env.ENVIRONMENT); // => "production"
 * console.log(env.PUBLIC_BASE_URL); // => undefined
 * ```
 */
declare module '$env/dynamic/private' {
	export const env: {
		SHELL: string;
		npm_command: string;
		LSCOLORS: string;
		WINDOWID: string;
		COLORTERM: string;
		FLUX_GITHUB_TOKEN: string;
		PYENV_SHELL: string;
		LESS: string;
		XPC_FLAGS: string;
		TERM_PROGRAM_VERSION: string;
		SWIFTLY_TOOLCHAINS_DIR: string;
		npm_config_npm_globalconfig: string;
		FPATH: string;
		_P9K_TTY: string;
		NODE: string;
		TAURI_ENV_DEBUG: string;
		__CFBundleIdentifier: string;
		SSH_AUTH_SOCK: string;
		ANTHROPIC_API_KEY: string;
		npm_config_verify_deps_before_run: string;
		MallocNanoZone: string;
		P9K_TTY: string;
		CURSEFORGE_API_KEY: string;
		npm_config__jsr_registry: string;
		OSLogRateLimit: string;
		NODENV_DIR: string;
		HOMEBREW_PREFIX: string;
		RBENV_SHELL: string;
		NODENV_VERSION: string;
		npm_config_globalconfig: string;
		EDITOR: string;
		JENV_SHELL: string;
		MACOSX_DEPLOYMENT_TARGET: string;
		GOENV_SHELL: string;
		PWD: string;
		LOGNAME: string;
		TAURI_ENV_PLATFORM: string;
		SWIFTLY_BIN_DIR: string;
		PNPM_HOME: string;
		COMMAND_MODE: string;
		NODENV_SHELL: string;
		HOME: string;
		MCFLY_HISTORY: string;
		NODENV_ORIG_PATH: string;
		LANG: string;
		MCFLY_HISTFILE: string;
		LS_COLORS: string;
		CARGO_HOME: string;
		RUSTUP_TOOLCHAIN: string;
		NODENV_ROOT: string;
		npm_package_version: string;
		VIRTUAL_ENV_DISABLE_PROMPT: string;
		TMPDIR: string;
		FG_HOME: string;
		ANTHROPIC_BASE_URL: string;
		pnpm_config_verify_deps_before_run: string;
		GOROOT: string;
		NODENV_HOOK_PATH: string;
		MCFLY_SESSION_ID: string;
		INIT_CWD: string;
		INFOPATH: string;
		npm_lifecycle_script: string;
		RUST_RECURSION_COUNT: string;
		NVM_DIR: string;
		SWIFTLY_HOME_DIR: string;
		TAURI_ENV_PLATFORM_VERSION: string;
		TAURI_ENV_FAMILY: string;
		TERM: string;
		npm_package_name: string;
		ZSH: string;
		RUSTUP_HOME: string;
		USER: string;
		npm_config_frozen_lockfile: string;
		PARADOX_GITHUB_TOKEN: string;
		HOMEBREW_CELLAR: string;
		MallocSpaceEfficient: string;
		MCFLY_HISTORY_FORMAT: string;
		TAURI_ENV_TARGET_TRIPLE: string;
		npm_lifecycle_event: string;
		SHLVL: string;
		EXTENSION_KIT_EXTENSION_TYPE: string;
		PAGER: string;
		GOENV_ROOT: string;
		TAURI_CLI_VERBOSITY: string;
		HOMEBREW_REPOSITORY: string;
		_P9K_SSH_TTY: string;
		XPC_SERVICE_NAME: string;
		npm_config_user_agent: string;
		KUBECONFIG: string;
		PNPM_SCRIPT_SRC_DIR: string;
		npm_execpath: string;
		HOMEBREW_GITHUB_API_TOKEN: string;
		SWIFTENV_ROOT: string;
		TFENV_ROOT: string;
		QLTY_INSTALL: string;
		NODE_PATH: string;
		PYENV_ROOT: string;
		LUAENV_SHELL: string;
		npm_package_json: string;
		TAURI_ENV_ARCH: string;
		P9K_SSH: string;
		LUAENV_ROOT: string;
		ZED_ENVIRONMENT: string;
		DOCKER_GITHUB_TOKEN: string;
		PATH: string;
		CARGO: string;
		RUSTUP_TOOLCHAIN_SOURCE: string;
		npm_config_registry: string;
		JENV_ROOT: string;
		JENV_LOADED: string;
		ALACRITTY_WINDOW_ID: string;
		RBENV_ROOT: string;
		ZED_TERM: string;
		npm_node_execpath: string;
		OLDPWD: string;
		GOPATH: string;
		__CF_USER_TEXT_ENCODING: string;
		TERM_PROGRAM: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * This module provides access to environment variables set _dynamically_ at runtime and that are _publicly_ accessible.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Dynamic environment variables are defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`.
 * 
 * **_Public_ access:**
 * 
 * - This module _can_ be imported into client-side code
 * - **Only** variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`) are included
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 * 
 * > [!NOTE] To get correct types, environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * >
 * > ```env
 * > MY_FEATURE_FLAG=
 * > ```
 * >
 * > You can override `.env` values from the command line like so:
 * >
 * > ```sh
 * > MY_FEATURE_FLAG="enabled" npm run dev
 * > ```
 * 
 * For example, given the following runtime environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://example.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.ENVIRONMENT); // => undefined, not public
 * console.log(env.PUBLIC_BASE_URL); // => "http://example.com"
 * ```
 * 
 * ```
 * 
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
