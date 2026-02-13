use crate::message::Message;
use crate::models::Cloze;
use crate::registry::{QueueItemStatus, QueueRegistry, WordRegistry};
use crate::state::generator::Generator;
use iced::Task;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct QueueGenerationResult {
    pub item_id: Uuid,
    pub cloze: Cloze,
}

#[derive(Debug, Clone)]
pub struct QueueState {
    pub queue_registry: QueueRegistry,
}

impl Default for QueueState {
    fn default() -> Self {
        Self::new()
    }
}

impl QueueState {
    pub fn new() -> Self {
        Self {
            queue_registry: QueueRegistry::new(),
        }
    }

    pub fn process(
        &mut self,
        generator: &Arc<Generator>,
        word_registry: &WordRegistry,
        meaning_registry: &crate::registry::MeaningRegistry,
    ) -> Task<Message> {
        let items: Vec<_> = self
            .queue_registry
            .get_items()
            .filter(|item| item.status == QueueItemStatus::Pending)
            .cloned()
            .collect();

        let count = items.len();
        tracing::info!("Processing queue: {} pending items", count);

        let tasks = items.into_iter().map(|item| {
            let meaning = meaning_registry.get_by_id(item.meaning_id).unwrap().clone();
            let word = word_registry.get_by_id(meaning.word_id).unwrap().clone();
            let generator = Arc::clone(generator);
            let item_id = item.id;

            self.queue_registry.set_processing(item_id);

            Task::perform(
                async move {
                    let cloze = generator.generate(&word, &meaning).await;
                    QueueGenerationResult { item_id, cloze }
                },
                Message::QueueGenerationResult,
            )
        });

        Task::batch(tasks)
    }
}
