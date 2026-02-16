//! Main window message types.

use super::queue::QueueMessage;
use super::words::WordsMessage;

/// Messages for the main window, dispatched to sub-panels.
#[derive(Debug, Clone)]
pub enum MainWindowMessage {
    Words(WordsMessage),
    Queue(QueueMessage),
}
