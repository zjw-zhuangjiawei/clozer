use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::models::PartOfSpeech;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TagDropdownState {
    #[default]
    None,
    Add,
    Remove,
}

#[derive(Debug, Default, Clone)]
pub struct UiState {
    pub words: WordsUiState,
    pub tags: TagsUiState,
    pub queue: QueueUiState,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            words: WordsUiState::new(),
            tags: TagsUiState::new(),
            queue: QueueUiState::new(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct WordsUiState {
    pub word_input: String,
    pub expanded_word_ids: HashSet<Uuid>,
    pub meaning_inputs: HashMap<Uuid, MeaningInputState>,
    pub cloze_inputs: HashMap<Uuid, String>,
    pub tag_filter: String,
    pub tag_dropdown_state: TagDropdownState,
    pub tag_search_input: String,
    pub tag_remove_search_input: String,
    pub active_tag_dropdown: Option<Uuid>,
    pub meanings_tag_dropdown_state: TagDropdownState,
    pub meanings_tag_search_input: String,
    pub meanings_tag_remove_search_input: String,
}

#[derive(Debug, Clone)]
pub struct MeaningInputState {
    pub definition: String,
    pub pos: PartOfSpeech,
    pub visible: bool,
}

impl Default for MeaningInputState {
    fn default() -> Self {
        Self {
            definition: String::new(),
            pos: PartOfSpeech::Noun,
            visible: false,
        }
    }
}

impl WordsUiState {
    pub fn new() -> Self {
        Self {
            word_input: String::new(),
            expanded_word_ids: HashSet::new(),
            meaning_inputs: HashMap::new(),
            cloze_inputs: HashMap::new(),
            tag_filter: String::new(),
            tag_dropdown_state: TagDropdownState::None,
            tag_search_input: String::new(),
            tag_remove_search_input: String::new(),
            active_tag_dropdown: None,
            meanings_tag_dropdown_state: TagDropdownState::None,
            meanings_tag_search_input: String::new(),
            meanings_tag_remove_search_input: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TagsUiState {
    pub input: String,
    pub collapsed_ids: HashSet<Uuid>,
    pub selected_ids: HashSet<Uuid>,
}

impl Default for TagsUiState {
    fn default() -> Self {
        Self::new()
    }
}

impl TagsUiState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            collapsed_ids: HashSet::new(),
            selected_ids: HashSet::new(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct QueueUiState {}

impl QueueUiState {
    pub fn new() -> Self {
        Self {}
    }
}
