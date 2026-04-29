//! Application messages for event handling.
//!
//! Contains the top-level Message enum with flat message types for
//! the single-window architecture.

use crate::state::QueueGenerationResult;
use crate::ui::AppTheme;
use crate::ui::nav::NavItem;
use crate::ui::queue::QueueMessage;
use crate::ui::settings::SettingsMessage;
use crate::ui::words::WordsMessage;

/// Top-level application messages for the single-window application.
///
/// Messages are organized by panel: Words, Queue, Settings.
/// Navigation and global messages are at the top level.
#[derive(Debug, Clone)]
pub enum Message {
    // Words panel
    Words(WordsMessage),

    // Queue panel
    Queue(QueueMessage),

    // Settings panel
    Settings(SettingsMessage),

    // Navigation
    Navigate(NavItem),

    // Global (not window-specific)
    QueueGenerationResult(QueueGenerationResult),

    // Application close request (from subscription)
    CloseRequested,

    // Window resize event for responsive layout
    WindowResized(u16),

    // Theme change request
    ThemeChanged(AppTheme),

    // Tab key pressed (for search suggestion acceptance)
    TabPressed,
}
