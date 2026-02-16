//! Queue panel message types.

use uuid::Uuid;

/// Messages for the queue panel.
#[derive(Debug, Clone)]
pub enum QueueMessage {
    SelectToggle(Uuid),
    SelectAll,
    DeselectAll,
    Process,
    ClearCompleted,
    Remove(Uuid),
}
