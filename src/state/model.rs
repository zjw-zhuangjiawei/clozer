//! Core data + business logic layer.
//!
//! Contains all data registries and business logic, separated from UI state.

use crate::config::AppConfig;
use crate::persistence::Db;
use crate::registry::{ClozeRegistry, MeaningRegistry, QueueRegistry, TagRegistry, WordRegistry};
use crate::state::generator::GeneratorState;
use std::sync::Arc;

#[derive(Debug)]
pub struct Model {
    pub word_registry: WordRegistry,
    pub meaning_registry: MeaningRegistry,
    pub tag_registry: TagRegistry,
    pub cloze_registry: ClozeRegistry,
    pub queue_registry: QueueRegistry,
    pub generator: GeneratorState,
    pub db: Db,
    pub app_config: Arc<AppConfig>,
}

impl Model {
    pub fn new(db: Db, app_config: AppConfig) -> Self {
        Self {
            word_registry: WordRegistry::new(),
            meaning_registry: MeaningRegistry::new(),
            tag_registry: TagRegistry::new(),
            cloze_registry: ClozeRegistry::new(),
            queue_registry: QueueRegistry::new(),
            generator: GeneratorState::new(),
            db,
            app_config: Arc::new(app_config),
        }
    }

    /// Load all data from database
    pub fn load_all(&mut self) {
        self.word_registry.load_all(&self.db);
        self.meaning_registry.load_all(&self.db);
        self.tag_registry.load_all(&self.db);
        self.cloze_registry.load_all(&self.db);
    }

    /// Flush all dirty entities across registries to the database
    pub fn flush_all(&mut self) -> Result<(), crate::persistence::DbError> {
        self.word_registry.flush_dirty(&self.db)?;
        self.meaning_registry.flush_dirty(&self.db)?;
        self.tag_registry.flush_dirty(&self.db)?;
        self.cloze_registry.flush_dirty(&self.db)?;
        Ok(())
    }

    /// Check if any registry has dirty entities
    pub fn has_dirty(&self) -> bool {
        self.word_registry.has_dirty()
            || self.meaning_registry.has_dirty()
            || self.tag_registry.has_dirty()
            || self.cloze_registry.has_dirty()
    }
}
