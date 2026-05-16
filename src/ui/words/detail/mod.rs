//! Detail panel views — orchestration and shared helpers.
//!
//! Dispatches to form views or detail panel views based on DetailPanelState.

mod forms;
mod panels;

pub(super) use self::forms::{meaning_form, word_form};
pub(super) use self::panels::{cloze_detail_view, meaning_detail_view, word_detail_view};

use crate::assets;
use crate::i18n::I18nManager;
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

fn try_meaning_view<'a>(
    meaning_id: crate::models::types::MeaningId,
    model: &'a Model,
    i18n: &'a I18nManager,
) -> Option<Element<'a, WordsMessage, AppTheme>> {
    let meaning = model.meaning_registry.get(meaning_id)?;
    let word = model.word_registry.get(meaning.word_id)?;
    Some(meaning_detail_view(meaning, word, model, i18n))
}

fn try_cloze_view<'a>(
    cloze_id: crate::models::types::ClozeId,
    model: &'a Model,
    i18n: &'a I18nManager,
) -> Option<Element<'a, WordsMessage, AppTheme>> {
    let cloze = model.cloze_registry.get(cloze_id)?;
    let meaning = model.meaning_registry.get(cloze.meaning_id)?;
    let word = model.word_registry.get(meaning.word_id)?;
    Some(cloze_detail_view(cloze_id, cloze, meaning, word, i18n))
}

pub fn view<'a>(
    state: &'a DetailPanelState,
    word_buffer: &'a WordEditBuffer,
    meaning_buffer: &'a MeaningEditBuffer,
    dictionary_loading: bool,
    dictionary_result: &'a Option<crate::dictionary::DictionaryEntry>,
    model: &'a Model,
    i18n: &'a I18nManager,
) -> Element<'a, WordsMessage, AppTheme> {
    match state {
        DetailPanelState::Empty => placeholder_view(),

        DetailPanelState::WordView { word_id } => model
            .word_registry
            .get(*word_id)
            .map(|word| word_detail_view(word, model, i18n))
            .unwrap_or_else(placeholder_view),

        DetailPanelState::MeaningView { meaning_id } => {
            try_meaning_view(*meaning_id, model, i18n).unwrap_or_else(placeholder_view)
        }

        DetailPanelState::ClozeView { cloze_id } => {
            try_cloze_view(*cloze_id, model, i18n).unwrap_or_else(placeholder_view)
        }

        DetailPanelState::WordCreating => word_form(
            i18n.tr("words-add-new-word").to_string(),
            word_buffer,
            meaning_buffer,
            WordsMessage::NewWordSaved,
            i18n,
        ),

        DetailPanelState::WordEditing { .. } => word_form(
            i18n.tr("words-edit-word").to_string(),
            word_buffer,
            meaning_buffer,
            WordsMessage::EditSaved,
            i18n,
        ),

        DetailPanelState::MeaningCreating { word_id, .. } => {
            let word_content = model
                .word_registry
                .get(*word_id)
                .map(|w| w.content.clone())
                .unwrap_or_default();
            let title = i18n.tr_with("words-add-meaning-to", &[&word_content]);
            meaning_form(
                title,
                &word_content,
                meaning_buffer,
                dictionary_loading,
                dictionary_result,
                WordsMessage::MeaningAddSaved,
                i18n,
            )
        }

        DetailPanelState::MeaningEditing { meaning_id, .. } => {
            if let Some(meaning) = model.meaning_registry.get(*meaning_id) {
                let word_content = model
                    .word_registry
                    .get(meaning.word_id)
                    .map(|w| w.content.clone())
                    .unwrap_or_default();
                let title = i18n.tr("words-edit-meaning").to_string();
                meaning_form(
                    title,
                    &word_content,
                    meaning_buffer,
                    dictionary_loading,
                    dictionary_result,
                    WordsMessage::EditSaved,
                    i18n,
                )
            } else {
                placeholder_view()
            }
        }
    }
}

fn placeholder_view<'a>() -> Element<'a, WordsMessage, AppTheme> {
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
    match assets::get_svg(name) {
        Some(data) => iced::widget::svg(iced::widget::svg::Handle::from_memory(data))
            .width(iced::Length::Fixed(size))
            .height(iced::Length::Fixed(size))
            .into(),
        None => {
            tracing::warn!(
                asset = %name,
                "SVG icon not found, using fallback text"
            );
            iced::widget::Text::new("?")
                .size(size)
                .width(iced::Length::Fixed(size))
                .height(iced::Length::Fixed(size))
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center)
                .into()
        }
    }
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
    on_primary: WordsMessage,
    on_cancel: WordsMessage,
    i18n: &I18nManager,
) -> Row<'a, WordsMessage, AppTheme> {
    Row::new()
        .spacing(Spacing::DEFAULT.s)
        .push(
            Button::new(Text::new(i18n.tr("words-save")).size(FontSize::Body.px()))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(on_primary),
        )
        .push(
            Button::new(Text::new(i18n.tr("words-cancel")).size(FontSize::Body.px()))
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
