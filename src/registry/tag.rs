use crate::models::Tag;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct TagRegistry {
    tags: HashMap<Uuid, Tag>,
}

impl TagRegistry {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
        }
    }

    pub fn insert(&mut self, tag: Tag) {
        self.tags.insert(tag.id, tag);
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<&Tag> {
        self.tags.get(&id)
    }

    pub fn delete(&mut self, id: Uuid) -> bool {
        self.tags.remove(&id).is_some()
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
}
