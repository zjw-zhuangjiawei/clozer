//! Word tree — build the word list with expand/collapse, selection, and actions.

use crate::assets;
use crate::models::types::WordId;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::button;
use crate::ui::widgets::{CheckboxState, svg_checkbox};
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::WordsState;
use iced::widget::{Button, Column, Container, Row, Space, Text, container, rule, svg};
use iced::{Border, Element, Length};

use super::meaning::build_meaning_node;

pub fn build_word_tree<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    let results = words_state.search.get_results();

    let word_nodes: Vec<Element<'a, WordsMessage, AppTheme>> = match results {
        Some(results) => results
            .iter()
            .filter_map(|(word_id, _)| model.word_registry.get(*word_id))
            .map(|word| build_word_node(words_state, model, word, theme))
            .collect(),
        None => model
            .word_registry
            .iter()
            .map(|(_, word)| build_word_node(words_state, model, word, theme))
            .collect(),
    };

    if word_nodes.is_empty() {
        Column::new()
            .push(
                Container::new(Text::new("No words found. Add a word to get started."))
                    .center_x(Length::Fill)
                    .padding(Spacing::DEFAULT.l2),
            )
            .into()
    } else {
        Column::with_children(word_nodes)
            .spacing(Spacing::DEFAULT.xs2)
            .into()
    }
}

pub fn build_word_node<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    word: &'a crate::models::Word,
    theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    let is_expanded = words_state.expansion.is_expanded(word.id);
    let is_selected = words_state.selection.is_word_selected(word);
    let is_partial = words_state.selection.is_word_partial(word);

    let colors = theme.colors();

    let expand_icon_name = if is_expanded {
        "keyboard_arrow_down_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    } else {
        "keyboard_arrow_right_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    };
    let expand_icon_handle = assets::get_svg(expand_icon_name)
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
    let expand_icon: Element<'a, WordsMessage, AppTheme> = Button::new(
        svg(expand_icon_handle)
            .width(Length::Fixed(16.0))
            .height(Length::Fixed(16.0)),
    )
    .style(button::secondary)
    .padding(ButtonSize::Small.to_iced_padding())
    .on_press(if is_expanded {
        WordsMessage::WordCollapsed(word.id)
    } else {
        WordsMessage::WordExpanded(word.id)
    })
    .into();

    let checkbox: Element<'a, WordsMessage, AppTheme> = if word.meaning_ids.is_empty() {
        let radio_handle =
            assets::get_svg("radio_button_unchecked_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
                .map(svg::Handle::from_memory)
                .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
        Button::new(
            svg(radio_handle)
                .width(Length::Fixed(20.0))
                .height(Length::Fixed(20.0)),
        )
        .style(button::tertiary)
        .padding([2, 6])
        .width(Length::Fixed(30.0))
        .into()
    } else if is_partial {
        svg_checkbox(
            CheckboxState::Indeterminate,
            WordsMessage::WordToggled(word.id),
            theme,
        )
    } else {
        svg_checkbox(is_selected, WordsMessage::WordToggled(word.id), theme)
    };

    let word_content: Element<'a, WordsMessage, AppTheme> =
        Button::new(Text::new(&word.content).size(FontSize::Subtitle.px()))
            .style(button::tertiary)
            .padding(ButtonSize::Small.to_iced_padding())
            .on_press(WordsMessage::WordSelected(word.id))
            .into();

    let word_header = Row::new()
        .push(expand_icon)
        .push(checkbox)
        .push(word_content)
        .push(Space::new())
        .push(build_word_actions(word.id))
        .spacing(Spacing::DEFAULT.s)
        .align_y(iced::Alignment::Center);

    if is_expanded {
        let mut content = Column::new()
            .push(word_header)
            .push(rule::horizontal(1))
            .spacing(Spacing::DEFAULT.xs2);

        content = content.push(
            Button::new(Text::new("+ Add Meaning"))
                .style(button::primary)
                .padding(ButtonSize::Medium.to_iced_padding())
                .on_press(WordsMessage::MeaningAddStarted { word_id: word.id }),
        );

        for meaning_id in &word.meaning_ids {
            if let Some(meaning) = model.meaning_registry.get(*meaning_id) {
                content = content.push(build_meaning_node(words_state, model, meaning, theme));
            }
        }

        Container::new(content)
            .padding(Spacing::DEFAULT.s)
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
    } else {
        Container::new(word_header)
            .padding(Spacing::DEFAULT.s)
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
}

pub fn build_word_actions<'a>(word_id: WordId) -> Element<'a, WordsMessage, AppTheme> {
    let delete_icon_handle = assets::get_svg("delete_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
    let delete_icon = svg(delete_icon_handle)
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0));

    Button::new(delete_icon)
        .style(button::danger)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::WordDeleted(word_id))
        .into()
}
