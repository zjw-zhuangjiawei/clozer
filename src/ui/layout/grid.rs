//! Grid layout implementation.

use iced::Element;
use iced::widget::{Column, Row};

/// Grid layout builder for multi-column arrangements.
pub struct GridLayout<'a, M, T> {
    columns: u8,
    column_spacing: f32,
    row_spacing: f32,
    items: Vec<Element<'a, M, T>>,
}

impl<'a, M: 'a, T: 'a> GridLayout<'a, M, T> {
    /// Create a new grid layout builder.
    pub fn new(columns: u8) -> Self {
        Self {
            columns,
            column_spacing: 16.0,
            row_spacing: 16.0,
            items: Vec::new(),
        }
    }

    /// Set column and row spacing.
    pub fn spacing(mut self, column: f32, row: f32) -> Self {
        self.column_spacing = column;
        self.row_spacing = row;
        self
    }

    /// Add an item to the grid.
    pub fn push<E: Into<Element<'a, M, T>>>(mut self, item: E) -> Self {
        self.items.push(item.into());
        self
    }

    /// Build the grid layout.
    /// Items are distributed across columns in row-major order.
    pub fn build(self) -> Element<'a, M, T> {
        if self.items.is_empty() {
            return Column::new().into();
        }

        // Distribute items across columns
        let mut rows: Vec<Element<'a, M, T>> = Vec::new();
        let mut current_row: Vec<Element<'a, M, T>> = Vec::new();

        for item in self.items {
            current_row.push(item);
            if current_row.len() >= self.columns as usize {
                // Row is full, create a row element and start a new one
                let row = Row::with_children(current_row)
                    .spacing(self.column_spacing)
                    .into();
                rows.push(row);
                current_row = Vec::new();
            }
        }

        // Add remaining items as the last row
        if !current_row.is_empty() {
            let row = Row::with_children(current_row)
                .spacing(self.column_spacing)
                .into();
            rows.push(row);
        }

        Column::with_children(rows).spacing(self.row_spacing).into()
    }
}

/// Create a grid layout with specified columns.
pub fn grid_layout<'a, M: 'a, T: 'a>(
    columns: u8,
    items: Vec<Element<'a, M, T>>,
) -> Element<'a, M, T> {
    let mut builder = GridLayout::new(columns);
    for item in items {
        builder = builder.push(item);
    }
    builder.build()
}
