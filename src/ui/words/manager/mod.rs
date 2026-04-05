//! Manager modules for state management.
//!
//! Each manager is responsible for a specific aspect of UI state:
//! - SearchManager: Search and filter state
//! - SelectionManager: Selection state for meanings and clozes
//! - ExpansionManager: Word expansion state
//! - DetailPanelManager: Detail panel state (view/edit) and buffers

pub mod edit;
pub mod expansion;
pub mod panel;
pub mod search;
pub mod selection;

pub use edit::{MeaningEditBuffer, WordEditBuffer};
pub use expansion::ExpansionManager;
pub use panel::{DetailPanelManager, DetailPanelState, TagDropdownState, TagDropdownTarget};
pub use search::SearchManager;
pub use selection::SelectionManager;
