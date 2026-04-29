//! Application UI state.

use crate::ui::AppTheme;
use crate::ui::nav::NavItem;
use crate::ui::settings::state::SettingsState;
use crate::ui::words::state::WordsState;

/// UI presentation state for the single-window application.
///
/// Contains sub-panel states, navigation, window dimensions, and theme.
/// Queue panel has no local state — selection is managed in QueueRegistry.
#[derive(Debug)]
pub struct UiState {
    /// Words panel state
    pub words: WordsState,
    /// Settings panel state
    pub settings: SettingsState,
    /// Current navigation view
    pub current_view: NavItem,
    /// Current window width for responsive layout
    pub window_width: u16,
    /// Current UI theme
    pub theme: AppTheme,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            words: WordsState::new(),
            settings: SettingsState::new(),
            current_view: NavItem::default(),
            window_width: 1024,
            theme: AppTheme::Light,
        }
    }
}

impl UiState {
    /// Creates a new UiState.
    pub fn new() -> Self {
        Self::default()
    }
}
