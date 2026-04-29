use crate::models::{Cloze, ClozeId, MeaningId};
use crate::persistence::db::CLOZES_TABLE;
use crate::persistence::{ClozeDto, DbError};
use crate::registry::dirty::{DirtyTracker, flush_registry};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default)]
pub struct ClozeRegistry {
    pub(crate) clozes: BTreeMap<ClozeId, Cloze>,
    pub(crate) dirty: DirtyTracker<ClozeId>,
    pub(crate) by_meaning: BTreeMap<MeaningId, BTreeSet<ClozeId>>,
}

impl ClozeRegistry {
    pub fn new() -> Self {
        Self {
            clozes: BTreeMap::new(),
            dirty: DirtyTracker::new(),
            by_meaning: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, cloze: Cloze) {
        let Cloze {
            id,
            meaning_id,
            segments: _,
        } = cloze.clone();
        self.clozes.insert(id, cloze);
        self.dirty.mark(id);
        self.by_meaning.entry(meaning_id).or_default().insert(id);
    }

    pub fn get(&self, id: ClozeId) -> Option<&Cloze> {
        self.clozes.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ClozeId, &Cloze)> {
        self.clozes.iter()
    }

    pub fn get_mut(&mut self, id: ClozeId) -> Option<&mut Cloze> {
        self.clozes.get_mut(&id)
    }

    pub fn iter_by_meaning_id(
        &self,
        meaning_id: MeaningId,
    ) -> impl Iterator<Item = (&ClozeId, &Cloze)> {
        self.by_meaning
            .get(&meaning_id)
            .map(|ids| ids.iter())
            .into_iter()
            .flatten()
            .filter_map(|id| self.clozes.get(id).map(|c| (id, c)))
    }

    pub fn delete(&mut self, id: ClozeId) -> bool {
        if let Some(cloze) = self.clozes.remove(&id) {
            self.dirty.mark(id);
            if let Some(ids) = self.by_meaning.get_mut(&cloze.meaning_id) {
                ids.remove(&id);
                if ids.is_empty() {
                    self.by_meaning.remove(&cloze.meaning_id);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn delete_by_meaning(&mut self, meaning_id: MeaningId) {
        if let Some(cloze_ids) = self.by_meaning.remove(&meaning_id) {
            for cloze_id in cloze_ids {
                self.dirty.mark(cloze_id);
                self.clozes.remove(&cloze_id);
            }
        }
    }

    pub fn count(&self) -> usize {
        self.clozes.len()
    }

    pub fn count_by_meaning(&self, meaning_id: MeaningId) -> usize {
        self.by_meaning
            .get(&meaning_id)
            .map(|s| s.len())
            .unwrap_or(0)
    }

    pub fn exists(&self, id: ClozeId) -> bool {
        self.clozes.contains_key(&id)
    }

    // Persistence
    /// Load all clozes from database
    pub fn load_all(&mut self, db: &crate::persistence::Db) {
        let count = self.clozes.len();
        match db.iter_entities::<ClozeDto>(CLOZES_TABLE) {
            Ok(items) => {
                for (id, mut dto) in items {
                    dto.id = id;
                    let cloze = Cloze::from(dto);
                    self.clozes.insert(cloze.id, cloze.clone());
                    self.by_meaning
                        .entry(cloze.meaning_id)
                        .or_default()
                        .insert(cloze.id);
                }
            }
            Err(e) => {
                tracing::error!(error = %e, source = "cloze_registry", "Failed to load clozes from database");
            }
        }
        let loaded = self.clozes.len() - count;
        tracing::debug!(count = loaded, "Loaded clozes from database");
    }

    /// Flush all dirty entities to the database
    pub fn flush_dirty(&mut self, db: &crate::persistence::Db) -> Result<(), DbError> {
        flush_registry(
            &self.clozes,
            &mut self.dirty,
            db,
            CLOZES_TABLE,
            |c| ClozeDto::from(c),
            "cloze",
        )
    }

    /// Check if there are any dirty entities
    pub fn has_dirty(&self) -> bool {
        self.dirty.has_dirty()
    }
}
