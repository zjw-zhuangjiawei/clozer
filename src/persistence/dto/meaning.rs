//! Meaning DTO for serialization.

use crate::models::Meaning;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Part of speech DTO for serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartOfSpeechDto {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Interjection,
    Determiner,
    Article,
    Modal,
    Numeral,
    Abbreviation,
}

impl From<crate::models::PartOfSpeech> for PartOfSpeechDto {
    fn from(pos: crate::models::PartOfSpeech) -> Self {
        match pos {
            crate::models::PartOfSpeech::Noun => PartOfSpeechDto::Noun,
            crate::models::PartOfSpeech::Verb => PartOfSpeechDto::Verb,
            crate::models::PartOfSpeech::Adjective => PartOfSpeechDto::Adjective,
            crate::models::PartOfSpeech::Adverb => PartOfSpeechDto::Adverb,
            crate::models::PartOfSpeech::Pronoun => PartOfSpeechDto::Pronoun,
            crate::models::PartOfSpeech::Preposition => PartOfSpeechDto::Preposition,
            crate::models::PartOfSpeech::Conjunction => PartOfSpeechDto::Conjunction,
            crate::models::PartOfSpeech::Interjection => PartOfSpeechDto::Interjection,
            crate::models::PartOfSpeech::Determiner => PartOfSpeechDto::Determiner,
            crate::models::PartOfSpeech::Article => PartOfSpeechDto::Article,
            crate::models::PartOfSpeech::Modal => PartOfSpeechDto::Modal,
            crate::models::PartOfSpeech::Numeral => PartOfSpeechDto::Numeral,
            crate::models::PartOfSpeech::Abbreviation => PartOfSpeechDto::Abbreviation,
        }
    }
}

impl From<PartOfSpeechDto> for crate::models::PartOfSpeech {
    fn from(pos: PartOfSpeechDto) -> Self {
        match pos {
            PartOfSpeechDto::Noun => crate::models::PartOfSpeech::Noun,
            PartOfSpeechDto::Verb => crate::models::PartOfSpeech::Verb,
            PartOfSpeechDto::Adjective => crate::models::PartOfSpeech::Adjective,
            PartOfSpeechDto::Adverb => crate::models::PartOfSpeech::Adverb,
            PartOfSpeechDto::Pronoun => crate::models::PartOfSpeech::Pronoun,
            PartOfSpeechDto::Preposition => crate::models::PartOfSpeech::Preposition,
            PartOfSpeechDto::Conjunction => crate::models::PartOfSpeech::Conjunction,
            PartOfSpeechDto::Interjection => crate::models::PartOfSpeech::Interjection,
            PartOfSpeechDto::Determiner => crate::models::PartOfSpeech::Determiner,
            PartOfSpeechDto::Article => crate::models::PartOfSpeech::Article,
            PartOfSpeechDto::Modal => crate::models::PartOfSpeech::Modal,
            PartOfSpeechDto::Numeral => crate::models::PartOfSpeech::Numeral,
            PartOfSpeechDto::Abbreviation => crate::models::PartOfSpeech::Abbreviation,
        }
    }
}

/// Meaning entity data (matches Meaning model structure).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeaningDto {
    pub id: Uuid,
    pub definition: String,
    pub pos: PartOfSpeechDto,
    pub word_id: Uuid,
    pub tag_ids: Vec<Uuid>,
    pub cloze_ids: Vec<Uuid>,
}

impl From<&Meaning> for MeaningDto {
    fn from(meaning: &Meaning) -> Self {
        MeaningDto {
            id: meaning.id,
            definition: meaning.definition.clone(),
            pos: PartOfSpeechDto::from(meaning.pos),
            word_id: meaning.word_id,
            tag_ids: meaning.tag_ids.iter().cloned().collect(),
            cloze_ids: Vec::new(),
        }
    }
}

impl From<MeaningDto> for Meaning {
    fn from(dto: MeaningDto) -> Self {
        Meaning {
            id: dto.id,
            definition: dto.definition,
            pos: dto.pos.into(),
            word_id: dto.word_id,
            tag_ids: dto.tag_ids.into_iter().collect(),
        }
    }
}
