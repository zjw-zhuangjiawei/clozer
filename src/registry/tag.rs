use crate::models::Tag;
use crate::persistence::DbError;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct TagRegistry {
    tags: BTreeMap<Uuid, Tag>,
    dirty_ids: BTreeSet<Uuid>,
}

impl TagRegistry {
    pub fn new() -> Self {
        Self {
            tags: BTreeMap::new(),
            dirty_ids: BTreeSet::new(),
        }
    }

    pub fn add(&mut self, tag: Tag) {
        self.tags.insert(tag.id, tag.clone());
        self.dirty_ids.insert(tag.id);
    }

    pub fn get(&self, id: Uuid) -> Option<&Tag> {
        self.tags.get(&id)
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Tag> {
        self.tags.get_mut(&id)
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
        let count = self.tags.len();
        match db.iter_tags() {
            Ok(items) => {
                for dto in items {
                    let tag = crate::models::Tag::from(dto);
                    self.tags.insert(tag.id, tag);
                }
            }
            Err(e) => {
                tracing::error!(error = %e, source = "tag_registry", "Failed to load tags from database");
            }
        }
        let loaded = self.tags.len() - count;
        tracing::debug!(count = loaded, "Loaded tags from database");
    }

    /// Flush all dirty entities to the database
    pub fn flush_dirty(&mut self, db: &crate::persistence::Db) -> Result<(), DbError> {
        let mut errors = 0;
        let dirty_ids: Vec<_> = self.dirty_ids.iter().copied().collect();
        for id in dirty_ids {
            if let Some(tag) = self.tags.get(&id) {
                let dto = crate::persistence::TagDto::from(tag);
                match db.save_tag(id, &dto) {
                    Ok(_) => {
                        self.dirty_ids.remove(&id);
                    }
                    Err(e) => {
                        errors += 1;
                        tracing::error!(tag_id = %id, error = %e, "Failed to save tag");
                    }
                }
            } else {
                match db.delete_tag(id) {
                    Ok(_) => {
                        self.dirty_ids.remove(&id);
                    }
                    Err(e) => {
                        errors += 1;
                        tracing::error!(tag_id = %id, error = %e, "Failed to delete tag");
                    }
                }
            }
        }
        if errors > 0 {
            tracing::warn!(errors = errors, "Some tags failed to persist");
        }
        Ok(())
    }

    /// Check if there are any dirty entities
    pub fn has_dirty(&self) -> bool {
        !self.dirty_ids.is_empty()
    }
}
