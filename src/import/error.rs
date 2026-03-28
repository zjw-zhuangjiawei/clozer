//! Import error types.

use std::fmt;

/// Import error variants.
#[derive(Debug)]
pub enum ImportError {
    /// File not found
    FileNotFound { path: String },
    /// Failed to read file
    IoError { path: String, message: String },
    /// Invalid format
    ParseError { source: String, message: String },
    /// Empty file
    EmptyFile { path: String },
    /// Entry limit exceeded
    EntryLimitExceeded { limit: usize },
    /// Import cancelled
    Cancelled,
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImportError::FileNotFound { path } => write!(f, "File not found: {path}"),
            ImportError::IoError { path, message } => {
                write!(f, "Failed to read file: {path}: {message}")
            }
            ImportError::ParseError { source, message } => {
                write!(f, "Invalid format in {source}: {message}")
            }
            ImportError::EmptyFile { path } => write!(f, "Empty file: {path}"),
            ImportError::EntryLimitExceeded { limit } => {
                write!(f, "Entry limit exceeded: {limit}")
            }
            ImportError::Cancelled => write!(f, "Import cancelled"),
        }
    }
}

impl std::error::Error for ImportError {}

impl ImportError {
    /// Create from IO error with path context.
    pub fn io_error(path: impl Into<String>, source: std::io::Error) -> Self {
        ImportError::IoError {
            path: path.into(),
            message: source.to_string(),
        }
    }

    /// Create a parse error.
    pub fn parse_error(source: impl Into<String>, message: impl Into<String>) -> Self {
        ImportError::ParseError {
            source: source.into(),
            message: message.into(),
        }
    }
}
