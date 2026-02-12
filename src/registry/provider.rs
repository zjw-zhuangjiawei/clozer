use crate::models::Provider;
use crate::persistence::DbError;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProviderRegistry {
    providers: HashMap<Uuid, Provider>,
    dirty_ids: HashSet<Uuid>,
    by_name: HashMap<String, Uuid>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            dirty_ids: HashSet::new(),
            by_name: HashMap::new(),
        }
    }

    pub fn add(&mut self, provider: Provider) {
        self.providers.insert(provider.id, provider.clone());
        self.dirty_ids.insert(provider.id);
        self.by_name.insert(provider.name.clone(), provider.id);
    }

    pub fn get(&self, id: Uuid) -> Option<&Provider> {
        self.providers.get(&id)
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Provider> {
        self.providers.get_mut(&id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Provider> {
        self.by_name.get(name).and_then(|id| self.providers.get(id))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &Provider)> {
        self.providers.iter()
    }

    pub fn delete(&mut self, id: Uuid) {
        if let Some(provider) = self.providers.remove(&id) {
            self.dirty_ids.insert(id);
            self.by_name.remove(&provider.name);
        }
    }

    pub fn len(&self) -> usize {
        self.providers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    // Persistence
    /// Load all providers from database
    pub fn load_all(&mut self, db: &crate::persistence::Db) {
        if let Ok(items) = db.iter_providers() {
            for (id, dto) in items {
                let provider = crate::models::Provider::from(dto);
                self.providers.insert(id, provider.clone());
                self.by_name.insert(provider.name.clone(), id);
            }
        }
    }

    /// Flush all dirty entities to the database
    pub fn flush_dirty(&mut self, db: &crate::persistence::Db) -> Result<(), DbError> {
        for id in &self.dirty_ids {
            if let Some(provider) = self.providers.get(id) {
                let dto = crate::persistence::ProviderDto::from(provider);
                db.save_provider(*id, &dto)?;
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

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
