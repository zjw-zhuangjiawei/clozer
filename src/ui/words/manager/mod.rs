//! Manager modules for state management.
//!
//! Each manager is responsible for a specific aspect of UI state:
//! - SearchManager: Search and filter state
//! - SelectionManager: Selection state for meanings and clozes
//! - ExpansionManager: Word expansion state
//! - DetailManager: Detail panel selection and tag dropdown
//! - EditManager: Edit session context and buffers

pub mod detail;
pub mod edit;
pub mod expansion;
pub mod search;
pub mod selection;

pub use detail::{DetailManager, DetailSelection, TagDropdownState, TagDropdownTarget};
pub use edit::{EditBuffer, EditContext, EditManager};
pub use expansion::ExpansionManager;
pub use search::SearchManager;
pub use selection::SelectionManager;
