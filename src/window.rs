//! Window types for multi-window support.
//!
//! Contains WindowType enum and Window enum.
//! UI state types are co-located in their respective ui/ modules.

use crate::ui::main_window::MainWindowState;

/// Window type enum for future extensibility.
///
/// Currently only Main is implemented. Additional window types
/// can be added here (e.g., Settings).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowType {
    Main,
    // Future: Settings,
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
