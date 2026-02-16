use std::collections::BTreeSet;
use uuid::Uuid;

/// Window type enum for future extensibility.
///
/// Currently only Main is implemented. Additional window types
/// can be added here (e.g., Settings, ClozeBrowser).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowType {
    Main,
    // Future: Settings,
    // Future: ClozeBrowser,
}

impl WindowType {
    /// Returns the window settings for this window type.
    pub fn window_settings(&self) -> iced::window::Settings {
        match self {
            WindowType::Main => iced::window::Settings {
                exit_on_close_request: false,
                ..Default::default()
            },
        }
    }
}

/// Window content enum containing state for each window type.
#[derive(Debug)]
pub enum Window {
    Main(MainWindowState),
}

impl Window {
    /// Creates a new Window with the specified type.
    pub fn new(window_type: WindowType) -> Self {
        match window_type {
            WindowType::Main => Window::Main(MainWindowState::new()),
        }
    }
}

/// State for the main application window.
///
/// Contains flattened selection state for words, meanings, and tags.
#[derive(Debug, Default)]
pub struct MainWindowState {
    pub selected_word_ids: BTreeSet<Uuid>,
    pub selected_meaning_ids: BTreeSet<Uuid>,
    pub selected_tag_ids: BTreeSet<Uuid>,
}

impl MainWindowState {
    /// Creates a new MainWindowState.
    pub fn new() -> Self {
        Self {
            selected_word_ids: BTreeSet::new(),
            selected_meaning_ids: BTreeSet::new(),
            selected_tag_ids: BTreeSet::new(),
        }
    }
}
