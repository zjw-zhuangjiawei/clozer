//! Window types and state for multi-window architecture.
//!
//! Provides WindowType enum for extensibility and Window enum
//! for managing different window content types.

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
            WindowType::Main => iced::window::Settings::default(),
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
/// Currently empty - window-specific state can be added here if needed.
/// All application state is now held at the App level.
#[derive(Debug, Default)]
pub struct MainWindowState {}

impl MainWindowState {
    /// Creates a new MainWindowState.
    pub fn new() -> Self {
        Self {}
    }
}
