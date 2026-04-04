//! Detail panel views for word/meaning/cloze.

use crate::assets;
use crate::models::Cloze;
use crate::models::types::{ClozeId, MeaningId, TagId, WordId};
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::button;
use crate::ui::words::manager::{DetailSelection, EditBuffer, EditContext};
use crate::ui::words::message::WordsMessage;
use iced::Element;
use iced::widget::Space;
use iced::widget::{Button, Column, Container, Row, Text, text_input};

/// Renders the detail panel based on current selection and edit mode.
pub fn view<'a>(
    selected_detail: Option<DetailSelection>,
    edit_mode: EditContext,
    edit_buffer: &'a EditBuffer,
    model: &'a Model,
    theme: crate::ui::AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    // NewWord mode - show form for creating new word
    if edit_mode == EditContext::NewWord {
        return new_word_view(edit_buffer);
    }

    // NewMeaning mode - show form for adding new meaning to a word
    if let EditContext::NewMeaning(word_id) = edit_mode
        && let Some(word) = model.word_registry.get(word_id)
    {
        return new_meaning_view(word.content.clone(), edit_buffer);
    }

    match selected_detail {
        Some(DetailSelection::Word(word_id)) => {
            if let Some(word) = model.word_registry.get(word_id) {
                if edit_mode == EditContext::Word(word_id) {
                    word_edit_view(word.id.into(), edit_buffer)
                } else {
                    word_detail_view(word.id, word.content.clone(), model, theme)
                }
            } else {
                placeholder_view(theme)
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
                    meaning_edit_view(meaning.id, word_content, edit_buffer)
                } else {
                    meaning_detail_view(
                        meaning.id,
                        word_content,
                        meaning.definition.clone(),
                        meaning.pos,
                        meaning.cefr_level,
                        &meaning.tag_ids,
                        model,
                        theme,
                    )
                }
            } else {
                placeholder_view(theme)
            }
        }
        Some(DetailSelection::Cloze(cloze_id)) => {
            if let Some(cloze) = model.cloze_registry.get(cloze_id) {
                cloze_detail_view(cloze_id, cloze, model)
            } else {
                placeholder_view(theme)
            }
        }
        None | Some(DetailSelection::None) => placeholder_view(theme),
    }
}

/// Renders the placeholder when nothing is selected.
fn placeholder_view<'a>(_theme: crate::ui::AppTheme) -> Element<'a, WordsMessage, AppTheme> {
    Column::new().into()
}

