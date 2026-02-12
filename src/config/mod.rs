//! Configuration module for environment variables and settings.
//!
//! Configuration priority (highest to lowest):
//! 1. CLI arguments
//! 2. Environment variables
//! 3. Defaults

pub mod cli;
pub mod env;
pub mod file;

pub use cli::CliConfig;
pub use env::EnvConfig;
pub use file::{FileConfig, GeneralConfig};

use std::path::PathBuf;

/// Unified application configuration.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Path to the data directory
    pub data_dir: PathBuf,

    /// Path to the config file
    pub config_file: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./data"),
            config_file: PathBuf::from("./clozer.toml"),
        }
    }
}

impl AppConfig {
    /// Loads configuration from CLI and environment sources.
    ///
    /// # Arguments
    ///
    /// - `cli`: CLI arguments (highest priority)
    /// - `env`: Environment variables
    pub fn load(cli: CliConfig, env: EnvConfig) -> Self {
        Self {
            data_dir: cli
                .data_dir
                .or(env.data_dir)
                .unwrap_or(PathBuf::from("./data")),
            config_file: cli
                .config_file
                .or(env.config_file.map(Into::into))
                .unwrap_or(PathBuf::from("./clozer.toml")),
        }
    }
}
