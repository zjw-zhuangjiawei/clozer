//! Queue panel message types.
//!
//! Messages are organized hierarchically by domain:
//! - Selection: Item selection
//! - Action: Queue operations

use uuid::Uuid;

// ============================================================================
// Root Message Enum
// ============================================================================

/// Root message enum for Queue panel.
///
/// Delegates to domain-specific message handlers.
#[derive(Debug, Clone)]
pub enum QueueMessage {
    /// Selection-related messages
    Selection(QueueSelectionMessage),
    /// Action-related messages
    Action(QueueActionMessage),
}

// ============================================================================
// Domain-Specific Messages
// ============================================================================

/// Selection-related messages for queue items.
#[derive(Debug, Clone)]
pub enum QueueSelectionMessage {
    /// Toggle selection for a queue item
    Toggle(Uuid),
    /// Select all queue items
    SelectAll,
    /// Deselect all queue items
    DeselectAll,
}

/// Action-related messages for queue operations.
#[derive(Debug, Clone)]
pub enum QueueActionMessage {
    /// Process all pending queue items
    Process,
    /// Clear all completed items
    ClearCompleted,
    /// Remove a specific item from queue
    Remove(Uuid),
}
