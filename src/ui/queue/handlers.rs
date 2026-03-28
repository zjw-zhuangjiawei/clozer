//! Queue panel command handlers.
//!
//! Command handlers process messages and update state.

use crate::message::Message;
use crate::state::Model;
use crate::state::process;
use crate::ui::queue::message::{QueueActionMessage, QueueMessage, QueueSelectionMessage};
use crate::ui::queue::QueueState;
use iced::Task;
use uuid::Uuid;

/// Handle selection-related messages.
pub fn selection(
    state: &mut QueueState,
    message: QueueSelectionMessage,
    model: &Model,
) {
    match message {
        QueueSelectionMessage::Toggle(item_id) => {
            if let Some(item) = model.queue_registry.get_item(item_id) {
                if item.selected {
                    model.queue_registry.deselect(item_id);
                } else {
                    model.queue_registry.select(item_id);
                }
            }
        }
        QueueSelectionMessage::SelectAll => {
            model.queue_registry.select_all();
        }
        QueueSelectionMessage::DeselectAll => {
            model.queue_registry.deselect_all();
        }
    }
}

/// Handle action-related messages.
pub fn action(
    state: &mut QueueState,
    message: QueueActionMessage,
    model: &mut Model,
) -> Task<Message> {
    match message {
        QueueActionMessage::Process => {
            let generator = model.generator.generator();
            return process(
                &mut model.queue_registry,
                &generator,
                &model.word_registry,
                &model.meaning_registry,
            );
        }
        QueueActionMessage::ClearCompleted => {
            model.queue_registry.clear_completed();
        }
        QueueActionMessage::Remove(item_id) => {
            model.queue_registry.remove(item_id);
        }
    }
    Task::none()
}

/// Handle all queue-related messages.
///
/// Returns `Task<Message>` because `Process` spawns async tasks
/// that produce `Message::QueueGenerationResult`.
pub fn update(
    state: &mut QueueState,
    message: QueueMessage,
    model: &mut Model,
) -> Task<Message> {
    use QueueMessage::*;
    match message {
        Selection(msg) => selection(state, msg, model),
        Action(msg) => return action(state, msg, model),
    }
    Task::none()
}
