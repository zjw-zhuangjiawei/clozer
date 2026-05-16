use crate::i18n::I18nManager;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::button;
use crate::ui::words::manager::TagDropdownTarget;
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::WordsState;
use iced::Element;
use iced::widget::{Button, Row, Space, Text};

use super::tags::{TagDropdownMode, build_tag_dropdown};

pub fn build_action_bar<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    i18n: &'a I18nManager,
) -> Element<'a, WordsMessage, AppTheme> {
    let meaning_selected_count = words_state.selection.meaning_count();
    let cloze_selected_count = words_state.selection.cloze_count();

    if cloze_selected_count > 0 {
        let selection_info = Text::new(i18n.tr_with(
            "words-clozes-selected",
            &[&cloze_selected_count.to_string()],
        ))
        .size(FontSize::Body.px());

        let export_btn = Button::new(Text::new(i18n.tr("words-export")).size(FontSize::Body.px()))
            .style(button::secondary)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::ExportPlaintext);

        let delete_btn =
            Button::new(Text::new(i18n.tr("words-delete-clozes")).size(FontSize::Body.px()))
                .style(button::danger)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(WordsMessage::ClozesDeleted);

        return Row::new()
            .push(selection_info)
            .push(Space::new())
            .push(export_btn)
            .push(delete_btn)
            .spacing(Spacing::DEFAULT.s2)
            .align_y(iced::Alignment::Center)
            .into();
    }

    if meaning_selected_count == 0 {
        let add_btn = Button::new(Text::new(i18n.tr("words-add-word")).size(FontSize::Body.px()))
            .style(button::primary)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::NewWordStarted);

        return Row::new().push(add_btn).spacing(Spacing::DEFAULT.s2).into();
    }

    let selection_info = Text::new(i18n.tr_with(
        "words-meanings-selected",
        &[&meaning_selected_count.to_string()],
    ))
    .size(FontSize::Body.px());

    let tag_open_label = i18n.tr("words-add-tag-open");
    let tag_label = i18n.tr("words-add-tag");
    let tag_btn: Element<'a, WordsMessage, AppTheme> =
        if let Some(dropdown) = words_state.panel.tag_dropdown() {
            match dropdown.target {
                TagDropdownTarget::SelectedMeanings => Row::new()
                    .push(
                        Button::new(Text::new(tag_open_label).size(FontSize::Body.px()))
                            .style(button::primary)
                            .padding(ButtonSize::Standard.to_iced_padding()),
                    )
                    .push(build_tag_dropdown(
                        dropdown,
                        model,
                        TagDropdownMode::Batch,
                        i18n,
                    ))
                    .spacing(Spacing::DEFAULT.xxs)
                    .into(),
                _ => Button::new(Text::new(tag_label.clone()).size(FontSize::Body.px()))
                    .style(button::secondary)
                    .padding(ButtonSize::Standard.to_iced_padding())
                    .on_press(WordsMessage::TagBatchDropdownOpened)
                    .into(),
            }
        } else {
            Button::new(Text::new(tag_label).size(FontSize::Body.px()))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(WordsMessage::TagBatchDropdownOpened)
                .into()
        };

    let queue_btn = Button::new(Text::new(i18n.tr("words-queue")).size(FontSize::Body.px()))
        .style(button::primary)
        .padding(ButtonSize::Standard.to_iced_padding())
        .on_press(WordsMessage::MeaningsQueuedForGeneration);

    let delete_btn =
        Button::new(Text::new(i18n.tr("words-delete-meanings-batch")).size(FontSize::Body.px()))
            .style(button::danger)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::MeaningsDeleted);

    Row::new()
        .push(selection_info)
        .push(Space::new())
        .push(tag_btn)
        .push(queue_btn)
        .push(delete_btn)
        .spacing(Spacing::DEFAULT.s2)
        .align_y(iced::Alignment::Center)
        .into()
}
