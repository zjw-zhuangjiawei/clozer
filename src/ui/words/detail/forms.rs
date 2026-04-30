use crate::models::PartOfSpeech;
use crate::ui::AppTheme;
use crate::ui::theme::{FontSize, Spacing};
use crate::ui::widgets::AdvancedInput;
use crate::ui::widgets::text as txt;
use crate::ui::words::manager::{MeaningEditBuffer, WordEditBuffer};
use crate::ui::words::message::WordsMessage;
use iced::Element;
use iced::widget::Space;
use iced::widget::{Button, Column, PickList, Row, Text};
use strum::VariantArray;

use super::CefrLevelOption;
use super::{build_footer_row, detail_panel};

fn validation_text_style(
    is_empty: bool,
    is_valid: bool,
) -> impl Fn(&AppTheme) -> iced::widget::text::Style {
    move |theme: &AppTheme| {
        let colors = theme.colors();
        let color = if is_empty {
            colors.semantic.text.tertiary
        } else if is_valid {
            colors.functional.success.w600()
        } else {
            colors.semantic.text.error
        };
        iced::widget::text::Style { color: Some(color) }
    }
}

pub fn word_form<'a>(
    title: String,
    word_buffer: &'a WordEditBuffer,
    _meaning_buffer: &'a MeaningEditBuffer,
    on_save: WordsMessage,
) -> Element<'a, WordsMessage, AppTheme> {
    let is_lang_empty = word_buffer.language_input.is_empty();
    let is_lang_valid = word_buffer.language.is_some();

    let (validation_icon, lang_status) = if is_lang_valid {
        (" \u{2713}", "Valid BCP 47 tag")
    } else if !is_lang_empty {
        (" \u{26A0}", "Not a valid language tag")
    } else {
        ("", "e.g. en, zh-CN, fr")
    };

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

    let lang_row = Row::new()
        .push(Element::new(lang_input))
        .push(
            Text::new(validation_icon)
                .size(FontSize::Body.px())
                .style(validation_text_style(is_lang_empty, is_lang_valid)),
        )
        .align_y(iced::Alignment::Center)
        .spacing(Spacing::DEFAULT.xs);

    let quick_langs: &[(&str, &str)] = &[
        ("en", "EN"),
        ("zh", "\u{4E2D}\u{6587}"),
        ("ja", "\u{65E5}\u{672C}\u{8A9E}"),
        ("ko", "\u{D55C}\u{AD6D}\u{C5B4}"),
        ("fr", "FR"),
        ("de", "DE"),
        ("es", "ES"),
        ("pt", "PT"),
    ];

    let quick_buttons: Vec<Element<'a, WordsMessage, AppTheme>> = quick_langs
        .iter()
        .map(|&(tag, label)| {
            let tag_owned = tag.to_string();
            Button::new(Text::new(label).size(FontSize::Caption.px()))
                .style(if word_buffer.language_input == tag {
                    crate::ui::widgets::button::primary
                } else {
                    crate::ui::widgets::button::secondary
                })
                .padding([2, 8])
                .on_press(WordsMessage::EditWordLanguageChanged {
                    input: tag_owned.clone(),
                    parsed: tag_owned.parse::<langtag::LangTagBuf>().ok(),
                })
                .into()
        })
        .collect();

    let quick_row = Row::new()
        .spacing(Spacing::DEFAULT.xs)
        .push(
            Text::new("Quick:")
                .size(FontSize::Caption.px())
                .style(txt::tertiary),
        )
        .extend(quick_buttons)
        .align_y(iced::Alignment::Center);

    let footer = build_footer_row("Save", on_save, WordsMessage::EditCancelled);

    let content = Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(header)
        .push(Element::new(word_input))
        .push(lang_row)
        .push(quick_row)
        .push(
            Text::new(lang_status)
                .size(FontSize::Caption.px())
                .style(validation_text_style(is_lang_empty, is_lang_valid)),
        )
        .push(Space::new())
        .push(footer);

    detail_panel(content)
}

pub fn meaning_form<'a>(
    title: String,
    word_content: &str,
    buffer: &'a MeaningEditBuffer,
    on_save: WordsMessage,
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
