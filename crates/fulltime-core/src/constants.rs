// These three are only read from `logging::init_sentry_client`, which is
// compiled only under the `sentry` feature; allow dead-code lint in default
// builds (`cargo check`/`cargo clippy` without `--tests`).

/// Sentry DSN. Presence of this variable (with the `sentry` feature compiled
/// in) is what switches Sentry reporting on; absence leaves it off.
#[cfg_attr(not(feature = "sentry"), allow(dead_code))]
pub const SENTRY_DSN_ENV: &str = "FULLTIME_SENTRY_DSN";
/// Optional override for the Sentry environment tag. Defaults to
/// `"production"`.
#[cfg_attr(not(feature = "sentry"), allow(dead_code))]
pub const SENTRY_ENVIRONMENT_ENV: &str = "FULLTIME_SENTRY_ENVIRONMENT";
/// Optional override for the Sentry release tag. Defaults to
/// `CARGO_PKG_VERSION`.
#[cfg_attr(not(feature = "sentry"), allow(dead_code))]
pub const SENTRY_RELEASE_ENV: &str = "FULLTIME_SENTRY_RELEASE";
#[cfg_attr(not(feature = "sentry"), allow(dead_code))]
pub const SENTRY_DEFAULT_ENVIRONMENT: &str = "production";
