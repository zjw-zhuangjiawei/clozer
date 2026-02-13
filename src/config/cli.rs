//! CLI argument configuration using clap.
//!
//! Provides CliConfig for parsing command-line arguments.

use std::path::PathBuf;

use clap::Parser;

use crate::config::LogLevel;

/// Configuration loaded from command-line arguments.
#[derive(Debug, Clone, Parser)]
pub struct CliConfig {
    /// Path to the data directory
    #[clap(short, long)]
    pub data_dir: Option<PathBuf>,

    /// Path to the config file
    #[clap(short, long)]
    pub config_file: Option<PathBuf>,

    /// Log level (trace, debug, info, warn, error)
    #[clap(long, value_enum)]
    pub log_level: Option<LogLevel>,
}

impl CliConfig {
    /// Parses CLI arguments from the provided iterator.
    ///
    /// # Arguments
    ///
    /// - `argv`: Iterator over command-line arguments (typically `std::env::args_os()`)
    pub fn load(argv: impl IntoIterator<Item = std::ffi::OsString>) -> Self {
        Self::parse_from(argv)
    }
}
