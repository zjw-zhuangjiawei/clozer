//! General configuration section.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::LogLevel;

/// General configuration for file-based config.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub data_dir: Option<PathBuf>,
    pub log_level: Option<LogLevel>,
}
