//! Declarative Row and Column builder DSL.
//!
//! Provides convenience functions for building horizontal and vertical layouts.
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::ui::components::dsl::{row, h_stack, v_stack};
//!
//! // Horizontal stack with default spacing
//! let h = h_stack(vec![elem1, elem2, elem3]);
//!
//! // Vertical stack with custom spacing
//! let v = v_stack(vec![elem1, elem2]).spacing(12);
//!
//! // Row builder for more control
//! let r = row::<Message>()
//!     .spacing(16.0)
//!     .align_y(iced::Alignment::Center)
//!     .padding(8.0);
//! ```

use iced::Alignment;
use iced::Element;
use iced::widget::{Column, Row};

/// Row builder for creating styled Row widgets.
#[derive(Debug, Clone)]
pub struct RowBuilder<M> {
    /// Spacing between elements
    spacing: f32,
    /// Vertical alignment
    align_y: Alignment,
    /// Padding
    padding: f32,
    _phantom: std::marker::PhantomData<M>,
}

impl<M> RowBuilder<M> {
    /// Create a new row builder.
    pub fn new() -> Self {
        Self {
            spacing: 8.0,
            align_y: Alignment::Center,
            padding: 0.0,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set the spacing between elements.
    pub fn spacing(mut self, s: f32) -> Self {
        self.spacing = s;
        self
    }

    /// Set the vertical alignment.
    pub fn align_y(mut self, a: Alignment) -> Self {
        self.align_y = a;
        self
    }

    /// Set the padding.
    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }

    /// Build into a Row widget.
    pub fn build<'a>(self) -> Row<'a, M> {
        Row::new()
            .spacing(self.spacing)
            .align_y(self.align_y)
            .padding(self.padding)
    }
}

impl<M> Default for RowBuilder<M> {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a basic row builder.
///
/// # Example
///
/// ```rust,ignore
/// let r = row::<Message>()
///     .spacing(16.0)
///     .align_y(iced::Alignment::Center);
/// ```
pub fn row<M>() -> RowBuilder<M> {
    RowBuilder::new()
}

/// Create a horizontal stack (Row) with default spacing.
///
/// # Example
///
/// ```rust,ignore
/// let h = h_stack(vec![elem1, elem2, elem3]);
/// ```
pub fn h_stack<'a, M>(elements: impl IntoIterator<Item = Element<'a, M>>) -> Row<'a, M> {
    Row::with_children(elements).spacing(8.0)
}

/// Create a vertical stack (Column) with default spacing.
///
/// # Example
///
/// ```rust,ignore
/// let v = v_stack(vec![elem1, elem2, elem3]);
/// ```
pub fn v_stack<'a, M>(elements: impl IntoIterator<Item = Element<'a, M>>) -> Column<'a, M> {
    Column::with_children(elements).spacing(8.0)
}
