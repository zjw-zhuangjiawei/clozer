//! Main window message types.

use crate::ui::nav::NavItem;
use crate::ui::queue::QueueMessage;
use crate::ui::settings::SettingsMessage;
use crate::ui::words::WordsMessage;

/// Messages for the main window, dispatched to sub-panels.
#[derive(Debug, Clone)]
pub enum MainWindowMessage {
    Words(WordsMessage),
    Queue(QueueMessage),
    Settings(SettingsMessage),
    Navigate(NavItem),
}
