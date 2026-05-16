//! Queue panel view function.

use super::message::{QueueActionMessage, QueueMessage, QueueSelectionMessage};
use crate::i18n::I18nManager;
use crate::models::types::MeaningId;
use crate::registry::QueueItemStatus;
use crate::state::Model;
use crate::ui::theme::{AppTheme, ButtonSize, FontSize, Spacing};
use crate::ui::widgets::svg_checkbox;
use iced::Element;
use iced::widget::{Button, Column, Row, Text};

use crate::ui::widgets::button;

fn status_label(status: &QueueItemStatus, i18n: &I18nManager) -> String {
    match status {
        QueueItemStatus::Pending => i18n.tr("queue-pending").to_string(),
        QueueItemStatus::Processing => i18n.tr("queue-processing").to_string(),
        QueueItemStatus::Completed => i18n.tr("queue-done").to_string(),
        QueueItemStatus::Failed(err) => i18n.tr_with("queue-failed", &[err]),
    }
}

fn meaning_content(
    meaning_id: MeaningId,
    meaning_registry: &crate::registry::MeaningRegistry,
    word_registry: &crate::registry::WordRegistry,
) -> String {
    if let Some(meaning) = meaning_registry.get(meaning_id) {
        if let Some(word) = word_registry.get(meaning.word_id) {
            format!("{} - {}: {}", word.content, meaning.pos, meaning.definition)
        } else {
            format!("<word: {}>", meaning.word_id)
        }
    } else {
        format!("<meaning: {}>", meaning_id)
    }
}

pub fn view<'a>(model: &'a Model, i18n: &'a I18nManager) -> Element<'a, QueueMessage, AppTheme> {
    let queue_registry = &model.queue_registry;
    let meaning_registry = &model.meaning_registry;
    let word_registry = &model.word_registry;

    let remove_label = i18n.tr("queue-remove");

    let items: Vec<Element<'a, QueueMessage, AppTheme>> = queue_registry
        .get_items()
        .map(|queue_item| {
            let content = meaning_content(queue_item.meaning_id, meaning_registry, word_registry);
            let item_id = queue_item.id;
            let selected = queue_item.selected;
            let status = queue_item.status.clone();
            let status_text = status_label(&status, i18n);
            let status_text_for_row = status_text.clone();

            let select_indicator: Element<'a, QueueMessage, AppTheme> =
                if matches!(status, QueueItemStatus::Pending) {
                    svg_checkbox(
                        selected,
                        QueueMessage::Selection(QueueSelectionMessage::Toggle(item_id)),
                    )
                } else {
                    Text::new(status_text).into()
                };

            let remove_btn = Button::new(Text::new(remove_label.clone()))
                .style(crate::ui::widgets::button::secondary)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(QueueMessage::Action(QueueActionMessage::Remove(item_id)));

            Row::new()
                .push(select_indicator)
                .push(Text::new(content).width(iced::Length::Fill))
                .push(Text::new(status_text_for_row).size(FontSize::Footnote.px()))
                .push(remove_btn)
                .spacing(Spacing::DEFAULT.s)
                .align_y(iced::Alignment::Center)
                .into()
        })
        .collect();

    let queue_column = Column::with_children(items).spacing(Spacing::DEFAULT.xs);

    let total = queue_registry.len();
    let pending = queue_registry
        .get_items()
        .filter(|i| matches!(i.status, QueueItemStatus::Pending))
        .count();
    let selected_count = queue_registry
        .get_items()
        .filter(|i| i.selected && matches!(i.status, QueueItemStatus::Pending))
        .count();

    let select_buttons = Row::new()
        .push(
            Button::new(Text::new(i18n.tr("queue-select-all")))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(QueueMessage::Selection(QueueSelectionMessage::SelectAll)),
        )
        .push(
            Button::new(Text::new(i18n.tr("queue-select-none")))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(QueueMessage::Selection(QueueSelectionMessage::DeselectAll)),
        )
        .spacing(Spacing::DEFAULT.xs);

    let has_completed = queue_registry
        .get_items()
        .any(|i| matches!(i.status, QueueItemStatus::Completed));

    let clear_button: Element<'a, QueueMessage, AppTheme> = if has_completed {
        Button::new(Text::new(i18n.tr("queue-clear-done")))
            .style(button::danger)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(QueueMessage::Action(QueueActionMessage::ClearCompleted))
            .into()
    } else {
        Text::new("").into()
    };

    let process_label = i18n.tr_with("queue-process", &[&selected_count.to_string()]);
    let process_button = Button::new(Text::new(process_label))
        .on_press_maybe(
            (selected_count > 0).then_some(QueueMessage::Action(QueueActionMessage::Process)),
        )
        .width(iced::Length::Fill)
        .style(button::primary)
        .padding(ButtonSize::Large.to_iced_padding());

    let status_text = i18n.tr_with(
        "queue-status",
        &[
            &total.to_string(),
            &pending.to_string(),
            &selected_count.to_string(),
        ],
    );

    Column::new()
        .push(Text::new(i18n.tr("queue-title")).size(FontSize::Display.px()))
        .push(Text::new(status_text).size(FontSize::Footnote.px()))
        .push(iced::widget::rule::horizontal(1))
        .push(select_buttons)
        .push(clear_button)
        .push(iced::widget::scrollable(queue_column).height(iced::Length::Fill))
        .push(process_button)
        .spacing(Spacing::DEFAULT.s)
        .padding(Spacing::DEFAULT.s)
        .height(iced::Length::Fill)
        .into()
}
