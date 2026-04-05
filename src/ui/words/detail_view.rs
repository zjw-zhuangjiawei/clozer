//! Detail panel views for word/meaning/cloze.

use crate::assets;
use crate::models::types::ClozeId;
use crate::models::{CefrLevel, Cloze, PartOfSpeech};
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::button;
use crate::ui::words::manager::{DetailPanelState, MeaningEditBuffer, WordEditBuffer};
use crate::ui::words::message::WordsMessage;
use iced::Element;
use iced::widget::Space;
use iced::widget::{Button, Column, Container, PickList, Row, Text, text_input};
use strum::VariantArray;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::Display, strum::VariantArray)]
pub enum CefrLevelOption {
    #[default]
    None,
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
}

impl CefrLevelOption {
    fn to_cefr(self) -> Option<CefrLevel> {
        match self {
            CefrLevelOption::None => None,
            CefrLevelOption::A1 => Some(CefrLevel::A1),
            CefrLevelOption::A2 => Some(CefrLevel::A2),
            CefrLevelOption::B1 => Some(CefrLevel::B1),
            CefrLevelOption::B2 => Some(CefrLevel::B2),
            CefrLevelOption::C1 => Some(CefrLevel::C1),
            CefrLevelOption::C2 => Some(CefrLevel::C2),
        }
    }

    fn from_cefr(cefr: Option<CefrLevel>) -> Self {
        match cefr {
            None => CefrLevelOption::None,
            Some(CefrLevel::A1) => CefrLevelOption::A1,
            Some(CefrLevel::A2) => CefrLevelOption::A2,
            Some(CefrLevel::B1) => CefrLevelOption::B1,
            Some(CefrLevel::B2) => CefrLevelOption::B2,
            Some(CefrLevel::C1) => CefrLevelOption::C1,
            Some(CefrLevel::C2) => CefrLevelOption::C2,
        }
    }
}

pub fn view<'a>(
    state: &'a DetailPanelState,
    word_buffer: &'a WordEditBuffer,
    meaning_buffer: &'a MeaningEditBuffer,
    model: &'a Model,
    theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    match state {
        DetailPanelState::Empty => placeholder_view(),

        DetailPanelState::WordView { word_id } => {
            if let Some(word) = model.word_registry.get(*word_id) {
                word_detail_view(word, model, theme)
            } else {
                placeholder_view()
            }
        }

        DetailPanelState::MeaningView { meaning_id } => {
            if let Some(meaning) = model.meaning_registry.get(*meaning_id) {
                if let Some(word) = model.word_registry.get(meaning.word_id) {
                    meaning_detail_view(meaning, word, model, theme)
                } else {
                    placeholder_view()
                }
            } else {
                placeholder_view()
            }
        }

        DetailPanelState::ClozeView { cloze_id } => {
            if let Some(cloze) = model.cloze_registry.get(*cloze_id) {
                if let Some(meaning) = model.meaning_registry.get(cloze.meaning_id) {
                    if let Some(word) = model.word_registry.get(meaning.word_id) {
                        cloze_detail_view(*cloze_id, cloze, meaning, word)
                    } else {
                        placeholder_view()
                    }
                } else {
                    placeholder_view()
                }
            } else {
                placeholder_view()
            }
        }

        DetailPanelState::WordCreating { .. } => word_form(
            "Add New Word".to_string(),
            word_buffer,
            meaning_buffer,
            WordsMessage::NewWordSaved,
        ),

        DetailPanelState::WordEditing { .. } => word_form(
            "Edit Word".to_string(),
            word_buffer,
            meaning_buffer,
            WordsMessage::EditSaved,
        ),

        DetailPanelState::MeaningCreating { word_id, .. } => {
            let word_content = model
                .word_registry
                .get(*word_id)
                .map(|w| w.content.clone())
                .unwrap_or_default();
            let title = format!("Add Meaning to \"{}\"", word_content);
            meaning_form(
                title,
                &word_content,
                meaning_buffer,
                WordsMessage::MeaningAddSaved,
                theme,
            )
        }

        DetailPanelState::MeaningEditing { meaning_id, .. } => {
            if let Some(meaning) = model.meaning_registry.get(*meaning_id) {
                let word_content = model
                    .word_registry
                    .get(meaning.word_id)
                    .map(|w| w.content.clone())
                    .unwrap_or_default();
                let title = "Edit Meaning".to_string();
                meaning_form(
                    title,
                    &word_content,
                    meaning_buffer,
                    WordsMessage::EditSaved,
                    theme,
                )
            } else {
                placeholder_view()
            }
        }
    }
}

fn placeholder_view() -> Element<'static, WordsMessage, AppTheme> {
    Column::new().into()
}

