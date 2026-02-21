use crate::config::AiConfig;
use crate::models::Model;
// use crate::persistence::DbError;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ModelRegistry {
    models: BTreeMap<Uuid, Model>,
    dirty_ids: BTreeSet<Uuid>,
    by_name: HashMap<String, Uuid>,
    by_provider: BTreeMap<Uuid, BTreeSet<Uuid>>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            models: BTreeMap::new(),
            dirty_ids: BTreeSet::new(),
            by_name: HashMap::new(),
            by_provider: BTreeMap::new(),
        }
    }

    /// Loads models from configuration.
    pub fn load_from_config(&mut self, config: &AiConfig) {
        for model_config in &config.models {
            let model = Model::from(model_config.clone());
            self.models.insert(model.id, model.clone());
            self.by_name.insert(model.name.clone(), model.id);
            self.by_provider
                .entry(model.provider_id)
                .or_default()
                .insert(model.id);
        }
    }

    pub fn add(&mut self, model: Model) {
        self.models.insert(model.id, model.clone());
        self.dirty_ids.insert(model.id);
        self.by_name.insert(model.name.clone(), model.id);
        self.by_provider
            .entry(model.provider_id)
            .or_default()
            .insert(model.id);
    }

    pub fn get(&self, id: Uuid) -> Option<&Model> {
        self.models.get(&id)
    }

    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut Model> {
        self.models.get_mut(&id)
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
            self.dirty_ids.insert(id);
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
