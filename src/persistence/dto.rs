//! Serialization formats for persistence.
//!
//! These structs define the on-disk format for each entity.
//! Uses serde + postcard for compact binary serialization.

use crate::models::{Cloze, ClozeSegment, Meaning, Model, Provider, Tag, Word};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Cloze segment DTO for serialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClozeSegmentDto {
    #[serde(rename = "text")]
    Text(String),
    #[serde(rename = "blank")]
    Blank(String),
}

impl From<&ClozeSegment> for ClozeSegmentDto {
    fn from(segment: &ClozeSegment) -> Self {
        match segment {
            ClozeSegment::Text(s) => ClozeSegmentDto::Text(s.clone()),
            ClozeSegment::Blank(a) => ClozeSegmentDto::Blank(a.clone()),
        }
    }
}

// Word entity data (matches Word model structure).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WordDto {
    pub content: String,
    pub meaning_ids: Vec<Uuid>,
}

impl From<&Word> for WordDto {
    fn from(word: &Word) -> Self {
        WordDto {
            content: word.content.clone(),
            meaning_ids: word.meaning_ids.iter().cloned().collect(),
        }
    }
}

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

// Meaning entity data (matches Meaning model structure).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeaningDto {
    pub definition: String,
    pub pos: PartOfSpeechDto,
    pub word_id: Uuid,
    pub tag_ids: Vec<Uuid>,
    pub cloze_ids: Vec<Uuid>,
}

impl From<&Meaning> for MeaningDto {
    fn from(meaning: &Meaning) -> Self {
        MeaningDto {
            definition: meaning.definition.clone(),
            pos: PartOfSpeechDto::from(meaning.pos),
            word_id: meaning.word_id,
            tag_ids: meaning.tag_ids.iter().cloned().collect(),
            cloze_ids: Vec::new(),
        }
    }
}

// Cloze entity data (matches Cloze model structure).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClozeDto {
    pub segments: Vec<ClozeSegmentDto>,
    pub meaning_id: Uuid,
}

impl From<&Cloze> for ClozeDto {
    fn from(cloze: &Cloze) -> Self {
        ClozeDto {
            segments: cloze.segments.iter().map(ClozeSegmentDto::from).collect(),
            meaning_id: cloze.meaning_id,
        }
    }
}

// Tag entity data (matches Tag model structure).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagDto {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub children_ids: Vec<Uuid>,
}

impl From<&Tag> for TagDto {
    fn from(tag: &Tag) -> Self {
        TagDto {
            name: tag.name.clone(),
            parent_id: tag.parent_id,
            children_ids: tag.children_ids.iter().cloned().collect(),
        }
    }
}

// Provider entity data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderDto {
    pub name: String,
    #[serde(rename = "type")]
    pub provider_type: String,
    pub base_url: String,
    pub api_key: String,
}

impl From<&Provider> for ProviderDto {
    fn from(provider: &Provider) -> Self {
        ProviderDto {
            name: provider.name.clone(),
            provider_type: format!("{:?}", provider.provider_type),
            base_url: provider.base_url.clone(),
            api_key: provider.api_key.clone(),
        }
    }
}

// Model entity data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelDto {
    pub name: String,
    pub provider_id: Uuid,
    pub model_id: String,
}

impl From<&Model> for ModelDto {
    fn from(model: &Model) -> Self {
        ModelDto {
            name: model.name.clone(),
            provider_id: model.provider_id,
            model_id: model.model_id.clone(),
        }
    }
}

// Queue item status encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum QueueItemStatusDto {
    Pending = 0,
    Processing = 1,
    Completed = 2,
    Failed = 3,
}

// Queue item entity data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueItemDto {
    pub meaning_id: Uuid,
    #[serde(with = "QueueItemStatusDto")]
    pub status: QueueItemStatusDto,
}
