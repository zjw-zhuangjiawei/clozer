//! Cloze DTO for serialization.

use crate::models::{Cloze, ClozeSegment};
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

impl From<ClozeSegmentDto> for ClozeSegment {
    fn from(dto: ClozeSegmentDto) -> Self {
        match dto {
            ClozeSegmentDto::Text(s) => ClozeSegment::Text(s),
            ClozeSegmentDto::Blank(a) => ClozeSegment::Blank(a),
        }
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
        ClozeDto {
            id: cloze.id,
            segments: cloze.segments.iter().map(ClozeSegmentDto::from).collect(),
            meaning_id: cloze.meaning_id,
        }
    }
}

impl From<ClozeDto> for Cloze {
    fn from(dto: ClozeDto) -> Self {
        Cloze {
            id: dto.id,
            meaning_id: dto.meaning_id,
            segments: dto.segments.into_iter().map(Into::into).collect(),
        }
    }
}
