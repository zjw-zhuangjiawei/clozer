//! Main window state.

use std::collections::BTreeSet;
use uuid::Uuid;

use super::queue::QueueUiState;
use super::words::{TagsUiState, WordsUiState};

/// State for the main application window.
///
/// Contains all UI state: selection, expansion, inputs, and dropdowns.
#[derive(Debug, Default)]
pub struct MainWindowState {
    // Selection
    pub selected_word_ids: BTreeSet<Uuid>,
    pub selected_meaning_ids: BTreeSet<Uuid>,
    pub selected_tag_ids: BTreeSet<Uuid>,
    // Expansion
    pub expanded_word_ids: BTreeSet<Uuid>,
    // Per-panel UI state
    pub words_ui: WordsUiState,
    pub tags_ui: TagsUiState,
    pub queue_ui: QueueUiState,
}

impl MainWindowState {
    /// Creates a new MainWindowState.
    pub fn new() -> Self {
        Self {
            selected_word_ids: BTreeSet::new(),
            selected_meaning_ids: BTreeSet::new(),
            selected_tag_ids: BTreeSet::new(),
            expanded_word_ids: BTreeSet::new(),
            words_ui: WordsUiState::new(),
            tags_ui: TagsUiState::new(),
            queue_ui: QueueUiState::new(),
        }
    }
}
