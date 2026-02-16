//! Words panel UI state types.

use std::collections::{BTreeMap, BTreeSet};

use crate::models::PartOfSpeech;
use uuid::Uuid;

/// Dropdown state for tag operations.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TagDropdownState {
    #[default]
    None,
    Add,
    Remove,
}

/// Input state for creating/editing meanings.
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

/// UI state for the words view.
#[derive(Debug, Default)]
pub struct WordsUiState {
    pub word_input: String,
    pub meaning_inputs: BTreeMap<Uuid, MeaningInputState>,
    pub tag_filter: String,
    pub active_tag_dropdown: Option<Uuid>,
    pub meanings_tag_dropdown_state: TagDropdownState,
    pub meanings_tag_search_input: String,
    pub meanings_tag_remove_search_input: String,
}

impl WordsUiState {
    pub fn new() -> Self {
        Self {
            word_input: String::new(),
            meaning_inputs: BTreeMap::new(),
            tag_filter: String::new(),
            active_tag_dropdown: None,
            meanings_tag_dropdown_state: TagDropdownState::None,
            meanings_tag_search_input: String::new(),
            meanings_tag_remove_search_input: String::new(),
        }
    }
}

/// UI state for the tags view.
#[derive(Debug, Default)]
pub struct TagsUiState {
    pub input: String,
    pub collapsed_ids: BTreeSet<Uuid>,
}

impl TagsUiState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            collapsed_ids: BTreeSet::new(),
        }
    }
}
