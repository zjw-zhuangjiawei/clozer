//! Environment variable configuration using envy.
//!
//! Reads values with `CLOZER_` prefix:
//! - `CLOZER_API_KEY` -> `api_key`
//! - `CLOZER_PROVIDER` -> `provider`
//! - `CLOZER_MODEL` -> `model`
//! - `CLOZER_BASE_URL` -> `base_url`

use serde::Deserialize;

/// Configuration loaded from environment variables.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EnvConfig {}

impl EnvConfig {
    /// Loads configuration from environment variables with `CLOZER_` prefix.
    pub fn load() -> Result<Self, envy::Error> {
        envy::prefixed("CLOZER_").from_env()
    }
}
