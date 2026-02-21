//! Application messages for event handling.
//!
//! Contains the top-level Message enum with hierarchical routing
//! to per-window message types.

use crate::state::QueueGenerationResult;
use crate::ui::message::MainWindowMessage;

/// Top-level application messages.
#[derive(Debug, Clone)]
pub enum Message {
    // Routed to main window (no window ID needed - single window)
    Main(MainWindowMessage),

    // Global (not window-specific)
    QueueGenerationResult(QueueGenerationResult),

    // Application close request (from subscription)
    CloseRequested,
}
