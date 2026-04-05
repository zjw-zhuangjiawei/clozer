//! Words panel module.

pub mod detail_view;
pub mod manager;
pub mod message;
pub mod state;
pub mod update;
pub mod view;

pub use self::manager::{
    DetailManager, DetailSelection, EditBuffer, EditContext, EditManager, ExpansionManager,
    NewMeaningForm, SearchManager, SelectionManager, TagDropdownState, TagDropdownTarget,
};
pub use self::message::WordsMessage;
pub use self::state::WordsState;
pub use self::update::update;
pub use self::view::view;
