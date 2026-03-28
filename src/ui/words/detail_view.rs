//! Detail panel views for word/meaning/cloze.

use crate::assets;
use crate::models::Cloze;
use crate::models::types::{ClozeId, MeaningId, TagId, WordId};
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::theme::ButtonSize;
use crate::ui::words::message::{DetailMessage, WordsMessage};
use crate::ui::words::state::{DetailSelection, EditBuffer, EditContext};
use iced::Element;
use iced::widget::{Button, Column, Container, Row, Text, button, text_input};

/// Renders the detail panel based on current selection and edit mode.
pub fn view<'a>(
    selected_detail: Option<DetailSelection>,
    edit_mode: EditContext,
    edit_buffer: &'a EditBuffer,
    model: &'a Model,
) -> Element<'a, WordsMessage> {
    // NewWord mode - show form for creating new word
    if edit_mode == EditContext::NewWord {
        return new_word_view(edit_buffer).map(WordsMessage::Detail);
    }

    // NewMeaning mode - show form for adding new meaning to a word
    if let EditContext::NewMeaning(word_id) = edit_mode {
        if let Some(word) = model.word_registry.get(word_id) {
            return new_meaning_view(word.content.clone(), edit_buffer).map(WordsMessage::Detail);
        }
    }

    match selected_detail {
        Some(DetailSelection::Word(word_id)) => {
            if let Some(word) = model.word_registry.get(word_id) {
                if edit_mode == EditContext::Word(word_id) {
                    word_edit_view(word.id.into(), edit_buffer)
                } else {
                    word_detail_view(word.id.into(), word.content.clone(), model)
                }
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
                if edit_mode == EditContext::Meaning(meaning_id) {
                    meaning_edit_view(meaning.id.into(), word_content, edit_buffer)
                } else {
                    meaning_detail_view(
                        meaning.id.into(),
                        word_content,
                        meaning.definition.clone(),
                        meaning.pos,
                        meaning.cefr_level,
                        &meaning.tag_ids,
                        model,
                    )
                }
            } else {
                placeholder_view()
            }
        }
        Some(DetailSelection::Cloze(cloze_id)) => {
            if let Some(cloze) = model.cloze_registry.get(cloze_id) {
                cloze_detail_view(cloze_id.into(), cloze, model)
            } else {
                placeholder_view()
            }
        }
        None | Some(DetailSelection::None) => placeholder_view(),
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

/// View for creating a new word in the detail panel.
fn new_word_view<'a>(buffer: &'a EditBuffer) -> Element<'a, DetailMessage> {
    let word_input = text_input("Word *", &buffer.word_content)
        .on_input(|s| DetailMessage::EditNewWordContent(s))
        .width(iced::Length::Fill);

    let lang_string = buffer
        .word_language
        .as_ref()
        .map(|l| l.to_string())
        .unwrap_or_default();
    let lang_input = text_input("Language (optional)", &lang_string)
        .on_input(|s| {
            let parsed = s.trim().parse::<langtag::LangTagBuf>().ok();
            DetailMessage::EditNewWordLanguage(parsed)
        })
        .width(iced::Length::Fill);

    let def_input = text_input("Definition (optional)", &buffer.meaning_definition)
        .on_input(|s| DetailMessage::EditMeaningDefinition(s))
        .width(iced::Length::Fill);

    let pos_text = buffer.meaning_pos.to_string();
    let pos_display = format!("POS: {}", pos_text);
    let cefr_text = buffer
        .meaning_cefr
        .map(|c| c.to_string())
        .unwrap_or_else(|| "None".to_string());
    let cefr_display = format!("CEFR: {}", cefr_text);

    let save_btn = Button::new(Text::new("Save"))
        .style(button::primary)
        .padding(ButtonSize::Medium.to_iced_padding())
        .on_press(DetailMessage::SaveNewWord);
    let cancel_btn = Button::new(Text::new("Cancel"))
        .style(button::secondary)
        .padding(ButtonSize::Medium.to_iced_padding())
        .on_press(DetailMessage::Cancel);

    Container::new(
        Column::new()
            .spacing(15)
            .push(Text::new("Add New Word").size(20))
            .push(word_input)
            .push(lang_input)
            .push(def_input)
            .push(
                Row::new()
                    .push(Text::new(pos_display))
                    .push(Text::new(" | "))
                    .push(Text::new(cefr_display)),
            )
            .push(Row::new().spacing(10).push(save_btn).push(cancel_btn)),
    )
    .padding(20)
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .center_x(iced::Length::Fill)
    .center_y(iced::Length::Fill)
    .into()
}