/// View for creating a new word in the detail panel.
fn new_word_view<'a>(buffer: &'a EditBuffer) -> Element<'a, WordsMessage, AppTheme> {
    let word_input = text_input("Word *", &buffer.word_content)
        .on_input(WordsMessage::EditNewWordContentChanged)
        .width(iced::Length::Fill);

    let lang_string = buffer
        .word_language
        .as_ref()
        .map(|l| l.to_string())
        .unwrap_or_default();
    let lang_input = text_input("Language (optional)", &lang_string)
        .on_input(|s| {
            let parsed = s.trim().parse::<langtag::LangTagBuf>().ok();
            WordsMessage::EditNewWordLanguageChanged(parsed)
        })
        .width(iced::Length::Fill);

    let def_input = text_input("Definition (optional)", &buffer.meaning_definition)
        .on_input(WordsMessage::EditMeaningDefinitionChanged)
        .width(iced::Length::Fill);

    let pos_text = buffer.meaning_pos.to_string();
    let pos_display = format!("POS: {}", pos_text);
    let cefr_text = buffer
        .meaning_cefr
        .map(|c| c.to_string())
        .unwrap_or_else(|| "None".to_string());
    let cefr_display = format!("CEFR: {}", cefr_text);

    let save_btn = Button::new("Save")
        .style(button::primary)
        .on_press(WordsMessage::NewWordSaved);
    let cancel_btn = Button::new("Cancel")
        .style(button::secondary)
        .on_press(WordsMessage::EditCancelled);

    Container::new(
        Column::new()
            .spacing(Spacing::DEFAULT.l)
            .push(Text::new("Add New Word").size(FontSize::Heading.px()))
            .push(word_input)
            .push(lang_input)
            .push(def_input)
            .push(
                Row::new()
                    .push(Text::new(pos_display))
                    .push(Text::new(" | "))
                    .push(Text::new(cefr_display)),
            )
            .push(
                Row::new()
                    .spacing(Spacing::DEFAULT.s)
                    .push(save_btn)
                    .push(cancel_btn),
            ),
    )
    .padding(Spacing::DEFAULT.l)
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
) -> Element<'a, WordsMessage, AppTheme> {
    let def_input = text_input("Definition *", &buffer.meaning_definition)
        .on_input(WordsMessage::EditMeaningDefinitionChanged)
        .width(iced::Length::Fill);

    let pos_text = buffer.meaning_pos.to_string();
    let pos_display = format!("POS: {}", pos_text);
    let cefr_text = buffer
        .meaning_cefr
        .map(|c| c.to_string())
        .unwrap_or_else(|| "None".to_string());
    let cefr_display = format!("CEFR: {}", cefr_text);

    let save_btn = Button::new("Save Meaning")
        .style(button::primary)
        .on_press(WordsMessage::NewMeaningSaved);
    let cancel_btn = Button::new("Cancel")
        .style(button::secondary)
        .on_press(WordsMessage::EditCancelled);

    Container::new(
        Column::new()
            .spacing(Spacing::DEFAULT.l)
            .push(
                Text::new(format!("Add Meaning to \"{}\"", word_content))
                    .size(FontSize::Heading.px()),
            )
            .push(def_input)
            .push(
                Row::new()
                    .push(Text::new(pos_display))
                    .push(Text::new(" | "))
                    .push(Text::new(cefr_display)),
            )
            .push(
                Row::new()
                    .spacing(Spacing::DEFAULT.s)
                    .push(save_btn)
                    .push(cancel_btn),
            ),
    )
    .padding(Spacing::DEFAULT.l)
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
    _theme: crate::ui::AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    // Get all meanings for this word
    let meaning_items: Vec<Element<'a, WordsMessage, AppTheme>> = model
        .word_registry
        .iter()
        .find(|(_, w)| w.id == word_id)
        .map(|(_, word)| {
            word.meaning_ids
                .iter()
                .filter_map(|mid| model.meaning_registry.get(*mid))
                .map(|meaning| {
                    let _cloze_count = model.cloze_registry.iter_by_meaning_id(meaning.id).count();

                    Row::new()
                        .push(
                            Button::new(Text::new(&meaning.definition).size(FontSize::Body.px()))
                                .style(button::secondary)
                                .padding(ButtonSize::Medium.to_iced_padding())
                                .on_press(WordsMessage::MeaningSelected(meaning.id)),
                        )
                        // .push(pos_badge::<WordsMessage>(meaning.pos, theme))
                        // .push(count_badge::<WordsMessage>(cloze_count, theme))
                        .push(Space::new())
                        .spacing(Spacing::DEFAULT.s)
                        .into()
                })
                .collect()
        })
        .unwrap_or_default();

    let close_btn = Button::new("×")
        .style(button::secondary)
        .on_press(WordsMessage::DetailClosed);

    let edit_icon_handle = assets::get_svg("edit_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(iced::widget::svg::Handle::from_memory)
        .unwrap_or_else(|| iced::widget::svg::Handle::from_memory(Vec::new()));
    let edit_icon = iced::widget::svg(edit_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));
    let edit_btn = Button::new(edit_icon)
        .style(button::secondary)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::EditWordStarted(word_id));

    let word_content = word_content.clone();
    Column::new()
        .push(
            Row::new()
                .push(Text::new(word_content).size(FontSize::Heading.px()))
                .push(Space::new())
                .push(edit_btn)
                .push(close_btn)
                .align_y(iced::Alignment::Center),
        )
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new("Meanings").size(FontSize::Body.px()))
        .extend(meaning_items)
        .spacing(Spacing::DEFAULT.s)
        .padding(Spacing::DEFAULT.m)
        .into()
}

