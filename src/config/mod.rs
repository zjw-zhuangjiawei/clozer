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
pub use file::{AiConfig, FileConfig, GeneralConfig, ModelConfig, ProviderConfig};
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

    /// AI configuration
    pub ai: AiConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            data_dir: paths::data_dir(),
            config_file: paths::config_file(),
            log_level: LogLevel::DEFAULT,
            ai: AiConfig::default(),
        }
    }
}

impl AppConfig {
    /// Saves the current configuration to the config file.
    pub fn save_to_file(&self) {
        tracing::debug!("Saving configuration to file: {:?}", self.config_file);

        if let Some(parent) = self.config_file.parent()
            && let Err(e) = std::fs::create_dir_all(parent)
        {
            tracing::error!("Failed to create config directory: {}", e);
            return;
        }

        let file_config = self.construct_file_config();
        let content = file_config.dump();

        if let Err(e) = std::fs::write(&self.config_file, content) {
            tracing::error!("Failed to write config file: {}", e);
            return;
        }

        tracing::info!("Configuration saved to: {:?}", self.config_file);
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
            ai: self.ai.clone(),
        }
    }

    /// Loads configuration from CLI, environment, and file sources.
    ///
    /// # Arguments
    ///
    /// - `cli`: CLI arguments (highest priority)
    /// - `env`: Environment variables
    pub fn load(cli: CliConfig, env: EnvConfig) -> Result<Self, Box<dyn std::error::Error>> {
        tracing::debug!("Loading configuration...");

        // Resolve config file path
        let config_file = cli
            .config_file
            .or(env.config_file.map(Into::into))
            .unwrap_or_else(paths::config_file);

        tracing::debug!("Config file path: {:?}", config_file);

        // Load file config (ignore errors - use defaults if file missing)
        let file_config = match std::fs::read_to_string(&config_file) {
            Ok(content) => match FileConfig::load(&content) {
                Ok(config) => {
                    tracing::info!("Loaded configuration from file: {:?}", config_file);
                    config
                }
                Err(e) => {
                    tracing::warn!("Failed to parse config file, using defaults: {}", e);
                    FileConfig::default()
                }
            },
            Err(e) => {
                tracing::debug!("No config file found at {:?}, using defaults: {}", config_file, e);
                FileConfig::default()
            }
        };

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

        tracing::info!(
            "Configuration loaded: data_dir={:?}, log_level={:?}",
            data_dir,
            log_level
        );

        Ok(Self {
            data_dir,
            config_file,
            log_level,
            ai: file_config.ai,
        })
    }
}
