//! Detail panel views for word/meaning/cloze.

use crate::assets;
use crate::models::Cloze;
use crate::state::Model;
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::DetailSelection;
use iced::Element;
use iced::widget::{Button, Column, Container, Row, Text, button};

/// Renders the detail panel based on current selection.
pub fn view(selected_detail: Option<DetailSelection>, model: &Model) -> Element<'_, WordsMessage> {
    match selected_detail {
        Some(DetailSelection::Word(word_id)) => {
            if let Some(word) = model.word_registry.get(word_id) {
                word_detail_view(word.id, word.content.clone(), model)
            } else {
                placeholder_view()
            }
        }
        Some(DetailSelection::Meaning(meaning_id)) => {
            if let Some(meaning) = model.meaning_registry.get(meaning_id) {
                let word_content = model
                    .word_registry
                    .get(meaning.word_id)
                    .map(|w| w.content.clone())
                    .unwrap_or_default();
                meaning_detail_view(
                    meaning.id,
                    word_content,
                    meaning.definition.clone(),
                    meaning.pos,
                    meaning.cefr_level,
                    &meaning.tag_ids,
                    model,
                )
            } else {
                placeholder_view()
            }
        }
        Some(DetailSelection::Cloze(cloze_id)) => {
            if let Some(cloze) = model.cloze_registry.get(cloze_id) {
                cloze_detail_view(cloze_id, cloze, model)
            } else {
                placeholder_view()
            }
        }
        None => placeholder_view(),
    }
}

/// Renders the placeholder when nothing is selected.
fn placeholder_view<'a>() -> Element<'a, WordsMessage> {
    Container::new(Text::new("Select an item to view details"))
        .center_x(iced::Length::Fill)
        .center_y(iced::Length::Fill)
        .padding(20)
        .into()
}

/// Renders word details in the detail panel.
fn word_detail_view<'a>(
    word_id: uuid::Uuid,
    word_content: String,
    model: &'a Model,
) -> Element<'a, WordsMessage> {
    // Get all meanings for this word
    let meaning_items: Vec<Element<'a, WordsMessage>> = model
        .word_registry
        .iter()
        .find(|(_, w)| w.id == word_id)
        .map(|(_, word)| {
            word.meaning_ids
                .iter()
                .filter_map(|mid| model.meaning_registry.get(*mid))
                .map(|meaning| {
                    let pos_text = format!("[{}]", meaning.pos);
                    let cloze_count = model.cloze_registry.iter_by_meaning_id(meaning.id).count();

                    Row::new()
                        .push(
                            Button::new(Text::new(&meaning.definition).size(14))
                                .style(button::secondary)
                                .padding([4, 8])
                                .on_press(WordsMessage::ToggleMeaningDetail(meaning.id)),
                        )
                        .push(Text::new(pos_text).size(12))
                        .push(Text::new(format!("{} clozes", cloze_count)).size(12))
                        .spacing(10)
                        .into()
                })
                .collect()
        })
        .unwrap_or_default();

    let close_btn = Button::new(Text::new("×"))
        .style(button::secondary)
        .padding([4, 8])
        .on_press(WordsMessage::ClearDetailSelection);

    let word_content = word_content.clone();
    Column::new()
        .push(
            Row::new()
                .push(Text::new(word_content).size(20))
                .push(Text::new(" ").width(iced::Length::Fill))
                .push(close_btn)
                .align_y(iced::Alignment::Center),
        )
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new("Meanings").size(14))
        .extend(meaning_items)
        .spacing(10)
        .padding(15)
        .into()
}

