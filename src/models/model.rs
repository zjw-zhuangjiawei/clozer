use typed_builder::TypedBuilder;

use super::{ModelId, ProviderId};

#[derive(Debug, Clone, TypedBuilder)]
pub struct Model {
    #[builder(default = ModelId::new())]
    pub id: ModelId,
    pub name: String,
    pub provider_id: ProviderId,
    pub model_id: String,
}
