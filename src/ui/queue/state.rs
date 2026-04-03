//! Queue panel UI state types.
//!
//! State is organized with focused sub-states:
//! - SelectionState: Queue item selection

use std::collections::HashSet;
use uuid::Uuid;

/// Selection state for queue items.
#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    /// Selected queue item IDs
    pub items: HashSet<Uuid>,
}

impl SelectionState {
    /// Toggle selection for a queue item.
    pub fn toggle(&mut self, item_id: Uuid) {
        if self.items.contains(&item_id) {
            self.items.remove(&item_id);
        } else {
            self.items.insert(item_id);
        }
    }

    /// Check if an item is selected.
    pub fn is_selected(&self, item_id: Uuid) -> bool {
        self.items.contains(&item_id)
    }

    /// Select all items.
    pub fn select_all(&mut self, item_ids: impl IntoIterator<Item = Uuid>) {
        self.items.extend(item_ids);
    }

    /// Clear all selections.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get count of selected items.
    pub fn count(&self) -> usize {
        self.items.len()
    }
}

/// Queue panel state.
#[derive(Debug, Default)]
pub struct QueueState {
    /// Selection state
    pub selection: SelectionState,
}

impl QueueState {
    /// Creates a new QueueState.
    pub fn new() -> Self {
        Self::default()
    }
}
