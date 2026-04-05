//! Words panel UI state types.
//!
//! State is organized using the Manager pattern with focused sub-states:
//! - SearchManager: Search and filter management
//! - SelectionManager: Selection state management
//! - ExpansionManager: Expansion state management
//! - DetailManager: Detail panel management
//! - EditManager: Edit session management

use crate::ui::words::manager::{
    DetailManager, EditManager, ExpansionManager, SearchManager, SelectionManager,
};

/// Complete state for Words panel using Manager pattern.
#[derive(Debug, Default)]
pub struct WordsState {
    /// Search and filter manager
    pub search: SearchManager,
    /// Selection manager
    pub selection: SelectionManager,
    /// Expansion manager
    pub expansion: ExpansionManager,
    /// Detail panel manager
    pub detail: DetailManager,
    /// Edit session manager
    pub edit: EditManager,
}

impl WordsState {
    /// Creates a new WordsState.
    pub fn new() -> Self {
        Self::default()
    }
}
