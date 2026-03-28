//! Detail panel reusable components.
//!
//! Provides consistent UI components for the detail panel:
//! - Badge: Tag-style labels for POS, CEFR, etc.
//! - SectionCard: Container with title for grouped content
//! - DetailHeader: Standard header with title, edit, and close buttons
//! - EmptyState: Placeholder when no item is selected

use crate::ui::theme::{AppTheme, ButtonSize, FontSize, Spacing};
use crate::ui::words::message::DetailMessage;
use iced::widget::{Button, Column, Container, Row, Text};
use iced::{Color, Element, Length};

/// Creates a badge/tag element with text and background color.
///
/// # Arguments
/// * `text` - The badge label text
/// * `bg` - Background color
/// * `text_color` - Text color
pub fn badge<'a>(text: &'a str, bg: Color, text_color: Color) -> Container<'a, DetailMessage> {
    Container::new(
        Text::new(text)
            .size(FontSize::Footnote.px())
            .color(text_color),
    )
    .padding([4, 8])
    .style(move |_| iced::widget::container::Style {
        background: Some(iced::Background::Color(bg)),
        ..Default::default()
    })
}

/// Creates a section card with a title and content.
///
/// # Arguments
/// * `title` - The section title
/// * `content` - The content element
pub fn section_card<'a>(
    title: &'a str,
    content: impl Into<Element<'a, DetailMessage>>,
) -> Container<'a, DetailMessage> {
    let colors = AppTheme::default().colors();

    Container::new(
        Column::new()
            .spacing(Spacing::DEFAULT.m)
            .push(
                Text::new(title)
                    .size(FontSize::Footnote.px())
                    .color(colors.text_secondary),
            )
            .push(content),
    )
    .padding(Spacing::DEFAULT.m)
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {
        background: Some(iced::Background::Color(colors.surface)),
        ..Default::default()
    })
}

/// Creates a row containing multiple badges/elements in a horizontal layout.
pub fn badge_row<'a>(
    items: impl IntoIterator<Item = Element<'a, DetailMessage>>,
) -> Row<'a, DetailMessage> {
    Row::new().spacing(Spacing::DEFAULT.s).extend(items)
}

/// Creates a horizontal row of action buttons (e.g., Save/Cancel).
pub fn action_row<'a>(
    primary: Button<'a, DetailMessage>,
    secondary: Button<'a, DetailMessage>,
) -> Row<'a, DetailMessage> {
    Row::new()
        .spacing(Spacing::DEFAULT.m)
        .push(secondary)
        .push(primary)
}

/// Creates the standard detail panel header.
///
/// # Arguments
/// * `title` - The header title text
/// * `on_edit` - Optional message when edit button is pressed
/// * `on_close` - Message when close button is pressed
pub fn detail_header<'a>(
    title: String,
    on_edit: Option<DetailMessage>,
    on_close: DetailMessage,
) -> Row<'a, DetailMessage> {
    // Close button
    let close_btn = Button::new(Text::new("×").size(FontSize::Title.px()))
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(on_close);

    // Edit button (optional)
    if let Some(msg) = on_edit {
        let edit_btn =
            match crate::assets::get_svg("edit_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg") {
                Some(icon) => {
                    let handle = iced::widget::svg::Handle::from_memory(icon);
                    Button::new(iced::widget::svg(handle).width(16).height(16))
                        .padding(6)
                        .on_press(msg)
                }
                None => {
                    tracing::warn!("Failed to load edit icon SVG");
                    Button::new(Text::new("✎").size(FontSize::Subtitle.px()))
                        .padding(6)
                        .on_press(msg)
                }
            };

        Row::new()
            .push(Text::new(title).size(FontSize::Title.px()))
            .push(Text::new(" ").width(Length::Fill))
            .push(edit_btn)
            .push(close_btn)
            .align_y(iced::Alignment::Center)
    } else {
        Row::new()
            .push(Text::new(title).size(FontSize::Title.px()))
            .push(Text::new(" ").width(Length::Fill))
            .push(close_btn)
            .align_y(iced::Alignment::Center)
    }
}

/// Creates an empty state placeholder.
///
/// # Arguments
/// * `title` - The empty state title
/// * `subtitle` - The descriptive subtitle
pub fn empty_state<'a>(title: &'a str, subtitle: &'a str) -> Element<'a, DetailMessage> {
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

/// Small text label for section titles.
pub fn section_title<'a>(text: &'a str) -> Text<'a> {
    let colors = AppTheme::default().colors();
    Text::new(text)
        .size(FontSize::Footnote.px())
        .color(colors.text_secondary)
}
