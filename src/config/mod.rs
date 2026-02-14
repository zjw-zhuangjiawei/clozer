//! Configuration module for environment variables and settings.
//!
//! Configuration priority (highest to lowest):
//! 1. CLI arguments
//! 2. Environment variables
//! 3. Defaults

pub mod cli;
pub mod constants;
pub mod env;
pub mod file;

use clap::ValueEnum;
pub use cli::CliConfig;
pub use constants::paths;
pub use env::EnvConfig;
pub use file::{FileConfig, GeneralConfig};
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

/// Log level enum for configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub const fn into_tracing_level(self) -> tracing::Level {
        match self {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }

    /// Default log level.
    pub const DEFAULT: Self = LogLevel::Info;
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Unified application configuration.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Path to the data directory
    pub data_dir: PathBuf,

    /// Path to the config file
    pub config_file: PathBuf,

    /// Log level
    pub log_level: LogLevel,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            data_dir: paths::data_dir(),
            config_file: paths::config_file(),
            log_level: LogLevel::DEFAULT,
        }
    }
}

impl AppConfig {
    /// Saves the current configuration to the config file.
    pub fn save_to_file(&self) {
        if let Some(parent) = self.config_file.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create config directory");
        }

        let file_config = self.construct_file_config();
        let content = file_config.dump();
        std::fs::write(&self.config_file, content).expect("Failed to write config file");
    }

    /// Constructs a `FileConfig` from this `AppConfig`.
    ///
    /// This method is used when saving configuration changes
    /// to persist the current settings to the config file.
    ///
    /// # Returns
    ///
    /// A `FileConfig` with settings derived from this `AppConfig`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let file_config = app_config.construct_file_config();
    /// ```
    pub fn construct_file_config(&self) -> FileConfig {
        FileConfig {
            general: GeneralConfig {
                data_dir: Some(self.data_dir.clone()),
                log_level: Some(self.log_level),
            },
        }
    }

    /// Loads configuration from CLI, environment, and file sources.
    ///
    /// # Arguments
    ///
    /// - `cli`: CLI arguments (highest priority)
    /// - `env`: Environment variables
    pub fn load(cli: CliConfig, env: EnvConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Resolve config file path
        let config_file = cli
            .config_file
            .or(env.config_file.map(Into::into))
            .unwrap_or_else(paths::config_file);

        // Load file config (ignore errors - use defaults if file missing)
        let file_config = std::fs::read_to_string(&config_file)
            .ok()
            .and_then(|s| FileConfig::load(&s).ok())
            .unwrap_or_default();

        // Resolve data directory (file config has lowest priority)
        let data_dir = cli
            .data_dir
            .or(env.data_dir)
            .or(file_config.general.data_dir)
            .unwrap_or_else(paths::data_dir);

        // Resolve log level with priority: CLI > env > file > default
        let log_level = cli
            .log_level
            .or(env.log_level)
            .or(file_config.general.log_level)
            .unwrap_or_default();

        Ok(Self {
            data_dir,
            config_file,
            log_level,
        })
    }
}
