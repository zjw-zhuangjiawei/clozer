//! File-based configuration using serde.
//!
//! Reads configuration from config file (e.g., clozer.toml).

pub mod ai;
pub mod general;

pub use ai::{AiConfig, ModelConfig, ProviderConfig};
pub use general::GeneralConfig;

use serde::{Deserialize, Serialize};

/// Configuration loaded from file.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct FileConfig {
    pub general: GeneralConfig,
    pub ai: AiConfig,
}

impl FileConfig {
    /// Loads configuration from a string.
    ///
    /// # Arguments
    ///
    /// - `s`: TOML content or file path as string
    pub fn load(s: impl AsRef<str>) -> Result<FileConfig, toml::de::Error> {
        toml::from_str(s.as_ref())
    }

    /// Serializes configuration to a TOML string.
    pub fn dump(&self) -> String {
        toml::to_string_pretty(self).expect("Failed to serialize config")
    }
}
