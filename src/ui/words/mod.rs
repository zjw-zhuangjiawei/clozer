//! Words panel module.

pub mod detail_view;
pub mod manager;
pub mod message;
pub mod state;
pub mod update;
pub mod view;

pub use self::manager::{
    DetailPanelManager, DetailPanelState, ExpansionManager, MeaningEditBuffer, SearchManager,
    SelectionManager, TagDropdownState, TagDropdownTarget, WordEditBuffer,
};
pub use self::message::WordsMessage;
pub use self::state::WordsState;
pub use self::update::update;
pub use self::view::view;
