//! Form views for creating/editing words and meanings.
//!
//! Used by WordCreating, WordEditing, MeaningCreating, MeaningEditing states.

use crate::models::PartOfSpeech;
use crate::ui::AppTheme;
use crate::ui::theme::{FontSize, Spacing};
use crate::ui::widgets::AdvancedInput;
use crate::ui::words::manager::{MeaningEditBuffer, WordEditBuffer};
use crate::ui::words::message::WordsMessage;
use iced::Element;
use iced::widget::Space;
use iced::widget::{Column, PickList, Row, Text};
use strum::VariantArray;

use super::CefrLevelOption;
use super::{build_footer_row, detail_panel};

pub fn word_form<'a>(
    title: String,
    word_buffer: &'a WordEditBuffer,
    _meaning_buffer: &'a MeaningEditBuffer,
    on_save: WordsMessage,
) -> Element<'a, WordsMessage, AppTheme> {
    let header = Row::new()
        .push(Text::new(title).size(FontSize::Heading.px()))
        .spacing(Spacing::DEFAULT.s);

    let word_input = AdvancedInput::new("Word *")
        .value(&word_buffer.content)
        .on_input(WordsMessage::EditWordContentChanged)
        .width(iced::Length::Fill)
        .padding(Spacing::DEFAULT.s);

    let lang_input = AdvancedInput::new("Language (optional)")
        .value(&word_buffer.language_input)
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
        .push(Element::new(word_input))
        .push(Element::new(lang_input))
        .push(Space::new())
        .push(footer);

    detail_panel(content)
}

pub fn meaning_form<'a>(
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

    let def_input = AdvancedInput::new("Definition *")
        .value(&buffer.definition)
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
        .push(Element::new(def_input))
        .push(meta_row)
        .push(Space::new())
        .push(footer);

    detail_panel(content)
}
