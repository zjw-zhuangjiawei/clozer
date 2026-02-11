use std::collections::HashSet;
use strum::{Display, VariantArray};
use typed_builder::TypedBuilder;
use uuid::Uuid;

/// Part of speech categories for classifying words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, VariantArray)]
pub enum PartOfSpeech {
    // Major
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Interjection,
    Determiner,
    // Articles & Modals
    Article,
    Modal,
    // Other
    Numeral,
    Abbreviation,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(mutators(
    fn with_tag(&mut self, tag_id: Uuid) {
        self.tag_ids.insert(tag_id);
    }
))]
pub struct Meaning {
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    pub word_id: Uuid,
    pub definition: String,
    pub pos: PartOfSpeech,
    #[builder(default, via_mutators)]
    pub tag_ids: HashSet<Uuid>,
}