fn word_form<'a>(
    title: String,
    word_buffer: &'a WordEditBuffer,
    _meaning_buffer: &'a MeaningEditBuffer,
    on_save: WordsMessage,
) -> Element<'a, WordsMessage, AppTheme> {
    let word_input = text_input("Word *", &word_buffer.content)
        .on_input(WordsMessage::EditWordContentChanged)
        .width(iced::Length::Fill);

    let lang_input = text_input("Language (optional)", &word_buffer.language_input)
        .on_input(|s| {
            let parsed = s.trim().parse::<langtag::LangTagBuf>().ok();
            WordsMessage::EditWordLanguageChanged { input: s, parsed }
        })
        .width(iced::Length::Fill);

    Container::new(
        Column::new()
            .spacing(Spacing::DEFAULT.l)
            .push(Text::new(title.clone()).size(FontSize::Heading.px()))
            .push(word_input)
            .push(lang_input)
            .push(
                Row::new()
                    .spacing(Spacing::DEFAULT.s)
                    .push(Button::new("Save").style(button::primary).on_press(on_save))
                    .push(
                        Button::new("Cancel")
                            .style(button::secondary)
                            .on_press(WordsMessage::EditCancelled),
                    ),
            ),
    )
    .padding(Spacing::DEFAULT.l)
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .center_x(iced::Length::Fill)
    .center_y(iced::Length::Fill)
    .into()
}

fn meaning_form<'a>(
    title: String,
    word_content: &str,
    buffer: &'a MeaningEditBuffer,
    on_save: WordsMessage,
    _theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    let def_input = text_input("Definition *", &buffer.definition)
        .on_input(WordsMessage::EditMeaningDefinitionChanged)
        .width(iced::Length::Fill);

    let pos_picker = PickList::new(
        PartOfSpeech::VARIANTS,
        Some(buffer.pos),
        WordsMessage::EditMeaningPosChanged,
    )
    .width(iced::Length::Fixed(140.0))
    .placeholder("POS");

    let cefr_picker = PickList::new(
        CefrLevelOption::VARIANTS,
        Some(CefrLevelOption::from_cefr(buffer.cefr)),
        |option| WordsMessage::EditMeaningCefrChanged(option.to_cefr()),
    )
    .width(iced::Length::Fixed(100.0))
    .placeholder("CEFR");

    Container::new(
        Column::new()
            .spacing(Spacing::DEFAULT.l)
            .push(Text::new(title.clone()).size(FontSize::Heading.px()))
            .push(Text::new(format!("Word: {}", word_content)).size(FontSize::Body.px()))
            .push(def_input)
            .push(
                Row::new()
                    .spacing(Spacing::DEFAULT.m)
                    .push(Text::new("POS:"))
                    .push(pos_picker)
                    .push(Space::new())
                    .push(Text::new("CEFR:"))
                    .push(cefr_picker),
            )
            .push(
                Row::new()
                    .spacing(Spacing::DEFAULT.s)
                    .push(Button::new("Save").style(button::primary).on_press(on_save))
                    .push(
                        Button::new("Cancel")
                            .style(button::secondary)
                            .on_press(WordsMessage::EditCancelled),
                    ),
            ),
    )
    .padding(Spacing::DEFAULT.l)
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .center_x(iced::Length::Fill)
    .center_y(iced::Length::Fill)
    .into()
}

fn word_detail_view<'a>(
    word: &'a crate::models::Word,
    model: &'a Model,
    theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    let meaning_items: Vec<Element<'a, WordsMessage, AppTheme>> = model
        .word_registry
        .iter()
        .find(|(_, w)| w.id == word.id)
        .map(|(_, word)| {
            word.meaning_ids
                .iter()
                .filter_map(|mid| model.meaning_registry.get(*mid))
                .map(|meaning| {
                    Row::new()
                        .push(
                            Button::new(Text::new(&meaning.definition).size(FontSize::Body.px()))
                                .style(button::secondary)
                                .padding(ButtonSize::Medium.to_iced_padding())
                                .on_press(WordsMessage::MeaningSelected(meaning.id)),
                        )
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
        .on_press(WordsMessage::EditWordStarted(word.id));

    let word_content = word.content.clone();
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

fn meaning_detail_view<'a>(
    meaning: &'a crate::models::Meaning,
    word: &'a crate::models::Word,
    model: &'a Model,
    _theme: AppTheme,
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
        .on_press(WordsMessage::EditMeaningStarted(meaning.id));

    let mut column = Column::new()
        .push(
            Row::new()
                .push(Text::new(word.content.clone()).size(FontSize::Heading.px()))
                .push(Space::new())
                .push(edit_btn)
                .push(close_btn)
                .align_y(iced::Alignment::Center),
        )
        .push(iced::widget::rule::horizontal(1))
        .spacing(Spacing::DEFAULT.s)
        .padding(Spacing::DEFAULT.m);

    column = column.push(Text::new(meaning.definition.clone()).size(FontSize::Subtitle.px()));

    let meta_row = Row::new();
    column = column.push(meta_row);

    if !tag_names.is_empty() {
        let tags_text = tag_names.join(", ");
        column =
            column.push(Text::new(format!("Tags: {}", tags_text)).size(FontSize::Footnote.px()));
    }

    if !cloze_items.is_empty() {
        column = column
            .push(iced::widget::rule::horizontal(1))
            .push(Text::new("Clozes").size(FontSize::Body.px()))
            .extend(cloze_items);
    }

    column.into()
}

fn cloze_detail_view<'a>(
    cloze_id: ClozeId,
    cloze: &'a Cloze,
    meaning: &'a crate::models::Meaning,
    word: &'a crate::models::Word,
) -> Element<'a, WordsMessage, AppTheme> {
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
                .push(Text::new(word.content.clone()).size(FontSize::Heading.px()))
                .push(Space::new())
                .push(close_btn)
                .align_y(iced::Alignment::Center),
        )
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new(meaning.definition.clone()).size(FontSize::Body.px()))
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
