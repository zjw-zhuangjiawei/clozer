use crate::{
    models::{Meaning, PartOfSpeech, Tag, Word},
    persistence::DbError,
    registry::{
        ClozeRegistry, MeaningRegistry, QueueRegistry,
        TagRegistry, WordRegistry,
    },
};

#[derive(Debug, Clone)]
pub struct DataState {
    pub word_registry: WordRegistry,
    pub meaning_registry: MeaningRegistry,
    pub tag_registry: TagRegistry,
    pub cloze_registry: ClozeRegistry,
    pub queue_registry: QueueRegistry,
}

impl Default for DataState {
    fn default() -> Self {
        Self::new()
    }
}

impl DataState {
    pub fn new() -> Self {
        Self {
            word_registry: WordRegistry::new(),
            meaning_registry: MeaningRegistry::new(),
            tag_registry: TagRegistry::new(),
            cloze_registry: ClozeRegistry::new(),
            queue_registry: QueueRegistry::new(),
        }
    }

    /// Load all data from database
    pub fn load_all(&mut self, db: &crate::persistence::Db) {
        self.word_registry.load_all(db);
        self.meaning_registry.load_all(db);
        self.tag_registry.load_all(db);
        self.cloze_registry.load_all(db);
        // model/provider/queue persistence commented out
        // self.model_registry.load_all(db);
        // self.provider_registry.load_all(db);
        // self.queue_registry.load_all(db);
    }

    /// Flush all dirty entities across registries to the database
    pub fn flush_all(&mut self, db: &crate::persistence::Db) -> Result<(), DbError> {
        self.word_registry.flush_dirty(db)?;
        self.meaning_registry.flush_dirty(db)?;
        self.tag_registry.flush_dirty(db)?;
        self.cloze_registry.flush_dirty(db)?;
        // model/provider/queue persistence commented out
        // self.model_registry.flush_dirty(db)?;
        // self.provider_registry.flush_dirty(db)?;
        // self.queue_registry.flush_dirty(db)?;
        Ok(())
    }

    /// Check if any registry has dirty entities
    pub fn has_dirty(&self) -> bool {
        self.word_registry.has_dirty()
            || self.meaning_registry.has_dirty()
            || self.tag_registry.has_dirty()
            || self.cloze_registry.has_dirty()
        // model/provider/queue persistence commented out
        // || self.model_registry.has_dirty()
        // || self.provider_registry.has_dirty()
        // || self.queue_registry.has_dirty()
    }

    pub fn with_sample_data(mut self) -> Self {
        // Create words
        let hello = Word::builder().content("Hello".to_string()).build();
        let world = Word::builder().content("World".to_string()).build();
        let rust = Word::builder().content("Rust".to_string()).build();

        self.word_registry.add(hello.clone());
        self.word_registry.add(world.clone());
        self.word_registry.add(rust.clone());

        // Create a meaning for "Hello"
        let greeting = Meaning::builder()
            .word_id(hello.id)
            .definition("a greeting".to_string())
            .pos(PartOfSpeech::Noun)
            .build();

        self.meaning_registry.add(greeting.clone());

        // Update word with meaning reference and re-insert
        let mut hello_with_meaning = hello.clone();
        hello_with_meaning.meaning_ids.insert(greeting.id);
        self.word_registry.add(hello_with_meaning);

        // Create tag
        let my_tag = Tag::builder().name("My Tag".to_string()).build();
        self.tag_registry.add(my_tag);

        self
    }
}
