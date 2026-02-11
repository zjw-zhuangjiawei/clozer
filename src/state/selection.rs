use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct SelectionState {
    pub selected_word_ids: HashSet<Uuid>,
    pub selected_meaning_ids: HashSet<Uuid>,
    pub selected_tag_ids: HashSet<Uuid>,
}

impl SelectionState {
    pub fn new() -> Self {
        Self {
            selected_word_ids: HashSet::new(),
            selected_meaning_ids: HashSet::new(),
            selected_tag_ids: HashSet::new(),
        }
    }
}
