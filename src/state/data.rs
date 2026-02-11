use crate::{
    models::{Meaning, PartOfSpeech, Tag, Word},
    registry::{ClozeRegistry, MeaningRegistry, TagRegistry, WordRegistry},
};

#[derive(Debug, Clone)]
pub struct DataState {
    pub word_registry: WordRegistry,
    pub meaning_registry: MeaningRegistry,
    pub tag_registry: TagRegistry,
    pub cloze_registry: ClozeRegistry,
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
        }
    }

    pub fn with_sample_data(mut self) -> Self {
        // Create words
        let hello = Word::builder().content("Hello".to_string()).build();
        let world = Word::builder().content("World".to_string()).build();
        let rust = Word::builder().content("Rust".to_string()).build();

        self.word_registry.insert(hello.clone());
        self.word_registry.insert(world.clone());
        self.word_registry.insert(rust.clone());

        // Create a meaning for "Hello"
        let greeting = Meaning::builder()
            .word_id(hello.id)
            .definition("a greeting".to_string())
            .pos(PartOfSpeech::Noun)
            .build();

        self.meaning_registry.insert(greeting.clone());

        // Update word with meaning reference and re-insert
        let mut hello_with_meaning = hello.clone();
        hello_with_meaning.meaning_ids.insert(greeting.id);
        self.word_registry.insert(hello_with_meaning);

        // Create tag
        let my_tag = Tag::builder().name("My Tag".to_string()).build();
        self.tag_registry.insert(my_tag);

        self
    }
}
