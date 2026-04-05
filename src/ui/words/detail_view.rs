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
use iced::widget::{Button, Column, Container, PickList, Row, Text, TextInput, container};
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

// === Helper Functions ===

fn detail_panel<'a>(
    content: Column<'a, WordsMessage, AppTheme>,
) -> Element<'a, WordsMessage, AppTheme> {
    Container::new(content)
        .padding(Spacing::DEFAULT.l)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(|_| container::Style {
            ..Default::default()
        })
        .into()
}

fn build_edit_icon() -> Element<'static, WordsMessage, AppTheme> {
    let handle = assets::get_svg("edit_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(iced::widget::svg::Handle::from_memory)
        .unwrap_or_else(|| iced::widget::svg::Handle::from_memory(Vec::new()));
    iced::widget::svg(handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0))
        .into()
}

fn build_delete_icon() -> Element<'static, WordsMessage, AppTheme> {
    let handle = assets::get_svg("delete_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(iced::widget::svg::Handle::from_memory)
        .unwrap_or_else(|| iced::widget::svg::Handle::from_memory(Vec::new()));
    iced::widget::svg(handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0))
        .into()
}

fn build_close_icon() -> Element<'static, WordsMessage, AppTheme> {
    let handle = assets::get_svg("close_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(iced::widget::svg::Handle::from_memory)
        .unwrap_or_else(|| iced::widget::svg::Handle::from_memory(Vec::new()));
    iced::widget::svg(handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0))
        .into()
}

fn build_header_row<'a>(
    title: String,
    edit_action: Option<WordsMessage>,
    close_action: WordsMessage,
) -> Row<'a, WordsMessage, AppTheme> {
    let title_text = Text::new(title).size(FontSize::Heading.px());

    let close_btn = Button::new(build_close_icon())
        .style(button::secondary)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(close_action);

    let mut row = Row::new()
        .push(title_text)
        .push(Space::new().width(iced::Length::Fill))
        .spacing(Spacing::DEFAULT.s);

    if let Some(msg) = edit_action {
        let edit_btn = Button::new(build_edit_icon())
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

// === Form Views ===

fn word_form<'a>(
    title: String,
    word_buffer: &'a WordEditBuffer,
    _meaning_buffer: &'a MeaningEditBuffer,
    on_save: WordsMessage,
) -> Element<'a, WordsMessage, AppTheme> {
    let header = Row::new()
        .push(Text::new(title).size(FontSize::Heading.px()))
        .spacing(Spacing::DEFAULT.s);

    let word_input = TextInput::new("Word *", &word_buffer.content)
        .on_input(WordsMessage::EditWordContentChanged)
        .width(iced::Length::Fill)
        .padding(Spacing::DEFAULT.s);

    let lang_input = TextInput::new("Language (optional)", &word_buffer.language_input)
        .on_input(|s| {
            let parsed = s.trim().parse::<langtag::LangTagBuf>().ok();
            WordsMessage::EditWordLanguageChanged { input: s, parsed }
        })
        .width(iced::Length::Fill)
        .padding(Spacing::DEFAULT.s);

    let footer = build_footer_row("Save", on_save, WordsMessage::EditCancelled);

    let content = Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(header)
        .push(word_input)
        .push(lang_input)
        .push(Space::new())
        .push(footer);

    detail_panel(content)
}

fn meaning_form<'a>(
    title: String,
    word_content: &str,
    buffer: &'a MeaningEditBuffer,
    on_save: WordsMessage,
    _theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    let header = Row::new()
        .push(Text::new(title).size(FontSize::Heading.px()))
        .spacing(Spacing::DEFAULT.s);

    let word_label = Text::new(format!("Word: {}", word_content)).size(FontSize::Body.px());

    let def_input = TextInput::new("Definition *", &buffer.definition)
        .on_input(WordsMessage::EditMeaningDefinitionChanged)
        .width(iced::Length::Fill)
        .padding(Spacing::DEFAULT.s);

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

    let meta_row = Row::new()
        .spacing(Spacing::DEFAULT.s)
        .push(Text::new("POS:").size(FontSize::Body.px()))
        .push(pos_picker)
        .push(Space::new().width(iced::Length::Fill))
        .push(Text::new("CEFR:").size(FontSize::Body.px()))
        .push(cefr_picker)
        .align_y(iced::Alignment::Center);

    let footer = build_footer_row("Save", on_save, WordsMessage::EditCancelled);

    let content = Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(header)
        .push(word_label)
        .push(def_input)
        .push(meta_row)
        .push(Space::new())
        .push(footer);

    detail_panel(content)
}

// === Detail Views ===

fn word_detail_view<'a>(
    word: &'a crate::models::Word,
    model: &'a Model,
    _theme: AppTheme,
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
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new("Meanings").size(FontSize::Body.px()))
        .extend(meaning_items);

    detail_panel(content)
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

    let header = build_header_row(
        word.content.clone(),
        Some(WordsMessage::EditMeaningStarted(meaning.id)),
        WordsMessage::DetailClosed,
    );

    let mut content = Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(header)
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new(meaning.definition.clone()).size(FontSize::Subtitle.px()));

    if !tag_names.is_empty() {
        let tags_text = tag_names.join(", ");
        content =
            content.push(Text::new(format!("Tags: {}", tags_text)).size(FontSize::Footnote.px()));
    }

    if !cloze_items.is_empty() {
        content = content
            .push(iced::widget::rule::horizontal(1))
            .push(Text::new("Clozes").size(FontSize::Body.px()))
            .extend(cloze_items);
    }

    detail_panel(content)
}

fn cloze_detail_view<'a>(
    cloze_id: ClozeId,
    cloze: &'a Cloze,
    meaning: &'a crate::models::Meaning,
    word: &'a crate::models::Word,
) -> Element<'a, WordsMessage, AppTheme> {
    let header = build_header_row(word.content.clone(), None, WordsMessage::DetailClosed);

    let delete_btn = build_icon_button(
        build_delete_icon(),
        button::danger,
        WordsMessage::ClozeDeleted(cloze_id),
    );

    let content = Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(header)
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new(meaning.definition.clone()).size(FontSize::Body.px()))
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new("Cloze Sentence").size(FontSize::Body.px()))
        .push(Text::new(cloze.render_blanks()).size(FontSize::Subtitle.px()))
        .push(iced::widget::rule::horizontal(1))
        .push(Text::new("Answer").size(FontSize::Body.px()))
        .push(Text::new(cloze.render_answers()).size(FontSize::Subtitle.px()))
        .push(iced::widget::rule::horizontal(1))
        .push(delete_btn);

    detail_panel(content)
}
