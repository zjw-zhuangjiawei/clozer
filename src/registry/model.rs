use crate::models::Model;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ModelRegistry {
    models: HashMap<Uuid, Model>,
    by_name: HashMap<String, Uuid>,
    by_provider: HashMap<Uuid, HashSet<Uuid>>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            by_name: HashMap::new(),
            by_provider: HashMap::new(),
        }
    }

    pub fn add(&mut self, model: Model) {
        self.models.insert(model.id, model.clone());
        self.by_name.insert(model.name.clone(), model.id);
        self.by_provider
            .entry(model.provider_id)
            .or_default()
            .insert(model.id);
    }

    pub fn get(&self, id: Uuid) -> Option<&Model> {
        self.models.get(&id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Model> {
        self.by_name.get(name).and_then(|id| self.models.get(id))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &Model)> {
        self.models.iter()
    }

    pub fn iter_by_provider(&self, provider_id: Uuid) -> impl Iterator<Item = (&Uuid, &Model)> {
        self.by_provider
            .get(&provider_id)
            .map(|ids| {
                Box::new(
                    ids.iter()
                        .filter_map(|id| self.models.get(id).map(|model| (id, model))),
                ) as Box<dyn Iterator<Item = (&Uuid, &Model)>>
            })
            .unwrap_or_else(|| Box::new(std::iter::empty()))
    }

    pub fn delete(&mut self, id: Uuid) {
        if let Some(model) = self.models.remove(&id) {
            self.by_name.remove(&model.name);
            if let Some(provider_models) = self.by_provider.get_mut(&model.provider_id) {
                provider_models.remove(&id);
                if provider_models.is_empty() {
                    self.by_provider.remove(&model.provider_id);
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.models.len()
    }

    pub fn is_empty(&self) -> bool {
        self.models.is_empty()
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}
