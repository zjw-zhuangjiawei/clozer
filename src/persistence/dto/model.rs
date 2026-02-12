//! Model DTO for serialization.

use crate::models::Model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Model entity data.
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

impl From<ModelDto> for Model {
    fn from(dto: ModelDto) -> Self {
        Model::builder()
            .name(dto.name)
            .provider_id(dto.provider_id)
            .model_id(dto.model_id)
            .build()
    }
}
