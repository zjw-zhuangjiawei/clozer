use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Model {
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    pub name: String,
    pub provider_id: Uuid,
    pub model_id: String,
}
