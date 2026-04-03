//! Detail panel state management.

use crate::models::types::{ClozeId, MeaningId, WordId};

/// Detail panel selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DetailSelection {
    /// Nothing selected
    #[default]
    None,
    /// Word detail selected
    Word(WordId),
    /// Meaning detail selected
    Meaning(MeaningId),
    /// Cloze detail selected
    Cloze(ClozeId),
}

impl DetailSelection {
    /// Toggle selection for a word.
    pub fn toggle_word(&mut self, word_id: WordId) {
        if *self == DetailSelection::Word(word_id) {
            *self = DetailSelection::None;
        } else {
            *self = DetailSelection::Word(word_id);
        }
    }

    /// Toggle selection for a meaning.
    pub fn toggle_meaning(&mut self, meaning_id: MeaningId) {
        if *self == DetailSelection::Meaning(meaning_id) {
            *self = DetailSelection::None;
        } else {
            *self = DetailSelection::Meaning(meaning_id);
        }
    }

    /// Toggle selection for a cloze.
    pub fn toggle_cloze(&mut self, cloze_id: ClozeId) {
        if *self == DetailSelection::Cloze(cloze_id) {
            *self = DetailSelection::None;
        } else {
            *self = DetailSelection::Cloze(cloze_id);
        }
    }

    /// Clear selection.
    pub fn clear(&mut self) {
        *self = DetailSelection::None;
    }
}

/// Target for tag dropdown operations.
#[derive(Debug, Clone)]
pub enum TagDropdownTarget {
    /// Batch operation on selected meanings
    SelectedMeanings,
    /// Single meaning operation
    SingleMeaning(MeaningId),
}

/// State for the tag dropdown.
#[derive(Debug, Clone)]
pub struct TagDropdownState {
    /// Target for the dropdown operation
    pub target: TagDropdownTarget,
    /// Search query for filtering tags
    pub search: String,
}

impl TagDropdownState {
    /// Create a new dropdown for single meaning.
    pub fn for_meaning(meaning_id: MeaningId) -> Self {
        Self {
            target: TagDropdownTarget::SingleMeaning(meaning_id),
            search: String::new(),
        }
    }

    /// Create a new dropdown for batch operation.
    pub fn for_batch() -> Self {
        Self {
            target: TagDropdownTarget::SelectedMeanings,
            search: String::new(),
        }
    }
}

/// Detail panel state manager.
#[derive(Debug)]
pub struct DetailManager {
    /// Current selection in detail panel
    selection: DetailSelection,
    /// Tag dropdown state (None = dropdown closed)
    tag_dropdown: Option<TagDropdownState>,
}

impl DetailManager {
    /// Creates a new DetailManager.
    pub fn new() -> Self {
        Self {
            selection: DetailSelection::None,
            tag_dropdown: None,
        }
    }

    /// Select a word.
    pub fn select_word(&mut self, word_id: WordId) {
        self.selection = DetailSelection::Word(word_id);
    }

    /// Select a meaning.
    pub fn select_meaning(&mut self, meaning_id: MeaningId) {
        self.selection = DetailSelection::Meaning(meaning_id);
    }

    /// Select a cloze.
    pub fn select_cloze(&mut self, cloze_id: ClozeId) {
        self.selection = DetailSelection::Cloze(cloze_id);
    }

    /// Clear selection.
    pub fn clear_selection(&mut self) {
        self.selection = DetailSelection::None;
    }

    /// Get current selection.
    pub fn get_selection(&self) -> DetailSelection {
        self.selection
    }

    /// Open tag dropdown.
    pub fn open_tag_dropdown(&mut self, target: TagDropdownTarget) {
        self.tag_dropdown = Some(match target {
            TagDropdownTarget::SingleMeaning(id) => TagDropdownState::for_meaning(id),
            TagDropdownTarget::SelectedMeanings => TagDropdownState::for_batch(),
        });
    }

    /// Close tag dropdown.
    pub fn close_tag_dropdown(&mut self) {
        self.tag_dropdown = None;
    }

    /// Check if tag dropdown is open.
    pub fn is_tag_dropdown_open(&self) -> bool {
        self.tag_dropdown.is_some()
    }

    /// Get mutable reference to tag dropdown state.
    pub fn tag_dropdown_mut(&mut self) -> Option<&mut TagDropdownState> {
        self.tag_dropdown.as_mut()
    }

    /// Get reference to tag dropdown state.
    pub fn tag_dropdown(&self) -> Option<&TagDropdownState> {
        self.tag_dropdown.as_ref()
    }
}

impl Default for DetailManager {
    fn default() -> Self {
        Self::new()
    }
}
