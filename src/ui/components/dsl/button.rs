//! Declarative Button component DSL.
//!
//! Provides a fluent API for creating styled buttons with consistent
//! states and accessibility support.
//!
//! # Button Variants
//!
//! - **Primary**: Filled button for main actions
//! - **Secondary**: Outlined button for secondary actions
//! - **Tertiary**: Text-only button for subtle actions
//! - **Danger**: Filled red button for destructive actions
//!
//! # Button States
//!
//! Each button variant supports visual states:
//! - Default: Normal appearance
//! - Hover: Slightly darker/lighter background (desktop only)
//! - Active/Pressed: Visual feedback when clicked
//! - Disabled: Reduced opacity, no interaction
//! - Focused: Focus ring for keyboard navigation
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

use crate::ui::design_tokens::SemanticColors;
use crate::ui::theme::{AppTheme, ButtonSize as ThemeButtonSize};
use iced::widget::{Button, Text};
use iced::{Color, Length};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Tertiary,
    Danger,
}

impl ButtonVariant {
    pub fn theme_colors(&self, theme: AppTheme) -> ButtonColors {
        let colors = if theme == AppTheme::Dark {
            SemanticColors::dark()
        } else {
            SemanticColors::light()
        };

        match self {
            ButtonVariant::Primary => ButtonColors {
                bg: colors.primary.scale(500),
                bg_hover: colors.primary.scale(600),
                bg_active: colors.primary.scale(700),
                text: Color::WHITE,
                border: colors.primary.scale(500),
                border_hover: colors.primary.scale(600),
            },
            ButtonVariant::Secondary => ButtonColors {
                bg: Color::TRANSPARENT,
                bg_hover: colors.neutral.scale(100),
                bg_active: colors.neutral.scale(200),
                text: colors.text.primary,
                border: colors.border.default,
                border_hover: colors.border.emphasis,
            },
            ButtonVariant::Tertiary => ButtonColors {
                bg: Color::TRANSPARENT,
                bg_hover: colors.neutral.scale(100),
                bg_active: colors.neutral.scale(200),
                text: colors.primary.scale(500),
                border: Color::TRANSPARENT,
                border_hover: Color::TRANSPARENT,
            },
            ButtonVariant::Danger => ButtonColors {
                bg: colors.danger.scale(500),
                bg_hover: colors.danger.scale(600),
                bg_active: colors.danger.scale(700),
                text: Color::WHITE,
                border: colors.danger.scale(500),
                border_hover: colors.danger.scale(600),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ButtonColors {
    pub bg: Color,
    pub bg_hover: Color,
    pub bg_active: Color,
    pub text: Color,
    pub border: Color,
    pub border_hover: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonStyle {
    pub variant: ButtonVariant,
    pub size: ThemeButtonSize,
}

impl ButtonStyle {
    pub fn from_theme(variant: ButtonVariant, size: ThemeButtonSize) -> Self {
        Self { variant, size }
    }

    pub fn primary(size: ThemeButtonSize) -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size,
        }
    }

    pub fn secondary(size: ThemeButtonSize) -> Self {
        Self {
            variant: ButtonVariant::Secondary,
            size,
        }
    }

    pub fn tertiary(size: ThemeButtonSize) -> Self {
        Self {
            variant: ButtonVariant::Tertiary,
            size,
        }
    }

    pub fn danger(size: ThemeButtonSize) -> Self {
        Self {
            variant: ButtonVariant::Danger,
            size,
        }
    }

    pub fn colors(&self, theme: AppTheme) -> ButtonColors {
        self.variant.theme_colors(theme)
    }
}

#[derive(Debug, Clone)]
pub struct ButtonBuilder<M> {
    text: String,
    style: ButtonStyle,
    on_press: Option<M>,
    width: Length,
}

impl<M> ButtonBuilder<M> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: ButtonStyle::secondary(ThemeButtonSize::Standard),
            on_press: None,
            width: Length::Shrink,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.style.variant = variant;
        self
    }

    pub fn size(mut self, size: ThemeButtonSize) -> Self {
        self.style.size = size;
        self
    }

    pub fn on_press(mut self, msg: M) -> Self {
        self.on_press = Some(msg);
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn primary(self) -> Self {
        self.variant(ButtonVariant::Primary)
    }

    pub fn secondary(self) -> Self {
        self.variant(ButtonVariant::Secondary)
    }

    pub fn tertiary(self) -> Self {
        self.variant(ButtonVariant::Tertiary)
    }

    pub fn danger(self) -> Self {
        self.variant(ButtonVariant::Danger)
    }

    pub fn small(self) -> Self {
        self.size(ThemeButtonSize::Small)
    }

    pub fn medium(self) -> Self {
        self.size(ThemeButtonSize::Medium)
    }

    pub fn standard(self) -> Self {
        self.size(ThemeButtonSize::Standard)
    }

    pub fn large(self) -> Self {
        self.size(ThemeButtonSize::Large)
    }

    pub fn fill_width(self) -> Self {
        self.width(Length::Fill)
    }

    pub fn fixed_width(self, width: f32) -> Self {
        self.width(Length::Fixed(width))
    }

    pub fn build<'a>(self) -> Button<'a, M> {
        let theme = AppTheme::default();
        let colors = self.style.colors(theme);
        let text = self.text.clone();
        let text_color = match self.style.variant {
            ButtonVariant::Primary => Color::WHITE,
            ButtonVariant::Secondary => colors.text,
            ButtonVariant::Tertiary => colors.text,
            ButtonVariant::Danger => Color::WHITE,
        };

        let button: Button<'a, M> = Button::new(
            Text::new(text)
                .size(self.style.size.font_size().px())
                .color(text_color),
        )
        .padding(self.style.size.to_iced_padding())
        .width(self.width);

        match self.style.variant {
            ButtonVariant::Primary => button.style(iced::widget::button::primary),
            ButtonVariant::Secondary => button.style(iced::widget::button::secondary),
            ButtonVariant::Tertiary => button.style(iced::widget::button::secondary),
            ButtonVariant::Danger => button.style(iced::widget::button::danger),
        }
    }
}

pub fn button<M>(text: impl Into<String>) -> ButtonBuilder<M> {
    ButtonBuilder::new(text)
}

pub fn primary_btn<'a, M>(text: impl Into<String>, msg: M) -> Button<'a, M> {
    button::<M>(text).primary().on_press(msg).build()
}

pub fn secondary_btn<'a, M>(text: impl Into<String>, msg: M) -> Button<'a, M> {
    button::<M>(text).secondary().on_press(msg).build()
}

pub fn tertiary_btn<'a, M>(text: impl Into<String>, msg: M) -> Button<'a, M> {
    button::<M>(text).tertiary().on_press(msg).build()
}

pub fn danger_btn<'a, M>(text: impl Into<String>, msg: M) -> Button<'a, M> {
    button::<M>(text).danger().on_press(msg).build()
}

#[derive(Debug, Clone)]
pub struct IconButtonStyle {
    pub size: ThemeButtonSize,
    pub variant: ButtonVariant,
}

impl IconButtonStyle {
    pub fn new(size: ThemeButtonSize, variant: ButtonVariant) -> Self {
        Self { size, variant }
    }

    pub fn primary(size: ThemeButtonSize) -> Self {
        Self::new(size, ButtonVariant::Primary)
    }

    pub fn secondary(size: ThemeButtonSize) -> Self {
        Self::new(size, ButtonVariant::Secondary)
    }

    pub fn danger(size: ThemeButtonSize) -> Self {
        Self::new(size, ButtonVariant::Danger)
    }
}

pub fn icon_button<'a, M>(
    icon: impl Into<String>,
    msg: M,
    style: IconButtonStyle,
) -> Button<'a, M> {
    let theme = AppTheme::default();
    let colors = style.variant.theme_colors(theme);
    let size = style.size;

    Button::new(
        Text::new(icon.into())
            .size(size.font_size().px())
            .color(colors.text),
    )
    .padding(size.to_iced_padding())
    .style(match style.variant {
        ButtonVariant::Primary => iced::widget::button::primary,
        ButtonVariant::Secondary | ButtonVariant::Tertiary => iced::widget::button::secondary,
        ButtonVariant::Danger => iced::widget::button::danger,
    })
    .on_press(msg)
}
