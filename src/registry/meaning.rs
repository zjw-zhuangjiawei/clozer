use crate::models::{CefrLevel, Meaning, MeaningId, PartOfSpeech, TagId, WordId};
use crate::persistence::db::MEANINGS_TABLE;
use crate::persistence::{DbError, MeaningDto};
use crate::registry::dirty::{DirtyTracker, flush_registry};
use either::Either;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Default, Clone)]
pub struct MeaningRegistry {
    pub(crate) meanings: BTreeMap<MeaningId, Meaning>,
    pub(crate) dirty: DirtyTracker<MeaningId>,
    pub(crate) by_word: BTreeMap<WordId, BTreeSet<MeaningId>>,
    pub(crate) by_tag: BTreeMap<TagId, BTreeSet<MeaningId>>,
}

impl MeaningRegistry {
    pub fn new() -> Self {
        Self {
            meanings: BTreeMap::new(),
            dirty: DirtyTracker::new(),
            by_word: BTreeMap::new(),
            by_tag: BTreeMap::new(),
        }
    }

    // CRUD
    pub fn add(&mut self, meaning: Meaning) {
        let meaning_id = meaning.id;
        let word_id = meaning.word_id;

        self.meanings.insert(meaning_id, meaning);
        self.dirty.mark(meaning_id);

        self.by_word.entry(word_id).or_default().insert(meaning_id);

        if let Some(meaning) = self.meanings.get(&meaning_id) {
            for tag_id in &meaning.tag_ids {
                self.by_tag.entry(*tag_id).or_default().insert(meaning_id);
            }
        }
    }

    pub fn get(&self, id: MeaningId) -> Option<&Meaning> {
        self.meanings.get(&id)
    }

    pub fn get_mut(&mut self, id: MeaningId) -> Option<&mut Meaning> {
        self.meanings.get_mut(&id)
    }

    pub fn delete(&mut self, id: MeaningId) -> bool {
        if let Some(meaning) = self.meanings.remove(&id) {
            self.dirty.mark(id);

            if let Some(meaning_ids) = self.by_word.get_mut(&meaning.word_id) {
                meaning_ids.remove(&id);
                if meaning_ids.is_empty() {
                    self.by_word.remove(&meaning.word_id);
                }
            }

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

    pub fn delete_by_word(&mut self, word_id: WordId) {
        if let Some(meaning_ids) = self.by_word.remove(&word_id) {
            for meaning_id in meaning_ids {
                self.dirty.mark(meaning_id);
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
    pub fn iter(&self) -> impl Iterator<Item = (&MeaningId, &Meaning)> {
        self.meanings.iter()
    }

    pub fn iter_by_word(&self, word_id: WordId) -> impl Iterator<Item = (&MeaningId, &Meaning)> {
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

    pub fn iter_by_tag(&self, tag_id: TagId) -> impl Iterator<Item = (&MeaningId, &Meaning)> {
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

    pub fn count_by_word(&self, word_id: WordId) -> usize {
        self.by_word.get(&word_id).map(|s| s.len()).unwrap_or(0)
    }

    pub fn exists(&self, id: MeaningId) -> bool {
        self.meanings.contains_key(&id)
    }

    /// Create a new meaning for a word.
    /// Returns the MeaningId if successful, None if definition is empty.
    pub fn create_meaning(
        &mut self,
        word_id: WordId,
        definition: &str,
        pos: PartOfSpeech,
        cefr_level: Option<CefrLevel>,
    ) -> Option<MeaningId> {
        let trimmed = definition.trim();
        if trimmed.is_empty() {
            return None;
        }

        let mut meaning = Meaning::builder()
            .word_id(word_id)
            .definition(trimmed.to_string())
            .pos(pos)
            .build();
        meaning.cefr_level = cefr_level;

        let id = meaning.id;
        self.add(meaning);
        Some(id)
    }

    // Tag management
    pub fn add_tag(&mut self, meaning_id: MeaningId, tag_id: TagId) -> bool {
        if let Some(meaning) = self.meanings.get_mut(&meaning_id) {
            meaning.tag_ids.insert(tag_id);
            self.by_tag.entry(tag_id).or_default().insert(meaning_id);
            self.dirty.mark(meaning_id);
            true
        } else {
            false
        }
    }

    pub fn remove_tag(&mut self, meaning_id: MeaningId, tag_id: TagId) -> bool {
        let mut removed = false;
        if let Some(meaning) = self.meanings.get_mut(&meaning_id) {
            removed = meaning.tag_ids.remove(&tag_id);
        }
        if removed {
            self.dirty.mark(meaning_id);
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
        let count = self.meanings.len();
        match db.iter_entities::<MeaningDto>(MEANINGS_TABLE) {
            Ok(items) => {
                for (id, mut dto) in items {
                    dto.id = id;
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
            Err(e) => {
                tracing::error!(error = %e, source = "meaning_registry", "Failed to load meanings from database");
            }
        }
        let loaded = self.meanings.len() - count;
        tracing::debug!(count = loaded, "Loaded meanings from database");
    }

    /// Flush all dirty entities to the database
    pub fn flush_dirty(&mut self, db: &crate::persistence::Db) -> Result<(), DbError> {
        flush_registry(
            &self.meanings,
            &mut self.dirty,
            db,
            MEANINGS_TABLE,
            |m| MeaningDto::from(m),
            "meaning",
        )
    }

    /// Check if there are any dirty entities
    pub fn has_dirty(&self) -> bool {
        self.dirty.has_dirty()
    }
}
