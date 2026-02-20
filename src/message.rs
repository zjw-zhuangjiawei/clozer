//! Application messages for event handling.
//!
//! Contains the top-level Message enum with hierarchical routing
//! to per-window message types.

use crate::state::QueueGenerationResult;
use crate::ui::main_window::MainWindowMessage;
use crate::ui::settings_window::SettingsMessage;
use crate::window::WindowType;

/// Top-level application messages.
///
/// Window lifecycle messages are handled by App directly.
/// Per-window messages are routed by window ID.
#[derive(Debug, Clone)]
pub enum Message {
    // Window lifecycle
    WindowOpened(iced::window::Id, WindowType),
    WindowCloseRequested(iced::window::Id),
    WindowClosed(iced::window::Id),

    // Routed to main window by window ID
    Main(iced::window::Id, MainWindowMessage),

    // Routed to settings window by window ID
    Settings(iced::window::Id, SettingsMessage),

    // Global (not window-specific)
    QueueGenerationResult(QueueGenerationResult),
}
