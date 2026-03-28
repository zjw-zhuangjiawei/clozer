//! Button style helpers for consistent button styling across the application.

use crate::ui::theme::ButtonSize;

impl ButtonSize {
    /// Returns the padding values as [vertical, horizontal] for use with iced::widget::Button::padding().
    ///
    /// Iced's padding is [vertical, horizontal] format.
    /// ButtonSize.padding() returns (horizontal, vertical) format.
    pub fn to_iced_padding(self) -> [f32; 2] {
        let (horizontal, vertical) = self.padding();
        [vertical, horizontal]
    }
}
