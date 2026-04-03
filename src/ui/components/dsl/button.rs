//! Declarative Button component DSL.
//!
//! Provides a fluent API for creating styled buttons.
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::ui::components::dsl::{button, primary_btn, secondary_btn};
//! use crate::ui::theme::ButtonSize;
//!
//! // Create a button with custom configuration
//! let my_btn = button::<Message>("Click me")
//!     .variant(ButtonVariant::Primary)
//!     .size(ButtonSize::Standard)
//!     .on_press(MyMessage::Clicked)
//!     .build();
//!
//! // Quick primary button
//! let primary = primary_btn("Submit", MyMessage::Submit);
//!
//! // Quick secondary button
//! let secondary = secondary_btn("Cancel", MyMessage::Cancel);
//! ```

use crate::ui::theme::{AppTheme, ButtonSize};
use iced::widget::{Button, Text};
use iced::{Color, Length};

/// Button style variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Primary button style (filled, prominent)
    Primary,
    /// Secondary button style (outlined, subtle)
    Secondary,
    /// Danger button style (red, destructive action)
    Danger,
}

/// Button style configuration.
#[derive(Debug, Clone, Copy)]
pub struct ButtonStyle {
    /// Button variant (primary, secondary, danger)
    pub variant: ButtonVariant,
    /// Button size
    pub size: ButtonSize,
}

impl ButtonStyle {
    /// Create a button style from the current theme.
    pub fn from_theme(variant: ButtonVariant, size: ButtonSize) -> Self {
        Self { variant, size }
    }

    /// Create a primary button style.
    pub fn primary(size: ButtonSize) -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size,
        }
    }

    /// Create a secondary button style.
    pub fn secondary(size: ButtonSize) -> Self {
        Self {
            variant: ButtonVariant::Secondary,
            size,
        }
    }

    /// Create a danger button style.
    pub fn danger(size: ButtonSize) -> Self {
        Self {
            variant: ButtonVariant::Danger,
            size,
        }
    }
}

/// Declarative Button component builder.
#[derive(Debug, Clone)]
pub struct ButtonBuilder<M> {
    text: String,
    style: ButtonStyle,
    on_press: Option<M>,
    width: Length,
}

impl<M> ButtonBuilder<M> {
    /// Create a new button with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: ButtonStyle::secondary(ButtonSize::Standard),
            on_press: None,
            width: Length::Shrink,
        }
    }

    /// Set the button variant.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.style.variant = variant;
        self
    }

    /// Set the button size.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.style.size = size;
        self
    }

    /// Set the on_press message.
    pub fn on_press(mut self, msg: M) -> Self {
        self.on_press = Some(msg);
        self
    }

    /// Set the button width.
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Set as primary variant.
    pub fn primary(self) -> Self {
        self.variant(ButtonVariant::Primary)
    }

    /// Set as secondary variant.
    pub fn secondary(self) -> Self {
        self.variant(ButtonVariant::Secondary)
    }

    /// Set as danger variant.
    pub fn danger(self) -> Self {
        self.variant(ButtonVariant::Danger)
    }

    /// Set as small size.
    pub fn small(self) -> Self {
        self.size(ButtonSize::Small)
    }

    /// Set as medium size.
    pub fn medium(self) -> Self {
        self.size(ButtonSize::Medium)
    }

    /// Set as standard size.
    pub fn standard(self) -> Self {
        self.size(ButtonSize::Standard)
    }

    /// Set as large size.
    pub fn large(self) -> Self {
        self.size(ButtonSize::Large)
    }

    /// Set width to fill available space.
    pub fn fill_width(self) -> Self {
        self.width(Length::Fill)
    }

    /// Set fixed width.
    pub fn fixed_width(self, width: f32) -> Self {
        self.width(Length::Fixed(width))
    }

    /// Build the button.
    pub fn build<'a>(self) -> Button<'a, M> {
        let colors = AppTheme::default().colors();

        let text_color = match self.style.variant {
            ButtonVariant::Primary => Color::WHITE,
            ButtonVariant::Secondary => colors.text,
            ButtonVariant::Danger => Color::WHITE,
        };

        let button: Button<'a, M> = Button::new(
            Text::new(self.text)
                .size(self.style.size.font_size().px())
                .color(text_color),
        )
        .padding(self.style.size.to_iced_padding())
        .width(self.width);

        match self.style.variant {
            ButtonVariant::Primary => button.style(iced::widget::button::primary),
            ButtonVariant::Secondary => button.style(iced::widget::button::secondary),
            ButtonVariant::Danger => button.style(iced::widget::button::danger),
        }
    }
}

/// Create a basic button with default style.
///
/// # Example
///
/// ```rust,ignore
/// let my_btn = button::<Message>("Click").on_press(MyMessage::Clicked).build();
/// ```
pub fn button<M>(text: impl Into<String>) -> ButtonBuilder<M> {
    ButtonBuilder::new(text)
}

/// Create a primary button (filled, prominent).
///
/// # Example
///
/// ```rust,ignore
/// let primary = primary_btn("Submit", MyMessage::Submit);
/// ```
pub fn primary_btn<'a, M>(text: impl Into<String>, msg: M) -> Button<'a, M> {
    button::<M>(text).primary().on_press(msg).build()
}

/// Create a secondary button (outlined, subtle).
///
/// # Example
///
/// ```rust,ignore
/// let secondary = secondary_btn("Cancel", MyMessage::Cancel);
/// ```
pub fn secondary_btn<'a, M>(text: impl Into<String>, msg: M) -> Button<'a, M> {
    button::<M>(text).secondary().on_press(msg).build()
}

/// Create a danger button (red, destructive action).
///
/// # Example
///
/// ```rust,ignore
/// let danger = danger_btn("Delete", MyMessage::Delete);
/// ```
pub fn danger_btn<'a, M>(text: impl Into<String>, msg: M) -> Button<'a, M> {
    button::<M>(text).danger().on_press(msg).build()
}
