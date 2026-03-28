//! Shared reusable UI components.

pub mod button;
pub mod checkbox;
pub mod detail;

pub use self::checkbox::{CheckboxState, svg_checkbox};
pub use self::detail::{
    action_row, badge, badge_row, detail_header, empty_state, section_card, section_title,
};
