use crate::message::Message;
use crate::models::{Cloze, WordId};
use crate::registry::{QueueItemStatus, QueueRegistry, WordRegistry};
use crate::state::generator::Generator;
use iced::Task;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum QueueGenerationResult {
    Success { item_id: WordId, cloze: Cloze },
    Failed { item_id: WordId, error: String },
}

/// Process pending queue items using the LLM generator.
/// Returns an empty task if no generator is available.
pub fn process(
    queue_registry: &mut QueueRegistry,
    generator: &Option<Arc<Generator>>,
    word_registry: &WordRegistry,
    meaning_registry: &crate::registry::MeaningRegistry,
) -> Task<Message> {
    let Some(generator) = generator else {
        tracing::warn!("No generator available — cannot process queue");
        return Task::none();
    };

    let items: Vec<_> = queue_registry
        .get_items()
        .filter(|item| item.status == QueueItemStatus::Pending)
        .cloned()
        .collect();

    let count = items.len();
    tracing::info!("Processing queue: {} pending items", count);

    // Mark items as processing before spawning tasks
    for item in &items {
        queue_registry.set_processing(item.id);
    }

    let tasks = items.into_iter().map(|item| {
        let meaning = match meaning_registry.get(item.meaning_id) {
            Some(m) => m.clone(),
            None => {
                tracing::error!(meaning_id = %item.meaning_id, "Meaning not found for queue item");
                queue_registry.set_failed(item.id, "Meaning not found".to_string());
                return None;
            }
        };
        let word = match word_registry.get(meaning.word_id) {
            Some(w) => w.clone(),
            None => {
                tracing::error!(word_id = %meaning.word_id, "Word not found for meaning");
                queue_registry.set_failed(item.id, "Word not found".to_string());
                return None;
            }
        };
        let generator = Arc::clone(generator);
        let item_id = item.id;

        Some(Task::perform(
            async move {
                match generator.generate(&word, &meaning).await {
                    Ok(cloze) => QueueGenerationResult::Success { item_id, cloze },
                    Err(e) => QueueGenerationResult::Failed {
                        item_id,
                        error: e.to_string(),
                    },
                }
            },
            Message::QueueGenerationResult,
        ))
    });

    Task::batch(tasks.flatten())
}
