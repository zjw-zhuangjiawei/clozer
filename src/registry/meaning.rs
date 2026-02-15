use crate::models::Meaning;
use crate::persistence::DbError;
use either::Either;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct MeaningRegistry {
    meanings: BTreeMap<Uuid, Meaning>,
    dirty_ids: BTreeSet<Uuid>,
    by_word: BTreeMap<Uuid, BTreeSet<Uuid>>,
    by_tag: BTreeMap<Uuid, BTreeSet<Uuid>>,
}

impl MeaningRegistry {
    pub fn new() -> Self {
        Self {
            meanings: BTreeMap::new(),
            dirty_ids: BTreeSet::new(),
            by_word: BTreeMap::new(),
            by_tag: BTreeMap::new(),
        }
    }

    // CRUD
    pub fn insert(&mut self, meaning: Meaning) {
        let meaning_id = meaning.id;
        let word_id = meaning.word_id;
        let tag_ids = meaning.tag_ids.clone();

        self.meanings.insert(meaning_id, meaning);
        self.dirty_ids.insert(meaning_id);

        // Update by_word index
        self.by_word.entry(word_id).or_default().insert(meaning_id);

        // Update by_tag index
        for tag_id in tag_ids {
            self.by_tag.entry(tag_id).or_default().insert(meaning_id);
        }
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<&Meaning> {
        self.meanings.get(&id)
    }

    pub fn get_by_id_mut(&mut self, id: Uuid) -> Option<&mut Meaning> {
        self.meanings.get_mut(&id)
    }

    pub fn delete(&mut self, id: Uuid) -> bool {
        if let Some(meaning) = self.meanings.remove(&id) {
            self.dirty_ids.insert(id);

            // Remove from by_word
            if let Some(meaning_ids) = self.by_word.get_mut(&meaning.word_id) {
                meaning_ids.remove(&id);
                if meaning_ids.is_empty() {
                    self.by_word.remove(&meaning.word_id);
                }
            }

            // Remove from by_tag
            for tag_id in meaning.tag_ids {
                if let Some(meaning_ids) = self.by_tag.get_mut(&tag_id) {
                    meaning_ids.remove(&id);
                    if meaning_ids.is_empty() {
                        self.by_tag.remove(&tag_id);
                    }
                }
            }
            true
        } else {
            false
        }
    }

    pub fn delete_by_word(&mut self, word_id: Uuid) {
        if let Some(meaning_ids) = self.by_word.remove(&word_id) {
            for meaning_id in meaning_ids {
                self.dirty_ids.insert(meaning_id);
                if let Some(meaning) = self.meanings.remove(&meaning_id) {
                    for tag_id in meaning.tag_ids {
                        if let Some(ids) = self.by_tag.get_mut(&tag_id) {
                            ids.remove(&meaning_id);
                            if ids.is_empty() {
                                self.by_tag.remove(&tag_id);
                            }
                        }
                    }
                }
            }
        }
    }

    // Iterators
    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &Meaning)> {
        self.meanings.iter()
    }

    pub fn iter_by_word(&self, word_id: Uuid) -> impl Iterator<Item = (&Uuid, &Meaning)> {
        self.by_word
            .get(&word_id)
            .map(|ids| {
                Either::Left(
                    ids.iter()
                        .filter_map(|id| self.meanings.get(id).map(|m| (id, m))),
                )
            })
            .unwrap_or_else(|| Either::Right(std::iter::empty()))
    }

    pub fn iter_by_tag(&self, tag_id: Uuid) -> impl Iterator<Item = (&Uuid, &Meaning)> {
        self.by_tag
            .get(&tag_id)
            .map(|ids| {
                Either::Left(
                    ids.iter()
                        .filter_map(|id| self.meanings.get(id).map(|m| (id, m))),
                )
            })
            .unwrap_or_else(|| Either::Right(std::iter::empty()))
    }

    // Helpers
    pub fn count(&self) -> usize {
        self.meanings.len()
    }

    pub fn count_by_word(&self, word_id: Uuid) -> usize {
        self.by_word.get(&word_id).map(|s| s.len()).unwrap_or(0)
    }

    pub fn exists(&self, id: Uuid) -> bool {
        self.meanings.contains_key(&id)
    }

    // Tag management
    pub fn add_tag(&mut self, meaning_id: Uuid, tag_id: Uuid) -> bool {
        if let Some(meaning) = self.meanings.get_mut(&meaning_id) {
            meaning.tag_ids.insert(tag_id);
            self.by_tag.entry(tag_id).or_default().insert(meaning_id);
            self.dirty_ids.insert(meaning_id);
            true
        } else {
            false
        }
    }

    pub fn remove_tag(&mut self, meaning_id: Uuid, tag_id: Uuid) -> bool {
        let mut removed = false;
        if let Some(meaning) = self.meanings.get_mut(&meaning_id) {
            removed = meaning.tag_ids.remove(&tag_id);
        }
        if removed {
            self.dirty_ids.insert(meaning_id);
            if let Some(ids) = self.by_tag.get_mut(&tag_id) {
                ids.remove(&meaning_id);
                if ids.is_empty() {
                    self.by_tag.remove(&tag_id);
                }
            }
        }
        removed
    }

    // Persistence
    /// Load all meanings from database
    pub fn load_all(&mut self, db: &crate::persistence::Db) {
        if let Ok(items) = db.iter_meanings() {
            for dto in items {
                let meaning = Meaning::from(dto);
                self.meanings.insert(meaning.id, meaning.clone());
                self.by_word
                    .entry(meaning.word_id)
                    .or_default()
                    .insert(meaning.id);
                for tag_id in &meaning.tag_ids {
                    self.by_tag.entry(*tag_id).or_default().insert(meaning.id);
                }
            }
        }
    }

    /// Flush all dirty entities to the database
    pub fn flush_dirty(&mut self, db: &crate::persistence::Db) -> Result<(), DbError> {
        for id in &self.dirty_ids {
            if let Some(meaning) = self.meanings.get(id) {
                let dto = crate::persistence::MeaningDto::from(meaning);
                db.save_meaning(*id, &dto)?;
            }
        }
        self.dirty_ids.clear();
        Ok(())
    }

    /// Check if there are any dirty entities
    pub fn has_dirty(&self) -> bool {
        !self.dirty_ids.is_empty()
    }
}
