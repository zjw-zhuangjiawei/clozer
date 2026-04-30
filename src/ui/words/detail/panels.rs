//! Detail panel views — word, meaning, and cloze views.

use crate::models::types::ClozeId;
use crate::models::{Cloze, Meaning, Word};
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::button;
use crate::ui::words::message::WordsMessage;
use iced::Element;
use iced::widget::{Button, Column, Text, rule};

use super::{build_header_row, build_icon_button, build_svg_icon, detail_panel};

pub fn word_detail_view<'a>(
    word: &'a Word,
    model: &'a Model,
) -> Element<'a, WordsMessage, AppTheme> {
    let header = build_header_row(
        word.content.clone(),
        Some(WordsMessage::EditWordStarted(word.id)),
        WordsMessage::DetailClosed,
    );

    let meaning_items: Vec<Element<'a, WordsMessage, AppTheme>> = model
        .word_registry
        .iter()
        .find(|(_, w)| w.id == word.id)
        .map(|(_, word)| {
            word.meaning_ids
                .iter()
                .filter_map(|mid| model.meaning_registry.get(*mid))
                .map(|meaning| {
                    Button::new(Text::new(&meaning.definition).size(FontSize::Body.px()))
                        .style(button::secondary)
                        .padding(ButtonSize::Medium.to_iced_padding())
                        .width(iced::Length::Fill)
                        .on_press(WordsMessage::MeaningSelected(meaning.id))
                        .into()
                })
                .collect()
        })
        .unwrap_or_default();

    let content = Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(header)
        .push(rule::horizontal(1))
        .push(Text::new("Meanings").size(FontSize::Body.px()))
        .extend(meaning_items);

    detail_panel(content)
}

pub fn meaning_detail_view<'a>(
    meaning: &'a Meaning,
    word: &'a Word,
    model: &'a Model,
) -> Element<'a, WordsMessage, AppTheme> {
    let tag_names: Vec<String> = meaning
        .tag_ids
        .iter()
        .filter_map(|tid| model.tag_registry.get(*tid).map(|t| t.name.clone()))
        .collect();

    let cloze_items: Vec<Element<'a, WordsMessage, AppTheme>> = model
        .cloze_registry
        .iter_by_meaning_id(meaning.id)
        .map(|(cloze_id, cloze)| {
            let text = cloze.render_blanks();
            Button::new(Text::new(text).size(FontSize::Body.px()))
                .style(button::secondary)
                .padding(ButtonSize::Medium.to_iced_padding())
                .width(iced::Length::Fill)
                .on_press(WordsMessage::ClozeSelected(*cloze_id))
                .into()
        })
        .collect();

    let header = build_header_row(
        word.content.clone(),
        Some(WordsMessage::EditMeaningStarted(meaning.id)),
        WordsMessage::DetailClosed,
    );

    let mut content = Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(header)
        .push(rule::horizontal(1))
        .push(Text::new(meaning.definition.clone()).size(FontSize::Subtitle.px()));

    if !tag_names.is_empty() {
        let tags_text = tag_names.join(", ");
        content =
            content.push(Text::new(format!("Tags: {}", tags_text)).size(FontSize::Footnote.px()));
    }

    if !cloze_items.is_empty() {
        content = content
            .push(rule::horizontal(1))
            .push(Text::new("Clozes").size(FontSize::Body.px()))
            .extend(cloze_items);
    }

    detail_panel(content)
}

pub fn cloze_detail_view<'a>(
    cloze_id: ClozeId,
    cloze: &'a Cloze,
    meaning: &'a Meaning,
    word: &'a Word,
) -> Element<'a, WordsMessage, AppTheme> {
    let header = build_header_row(word.content.clone(), None, WordsMessage::DetailClosed);

    let delete_icon = build_svg_icon("delete_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg", 16.0);
    let delete_btn = build_icon_button(
        delete_icon,
        button::danger,
        WordsMessage::ClozeDeleted(cloze_id),
    );

    let content = Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(header)
        .push(rule::horizontal(1))
        .push(Text::new(meaning.definition.clone()).size(FontSize::Body.px()))
        .push(rule::horizontal(1))
        .push(Text::new("Cloze Sentence").size(FontSize::Body.px()))
        .push(Text::new(cloze.render_blanks()).size(FontSize::Subtitle.px()))
        .push(rule::horizontal(1))
        .push(Text::new("Answer").size(FontSize::Body.px()))
        .push(Text::new(cloze.render_answers()).size(FontSize::Subtitle.px()))
        .push(rule::horizontal(1))
        .push(delete_btn);

    detail_panel(content)
}
