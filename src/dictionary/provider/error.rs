use thiserror::Error;

#[derive(Error, Debug)]
pub enum DictionaryError {
    #[error("Word not found: {word}")]
    WordNotFound { word: String },

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
}
