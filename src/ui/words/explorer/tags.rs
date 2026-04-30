//! Tag row and unified tag dropdown for word explorer.

use crate::models::types::MeaningId;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::AdvancedInput;
use crate::ui::widgets::button;
use crate::ui::words::manager::{TagDropdownState, TagDropdownTarget};
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::WordsState;
use iced::widget::{Button, Column, Container, Row, Text, container};
use iced::{Border, Element};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub(super) enum TagDropdownMode {
    Single { meaning_id: MeaningId },
    Batch,
}

pub fn build_tags_row<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    meaning: &'a crate::models::Meaning,
    theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    let mut tag_chips: Vec<Element<'a, WordsMessage, AppTheme>> = meaning
        .tag_ids
        .iter()
        .filter_map(|tag_id| model.tag_registry.get(*tag_id))
        .map(|tag| Button::new(Text::new(tag.name.clone())).into())
        .collect();

    tag_chips.push(
        Button::new(Text::new("+").size(FontSize::Caption.px()))
            .style(button::tertiary)
            .padding(ButtonSize::Small.to_iced_padding())
            .on_press(WordsMessage::TagDropdownOpened {
                for_meaning: meaning.id,
            })
            .into(),
    );

    let tag_dropdown: Option<Element<'a, WordsMessage, AppTheme>> = if let Some(dropdown) =
        words_state.panel.tag_dropdown()
    {
        match dropdown.target {
            TagDropdownTarget::SingleMeaning(mid) if mid == meaning.id => Some(build_tag_dropdown(
                dropdown,
                model,
                theme,
                TagDropdownMode::Single { meaning_id: mid },
            )),
            _ => None,
        }
    } else {
        None
    };

    let mut row = Row::new().extend(tag_chips).spacing(Spacing::DEFAULT.xxs);

    if let Some(dropdown) = tag_dropdown {
        row = row.push(dropdown);
    }

    row.into()
}

pub fn build_tag_dropdown<'a>(
    dropdown: &'a TagDropdownState,
    model: &'a Model,
    theme: AppTheme,
    mode: TagDropdownMode,
) -> Element<'a, WordsMessage, AppTheme> {
    let colors = theme.colors();

    let placeholder = match mode {
        TagDropdownMode::Single { .. } => "Search or create...",
        TagDropdownMode::Batch => "Search...",
    };
    let search = AdvancedInput::new(placeholder)
        .value(&dropdown.search)
        .on_input(WordsMessage::TagSearchChanged)
        .width(iced::Length::Fixed(150.0))
        .padding(Spacing::DEFAULT.xs);

    let filtered_tags: Vec<_> = model
        .tag_registry
        .iter()
        .filter(|(_, tag)| {
            dropdown.search.is_empty()
                || tag
                    .name
                    .to_lowercase()
                    .contains(&dropdown.search.to_lowercase())
        })
        .map(|(_, tag)| tag)
        .take(5)
        .collect();

    let mut tag_items: Vec<Element<'a, WordsMessage, AppTheme>> = filtered_tags
        .iter()
        .map(|tag| {
            let on_press = match mode {
                TagDropdownMode::Single { meaning_id } => WordsMessage::TagAddedToMeaning {
                    meaning_id,
                    tag_id: tag.id,
                },
                TagDropdownMode::Batch => WordsMessage::TagAddedToSelected { tag_id: tag.id },
            };
            Button::new(Text::new(&tag.name).size(FontSize::Footnote.px()))
                .width(iced::Length::Fill)
                .on_press(on_press)
                .into()
        })
        .collect();

    if matches!(mode, TagDropdownMode::Single { .. })
        && !dropdown.search.is_empty()
        && !model
            .tag_registry
            .iter()
            .any(|(_, t)| t.name.to_lowercase() == dropdown.search.to_lowercase())
    {
        let search = dropdown.search.clone();
        let meaning_id = if let TagDropdownMode::Single { meaning_id } = mode {
            meaning_id
        } else {
            MeaningId(Uuid::nil())
        };
        tag_items.push(
            Button::new(Text::new(format!("Create \"{}\"", search)).size(FontSize::Footnote.px()))
                .width(iced::Length::Fill)
                .on_press(WordsMessage::TagQuickCreated {
                    meaning_id,
                    name: search,
                })
                .into(),
        );
    }

    Container::new(
        Column::new()
            .push(Element::new(search))
            .extend(tag_items)
            .spacing(Spacing::DEFAULT.xs)
            .padding(Spacing::DEFAULT.xs2),
    )
    .width(iced::Length::Fixed(170.0))
    .style(move |_| container::Style {
        background: Some(colors.semantic.surface.raised.into()),
        border: Border {
            color: colors.semantic.border.default,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    })
    .into()
}