/// Renders word edit form in the detail panel.
fn word_edit_view<'a>(
    _word_id: uuid::Uuid,
    buffer: &'a EditBuffer,
) -> Element<'a, WordsMessage, AppTheme> {
    let word_input = text_input("Word *", &buffer.word_content)
        .on_input(WordsMessage::EditWordContentChanged)
        .width(iced::Length::Fill);

    let lang_string = buffer
        .word_language
        .as_ref()
        .map(|l| l.to_string())
        .unwrap_or_default();
    let lang_input = text_input("Language (optional)", &lang_string)
        .on_input(|s| {
            let parsed = s.trim().parse::<langtag::LangTagBuf>().ok();
            WordsMessage::EditWordLanguageChanged(parsed)
        })
        .width(iced::Length::Fill);

    let def_input = text_input("Definition (optional)", &buffer.meaning_definition)
        .on_input(WordsMessage::EditMeaningDefinitionChanged)
        .width(iced::Length::Fill);

    let pos_text = buffer.meaning_pos.to_string();
    let pos_display = format!("POS: {}", pos_text);
    let cefr_text = buffer
        .meaning_cefr
        .map(|c| c.to_string())
        .unwrap_or_else(|| "None".to_string());
    let cefr_display = format!("CEFR: {}", cefr_text);

    let save_btn = Button::new("Save")
        .style(button::primary)
        .on_press(WordsMessage::EditSaved);
    let cancel_btn = Button::new("Cancel")
        .style(button::secondary)
        .on_press(WordsMessage::EditCancelled);

    Container::new(
        Column::new()
            .spacing(Spacing::DEFAULT.l)
            .push(Text::new("Edit Word").size(FontSize::Heading.px()))
            .push(word_input)
            .push(lang_input)
            .push(def_input)
            .push(
                Row::new()
                    .push(Text::new(pos_display))
                    .push(Text::new(" | "))
                    .push(Text::new(cefr_display)),
            )
            .push(
                Row::new()
                    .spacing(Spacing::DEFAULT.s)
                    .push(save_btn)
                    .push(cancel_btn),
            ),
    )
    .padding(Spacing::DEFAULT.l)
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
    _pos: crate::models::PartOfSpeech,
    _cefr_level: Option<crate::models::CefrLevel>,
    tag_ids: &std::collections::BTreeSet<TagId>,
    model: &'a Model,
    _theme: crate::ui::AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    // Get tag names
    let tag_names: Vec<String> = tag_ids
        .iter()
        .filter_map(|tid| model.tag_registry.get(*tid).map(|t| t.name.clone()))
        .collect();

    // Get all clozes for this meaning
    let cloze_items: Vec<Element<'a, WordsMessage, AppTheme>> = model
        .cloze_registry
        .iter_by_meaning_id(meaning_id)
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

    let close_btn = Button::new("×")
        .style(button::secondary)
        .on_press(WordsMessage::DetailClosed);

    let edit_icon_handle = assets::get_svg("edit_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(iced::widget::svg::Handle::from_memory)
        .unwrap_or_else(|| iced::widget::svg::Handle::from_memory(Vec::new()));
    let edit_icon = iced::widget::svg(edit_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));
    let edit_btn = Button::new(edit_icon)
        .style(button::secondary)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::EditMeaningStarted(meaning_id));

    let mut column = Column::new()
        .push(
            Row::new()
                .push(Text::new(word_content).size(FontSize::Heading.px()))
                .push(Space::new())
                .push(edit_btn)
                .push(close_btn)
                .align_y(iced::Alignment::Center),
        )
        .push(iced::widget::rule::horizontal(1))
        .spacing(Spacing::DEFAULT.s)
        .padding(Spacing::DEFAULT.m);

    // Definition
    column = column.push(Text::new(definition).size(FontSize::Subtitle.px()));

    // POS and CEFR badges
    let meta_row = Row::new();
    // .push(pos_badge::<WordsMessage>(pos, theme));

    // if let Some(cefr) = cefr_level {
    //     meta_row = meta_row.push(cefr_badge::<WordsMessage>(cefr, theme));
    // }
    column = column.push(meta_row);

    // Tags
    if !tag_names.is_empty() {
        let tags_text = tag_names.join(", ");
        column =
            column.push(Text::new(format!("Tags: {}", tags_text)).size(FontSize::Footnote.px()));
    }

    // Clozes section
    if !cloze_items.is_empty() {
        column = column
            .push(iced::widget::rule::horizontal(1))
            .push(Text::new("Clozes").size(FontSize::Body.px()))
            .extend(cloze_items);
    }

    column.into()
}