/// Renders meaning details in the detail panel.
fn meaning_detail_view<'a>(
    meaning_id: uuid::Uuid,
    word_content: String,
    definition: String,
    pos: crate::models::PartOfSpeech,
    cefr_level: Option<crate::models::CefrLevel>,
    tag_ids: &std::collections::BTreeSet<uuid::Uuid>,
    model: &'a Model,
) -> Element<'a, WordsMessage> {
    // Get tag names
    let tag_names: Vec<String> = tag_ids
        .iter()
        .filter_map(|tid| model.tag_registry.get(*tid).map(|t| t.name.clone()))
        .collect();

    // Get all clozes for this meaning
    let cloze_items: Vec<Element<'a, WordsMessage>> = model
        .cloze_registry
        .iter_by_meaning_id(meaning_id)
        .map(|(cloze_id, cloze)| {
            let text = cloze.render_blanks();
            Button::new(Text::new(text).size(13))
                .style(button::secondary)
                .width(iced::Length::Fill)
                .padding([6, 8])
                .on_press(WordsMessage::ToggleClozeDetail(*cloze_id))
                .into()
        })
        .collect();

    let close_btn = Button::new(Text::new("×"))
        .style(button::secondary)
        .padding([4, 8])
        .on_press(WordsMessage::ClearDetailSelection);

    let mut column = Column::new()
        .push(
            Row::new()
                .push(Text::new(word_content).size(20))
                .push(Text::new(" ").width(iced::Length::Fill))
                .push(close_btn)
                .align_y(iced::Alignment::Center),
        )
        .push(iced::widget::rule::horizontal(1))
        .spacing(10)
        .padding(15);

    // Definition
    column = column.push(Text::new(definition).size(16));

    // POS and CEFR
    let mut meta_row = Row::new()
        .push(Text::new(format!("[{}]", pos)).size(14))
        .spacing(10);

    if let Some(cefr) = cefr_level {
        meta_row = meta_row.push(Text::new(cefr.to_string()).size(14));
    }
    column = column.push(meta_row);

    // Tags
    if !tag_names.is_empty() {
        let tags_text = tag_names.join(", ");
        column = column.push(Text::new(format!("Tags: {}", tags_text)).size(12));
    }

    // Clozes section
    if !cloze_items.is_empty() {
        column = column
            .push(iced::widget::rule::horizontal(1))
            .push(Text::new("Clozes").size(14))
            .extend(cloze_items);
    }

    column.into()
}

/// Renders cloze details in the detail panel.
fn cloze_detail_view<'a>(
    cloze_id: uuid::Uuid,
    cloze: &Cloze,
    model: &'a Model,
) -> Element<'a, WordsMessage> {
    // Get the source meaning and word
    let (word_content, definition) = model
        .meaning_registry
        .get(cloze.meaning_id)
        .map(|m| {
            let word = model.word_registry.get(m.word_id);
            (
                word.map(|w| w.content.clone()).unwrap_or_default(),
                m.definition.clone(),
            )
        })
        .unwrap_or_default();

    let close_btn = Button::new(Text::new("×"))
        .style(button::secondary)
        .padding([4, 8])
        .on_press(WordsMessage::ClearDetailSelection);

    let regenerate_icon_handle = assets::get_svg("edit_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(iced::widget::svg::Handle::from_memory)
        .unwrap_or_else(|| iced::widget::svg::Handle::from_memory(Vec::new()));
    let regenerate_icon = iced::widget::svg(regenerate_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));

    let delete_icon_handle = assets::get_svg("delete_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(iced::widget::svg::Handle::from_memory)
        .unwrap_or_else(|| iced::widget::svg::Handle::from_memory(Vec::new()));
    let delete_icon = iced::widget::svg(delete_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));

    Column::new()
        .push(
            Row::new()
                .push(Text::new(word_content).size(20))
                .push(Text::new(" ").width(iced::Length::Fill))
                .push(close_btn)
                .align_y(iced::Alignment::Center),
        )
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new(definition).size(14))
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new("Cloze Sentence").size(14))
        .push(Text::new(cloze.render_blanks()).size(16))
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new("Answer").size(14))
        .push(Text::new(cloze.render_answers()).size(16))
        .push(iced::widget::rule::horizontal(1))
        .push(
            Row::new()
                .push(
                    Button::new(regenerate_icon)
                        .style(button::secondary)
                        .padding([4, 8])
                        .on_press(WordsMessage::RegenerateCloze(cloze_id)),
                )
                .push(
                    Button::new(delete_icon)
                        .style(button::danger)
                        .padding([4, 8])
                        .on_press(WordsMessage::DeleteCloze(cloze_id)),
                )
                .spacing(10),
        )
        .spacing(10)
        .padding(15)
        .into()
}
