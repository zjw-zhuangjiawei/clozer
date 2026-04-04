//! Declarative Badge component DSL.
//!
//! Provides a fluent API for creating styled badge/tag elements.
//! Badges are categorized into semantic variants for consistent styling:
//!
//! # Badge Variants
//!
//! - **Category**: Classification badges (POS, tags) - uses neutral/category colors
//! - **Status**: State badges (success, warning, danger) - semantic colors
//! - **Count**: Numeric badges with optional max value
//! - **Level**: CEFR proficiency level badges with color coding
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::ui::components::dsl::{badge, pos_badge, cefr_badge};
//!
//! // Create a POS badge
//! let pos = pos_badge(PartOfSpeech::Noun);
//!
//! // Create a CEFR badge
//! let cefr = cefr_badge(CefrLevel::A1);
//!
//! // Create a status badge
//! let status = status_badge("✓", Severity::Success);
//! ```

use crate::models::{CefrLevel, PartOfSpeech};
use crate::ui::design_tokens::{CefrBadgeStyle, SemanticColors};
use crate::ui::theme::{AppTheme, FontSize};
use iced::Color;
use iced::widget::{Container, Text};

#[derive(Debug, Clone)]
pub struct BadgeStyle {
    pub bg: Color,
    pub text_color: Color,
    pub border_color: Option<Color>,
    pub padding_h: f32,
    pub padding_v: f32,
    pub radius: f32,
}

impl BadgeStyle {
    pub fn from_theme(theme: AppTheme) -> Self {
        let colors = if theme == AppTheme::Dark {
            SemanticColors::dark()
        } else {
            SemanticColors::light()
        };

        Self {
            bg: colors.primary.scale(500),
            text_color: colors.text.on_primary,
            border_color: None,
            padding_h: 6.0,
            padding_v: 2.0,
            radius: 4.0,
        }
    }

    pub fn cefr(level: CefrLevel, _theme: AppTheme) -> Self {
        let style = CefrBadgeStyle::for_level(level);

        Self {
            bg: style.bg_color,
            text_color: style.text_color,
            border_color: Some(style.border_color),
            padding_h: 6.0,
            padding_v: 2.0,
            radius: 4.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Badge<M> {
    text: String,
    style: BadgeStyle,
    _phantom: std::marker::PhantomData<M>,
}

impl<M> Badge<M> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: BadgeStyle::from_theme(AppTheme::default()),
            _phantom: std::marker::PhantomData,
        }
    }

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
                color: self.style.border_color.unwrap_or(self.style.bg),
                width: 1.0,
                radius: self.style.radius.into(),
            },
            ..Default::default()
        })
    }
}

pub fn badge<M>(text: impl Into<String>) -> Badge<M> {
    Badge::new(text)
}

pub fn pos_badge<'a, M>(pos: PartOfSpeech) -> Container<'a, M> {
    let theme = AppTheme::default();
    let colors = theme.colors();

    Container::new(
        Text::new(format!("[{}]", pos))
            .size(FontSize::Footnote.px())
            .color(colors.pos_badge_text),
    )
    .padding([2.0, 6.0])
    .style(move |_| iced::widget::container::Style {
        background: Some(iced::Background::Color(colors.pos_badge_bg)),
        border: iced::Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
}

pub fn cefr_badge<'a, M>(level: CefrLevel) -> Container<'a, M> {
    let theme = AppTheme::default();
    let style = BadgeStyle::cefr(level, theme);

    Container::new(
        Text::new(level.to_string())
            .size(FontSize::Footnote.px())
            .color(style.text_color),
    )
    .padding([style.padding_v, style.padding_h])
    .style(move |_| iced::widget::container::Style {
        background: Some(iced::Background::Color(style.bg)),
        border: iced::Border {
            color: style.border_color.unwrap_or(style.bg),
            width: 1.0,
            radius: style.radius.into(),
        },
        ..Default::default()
    })
}

pub fn count_badge<'a, M>(value: usize) -> Container<'a, M> {
    let theme = AppTheme::default();
    let colors = theme.colors();

    Container::new(
        Text::new(value.to_string())
            .size(FontSize::Footnote.px())
            .color(colors.text),
    )
    .padding([2.0, 8.0])
    .style(move |_| iced::widget::container::Style {
        background: Some(iced::Background::Color(colors.secondary)),
        border: iced::Border {
            radius: 12.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
}

pub fn tag_badge<'a, M>(name: &'a str) -> Container<'a, M> {
    let theme = AppTheme::default();
    let colors = theme.colors();

    Container::new(
        Text::new(name)
            .size(FontSize::Caption.px())
            .color(colors.text),
    )
    .padding([2.0, 8.0])
    .style(move |_| iced::widget::container::Style {
        background: Some(iced::Background::Color(colors.surface)),
        border: iced::Border {
            color: colors.border,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
}
