use crate::models::Cloze;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct ClozeRegistry {
    clozes: HashMap<Uuid, Cloze>,
    by_meaning: HashMap<Uuid, HashSet<Uuid>>,
}

impl ClozeRegistry {
    pub fn new() -> Self {
        Self {
            clozes: HashMap::new(),
            by_meaning: HashMap::new(),
        }
    }

    pub fn insert(&mut self, cloze: Cloze) {
        let Cloze { id, meaning_id, .. } = cloze;
        self.clozes.insert(id, cloze);
        self.by_meaning.entry(meaning_id).or_default().insert(id);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &Cloze)> {
        self.clozes.iter()
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<&Cloze> {
        self.clozes.get(&id)
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
}
