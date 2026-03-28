//! Words panel UI state types.
//!
//! State is organized with focused sub-states:
//! - FilterState: Filter configuration
//! - SelectionState: Meaning and cloze selection
//! - ExpansionState: Word expansion state
//! - DetailSelection: Current detail panel selection
//! - EditContext: Current editing context
//! - EditBuffer: Edit form data
//! - NewMeaningForm: New meaning form data
//! - TagDropdownState: Tag dropdown state

use std::collections::HashSet;

use crate::models::types::{ClozeId, MeaningId, TagId, WordId};
use crate::models::{CefrLevel, PartOfSpeech, Word};
use crate::registry::MeaningRegistry;
use strum::Display;

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
    /// Current cloze status filter
    pub cloze_status: ClozeFilter,
    /// Current tag filter (None = no tag filter)
    pub tag_id: Option<TagId>,
}

/// Selection state for meanings and clozes.
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    /// Selected meaning IDs
    pub meanings: HashSet<MeaningId>,
    /// Selected cloze IDs (independent of meaning selection)
    pub clozes: HashSet<ClozeId>,
}

impl SelectionState {
    /// Check if a word is "fully selected" (all its meanings are selected).
    pub fn is_word_selected(&self, word: &Word) -> bool {
        if word.meaning_ids.is_empty() {
            return false;
        }
        word.meaning_ids
            .iter()
            .all(|mid| self.meanings.contains(mid))
    }

    /// Check if a word is "partially selected" (some but not all meanings selected).
    pub fn is_word_partial(&self, word: &Word) -> bool {
        if word.meaning_ids.is_empty() {
            return false;
        }
        let selected_count = word
            .meaning_ids
            .iter()
            .filter(|mid| self.meanings.contains(*mid))
            .count();
        selected_count > 0 && selected_count < word.meaning_ids.len()
    }

    /// Toggle word selection (select all meanings or deselect all).
    pub fn toggle_word(&mut self, word: &Word) {
        if self.is_word_selected(word) {
            for mid in &word.meaning_ids {
                self.meanings.remove(mid);
            }
        } else {
            self.meanings.extend(word.meaning_ids.iter());
        }
    }

    /// Toggle a single meaning's selection.
    pub fn toggle_meaning(&mut self, meaning_id: MeaningId) {
        if self.meanings.contains(&meaning_id) {
            self.meanings.remove(&meaning_id);
        } else {
            self.meanings.insert(meaning_id);
        }
    }

    /// Get the count of selected meanings.
    pub fn meaning_count(&self) -> usize {
        self.meanings.len()
    }

    /// Check if there are any selected meanings.
    pub fn has_meaning_selection(&self) -> bool {
        !self.meanings.is_empty()
    }

    /// Clear all selections.
    pub fn clear(&mut self) {
        self.meanings.clear();
        self.clozes.clear();
    }

    /// Select all meanings in the registry.
    pub fn select_all(&mut self, meaning_registry: &MeaningRegistry) {
        for (id, _) in meaning_registry.iter() {
            self.meanings.insert(*id);
        }
    }

    /// Check if a cloze is selected.
    pub fn is_cloze_selected(&self, cloze_id: ClozeId) -> bool {
        self.clozes.contains(&cloze_id)
    }

    /// Toggle a cloze's selection.
    pub fn toggle_cloze(&mut self, cloze_id: ClozeId) {
        if self.clozes.contains(&cloze_id) {
            self.clozes.remove(&cloze_id);
        } else {
            self.clozes.insert(cloze_id);
        }
    }

    /// Get the count of selected clozes.
    pub fn cloze_count(&self) -> usize {
        self.clozes.len()
    }

    /// Check if there are any selected clozes.
    pub fn has_cloze_selection(&self) -> bool {
        !self.clozes.is_empty()
    }

    /// Clear cloze selections.
    pub fn clear_clozes(&mut self) {
        self.clozes.clear();
    }

    /// Get total selection count (meanings + clozes).
    pub fn total_count(&self) -> usize {
        self.meanings.len() + self.clozes.len()
    }
}

/// Expansion state for words tree.
#[derive(Debug, Clone, Default)]
pub struct ExpansionState {
    /// Expanded word IDs (words whose meanings are visible)
    pub words: HashSet<WordId>,
}

