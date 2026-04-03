//! Declarative DSL component library for consistent UI construction.
//!
//! This module provides a fluent, declarative API for building UI components
//! with consistent styling across the application.
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::ui::components::dsl::{badge, pos_badge, card};
//!
//! // Create a POS badge
//! let pos = pos_badge(PartOfSpeech::Noun);
//!
//! // Create a card with content
//! let my_card = card::<Message>()
//!     .padding(16.0)
//!     .push(some_element)
//!     .build();
//! ```

pub mod badge;
pub mod button;
pub mod card;
pub mod input;
pub mod row;

use crate::ui::theme::{AppTheme, FontSize, Spacing};
use crate::ui::words::message::WordsMessage;
use iced::widget::{Column, Container, Text};
use iced::{Element, Length};

pub use badge::{Badge, BadgeStyle, badge, cefr_badge, pos_badge};
pub use button::{
    ButtonBuilder, ButtonStyle, ButtonVariant, button, danger_btn, primary_btn, secondary_btn,
};
pub use card::{Card, CardStyle, card};
pub use row::{RowBuilder, h_stack, row, v_stack};

/// Creates an empty state placeholder.
///
/// # Arguments
/// * `title` - The empty state title
/// * `subtitle` - The descriptive subtitle
pub fn empty_state<'a>(title: &'a str, subtitle: &'a str) -> Element<'a, WordsMessage> {
    let colors = AppTheme::default().colors();

    Container::new(
        Column::new()
            .spacing(Spacing::DEFAULT.m)
            .push(
                Container::new(
                    Text::new("[ ]")
                        .size(FontSize::Display.px())
                        .color(colors.text_secondary),
                )
                .padding(8),
            )
            .push(
                Text::new(title)
                    .size(FontSize::Subtitle.px())
                    .color(colors.text),
            )
            .push(
                Text::new(subtitle)
                    .size(FontSize::Footnote.px())
                    .color(colors.text_secondary),
            )
            .align_x(iced::Alignment::Center),
    )
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
