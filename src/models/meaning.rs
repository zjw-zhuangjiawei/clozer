use std::collections::BTreeSet;
use strum::{Display, VariantArray};
use typed_builder::TypedBuilder;

use super::{MeaningId, TagId, WordId};

/// Part of speech categories for classifying words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, VariantArray, Default)]
pub enum PartOfSpeech {
    // Major
    #[default]
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

impl PartOfSpeech {
    /// Attempt to parse a part of speech from a dictionary API string.
    /// Returns `None` if the string doesn't match any known part of speech.
    pub fn try_from_str(s: &str) -> Option<Self> {
        let lower = s.to_lowercase();
        match lower.as_str() {
            "noun" => Some(PartOfSpeech::Noun),
            "verb" => Some(PartOfSpeech::Verb),
            "adjective" => Some(PartOfSpeech::Adjective),
            "adverb" => Some(PartOfSpeech::Adverb),
            "pronoun" => Some(PartOfSpeech::Pronoun),
            "preposition" => Some(PartOfSpeech::Preposition),
            "conjunction" => Some(PartOfSpeech::Conjunction),
            "interjection" => Some(PartOfSpeech::Interjection),
            "determiner" => Some(PartOfSpeech::Determiner),
            "article" => Some(PartOfSpeech::Article),
            "modal" | "modal verb" | "modal_verb" => Some(PartOfSpeech::Modal),
            "numeral" | "number" => Some(PartOfSpeech::Numeral),
            "abbreviation" | "abbrev" => Some(PartOfSpeech::Abbreviation),
            _ => None,
        }
    }
}

/// CEFR (Common European Framework of Reference) language proficiency levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, VariantArray)]
pub enum CefrLevel {
    /// A1: Beginner
    A1,
    /// A2: Elementary
    A2,
    /// B1: Intermediate
    B1,
    /// B2: Upper Intermediate
    B2,
    /// C1: Advanced
    C1,
    /// C2: Proficient
    C2,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(mutators(
    fn with_tag(&mut self, tag_id: TagId) {
        self.tag_ids.insert(tag_id);
    }
))]
pub struct Meaning {
    #[builder(default = MeaningId::new())]
    pub id: MeaningId,
    pub word_id: WordId,
    pub definition: String,
    pub pos: PartOfSpeech,
    #[builder(default)]
    pub cefr_level: Option<CefrLevel>,
    #[builder(default, via_mutators)]
    pub tag_ids: BTreeSet<TagId>,
}