/// Renders meaning edit form in the detail panel.
fn meaning_edit_view<'a>(
    _meaning_id: MeaningId,
    word_content: String,
    buffer: &'a EditBuffer,
) -> Element<'a, WordsMessage, AppTheme> {
    let close_btn = Button::new("×")
        .style(button::primary)
        .on_press(WordsMessage::DetailClosed);
    let save_btn = Button::new("Save")
        .style(button::primary)
        .on_press(WordsMessage::EditSaved);
    let cancel_btn = Button::new("Cancel")
        .style(button::secondary)
        .on_press(WordsMessage::EditCancelled);

    let pos_selected = buffer.meaning_pos;
    let cefr_selected = buffer.meaning_cefr;

    Column::new()
        .push(
            Row::new()
                .push(Text::new("Edit Meaning").size(FontSize::Heading.px()))
                .push(Space::new())
                .push(close_btn)
                .align_y(iced::Alignment::Center),
        )
        .push(iced::widget::rule::horizontal(1))
        .push(
            Column::new()
                .push(Text::new("Word").size(FontSize::Body.px()))
                .push(Text::new(word_content).size(FontSize::Subtitle.px()))
                .spacing(Spacing::DEFAULT.xs),
        )
        .push(
            Column::new()
                .push(Text::new("Definition").size(FontSize::Body.px()))
                .push(
                    text_input("Definition", &buffer.meaning_definition)
                        .on_input(WordsMessage::EditMeaningDefinitionChanged),
                )
                .spacing(Spacing::DEFAULT.s),
        )
        .push(
            Column::new()
                .push(Text::new("Part of Speech").size(FontSize::Body.px()))
                .push(
                    Row::new()
                        .push(
                            Button::new(
                                Text::new(pos_selected.to_string()).size(FontSize::Body.px()),
                            )
                            .style(button::secondary)
                            .padding(ButtonSize::Medium.to_iced_padding()),
                        )
                        .spacing(Spacing::DEFAULT.xs),
                )
                .spacing(Spacing::DEFAULT.xs),
        )
        .push(
            Column::new()
                .push(Text::new("CEFR Level").size(FontSize::Body.px()))
                .push(
                    Row::new()
                        .push(
                            Button::new(
                                Text::new(
                                    cefr_selected
                                        .map(|c| c.to_string())
                                        .unwrap_or_else(|| "None".to_string()),
                                )
                                .size(FontSize::Body.px()),
                            )
                            .style(button::secondary)
                            .padding(ButtonSize::Medium.to_iced_padding()),
                        )
                        .spacing(Spacing::DEFAULT.xs),
                )
                .spacing(Spacing::DEFAULT.xs),
        )
        .push(
            Row::new()
                .push(Space::new())
                .push(save_btn)
                .push(cancel_btn)
                .align_y(iced::Alignment::Center),
        )
        .spacing(Spacing::DEFAULT.s)
        .padding(Spacing::DEFAULT.m)
        .into()
}

/// Renders cloze details in the detail panel.
fn cloze_detail_view<'a>(
    cloze_id: ClozeId,
    cloze: &Cloze,
    model: &'a Model,
) -> Element<'a, WordsMessage, AppTheme> {
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

    let close_btn = Button::new("×")
        .style(button::secondary)
        .on_press(WordsMessage::DetailClosed);

    let delete_icon_handle = assets::get_svg("delete_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(iced::widget::svg::Handle::from_memory)
        .unwrap_or_else(|| iced::widget::svg::Handle::from_memory(Vec::new()));
    let delete_icon = iced::widget::svg(delete_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));

    Column::new()
        .push(
            Row::new()
                .push(Text::new(word_content).size(FontSize::Heading.px()))
                .push(Space::new())
                .push(close_btn)
                .align_y(iced::Alignment::Center),
        )
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new(definition).size(FontSize::Body.px()))
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new("Cloze Sentence").size(FontSize::Body.px()))
        .push(Text::new(cloze.render_blanks()).size(FontSize::Subtitle.px()))
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new("Answer").size(FontSize::Body.px()))
        .push(Text::new(cloze.render_answers()).size(FontSize::Subtitle.px()))
        .push(iced::widget::rule::horizontal(1))
        .push(
            Button::new(delete_icon)
                .style(button::danger)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(WordsMessage::ClozeDeleted(cloze_id)),
        )
        .spacing(Spacing::DEFAULT.s)
        .padding(Spacing::DEFAULT.m)
        .into()
}
