use crate::models::{Tag, TagId};
use crate::persistence::db::TAGS_TABLE;
use crate::persistence::{DbError, TagDto};
use crate::registry::dirty::{DirtyTracker, flush_registry};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct TagRegistry {
    pub(crate) tags: BTreeMap<TagId, Tag>,
    pub(crate) dirty: DirtyTracker<TagId>,
}

impl TagRegistry {
    pub fn new() -> Self {
        Self {
            tags: BTreeMap::new(),
            dirty: DirtyTracker::new(),
        }
    }

    pub fn add(&mut self, tag: Tag) {
        self.tags.insert(tag.id, tag.clone());
        self.dirty.mark(tag.id);
    }

    pub fn get(&self, id: TagId) -> Option<&Tag> {
        self.tags.get(&id)
    }

    pub fn get_mut(&mut self, id: TagId) -> Option<&mut Tag> {
        self.tags.get_mut(&id)
    }

    pub fn delete(&mut self, id: TagId) -> bool {
        if self.tags.remove(&id).is_some() {
            self.dirty.mark(id);
            true
        } else {
            false
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TagId, &Tag)> {
        self.tags.iter()
    }

    pub fn count(&self) -> usize {
        self.tags.len()
    }

    pub fn exists(&self, id: TagId) -> bool {
        self.tags.contains_key(&id)
    }

    // Persistence
    /// Load all tags from database
    pub fn load_all(&mut self, db: &crate::persistence::Db) {
        let count = self.tags.len();
        match db.iter_entities::<TagDto>(TAGS_TABLE) {
            Ok(items) => {
                for (id, mut dto) in items {
                    dto.id = id;
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
        flush_registry(
            &self.tags,
            &mut self.dirty,
            db,
            TAGS_TABLE,
            |t| TagDto::from(t),
            "tag",
        )
    }

    /// Check if there are any dirty entities
    pub fn has_dirty(&self) -> bool {
        self.dirty.has_dirty()
    }
}
