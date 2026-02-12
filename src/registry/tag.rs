use crate::models::Tag;
use crate::persistence::DbError;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct TagRegistry {
    tags: HashMap<Uuid, Tag>,
    dirty_ids: HashSet<Uuid>,
}

impl TagRegistry {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
            dirty_ids: HashSet::new(),
        }
    }

    pub fn insert(&mut self, tag: Tag) {
        self.tags.insert(tag.id, tag.clone());
        self.dirty_ids.insert(tag.id);
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<&Tag> {
        self.tags.get(&id)
    }

    pub fn delete(&mut self, id: Uuid) -> bool {
        if self.tags.remove(&id).is_some() {
            self.dirty_ids.insert(id);
            true
        } else {
            false
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &Tag)> {
        self.tags.iter()
    }

    pub fn count(&self) -> usize {
        self.tags.len()
    }

    pub fn exists(&self, id: Uuid) -> bool {
        self.tags.contains_key(&id)
    }

    // Persistence
    /// Load all tags from database
    pub fn load_all(&mut self, db: &crate::persistence::Db) {
        if let Ok(items) = db.iter_tags() {
            for dto in items {
                let tag = crate::models::Tag::from(dto);
                self.tags.insert(tag.id, tag);
            }
        }
    }

    /// Flush all dirty entities to the database
    pub fn flush_dirty(&mut self, db: &crate::persistence::Db) -> Result<(), DbError> {
        for id in &self.dirty_ids {
            if let Some(tag) = self.tags.get(id) {
                let dto = crate::persistence::TagDto::from(tag);
                db.save_tag(*id, &dto)?;
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
