//! Main window state.

use crate::ui::AppTheme;
use crate::ui::nav::NavItem;
use crate::ui::queue::state::QueueState;
use crate::ui::settings::state::SettingsState;
use crate::ui::words::state::WordsState;

/// State for the main application window.
///
/// Contains all UI state: selection, expansion, inputs, and dropdowns.
#[derive(Debug)]
pub struct MainWindowState {
    /// Words panel state
    pub words: WordsState,
    /// Queue panel state
    pub queue: QueueState,
    /// Settings panel state
    pub settings: SettingsState,
    /// Current navigation view
    pub current_view: NavItem,
    /// Current window width for responsive layout
    pub window_width: u16,
    /// Current UI theme
    pub theme: AppTheme,
}

impl Default for MainWindowState {
    fn default() -> Self {
        Self {
            words: WordsState::new(),
            queue: QueueState::new(),
            settings: SettingsState::new(),
            current_view: NavItem::default(),
            window_width: 1024,
            theme: AppTheme::Light,
        }
    }
}

impl MainWindowState {
    /// Creates a new MainWindowState.
    pub fn new() -> Self {
        Self::default()
    }
}
