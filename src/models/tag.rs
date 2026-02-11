use std::collections::HashSet;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(mutators(
    fn with_child(&mut self, child_id: Uuid) {
        self.children_ids.insert(child_id);
    }
))]
pub struct Tag {
    #[builder(default=Uuid::new_v4())]
    pub id: Uuid,
    pub name: String,
    #[builder(default)]
    pub parent_id: Option<Uuid>,
    #[builder(default, via_mutators)]
    pub children_ids: HashSet<Uuid>,
}
