//! File-based configuration using serde.
//!
//! Reads configuration from config file (e.g., clozer.toml).

use serde::{Deserialize, Serialize};

/// Configuration loaded from file.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct FileConfig {}
