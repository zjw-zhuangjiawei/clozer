//! Main window message types.

use super::nav::NavItem;
use super::queue::QueueMessage;
use super::settings::SettingsMessage;
use super::words::WordsMessage;

/// Messages for the main window, dispatched to sub-panels.
#[derive(Debug, Clone)]
pub enum MainWindowMessage {
    Words(WordsMessage),
    Queue(QueueMessage),
    Settings(SettingsMessage),
    Navigate(NavItem),
}
