//! Main window state.

use crate::ui::nav::NavItem;
use crate::ui::queue::QueueUiState;
use crate::ui::words::{TagsUiState, WordsUiState};

/// State for the main application window.
///
/// Contains all UI state: selection, expansion, inputs, and dropdowns.
#[derive(Debug, Default)]
pub struct MainWindowState {
    // Per-panel UI state
    pub words_ui: WordsUiState,
    pub tags_ui: TagsUiState,
    pub queue_ui: QueueUiState,

    // Navigation state
    pub current_view: NavItem,
}

impl MainWindowState {
    /// Creates a new MainWindowState.
    pub fn new() -> Self {
        Self {
            words_ui: WordsUiState::new(),
            tags_ui: TagsUiState::new(),
            queue_ui: QueueUiState::new(),
            current_view: NavItem::default(),
        }
    }
}
