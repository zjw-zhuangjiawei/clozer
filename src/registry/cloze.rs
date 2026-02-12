use crate::models::Cloze;
use crate::persistence::DbError;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct ClozeRegistry {
    clozes: HashMap<Uuid, Cloze>,
    dirty_ids: HashSet<Uuid>,
    by_meaning: HashMap<Uuid, HashSet<Uuid>>,
}

impl ClozeRegistry {
    pub fn new() -> Self {
        Self {
            clozes: HashMap::new(),
            dirty_ids: HashSet::new(),
            by_meaning: HashMap::new(),
        }
    }

    pub fn insert(&mut self, cloze: Cloze) {
        let Cloze { id, meaning_id, .. } = cloze.clone();
        self.clozes.insert(id, cloze.clone());
        self.dirty_ids.insert(id);
        self.by_meaning.entry(meaning_id).or_default().insert(id);
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<&Cloze> {
        self.clozes.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &Cloze)> {
        self.clozes.iter()
    }

    pub fn get_by_id_mut(&mut self, id: Uuid) -> Option<&mut Cloze> {
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
        if let Ok(items) = db.iter_clozes() {
            for dto in items {
                let cloze = Cloze::from(dto);
                self.clozes.insert(cloze.id, cloze.clone());
                self.by_meaning
                    .entry(cloze.meaning_id)
                    .or_default()
                    .insert(cloze.id);
            }
        }
    }

    /// Flush all dirty entities to the database
    pub fn flush_dirty(&mut self, db: &crate::persistence::Db) -> Result<(), DbError> {
        for id in &self.dirty_ids {
            if let Some(cloze) = self.clozes.get(id) {
                let dto = crate::persistence::ClozeDto::from(cloze);
                db.save_cloze(*id, &dto)?;
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
