//! Cloze DTO for serialization.

use crate::models::{Cloze, ClozeSegment};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Cloze segment DTO for serialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "content")]
pub enum ClozeSegmentDto {
    Text(String),
    Blank(String),
}

impl From<&ClozeSegment> for ClozeSegmentDto {
    fn from(segment: &ClozeSegment) -> Self {
        let result = match segment {
            ClozeSegment::Text(s) => ClozeSegmentDto::Text(s.clone()),
            ClozeSegment::Blank(a) => ClozeSegmentDto::Blank(a.clone()),
        };
        tracing::trace!(?segment, "ClozeSegment -> ClozeSegmentDto");
        result
    }
}

impl From<ClozeSegmentDto> for ClozeSegment {
    fn from(dto: ClozeSegmentDto) -> Self {
        let result = match dto {
            ClozeSegmentDto::Text(s) => ClozeSegment::Text(s),
            ClozeSegmentDto::Blank(a) => ClozeSegment::Blank(a),
        };
        // Note: Can't log dto after move, so just log the conversion
        tracing::trace!("ClozeSegmentDto -> ClozeSegment");
        result
    }
}

/// Cloze entity data (matches Cloze model structure).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClozeDto {
    pub id: Uuid,
    pub segments: Vec<ClozeSegmentDto>,
    pub meaning_id: Uuid,
}

impl From<&Cloze> for ClozeDto {
    fn from(cloze: &Cloze) -> Self {
        tracing::trace!(cloze_id = %cloze.id, "Cloze -> ClozeDto");
        ClozeDto {
            id: cloze.id,
            segments: cloze.segments.iter().map(ClozeSegmentDto::from).collect(),
            meaning_id: cloze.meaning_id,
        }
    }
}

impl From<ClozeDto> for Cloze {
    fn from(dto: ClozeDto) -> Self {
        tracing::trace!(cloze_id = %dto.id, meaning_id = %dto.meaning_id, "ClozeDto -> Cloze");
        Cloze {
            id: dto.id,
            meaning_id: dto.meaning_id,
            segments: dto.segments.into_iter().map(Into::into).collect(),
        }
    }
}
