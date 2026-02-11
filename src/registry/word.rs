use crate::models::Word;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct WordRegistry {
    words: HashMap<Uuid, Word>,
}

impl WordRegistry {
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
        }
    }

    // CRUD
    pub fn insert(&mut self, word: Word) {
        self.words.insert(word.id, word);
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<&Word> {
        self.words.get(&id)
    }

    pub fn get_by_id_mut(&mut self, id: Uuid) -> Option<&mut Word> {
        self.words.get_mut(&id)
    }

    pub fn delete(&mut self, id: Uuid) -> bool {
        self.words.remove(&id).is_some()
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
            true
        } else {
            false
        }
    }

    pub fn remove_meaning(&mut self, word_id: Uuid, meaning_id: Uuid) -> bool {
        if let Some(word) = self.words.get_mut(&word_id) {
            word.meaning_ids.remove(&meaning_id)
        } else {
            false
        }
    }
}
