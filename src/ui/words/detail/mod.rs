//! Detail panel views — orchestration and shared helpers.
//!
//! Dispatches to form views or detail panel views based on DetailPanelState.

mod forms;
mod panels;

pub(super) use self::forms::{meaning_form, word_form};
pub(super) use self::panels::{cloze_detail_view, meaning_detail_view, word_detail_view};

use crate::assets;
use crate::models::CefrLevel;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::button;
use crate::ui::widgets::container::card;
use crate::ui::words::manager::{DetailPanelState, MeaningEditBuffer, WordEditBuffer};
use crate::ui::words::message::WordsMessage;
use iced::Element;
use iced::widget::Space;
use iced::widget::{Button, Column, Container, Row, Text};

/// PickList adapter for optional CEFR levels.
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
    dictionary_loading: bool,
    dictionary_result: &'a Option<crate::dictionary::DictionaryEntry>,
    model: &'a Model,
) -> Element<'a, WordsMessage, AppTheme> {
    match state {
        DetailPanelState::Empty => placeholder_view(),

        DetailPanelState::WordView { word_id } => {
            if let Some(word) = model.word_registry.get(*word_id) {
                word_detail_view(word, model)
            } else {
                placeholder_view()
            }
        }

        DetailPanelState::MeaningView { meaning_id } => {
            if let Some(meaning) = model.meaning_registry.get(*meaning_id) {
                if let Some(word) = model.word_registry.get(meaning.word_id) {
                    meaning_detail_view(meaning, word, model)
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

        DetailPanelState::WordCreating => word_form(
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
                dictionary_loading,
                dictionary_result,
                WordsMessage::MeaningAddSaved,
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
                    dictionary_loading,
                    dictionary_result,
                    WordsMessage::EditSaved,
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

fn detail_panel<'a>(
    content: Column<'a, WordsMessage, AppTheme>,
) -> Element<'a, WordsMessage, AppTheme> {
    Container::new(content)
        .padding(Spacing::DEFAULT.l)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(card)
        .into()
}

fn build_svg_icon(name: &str, size: f32) -> Element<'static, WordsMessage, AppTheme> {
    let handle = assets::get_svg(name)
        .map(iced::widget::svg::Handle::from_memory)
        .unwrap_or_else(|| iced::widget::svg::Handle::from_memory(Vec::new()));
    iced::widget::svg(handle)
        .width(iced::Length::Fixed(size))
        .height(iced::Length::Fixed(size))
        .into()
}

fn build_header_row<'a>(
    title: String,
    edit_action: Option<WordsMessage>,
    close_action: WordsMessage,
) -> Row<'a, WordsMessage, AppTheme> {
    let title_text = Text::new(title).size(FontSize::Heading.px());

    let close_icon = build_svg_icon("close_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg", 16.0);
    let close_btn = Button::new(close_icon)
        .style(button::secondary)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(close_action);

    let mut row = Row::new()
        .push(title_text)
        .push(Space::new().width(iced::Length::Fill))
        .spacing(Spacing::DEFAULT.s);

    if let Some(msg) = edit_action {
        let edit_icon = build_svg_icon("edit_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg", 16.0);
        let edit_btn = Button::new(edit_icon)
            .style(button::secondary)
            .padding(ButtonSize::Small.to_iced_padding())
            .on_press(msg);
        row = row.push(edit_btn);
    }

    row.push(close_btn).align_y(iced::Alignment::Center)
}

fn build_footer_row<'a>(
    primary_label: &'a str,
    on_primary: WordsMessage,
    on_cancel: WordsMessage,
) -> Row<'a, WordsMessage, AppTheme> {
    Row::new()
        .spacing(Spacing::DEFAULT.s)
        .push(
            Button::new(Text::new(primary_label).size(FontSize::Body.px()))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(on_primary),
        )
        .push(
            Button::new(Text::new("Cancel").size(FontSize::Body.px()))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(on_cancel),
        )
}

fn build_icon_button<'a>(
    icon: Element<'a, WordsMessage, AppTheme>,
    style: fn(&AppTheme, iced::widget::button::Status) -> iced::widget::button::Style,
    on_press: WordsMessage,
) -> Button<'a, WordsMessage, AppTheme> {
    Button::new(icon)
        .style(style)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(on_press)
}