impl ExpansionState {
    /// Toggle word expansion.
    pub fn toggle(&mut self, word_id: WordId) {
        if self.words.contains(&word_id) {
            self.words.remove(&word_id);
        } else {
            self.words.insert(word_id);
        }
    }

    /// Check if a word is expanded.
    pub fn is_expanded(&self, word_id: WordId) -> bool {
        self.words.contains(&word_id)
    }

    /// Expand all words.
    pub fn expand_all(&mut self, word_ids: impl IntoIterator<Item = WordId>) {
        self.words.extend(word_ids);
    }

    /// Collapse all words.
    pub fn collapse_all(&mut self) {
        self.words.clear();
    }
}

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

/// Detail panel editing context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditContext {
    /// Not editing anything
    #[default]
    None,
    /// Creating a new word
    NewWord,
    /// Creating a new meaning for a word
    NewMeaning(WordId),
    /// Editing a word
    Word(WordId),
    /// Editing a meaning
    Meaning(MeaningId),
}

impl EditContext {
    /// Clear editing context.
    pub fn clear(&mut self) {
        *self = EditContext::None;
    }
}

/// Buffer for storing edits in progress.
#[derive(Debug, Clone, Default)]
pub struct EditBuffer {
    /// Word content being edited
    pub word_content: String,
    /// Word language being edited
    pub word_language: Option<langtag::LangTagBuf>,
    /// Meaning definition being edited
    pub meaning_definition: String,
    /// Meaning part of speech being edited
    pub meaning_pos: PartOfSpeech,
    /// Meaning CEFR level being edited
    pub meaning_cefr: Option<CefrLevel>,
}

impl EditBuffer {
    /// Clear all fields for new word editing
    pub fn clear_new_word(&mut self) {
        self.word_content.clear();
        self.word_language = None;
        self.meaning_definition.clear();
        self.meaning_pos = PartOfSpeech::Noun;
        self.meaning_cefr = None;
    }

    /// Clear all fields for new meaning editing
    pub fn clear_new_meaning(&mut self) {
        self.meaning_definition.clear();
        self.meaning_pos = PartOfSpeech::Noun;
        self.meaning_cefr = None;
    }
}

/// Form for adding new meaning.
#[derive(Debug, Clone, Default)]
pub struct NewMeaningForm {
    /// Word ID to add meaning to (None if not adding)
    pub word_id: Option<WordId>,
    /// Meaning definition input
    pub definition: String,
    /// Meaning part of speech
    pub pos: PartOfSpeech,
    /// Meaning CEFR level
    pub cefr_level: Option<CefrLevel>,
}

impl NewMeaningForm {
    /// Check if currently adding a meaning.
    pub fn is_active(&self) -> bool {
        self.word_id.is_some()
    }

    /// Start adding meaning to a word.
    pub fn start(&mut self, word_id: WordId) {
        self.word_id = Some(word_id);
        self.definition.clear();
        self.pos = PartOfSpeech::default();
        self.cefr_level = None;
    }

    /// Cancel adding meaning.
    pub fn cancel(&mut self) {
        self.word_id = None;
        self.definition.clear();
        self.pos = PartOfSpeech::default();
        self.cefr_level = None;
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

/// Complete state for Words panel.
#[derive(Debug, Default)]
pub struct WordsState {
    /// Search query
    pub query: String,
    /// Filter configuration
    pub filter: FilterState,
    /// Selection state
    pub selection: SelectionState,
    /// Expansion state
    pub expansion: ExpansionState,
    /// Detail panel selection
    pub detail_selection: DetailSelection,
    /// Current editing context
    pub edit_context: EditContext,
    /// Edit buffer
    pub edit_buffer: EditBuffer,
    /// New meaning form
    pub new_meaning: NewMeaningForm,
    /// Tag dropdown state (None = dropdown closed)
    pub tag_dropdown: Option<TagDropdownState>,
}

impl WordsState {
    /// Creates a new WordsState.
    pub fn new() -> Self {
        Self::default()
    }
}

// Aliases for backward compatibility
pub type WordsUiState = WordsState;
pub type DetailEditMode = EditContext;
pub type MeaningInputState = NewMeaningForm;
