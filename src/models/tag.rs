use std::collections::BTreeSet;
use typed_builder::TypedBuilder;

use super::TagId;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(mutators(
    fn with_child(&mut self, child_id: TagId) {
        self.children_ids.insert(child_id);
    }
))]
pub struct Tag {
    #[builder(default = TagId::new())]
    pub id: TagId,
    pub name: String,
    #[builder(default)]
    pub parent_id: Option<TagId>,
    #[builder(default, via_mutators)]
    pub children_ids: BTreeSet<TagId>,
}
