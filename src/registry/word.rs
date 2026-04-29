use langtag::LangTagBuf;

use crate::models::{MeaningId, Word, WordId};
use crate::persistence::db::WORDS_TABLE;
use crate::persistence::{DbError, WordDto};
use crate::registry::dirty::{DirtyTracker, flush_registry};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct WordRegistry {
    pub(crate) words: BTreeMap<WordId, Word>,
    pub(crate) dirty: DirtyTracker<WordId>,
}

impl WordRegistry {
    pub fn new() -> Self {
        Self {
            words: BTreeMap::new(),
            dirty: DirtyTracker::new(),
        }
    }

    // CRUD
    pub fn add(&mut self, word: Word) {
        self.words.insert(word.id, word.clone());
        self.dirty.mark(word.id);
    }

    pub fn get(&self, id: WordId) -> Option<&Word> {
        self.words.get(&id)
    }

    pub fn get_mut(&mut self, id: WordId) -> Option<&mut Word> {
        self.words.get_mut(&id)
    }

    pub fn delete(&mut self, id: WordId) -> bool {
        if self.words.remove(&id).is_some() {
            self.dirty.mark(id);
            true
        } else {
            false
        }
    }

    // Iterators
    pub fn iter(&self) -> impl Iterator<Item = (&WordId, &Word)> {
        self.words.iter()
    }

    // Helpers
    pub fn count(&self) -> usize {
        self.words.len()
    }

    pub fn exists(&self, id: WordId) -> bool {
        self.words.contains_key(&id)
    }

    pub fn exists_with_content(&self, content: &str) -> bool {
        self.words
            .iter()
            .any(|(_, w)| w.content.to_lowercase() == content.to_lowercase())
    }

    /// Create a new word with the given content and optional language.
    /// Returns the WordId if successful, None if content is empty or duplicate.
    pub fn create_word(&mut self, content: &str, language: Option<LangTagBuf>) -> Option<WordId> {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return None;
        }

        if self.exists_with_content(trimmed) {
            return None;
        }

        let word = if let Some(ref lang) = language {
            Word::builder()
                .content(trimmed.to_string())
                .language(lang.clone())
                .build()
        } else {
            Word::builder().content(trimmed.to_string()).build()
        };

        let id = word.id;
        self.add(word);
        Some(id)
    }

    // Meaning ID management (syncs with MeaningRegistry)
    pub fn add_meaning(&mut self, word_id: WordId, meaning_id: MeaningId) -> bool {
        if let Some(word) = self.words.get_mut(&word_id) {
            word.meaning_ids.insert(meaning_id);
            self.dirty.mark(word_id);
            true
        } else {
            false
        }
    }

    pub fn remove_meaning(&mut self, word_id: WordId, meaning_id: MeaningId) -> bool {
        if let Some(word) = self.words.get_mut(&word_id) {
            let removed = word.meaning_ids.remove(&meaning_id);
            if removed {
                self.dirty.mark(word_id);
            }
            removed
        } else {
            false
        }
    }

    // Persistence
    /// Load all words from database
    pub fn load_all(&mut self, db: &crate::persistence::Db) {
        let count = self.words.len();
        match db.iter_entities::<WordDto>(WORDS_TABLE) {
            Ok(items) => {
                for (id, mut dto) in items {
                    dto.id = id;
                    let word = Word::from(dto);
                    self.words.insert(word.id, word);
                }
            }
            Err(e) => {
                tracing::error!(error = %e, source = "word_registry", "Failed to load words from database");
            }
        }
        let loaded = self.words.len() - count;
        tracing::debug!(count = loaded, "Loaded words from database");
    }

    /// Flush all dirty entities to the database
    pub fn flush_dirty(&mut self, db: &crate::persistence::Db) -> Result<(), DbError> {
        flush_registry(
            &self.words,
            &mut self.dirty,
            db,
            WORDS_TABLE,
            |w| WordDto::from(w),
            "word",
        )
    }

    /// Check if there are any dirty entities
    pub fn has_dirty(&self) -> bool {
        self.dirty.has_dirty()
    }
}
