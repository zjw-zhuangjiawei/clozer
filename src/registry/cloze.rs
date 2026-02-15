use crate::models::Cloze;
use crate::persistence::DbError;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct ClozeRegistry {
    clozes: BTreeMap<Uuid, Cloze>,
    dirty_ids: BTreeSet<Uuid>,
    by_meaning: BTreeMap<Uuid, BTreeSet<Uuid>>,
}

impl ClozeRegistry {
    pub fn new() -> Self {
        Self {
            clozes: BTreeMap::new(),
            dirty_ids: BTreeSet::new(),
            by_meaning: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, cloze: Cloze) {
        let Cloze { id, meaning_id, segments } = cloze.clone();
        tracing::debug!(cloze_id = %id, meaning_id = %meaning_id, segments_count = segments.len(), "Adding cloze to registry");
        self.clozes.insert(id, cloze.clone());
        self.dirty_ids.insert(id);
        self.by_meaning.entry(meaning_id).or_default().insert(id);
    }

    pub fn get(&self, id: Uuid) -> Option<&Cloze> {
        self.clozes.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &Cloze)> {
        self.clozes.iter()
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Cloze> {
        self.clozes.get_mut(&id)
    }

    pub fn iter_by_meaning_id(&self, meaning_id: Uuid) -> impl Iterator<Item = (&Uuid, &Cloze)> {
        self.by_meaning
            .get(&meaning_id)
            .map(|ids| ids.iter())
            .into_iter()
            .flatten()
            .filter_map(|id| self.clozes.get(id).map(|c| (id, c)))
    }

    pub fn delete(&mut self, id: Uuid) -> bool {
        if let Some(cloze) = self.clozes.remove(&id) {
            self.dirty_ids.insert(id);
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

    pub fn delete_by_meaning(&mut self, meaning_id: Uuid) {
        if let Some(cloze_ids) = self.by_meaning.remove(&meaning_id) {
            for cloze_id in cloze_ids {
                self.dirty_ids.insert(cloze_id);
                self.clozes.remove(&cloze_id);
            }
        }
    }

    pub fn count(&self) -> usize {
        self.clozes.len()
    }

    pub fn count_by_meaning(&self, meaning_id: Uuid) -> usize {
        self.by_meaning
            .get(&meaning_id)
            .map(|s| s.len())
            .unwrap_or(0)
    }

    pub fn exists(&self, id: Uuid) -> bool {
        self.clozes.contains_key(&id)
    }

    // Persistence
    /// Load all clozes from database
    pub fn load_all(&mut self, db: &crate::persistence::Db) {
        tracing::debug!("ClozeRegistry: Loading clozes from database");
        let count = self.clozes.len();
        match db.iter_clozes() {
            Ok(items) => {
                for dto in items {
                    tracing::trace!(cloze_id = %dto.id, meaning_id = %dto.meaning_id, "Converting cloze DTO to model");
                    let cloze = Cloze::from(dto);
                    tracing::trace!(cloze_id = %cloze.id, segments_count = cloze.segments.len(), "Inserting cloze into registry");
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
        tracing::debug!(count = loaded, "ClozeRegistry: Loaded clozes from database");
    }

    /// Flush all dirty entities to the database
    pub fn flush_dirty(&mut self, db: &crate::persistence::Db) -> Result<(), DbError> {
        tracing::debug!(dirty_count = self.dirty_ids.len(), "ClozeRegistry: Flushing dirty clozes");
        let mut successful = Vec::new();
        let mut errors = 0;
        for id in &self.dirty_ids {
            tracing::trace!(cloze_id = %id, "Processing dirty cloze");
            if let Some(cloze) = self.clozes.get(id) {
                tracing::trace!(cloze_id = %id, meaning_id = %cloze.meaning_id, segments = cloze.segments.len(), "Converting cloze to DTO");
                let dto = crate::persistence::ClozeDto::from(cloze);
                tracing::trace!(cloze_id = %id, "Saving cloze to database");
                match db.save_cloze(*id, &dto) {
                    Ok(_) => {
                        tracing::debug!(cloze_id = %id, "Successfully saved cloze");
                        successful.push(*id);
                    }
                    Err(e) => {
                        errors += 1;
                        tracing::error!(cloze_id = %id, error = %e, "Failed to save cloze");
                    }
                }
            } else {
                tracing::trace!(cloze_id = %id, "Cloze not in registry, deleting from database");
                match db.delete_cloze(*id) {
                    Ok(_) => successful.push(*id),
                    Err(e) => {
                        errors += 1;
                        tracing::error!(cloze_id = %id, error = %e, "Failed to delete cloze");
                    }
                }
            }
        }
        for id in &successful {
            self.dirty_ids.remove(id);
        }
        if errors > 0 {
            tracing::warn!(errors = errors, "Some clozes failed to persist");
        }
        tracing::debug!(saved = successful.len(), errors = errors, "ClozeRegistry: Flush complete");
        Ok(())
    }

    /// Check if there are any dirty entities
    pub fn has_dirty(&self) -> bool {
        !self.dirty_ids.is_empty()
    }
}
