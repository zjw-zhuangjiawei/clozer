use crate::models::Word;
use crate::persistence::DbError;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct WordRegistry {
    words: BTreeMap<Uuid, Word>,
    dirty_ids: BTreeSet<Uuid>,
}

impl WordRegistry {
    pub fn new() -> Self {
        Self {
            words: BTreeMap::new(),
            dirty_ids: BTreeSet::new(),
        }
    }

    // CRUD
    pub fn add(&mut self, word: Word) {
        self.words.insert(word.id, word.clone());
        self.dirty_ids.insert(word.id);
    }

    pub fn get(&self, id: Uuid) -> Option<&Word> {
        self.words.get(&id)
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Word> {
        self.words.get_mut(&id)
    }

    pub fn delete(&mut self, id: Uuid) -> bool {
        if self.words.remove(&id).is_some() {
            self.dirty_ids.insert(id);
            true
        } else {
            false
        }
    }

    // Iterators
    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &Word)> {
        self.words.iter()
    }

    // Helpers
    pub fn count(&self) -> usize {
        self.words.len()
    }

    pub fn exists(&self, id: Uuid) -> bool {
        self.words.contains_key(&id)
    }

    // Meaning ID management (syncs with MeaningRegistry)
    pub fn add_meaning(&mut self, word_id: Uuid, meaning_id: Uuid) -> bool {
        if let Some(word) = self.words.get_mut(&word_id) {
            word.meaning_ids.insert(meaning_id);
            self.dirty_ids.insert(word_id);
            true
        } else {
            false
        }
    }

    pub fn remove_meaning(&mut self, word_id: Uuid, meaning_id: Uuid) -> bool {
        if let Some(word) = self.words.get_mut(&word_id) {
            let removed = word.meaning_ids.remove(&meaning_id);
            if removed {
                self.dirty_ids.insert(word_id);
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
        match db.iter_words() {
            Ok(items) => {
                for dto in items {
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
        let mut errors = 0;
        let dirty_ids: Vec<_> = self.dirty_ids.iter().copied().collect();
        for id in dirty_ids {
            if let Some(word) = self.words.get(&id) {
                let dto = crate::persistence::WordDto::from(word);
                match db.save_word(id, &dto) {
                    Ok(_) => {
                        self.dirty_ids.remove(&id);
                    }
                    Err(e) => {
                        errors += 1;
                        tracing::error!(word_id = %id, error = %e, "Failed to save word");
                    }
                }
            } else {
                match db.delete_word(id) {
                    Ok(_) => {
                        self.dirty_ids.remove(&id);
                    }
                    Err(e) => {
                        errors += 1;
                        tracing::error!(word_id = %id, error = %e, "Failed to delete word");
                    }
                }
            }
        }
        if errors > 0 {
            tracing::warn!(errors = errors, "Some words failed to persist");
        }
        Ok(())
    }

    /// Check if there are any dirty entities
    pub fn has_dirty(&self) -> bool {
        !self.dirty_ids.is_empty()
    }
}
