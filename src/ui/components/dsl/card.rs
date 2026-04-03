//! Declarative Card component DSL.
//!
//! Provides a fluent API for creating styled card/container elements.
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::ui::components::dsl::{card, CardStyle};
//! use crate::ui::theme::Spacing;
//!
//! // Create a basic card
//! let my_card = card::<Message>()
//!     .push(some_element)
//!     .build();
//!
//! // Create an elevated card
//! let elevated = card::<Message>()
//!     .style(CardStyle::from_theme().elevated())
//!     .padding(Spacing::DEFAULT.l)
//!     .push(content)
//!     .build();
//!
//! // Create a card with custom styling
//! let custom = card::<Message>()
//!     .padding(16.0)
//!     .radius(8.0)
//!     .push(content)
//!     .build();
//! ```

use crate::ui::theme::{AppTheme, Spacing};
use iced::Color;
use iced::widget::{Column, Container};

/// Card style configuration.
#[derive(Debug, Clone)]
pub struct CardStyle {
    /// Background color
    pub bg: Color,
    /// Border color
    pub border_color: Color,
    /// Border width
    pub border_width: f32,
    /// Padding
    pub padding: f32,
    /// Border radius
    pub radius: f32,
}

impl CardStyle {
    /// Create a card style from the current theme.
    pub fn from_theme() -> Self {
        let colors = AppTheme::default().colors();
        Self {
            bg: colors.surface,
            border_color: colors.border,
            border_width: 1.0,
            padding: Spacing::DEFAULT.m,
            radius: 4.0,
        }
    }

    /// Apply elevated styling (lighter background).
    pub fn elevated(self) -> Self {
        let colors = AppTheme::default().colors();
        Self {
            bg: colors.surface_elevated,
            ..self
        }
    }

    /// Set background color.
    pub fn with_bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    /// Set border color.
    pub fn with_border_color(mut self, c: Color) -> Self {
        self.border_color = c;
        self
    }

    /// Set border width.
    pub fn with_border_width(mut self, w: f32) -> Self {
        self.border_width = w;
        self
    }

    /// Set padding.
    pub fn with_padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }

    /// Set border radius.
    pub fn with_radius(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }
}

/// Declarative Card component builder.
pub struct Card<'a, M> {
    style: CardStyle,
    elements: Vec<iced::Element<'a, M>>,
}

impl<'a, M: 'a> Card<'a, M> {
    /// Create a new card.
    pub fn new() -> Self {
        Self {
            style: CardStyle::from_theme(),
            elements: Vec::new(),
        }
    }

    /// Apply a custom style.
    pub fn style(mut self, style: CardStyle) -> Self {
        self.style = style;
        self
    }

    /// Set padding.
    pub fn padding(mut self, p: f32) -> Self {
        self.style.padding = p;
        self
    }

    /// Set border radius.
    pub fn radius(mut self, r: f32) -> Self {
        self.style.radius = r;
        self
    }

    /// Push an element into the card.
    pub fn push<E: Into<iced::Element<'a, M>>>(mut self, element: E) -> Self {
        self.elements.push(element.into());
        self
    }

    /// Build the card into a Container widget.
    pub fn build(self) -> Container<'a, M> {
        let content = Column::with_children(self.elements).spacing(Spacing::DEFAULT.s);

        Container::new(content)
            .padding(self.style.padding)
            .width(iced::Length::Fill)
            .style(move |_| iced::widget::container::Style {
                background: Some(iced::Background::Color(self.style.bg)),
                border: iced::Border {
                    color: self.style.border_color,
                    width: self.style.border_width,
                    radius: self.style.radius.into(),
                },
                ..Default::default()
            })
    }
}

impl<'a, M: 'a> Default for Card<'a, M> {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a basic card with default theme style.
///
/// # Example
///
/// ```rust,ignore
/// let my_card = card::<Message>()
///     .push(element1)
///     .push(element2)
///     .build();
/// ```
pub fn card<'a, M: 'a>() -> Card<'a, M> {
    Card::new()
}
