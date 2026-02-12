//! Environment variable configuration using envy.

use std::path::PathBuf;

use serde::Deserialize;

/// Configuration loaded from environment variables.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct EnvConfig {
    pub data_dir: Option<PathBuf>,
    pub config_file: Option<String>,
}

impl EnvConfig {
    /// Loads configuration from environment variables with `CLOZER_` prefix.
    pub fn load() -> Result<Self, envy::Error> {
        envy::prefixed("CLOZER_").from_env()
    }
}
