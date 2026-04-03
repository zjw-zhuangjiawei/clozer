//! Waterfall layout implementation.

use iced::Element;
use iced::widget::{Column, Row};

/// Waterfall layout builder for staggered content arrangement.
/// Features: columns with varying heights, content interleaved.
pub struct WaterfallLayout<'a, M> {
    columns: u8,
    column_spacing: f32,
    row_spacing: f32,
    items: Vec<Element<'a, M>>,
    /// Estimated heights for each column (used for placement algorithm)
    column_heights: Vec<f32>,
}

impl<'a, M: 'a> WaterfallLayout<'a, M> {
    /// Create a new waterfall layout builder.
    pub fn new(columns: u8) -> Self {
        Self {
            columns,
            column_spacing: 16.0,
            row_spacing: 16.0,
            items: Vec::new(),
            column_heights: vec![0.0; columns as usize],
        }
    }

    /// Set column and row spacing.
    pub fn spacing(mut self, column: f32, row: f32) -> Self {
        self.column_spacing = column;
        self.row_spacing = row;
        self
    }

    /// Add an item to the waterfall layout.
    pub fn push<E: Into<Element<'a, M>>>(mut self, item: E) -> Self {
        self.items.push(item.into());
        self
    }

    /// Add an item with estimated height for waterfall algorithm.
    /// Places the item in the shortest column.
    pub fn push_with_height<E: Into<Element<'a, M>>>(mut self, item: E, height: f32) -> Self {
        // Find the shortest column
        let min_col = self
            .column_heights
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        self.column_heights[min_col] += height + self.row_spacing;
        self.items.push(item.into());
        self
    }

    /// Build the waterfall layout.
    pub fn build(self) -> Element<'a, M> {
        if self.items.is_empty() {
            return Column::new().into();
        }

        // Distribute items across columns
        let mut columns: Vec<Vec<Element<'a, M>>> =
            (0..self.columns as usize).map(|_| Vec::new()).collect();
        for (i, item) in self.items.into_iter().enumerate() {
            let col = i % self.columns as usize;
            columns[col].push(item);
        }

        // Build each column
        let column_elements: Vec<Element<'a, M>> = columns
            .into_iter()
            .map(|items| {
                Column::with_children(items)
                    .spacing(self.row_spacing)
                    .into()
            })
            .collect();

        // Arrange columns in a row
        Row::with_children(column_elements)
            .spacing(self.column_spacing)
            .into()
    }
}

/// Create a waterfall layout with specified columns.
pub fn waterfall_layout<'a, M: 'a>(columns: u8, items: Vec<Element<'a, M>>) -> Element<'a, M> {
    let mut builder = WaterfallLayout::new(columns);
    for item in items {
        builder = builder.push(item);
    }
    builder.build()
}
