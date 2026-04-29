use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Parse error: {0}")]
    Parse(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
