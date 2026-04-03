//! Declarative Badge component DSL.
//!
//! Provides a fluent API for creating styled badge/tag elements.
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::ui::components::dsl::{badge, pos_badge, cefr_badge};
//!
//! // Create a custom badge
//! let custom = badge::<Message>("VIP")
//!     .style(BadgeStyle {
//!         bg: Color::from_rgb(0.8, 0.2, 0.2),
//!         text_color: Color::WHITE,
//!         padding_h: 8.0,
//!         padding_v: 4.0,
//!         radius: 4.0,
//!     })
//!     .build();
//!
//! // Create a POS badge
//! let pos = pos_badge(PartOfSpeech::Noun);
//!
//! // Create a CEFR badge
//! let cefr = cefr_badge(CefrLevel::A1);
//! ```

use crate::ui::theme::{AppTheme, FontSize};
use iced::Color;
use iced::widget::{Container, Text};

/// Badge style configuration.
#[derive(Debug, Clone)]
pub struct BadgeStyle {
    /// Background color
    pub bg: Color,
    /// Text color
    pub text_color: Color,
    /// Horizontal padding
    pub padding_h: f32,
    /// Vertical padding
    pub padding_v: f32,
    /// Border radius
    pub radius: f32,
}

impl BadgeStyle {
    /// Create a badge style from the current theme.
    pub fn from_theme() -> Self {
        let colors = AppTheme::default().colors();
        Self {
            bg: colors.pos_badge_bg,
            text_color: colors.pos_badge_text,
            padding_h: 6.0,
            padding_v: 2.0,
            radius: 4.0,
        }
    }

    /// Set background color.
    pub fn with_bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    /// Set text color.
    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    /// Set horizontal padding.
    pub fn with_padding_h(mut self, h: f32) -> Self {
        self.padding_h = h;
        self
    }

    /// Set vertical padding.
    pub fn with_padding_v(mut self, v: f32) -> Self {
        self.padding_v = v;
        self
    }

    /// Set border radius.
    pub fn with_radius(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }
}

/// Declarative Badge component builder.
#[derive(Debug, Clone)]
pub struct Badge<M> {
    text: String,
    style: BadgeStyle,
    _phantom: std::marker::PhantomData<M>,
}

impl<M> Badge<M> {
    /// Create a new badge with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: BadgeStyle::from_theme(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Apply a custom style.
    pub fn style(mut self, style: BadgeStyle) -> Self {
        self.style = style;
        self
    }

    /// Apply POS (Part of Speech) badge style.
    pub fn pos_style(self) -> Self {
        self.style(BadgeStyle::from_theme())
    }

    /// Apply CEFR level badge style.
    pub fn cefr_style(self) -> Self {
        let colors = AppTheme::default().colors();
        self.style(BadgeStyle {
            bg: colors.surface_elevated,
            text_color: colors.text_secondary,
            padding_h: 6.0,
            padding_v: 2.0,
            radius: 4.0,
        })
    }

    /// Apply tag badge style.
    pub fn tag_style(self) -> Self {
        let colors = AppTheme::default().colors();
        self.style(BadgeStyle {
            bg: colors.surface,
            text_color: colors.text,
            padding_h: 8.0,
            padding_v: 4.0,
            radius: 4.0,
        })
    }

    /// Apply danger badge style.
    pub fn danger_style(self) -> Self {
        let colors = AppTheme::default().colors();
        self.style(BadgeStyle {
            bg: colors.danger,
            text_color: Color::WHITE,
            padding_h: 6.0,
            padding_v: 2.0,
            radius: 4.0,
        })
    }

    /// Build the badge into a Container widget.
    pub fn build<'a>(self) -> Container<'a, M> {
        Container::new(
            Text::new(self.text)
                .size(FontSize::Footnote.px())
                .color(self.style.text_color),
        )
        .padding([self.style.padding_v, self.style.padding_h])
        .style(move |_| iced::widget::container::Style {
            background: Some(iced::Background::Color(self.style.bg)),
            border: iced::Border {
                radius: self.style.radius.into(),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}

/// Create a basic badge with default theme style.
///
/// # Example
///
/// ```rust,ignore
/// let my_badge = badge::<Message>("Label").build();
/// ```
pub fn badge<M>(text: impl Into<String>) -> Badge<M> {
    Badge::new(text)
}

/// Create a POS (Part of Speech) badge.
///
/// # Example
///
/// ```rust,ignore
/// let pos = pos_badge(PartOfSpeech::Noun);
/// ```
pub fn pos_badge<'a, M>(pos: impl std::fmt::Display) -> Container<'a, M> {
    badge::<M>(format!("[{}]", pos)).pos_style().build()
}

/// Create a CEFR level badge.
///
/// # Example
///
/// ```rust,ignore
/// let cefr = cefr_badge(CefrLevel::A1);
/// ```
pub fn cefr_badge<'a, M>(level: impl std::fmt::Display) -> Container<'a, M> {
    badge::<M>(level.to_string()).cefr_style().build()
}
