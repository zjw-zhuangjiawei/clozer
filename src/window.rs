//! Window types for multi-window support.
//!
//! Contains WindowType enum and Window enum.
//! UI state types are co-located in their respective ui/ modules.

use crate::ui::main_window::MainWindowState;
use crate::ui::settings_window::SettingsUiState;

/// Window type enum for future extensibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowType {
    Main,
    Settings,
}

impl WindowType {
    /// Returns the window settings for this window type.
    pub fn window_settings(&self) -> iced::window::Settings {
        match self {
            WindowType::Main => iced::window::Settings {
                exit_on_close_request: false,
                ..Default::default()
            },
            WindowType::Settings => iced::window::Settings {
                exit_on_close_request: false,
                size: iced::Size::new(400.0, 300.0),
                ..Default::default()
            },
        }
    }
}

/// Window content enum containing state for each window type.
#[derive(Debug)]
pub enum Window {
    Main(MainWindowState),
    Settings(SettingsUiState),
}

impl Window {
    /// Creates a new Window with the specified type.
    pub fn new(window_type: WindowType) -> Self {
        match window_type {
            WindowType::Main => Window::Main(MainWindowState::new()),
            WindowType::Settings => Window::Settings(SettingsUiState::new()),
        }
    }
}
