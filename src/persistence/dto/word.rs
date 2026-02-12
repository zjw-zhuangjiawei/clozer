//! Word DTO for serialization.

use crate::models::Word;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Word entity data (matches Word model structure).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WordDto {
    pub id: Uuid,
    pub content: String,
    pub meaning_ids: Vec<Uuid>,
}

impl From<&Word> for WordDto {
    fn from(word: &Word) -> Self {
        WordDto {
            id: word.id,
            content: word.content.clone(),
            meaning_ids: word.meaning_ids.iter().cloned().collect(),
        }
    }
}

impl From<WordDto> for Word {
    fn from(dto: WordDto) -> Self {
        Word {
            id: dto.id,
            content: dto.content,
            meaning_ids: dto.meaning_ids.into_iter().collect(),
        }
    }
}
