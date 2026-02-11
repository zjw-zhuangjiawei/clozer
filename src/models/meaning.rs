use std::collections::HashSet;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(mutators(
    fn with_tag(&mut self, tag_id: Uuid) {
        self.tag_ids.insert(tag_id);
    }
))]
pub struct Meaning {
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    pub word_id: Uuid,
    pub definition: String,
    pub pos: String,
    #[builder(default, via_mutators)]
    pub tag_ids: HashSet<Uuid>,
}
