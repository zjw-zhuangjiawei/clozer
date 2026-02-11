use std::collections::HashSet;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(mutators(
    fn with_meaning(&mut self, meaning_id: Uuid) {
        self.meaning_ids.insert(meaning_id);
    }
))]
pub struct Word {
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    pub content: String,
    #[builder(default, via_mutators)]
    pub meaning_ids: HashSet<Uuid>,
}