/// View for adding a new meaning in the detail panel.
fn new_meaning_view<'a>(
    word_content: String,
    buffer: &'a EditBuffer,
) -> Element<'a, DetailMessage> {
    let def_input = text_input("Definition *", &buffer.meaning_definition)
        .on_input(|s| DetailMessage::EditMeaningDefinition(s))
        .width(iced::Length::Fill);

    let pos_text = buffer.meaning_pos.to_string();
    let pos_display = format!("POS: {}", pos_text);
    let cefr_text = buffer
        .meaning_cefr
        .map(|c| c.to_string())
        .unwrap_or_else(|| "None".to_string());
    let cefr_display = format!("CEFR: {}", cefr_text);

    let save_btn = Button::new(Text::new("Save Meaning"))
        .style(button::primary)
        .padding(ButtonSize::Medium.to_iced_padding())
        .on_press(DetailMessage::SaveNewMeaning);
    let cancel_btn = Button::new(Text::new("Cancel"))
        .style(button::secondary)
        .padding(ButtonSize::Medium.to_iced_padding())
        .on_press(DetailMessage::Cancel);

    Container::new(
        Column::new()
            .spacing(15)
            .push(Text::new(format!("Add Meaning to \"{}\"", word_content)).size(20))
            .push(def_input)
            .push(
                Row::new()
                    .push(Text::new(pos_display))
                    .push(Text::new(" | "))
                    .push(Text::new(cefr_display)),
            )
            .push(Row::new().spacing(10).push(save_btn).push(cancel_btn)),
    )
    .padding(20)
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .center_x(iced::Length::Fill)
    .center_y(iced::Length::Fill)
    .into()
}

/// Renders word details in the detail panel.
fn word_detail_view<'a>(
    word_id: WordId,
    word_content: String,
    model: &'a Model,
) -> Element<'a, WordsMessage> {
    // Get theme colors
    let colors = AppTheme::default().colors();

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
                                .padding(ButtonSize::Medium.to_iced_padding())
                                .on_press(WordsMessage::Detail(DetailMessage::SelectMeaning(
                                    meaning.id,
                                ))),
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
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::Detail(DetailMessage::Clear));

    let edit_icon_handle = assets::get_svg("edit_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(iced::widget::svg::Handle::from_memory)
        .unwrap_or_else(|| iced::widget::svg::Handle::from_memory(Vec::new()));
    let edit_icon = iced::widget::svg(edit_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));
    let edit_btn = Button::new(edit_icon)
        .style(button::secondary)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::Detail(DetailMessage::StartEditWord(
            word_id.into(),
        )));

    let word_content = word_content.clone();
    Column::new()
        .push(
            Row::new()
                .push(Text::new(word_content).size(20))
                .push(Text::new(" ").width(iced::Length::Fill))
                .push(edit_btn)
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

/// Renders word edit form in the detail panel.
fn word_edit_view<'a>(_word_id: uuid::Uuid, buffer: &'a EditBuffer) -> Element<'a, WordsMessage> {
    let word_input = text_input("Word *", &buffer.word_content)
        .on_input(|s| WordsMessage::Detail(DetailMessage::EditWordContent(s)))
        .width(iced::Length::Fill);

    let lang_string = buffer
        .word_language
        .as_ref()
        .map(|l| l.to_string())
        .unwrap_or_default();
    let lang_input = text_input("Language (optional)", &lang_string)
        .on_input(|s| {
            let parsed = s.trim().parse::<langtag::LangTagBuf>().ok();
            WordsMessage::Detail(DetailMessage::EditWordLanguage(parsed))
        })
        .width(iced::Length::Fill);

    let def_input = text_input("Definition (optional)", &buffer.meaning_definition)
        .on_input(|s| WordsMessage::Detail(DetailMessage::EditMeaningDefinition(s)))
        .width(iced::Length::Fill);

    let pos_text = buffer.meaning_pos.to_string();
    let pos_display = format!("POS: {}", pos_text);
    let cefr_text = buffer
        .meaning_cefr
        .map(|c| c.to_string())
        .unwrap_or_else(|| "None".to_string());
    let cefr_display = format!("CEFR: {}", cefr_text);

    let save_btn = Button::new(Text::new("Save"))
        .style(button::primary)
        .padding(ButtonSize::Medium.to_iced_padding())
        .on_press(WordsMessage::Detail(DetailMessage::Save));

    let cancel_btn = Button::new(Text::new("Cancel"))
        .style(button::secondary)
        .padding(ButtonSize::Medium.to_iced_padding())
        .on_press(WordsMessage::Detail(DetailMessage::Cancel));

    Container::new(
        Column::new()
            .spacing(15)
            .push(Text::new("Edit Word").size(20))
            .push(word_input)
            .push(lang_input)
            .push(def_input)
            .push(
                Row::new()
                    .push(Text::new(pos_display))
                    .push(Text::new(" | "))
                    .push(Text::new(cefr_display)),
            )
            .push(Row::new().spacing(10).push(save_btn).push(cancel_btn)),
    )
    .padding(20)
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .center_x(iced::Length::Fill)
    .center_y(iced::Length::Fill)
    .into()
}

