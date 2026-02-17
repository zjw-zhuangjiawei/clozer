//! Words panel UI state types.

use std::collections::HashSet;

use crate::models::{PartOfSpeech, Word};
use crate::registry::MeaningRegistry;
use strum::Display;
use uuid::Uuid;

/// Filter state for cloze generation status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display)]
pub enum ClozeFilter {
    #[default]
    All,
    HasClozes,
    Pending,
    Failed,
}

/// Filter state for the words tree.
#[derive(Debug, Clone, Default)]
pub struct FilterState {
    pub cloze_status: ClozeFilter,
    pub tag_id: Option<Uuid>,
}

/// Target for tag dropdown operations.
#[derive(Debug, Clone)]
pub enum TagDropdownTarget {
    /// Batch operation on selected meanings
    SelectedMeanings,
    /// Single meaning operation
    SingleMeaning(Uuid),
}

/// State for the tag dropdown.
#[derive(Debug, Clone)]
pub struct TagDropdownState {
    pub target: TagDropdownTarget,
    pub search: String,
}

impl TagDropdownState {
    pub fn new(target: TagDropdownTarget) -> Self {
        Self {
            target,
            search: String::new(),
        }
    }
}

/// Input state for creating/editing meanings.
#[derive(Debug, Clone)]
pub struct MeaningInputState {
    pub definition: String,
    pub pos: PartOfSpeech,
}

impl Default for MeaningInputState {
    fn default() -> Self {
        Self {
            definition: String::new(),
            pos: PartOfSpeech::Noun,
        }
    }
}

/// UI state for the words view.
#[derive(Debug, Default)]
pub struct WordsUiState {
    // Search & Filter
    pub search_query: String,
    pub filter: FilterState,

    // Expansion
    pub expanded_word_ids: HashSet<Uuid>,
    pub expanded_cloze_ids: HashSet<Uuid>,

    // Editing
    pub editing_word_id: Option<Uuid>,
    pub editing_word_text: String,
    pub editing_meaning_id: Option<Uuid>,
    pub editing_meaning_text: String,
    pub adding_meaning_to_word: Option<Uuid>,
    pub meaning_input: MeaningInputState,

    // Selection
    // Meanings (words are derived from meanings)
    pub selected_meaning_ids: HashSet<Uuid>,
    // Clozes (independent selection)
    pub selected_cloze_ids: HashSet<Uuid>,

    // Tag dropdown
    pub tag_dropdown: Option<TagDropdownState>,
}

impl WordsUiState {
    /// Creates a new WordsUiState.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a word is "fully selected" (all its meanings are selected).
    pub fn is_word_selected(&self, word: &Word) -> bool {
        if word.meaning_ids.is_empty() {
            return false;
        }
        word.meaning_ids
            .iter()
            .all(|mid| self.selected_meaning_ids.contains(mid))
    }

    /// Check if a word is "partially selected" (some but not all meanings selected).
    pub fn is_word_partial(&self, word: &Word) -> bool {
        if word.meaning_ids.is_empty() {
            return false;
        }
        let selected_count = word
            .meaning_ids
            .iter()
            .filter(|mid| self.selected_meaning_ids.contains(*mid))
            .count();
        selected_count > 0 && selected_count < word.meaning_ids.len()
    }

    /// Toggle word selection (select all meanings or deselect all).
    pub fn toggle_word_selection(&mut self, word: &Word) {
        if self.is_word_selected(word) {
            // Deselect all meanings of this word
            for mid in &word.meaning_ids {
                self.selected_meaning_ids.remove(mid);
            }
        } else {
            // Select all meanings of this word
            self.selected_meaning_ids.extend(word.meaning_ids.iter());
        }
    }

    /// Toggle a single meaning's selection.
    pub fn toggle_meaning_selection(&mut self, meaning_id: Uuid) {
        if self.selected_meaning_ids.contains(&meaning_id) {
            self.selected_meaning_ids.remove(&meaning_id);
        } else {
            self.selected_meaning_ids.insert(meaning_id);
        }
    }

    /// Get the count of selected meanings.
    pub fn selected_count(&self) -> usize {
        self.selected_meaning_ids.len()
    }

    /// Check if there are any selected meanings.
    pub fn has_selection(&self) -> bool {
        !self.selected_meaning_ids.is_empty()
    }

    /// Clear all selections.
    pub fn clear_selection(&mut self) {
        self.selected_meaning_ids.clear();
        self.selected_cloze_ids.clear();
    }

    /// Select all meanings in the registry.
    pub fn select_all(&mut self, meaning_registry: &MeaningRegistry) {
        for (id, _) in meaning_registry.iter() {
            self.selected_meaning_ids.insert(*id);
        }
    }

    /// Check if a cloze is selected.
    pub fn is_cloze_selected(&self, cloze_id: Uuid) -> bool {
        self.selected_cloze_ids.contains(&cloze_id)
    }

    /// Toggle a cloze's selection.
    pub fn toggle_cloze_selection(&mut self, cloze_id: Uuid) {
        if self.selected_cloze_ids.contains(&cloze_id) {
            self.selected_cloze_ids.remove(&cloze_id);
        } else {
            self.selected_cloze_ids.insert(cloze_id);
        }
    }

    /// Get the count of selected clozes.
    pub fn selected_cloze_count(&self) -> usize {
        self.selected_cloze_ids.len()
    }

    /// Check if there are any selected clozes.
    pub fn has_cloze_selection(&self) -> bool {
        !self.selected_cloze_ids.is_empty()
    }

    /// Clear cloze selections.
    pub fn clear_cloze_selection(&mut self) {
        self.selected_cloze_ids.clear();
    }

    /// Get total selection count (meanings + clozes).
    pub fn total_selection_count(&self) -> usize {
        self.selected_meaning_ids.len() + self.selected_cloze_ids.len()
    }
}

/// UI state for the tags view.
#[derive(Debug, Default)]
pub struct TagsUiState {
    pub input: String,
    pub collapsed_ids: HashSet<Uuid>,
}

impl TagsUiState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            collapsed_ids: HashSet::new(),
        }
    }
}
