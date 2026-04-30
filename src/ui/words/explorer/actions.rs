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
) -> Element<'a, WordsMessage, AppTheme> {
    let meaning_selected_count = words_state.selection.meaning_count();
    let cloze_selected_count = words_state.selection.cloze_count();

    if cloze_selected_count > 0 {
        let selection_info = Text::new(format!("{} clozes selected", cloze_selected_count))
            .size(FontSize::Body.px());

        let export_btn = Button::new(Text::new("Export").size(FontSize::Body.px()))
            .style(button::secondary)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::ExportPlaintext);

        let delete_btn = Button::new(Text::new("Delete Clozes").size(FontSize::Body.px()))
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
        let add_btn = Button::new(Text::new("+ Add Word").size(FontSize::Body.px()))
            .style(button::primary)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::NewWordStarted);

        return Row::new().push(add_btn).spacing(Spacing::DEFAULT.s2).into();
    }

    let selection_info = Text::new(format!("{} meanings selected", meaning_selected_count))
        .size(FontSize::Body.px());

    let tag_btn: Element<'a, WordsMessage, AppTheme> =
        if let Some(dropdown) = words_state.panel.tag_dropdown() {
            match dropdown.target {
                TagDropdownTarget::SelectedMeanings => Row::new()
                    .push(
                        Button::new(Text::new("Add Tag ▾").size(FontSize::Body.px()))
                            .style(button::primary)
                            .padding(ButtonSize::Standard.to_iced_padding()),
                    )
                    .push(build_tag_dropdown(dropdown, model, TagDropdownMode::Batch))
                    .spacing(Spacing::DEFAULT.xxs)
                    .into(),
                _ => Button::new(Text::new("Add Tag").size(FontSize::Body.px()))
                    .style(button::secondary)
                    .padding(ButtonSize::Standard.to_iced_padding())
                    .on_press(WordsMessage::TagBatchDropdownOpened)
                    .into(),
            }
        } else {
            Button::new(Text::new("Add Tag").size(FontSize::Body.px()))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(WordsMessage::TagBatchDropdownOpened)
                .into()
        };

    let queue_btn = Button::new(Text::new("Queue").size(FontSize::Body.px()))
        .style(button::primary)
        .padding(ButtonSize::Standard.to_iced_padding())
        .on_press(WordsMessage::MeaningsQueuedForGeneration);

    let delete_btn = Button::new(Text::new("Delete").size(FontSize::Body.px()))
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
