use crate::models::Provider;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProviderRegistry {
    providers: HashMap<Uuid, Provider>,
    by_name: HashMap<String, Uuid>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            by_name: HashMap::new(),
        }
    }

    pub fn add(&mut self, provider: Provider) {
        self.providers.insert(provider.id, provider.clone());
        self.by_name.insert(provider.name.clone(), provider.id);
    }

    pub fn get(&self, id: Uuid) -> Option<&Provider> {
        self.providers.get(&id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Provider> {
        self.by_name.get(name).and_then(|id| self.providers.get(id))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &Provider)> {
        self.providers.iter()
    }

    pub fn delete(&mut self, id: Uuid) {
        if let Some(provider) = self.providers.remove(&id) {
            self.by_name.remove(&provider.name);
        }
    }

    pub fn len(&self) -> usize {
        self.providers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}
