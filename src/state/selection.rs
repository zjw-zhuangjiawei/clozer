use std::collections::BTreeSet;
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct SelectionState {
    pub selected_word_ids: BTreeSet<Uuid>,
    pub selected_meaning_ids: BTreeSet<Uuid>,
    pub selected_tag_ids: BTreeSet<Uuid>,
}

impl SelectionState {
    pub fn new() -> Self {
        Self {
            selected_word_ids: BTreeSet::new(),
            selected_meaning_ids: BTreeSet::new(),
            selected_tag_ids: BTreeSet::new(),
        }
    }
}
