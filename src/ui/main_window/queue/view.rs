//! Queue panel view function.

use super::message::QueueMessage;
use crate::registry::QueueItemStatus;
use crate::state::Model;
use crate::ui::components::svg_checkbox;
use iced::Element;
use iced::widget::{Button, Column, Row, Text, button, text_input};

fn generator_settings_panel<'a>() -> Element<'a, QueueMessage> {
    let title = Text::new("Generator Settings").size(16);
    let include_word_checkbox = Text::new("☐ Include word in sentence").size(14);
    let template_dropdown =
        text_input::TextInput::new("Template mode...", "Template mode (placeholder)")
            .padding(8)
            .size(14);

    Column::new()
        .push(title)
        .push(include_word_checkbox)
        .push(template_dropdown)
        .spacing(8)
        .padding(10)
        .into()
}

fn status_label(status: &QueueItemStatus) -> String {
    match status {
        QueueItemStatus::Pending => "Pending".to_string(),
        QueueItemStatus::Processing => "Processing...".to_string(),
        QueueItemStatus::Completed => "Done".to_string(),
        QueueItemStatus::Failed(err) => format!("Failed: {}", err),
    }
}

fn meaning_content(
    meaning_id: uuid::Uuid,
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

pub fn view<'a>(model: &'a Model) -> Element<'a, QueueMessage> {
    let queue_registry = &model.queue_registry;
    let meaning_registry = &model.meaning_registry;
    let word_registry = &model.word_registry;

    let items: Vec<Element<'a, QueueMessage>> = queue_registry
        .get_items()
        .map(|queue_item| {
            let content = meaning_content(queue_item.meaning_id, meaning_registry, word_registry);
            let item_id = queue_item.id;
            let selected = queue_item.selected;
            let status = queue_item.status.clone();
            let status_text = status_label(&status);
            let status_text_for_row = status_text.clone();

            let select_indicator: Element<'a, QueueMessage> =
                if matches!(status, QueueItemStatus::Pending) {
                    svg_checkbox(selected, QueueMessage::SelectToggle(item_id))
                } else {
                    Text::new(status_text).into()
                };

            let remove_btn = Button::new(Text::new("remove"))
                .style(button::secondary)
                .padding([2, 6])
                .on_press(QueueMessage::Remove(item_id));

            Row::new()
                .push(select_indicator)
                .push(Text::new(content).width(iced::Length::Fill))
                .push(Text::new(status_text_for_row).size(12))
                .push(remove_btn)
                .spacing(10)
                .align_y(iced::Alignment::Center)
                .into()
        })
        .collect();

    let queue_column = Column::with_children(items).spacing(5);

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
            Button::new(Text::new("Select All"))
                .style(button::secondary)
                .padding([8, 16])
                .on_press(QueueMessage::SelectAll),
        )
        .push(
            Button::new(Text::new("Select None"))
                .style(button::secondary)
                .padding([8, 16])
                .on_press(QueueMessage::DeselectAll),
        )
        .spacing(5);

    let has_completed = queue_registry
        .get_items()
        .any(|i| matches!(i.status, QueueItemStatus::Completed));

    let clear_button: Element<'a, QueueMessage> = if has_completed {
        Button::new(Text::new("Clear Done"))
            .style(button::secondary)
            .padding([8, 16])
            .on_press(QueueMessage::ClearCompleted)
            .into()
    } else {
        Text::new("").into()
    };

    let process_button = Button::new(Text::new(format!("Process ({})", selected_count)))
        .on_press_maybe((selected_count > 0).then_some(QueueMessage::Process))
        .width(iced::Length::Fill)
        .style(if selected_count > 0 {
            button::primary
        } else {
            button::secondary
        })
        .padding([12, 16]);

    Column::new()
        .push(Text::new("Queue").size(24))
        .push(
            Text::new(format!(
                "Total: {} | Pending: {} | Selected: {}",
                total, pending, selected_count
            ))
            .size(12),
        )
        .push(iced::widget::rule::horizontal(1))
        .push(select_buttons)
        .push(clear_button)
        .push(iced::widget::scrollable(queue_column).height(iced::Length::Fill))
        .push(generator_settings_panel())
        .push(process_button)
        .spacing(10)
        .padding(10)
        .height(iced::Length::Fill)
        .into()
}