/// Renders meaning details in the detail panel.
fn meaning_detail_view<'a>(
    meaning_id: MeaningId,
    word_content: String,
    definition: String,
    pos: crate::models::PartOfSpeech,
    cefr_level: Option<crate::models::CefrLevel>,
    tag_ids: &std::collections::BTreeSet<TagId>,
    model: &'a Model,
) -> Element<'a, WordsMessage> {
    // Get theme colors
    let colors = AppTheme::default().colors();

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
                .padding(ButtonSize::Medium.to_iced_padding())
                .width(iced::Length::Fill)
                .on_press(WordsMessage::Detail(DetailMessage::SelectCloze(*cloze_id)))
                .into()
        })
        .collect();

    let close_btn = Button::new(Text::new("×"))
        .style(button::secondary)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::Detail(DetailMessage::Clear));

    let edit_icon_handle = assets::get_svg("edit_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(iced::widget::svg::Handle::from_memory)
        .unwrap_or_else(|| iced::widget::svg::Handle::from_memory(Vec::new()));
    let edit_icon = iced::widget::svg(edit_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));
    let edit_btn = Button::new(edit_icon)
        .style(button::secondary)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::Detail(DetailMessage::StartEditMeaning(
            meaning_id,
        )));

    let mut column = Column::new()
        .push(
            Row::new()
                .push(Text::new(word_content).size(20))
                .push(Text::new(" ").width(iced::Length::Fill))
                .push(edit_btn)
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

/// Renders meaning edit form in the detail panel.
fn meaning_edit_view<'a>(
    _meaning_id: MeaningId,
    word_content: String,
    buffer: &'a EditBuffer,
) -> Element<'a, WordsMessage> {
    // Get theme colors
    let colors = AppTheme::default().colors();

    let close_btn = Button::new(Text::new("×"))
        .style(button::secondary)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::Detail(DetailMessage::Clear));

    let save_btn = Button::new(Text::new("Save"))
        .style(button::primary)
        .padding(ButtonSize::Medium.to_iced_padding())
        .on_press(WordsMessage::Detail(DetailMessage::Save));

    let cancel_btn = Button::new(Text::new("Cancel"))
        .style(button::secondary)
        .padding(ButtonSize::Medium.to_iced_padding())
        .on_press(WordsMessage::Detail(DetailMessage::Cancel));

    let pos_selected = buffer.meaning_pos;
    let cefr_selected = buffer.meaning_cefr;

    Column::new()
        .push(
            Row::new()
                .push(Text::new("Edit Meaning").size(20))
                .push(Text::new(" ").width(iced::Length::Fill))
                .push(close_btn)
                .align_y(iced::Alignment::Center),
        )
        .push(iced::widget::rule::horizontal(1))
        .push(
            Column::new()
                .push(Text::new("Word").size(14))
                .push(Text::new(word_content).size(16))
                .spacing(4),
        )
        .push(
            Column::new()
                .push(Text::new("Definition").size(14))
                .push(
                    text_input("Definition", &buffer.meaning_definition).on_input(|s| {
                        WordsMessage::Detail(DetailMessage::EditMeaningDefinition(s))
                    }),
                )
                .spacing(8),
        )
        .push(
            Column::new()
                .push(Text::new("Part of Speech").size(14))
                .push(
                    Row::new()
                        .push(
                            Button::new(Text::new(pos_selected.to_string()).size(14))
                                .style(button::secondary)
                                .padding(ButtonSize::Medium.to_iced_padding()),
                        )
                        .spacing(4),
                )
                .spacing(4),
        )
        .push(
            Column::new()
                .push(Text::new("CEFR Level").size(14))
                .push(
                    Row::new()
                        .push(
                            Button::new(
                                Text::new(
                                    cefr_selected
                                        .map(|c| c.to_string())
                                        .unwrap_or_else(|| "None".to_string()),
                                )
                                .size(14),
                            )
                            .style(button::secondary)
                            .padding(ButtonSize::Medium.to_iced_padding()),
                        )
                        .spacing(4),
                )
                .spacing(4),
        )
        .push(
            Row::new()
                .push(Text::new(" ").width(iced::Length::Fill))
                .push(save_btn)
                .push(cancel_btn)
                .align_y(iced::Alignment::Center),
        )
        .spacing(10)
        .padding(15)
        .into()
}

/// Renders cloze details in the detail panel.
fn cloze_detail_view<'a>(
    cloze_id: ClozeId,
    cloze: &Cloze,
    model: &'a Model,
) -> Element<'a, WordsMessage> {
    // Get theme colors
    let colors = AppTheme::default().colors();

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
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::Detail(DetailMessage::Clear));

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
            Button::new(delete_icon)
                .style(button::danger)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(WordsMessage::Cloze(
                    crate::ui::words::message::ClozeMessage::Delete { id: cloze_id },
                )),
        )
        .spacing(10)
        .padding(15)
        .into()
}
