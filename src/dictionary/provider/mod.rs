pub mod error;
pub mod free_dictionary;

use crate::dictionary::DictionaryEntry;
pub use error::DictionaryError;

#[derive(Clone)]
pub enum DictionaryProvider {
    FreeDictionary,
}

impl DictionaryProvider {
    pub async fn lookup(&self, word: &str) -> Result<DictionaryEntry, DictionaryError> {
        match self {
            DictionaryProvider::FreeDictionary => free_dictionary::lookup(word).await,
        }
    }
}
