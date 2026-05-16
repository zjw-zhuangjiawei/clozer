use crate::assets;
use crate::i18n::I18nManager;
use crate::models::types::WordId;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::button;
use crate::ui::widgets::container::{badge, card};
use crate::ui::widgets::text as txt;
use crate::ui::widgets::{CheckboxState, svg_checkbox};
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::WordsState;
use iced::widget::{Button, Column, Container, Row, Space, Text, rule, svg};
use iced::{Element, Length};

use super::meaning::build_meaning_node;

fn load_svg_handle(name: &str) -> svg::Handle {
    assets::get_svg(name)
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| {
            tracing::warn!(
                asset = %name,
                "SVG icon not found, using empty handle"
            );
            svg::Handle::from_memory(Vec::new())
        })
}

pub fn build_word_tree<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    i18n: &'a I18nManager,
) -> Element<'a, WordsMessage, AppTheme> {
    let results = words_state.search.get_results();

    let word_nodes: Vec<Element<'a, WordsMessage, AppTheme>> = match results {
        Some(results) => results
            .iter()
            .filter_map(|(word_id, _)| model.word_registry.get(*word_id))
            .map(|word| build_word_node(words_state, model, word, i18n))
            .collect(),
        None => model
            .word_registry
            .iter()
            .map(|(_, word)| build_word_node(words_state, model, word, i18n))
            .collect(),
    };

    if word_nodes.is_empty() {
        Column::new()
            .push(
                Container::new(Text::new(i18n.tr("words-no-words")))
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

fn build_expand_icon<'a>(
    is_expanded: bool,
    word_id: WordId,
) -> Element<'a, WordsMessage, AppTheme> {
    let expand_icon_name = if is_expanded {
        "keyboard_arrow_down_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    } else {
        "keyboard_arrow_right_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    };
    let expand_icon_handle = load_svg_handle(expand_icon_name);
    Button::new(
        svg(expand_icon_handle)
            .width(Length::Fixed(16.0))
            .height(Length::Fixed(16.0)),
    )
    .style(button::secondary)
    .padding(ButtonSize::Small.to_iced_padding())
    .on_press(if is_expanded {
        WordsMessage::WordCollapsed(word_id)
    } else {
        WordsMessage::WordExpanded(word_id)
    })
    .into()
}

fn build_word_checkbox<'a>(
    word: &'a crate::models::Word,
    is_selected: bool,
    is_partial: bool,
) -> Element<'a, WordsMessage, AppTheme> {
    if word.meaning_ids.is_empty() {
        let radio_handle =
            load_svg_handle("radio_button_unchecked_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg");
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
        )
    } else {
        svg_checkbox(is_selected, WordsMessage::WordToggled(word.id))
    }
}

fn build_word_header_row<'a>(
    word: &'a crate::models::Word,
    words_state: &'a WordsState,
) -> Row<'a, WordsMessage, AppTheme> {
    let is_expanded = words_state.expansion.is_expanded(word.id);
    let is_selected = words_state.selection.is_word_selected(word);
    let is_partial = words_state.selection.is_word_partial(word);

    let expand_icon = build_expand_icon(is_expanded, word.id);
    let checkbox = build_word_checkbox(word, is_selected, is_partial);

    let word_content: Element<'a, WordsMessage, AppTheme> =
        Button::new(Text::new(&word.content).size(FontSize::Subtitle.px()))
            .style(button::tertiary)
            .padding(ButtonSize::Small.to_iced_padding())
            .on_press(WordsMessage::WordSelected(word.id))
            .into();

    let lang_badge: Option<Element<'a, WordsMessage, AppTheme>> =
        word.language.as_ref().map(|lang| {
            Container::new(
                Text::new(lang.to_string())
                    .size(FontSize::Caption.px())
                    .style(txt::primary_alt),
            )
            .padding([1, 6])
            .style(badge)
            .into()
        });

    let mut word_header = Row::new()
        .push(expand_icon)
        .push(checkbox)
        .push(word_content)
        .spacing(Spacing::DEFAULT.s)
        .align_y(iced::Alignment::Center);

    if let Some(badge) = lang_badge {
        word_header = word_header.push(badge);
    }

    word_header
        .push(Space::new())
        .push(build_word_actions(word.id))
}

fn build_word_expanded_content<'a>(
    word: &'a crate::models::Word,
    words_state: &'a WordsState,
    model: &'a Model,
    i18n: &'a I18nManager,
) -> Column<'a, WordsMessage, AppTheme> {
    let mut content = Column::new()
        .push(build_word_header_row(word, words_state))
        .push(rule::horizontal(1))
        .spacing(Spacing::DEFAULT.xs2);

    content = content.push(
        Button::new(Text::new(i18n.tr("words-add-meaning")))
            .style(button::primary)
            .padding(ButtonSize::Medium.to_iced_padding())
            .on_press(WordsMessage::MeaningAddStarted { word_id: word.id }),
    );

    for meaning_id in &word.meaning_ids {
        if let Some(meaning) = model.meaning_registry.get(*meaning_id) {
            content = content.push(build_meaning_node(words_state, model, meaning, i18n));
        }
    }

    content
}

pub fn build_word_node<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    word: &'a crate::models::Word,
    i18n: &'a I18nManager,
) -> Element<'a, WordsMessage, AppTheme> {
    let is_expanded = words_state.expansion.is_expanded(word.id);

    if is_expanded {
        Container::new(build_word_expanded_content(word, words_state, model, i18n))
            .padding(Spacing::DEFAULT.s)
            .style(card)
            .into()
    } else {
        Container::new(build_word_header_row(word, words_state))
            .padding(Spacing::DEFAULT.s)
            .style(card)
            .into()
    }
}

pub fn build_word_actions<'a>(word_id: WordId) -> Element<'a, WordsMessage, AppTheme> {
    let delete_icon_handle = load_svg_handle("delete_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg");
    let delete_icon = svg(delete_icon_handle)
        .width(Length::Fixed(16.0))
        .height(Length::Fixed(16.0));

    Button::new(delete_icon)
        .style(button::danger)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::WordDeleted(word_id))
        .into()
}
