//! Tag DTO for serialization.

use crate::models::{Tag, TagId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tag entity data (matches Tag model structure).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagDto {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub children_ids: Vec<Uuid>,
}

impl From<&Tag> for TagDto {
    fn from(tag: &Tag) -> Self {
        TagDto {
            id: tag.id.into(),
            name: tag.name.clone(),
            parent_id: tag.parent_id.map(|id| id.into()),
            children_ids: tag.children_ids.iter().map(|id| (*id).into()).collect(),
        }
    }
}

impl From<TagDto> for Tag {
    fn from(dto: TagDto) -> Self {
        Tag {
            id: TagId(dto.id),
            name: dto.name,
            parent_id: dto.parent_id.map(TagId),
            children_ids: dto.children_ids.into_iter().map(TagId).collect(),
        }
    }
}
