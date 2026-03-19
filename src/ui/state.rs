//! Main window state.

use crate::ui::nav::NavItem;
use crate::ui::queue::state::QueueState;
use crate::ui::settings::state::SettingsState;
use crate::ui::words::state::WordsState;

/// State for the main application window.
///
/// Contains all UI state: selection, expansion, inputs, and dropdowns.
#[derive(Debug, Default)]
pub struct MainWindowState {
    /// Words panel state
    pub words: WordsState,
    /// Queue panel state
    pub queue: QueueState,
    /// Settings panel state
    pub settings: SettingsState,
    /// Current navigation view
    pub current_view: NavItem,
}

impl MainWindowState {
    /// Creates a new MainWindowState.
    pub fn new() -> Self {
        Self {
            words: WordsState::new(),
            queue: QueueState::new(),
            settings: SettingsState::new(),
            current_view: NavItem::default(),
        }
    }
}

// Backward compatibility aliases
#[allow(deprecated)]
pub type WordsUiState = WordsState;
#[allow(deprecated)]
pub type QueueUiState = QueueState;
#[allow(deprecated)]
pub type SettingsUiState = SettingsState;
