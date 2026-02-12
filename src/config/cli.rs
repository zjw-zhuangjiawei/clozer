//! CLI argument configuration using clap.
//!
//! Provides CliConfig for parsing command-line arguments.

use clap::Parser;

/// Configuration loaded from command-line arguments.
#[derive(Debug, Clone, Parser)]
pub struct CliConfig {}
