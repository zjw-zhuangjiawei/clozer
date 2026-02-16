//! Queue panel update handler.

use super::message::QueueMessage;
use crate::message::Message;
use crate::state::{Model, QueueState};
use iced::Task;

/// Handles all queue-related messages.
///
/// Returns `Task<Message>` (not `Task<QueueMessage>`) because `QueueProcess`
/// spawns async tasks that produce `Message::QueueGenerationResult`.
pub fn update(message: QueueMessage, model: &mut Model) -> Task<Message> {
    match message {
        QueueMessage::SelectToggle(item_id) => {
            if let Some(item) = model.queue_registry.get_item(item_id) {
                if item.selected {
                    model.queue_registry.deselect(item_id);
                } else {
                    model.queue_registry.select(item_id);
                }
            }
        }
        QueueMessage::SelectAll => {
            model.queue_registry.select_all();
        }
        QueueMessage::DeselectAll => {
            model.queue_registry.deselect_all();
        }
        QueueMessage::Process => {
            let generator = model.generator.generator();
            return QueueState::process(
                &mut model.queue_registry,
                &generator,
                &model.word_registry,
                &model.meaning_registry,
            );
        }
        QueueMessage::ClearCompleted => {
            model.queue_registry.clear_completed();
        }
        QueueMessage::Remove(item_id) => {
            model.queue_registry.remove(item_id);
        }
    }
    Task::none()
}
