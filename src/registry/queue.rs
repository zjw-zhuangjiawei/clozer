// use crate::persistence::DbError;
use crate::models::{MeaningId, WordId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq)]
pub enum QueueItemStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct QueueItem {
    pub id: WordId,
    pub meaning_id: MeaningId,
    pub status: QueueItemStatus,
    pub selected: bool,
}

impl QueueItem {
    pub fn new(meaning_id: MeaningId) -> Self {
        Self {
            id: WordId::new(),
            meaning_id,
            status: QueueItemStatus::Pending,
            selected: true,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct QueueRegistry {
    items: BTreeMap<WordId, QueueItem>,
    dirty_ids: BTreeSet<WordId>,
}

impl QueueRegistry {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
            dirty_ids: BTreeSet::new(),
        }
    }

    pub fn enqueue(&mut self, meaning_id: MeaningId) {
        let item = QueueItem::new(meaning_id);
        self.items.insert(item.id, item.clone());
        self.dirty_ids.insert(item.id);
    }

    pub fn get_item(&self, id: WordId) -> Option<&QueueItem> {
        self.items.get(&id)
    }

    pub fn contains(&self, meaning_id: MeaningId) -> bool {
        self.items
            .values()
            .any(|item| item.meaning_id == meaning_id)
    }

    pub fn has_pending(&self) -> bool {
        self.items
            .values()
            .any(|item| item.status == QueueItemStatus::Pending)
    }

    pub fn has_selected(&self) -> bool {
        self.items
            .values()
            .any(|item| item.selected && item.status == QueueItemStatus::Pending)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn get_items(&self) -> impl Iterator<Item = &QueueItem> {
        self.items.values()
    }

    pub fn remove(&mut self, id: WordId) {
        if self.items.remove(&id).is_some() {
            self.dirty_ids.insert(id);
        }
    }

    pub fn select(&mut self, id: WordId) {
        if let Some(item) = self.items.get_mut(&id) {
            item.selected = true
        }
    }

    pub fn deselect(&mut self, id: WordId) {
        if let Some(item) = self.items.get_mut(&id) {
            item.selected = false
        }
    }

    pub fn select_all(&mut self) {
        for (_, item) in self.items.iter_mut() {
            if item.status == QueueItemStatus::Pending {
                item.selected = true;
            }
        }
    }

    pub fn deselect_all(&mut self) {
        for (_, item) in self.items.iter_mut() {
            if item.status == QueueItemStatus::Pending {
                item.selected = false;
            }
        }
    }

    pub fn set_processing(&mut self, id: WordId) {
        if let Some(item) = self.items.get_mut(&id) {
            item.status = QueueItemStatus::Processing;
            self.dirty_ids.insert(id);
        }
    }

    pub fn set_completed(&mut self, id: WordId) {
        if let Some(item) = self.items.get_mut(&id) {
            item.status = QueueItemStatus::Completed;
            item.selected = false;
            self.dirty_ids.insert(id);
        }
    }

    pub fn set_failed(&mut self, id: WordId, error: String) {
        if let Some(item) = self.items.get_mut(&id) {
            item.status = QueueItemStatus::Failed(error);
            item.selected = false;
            self.dirty_ids.insert(id);
        }
    }

    pub fn clear_completed(&mut self) {
        let completed_ids: Vec<WordId> = self
            .items
            .iter()
            .filter(|(_, item)| item.status == QueueItemStatus::Completed)
            .map(|(id, _)| *id)
            .collect();

        for id in &completed_ids {
            self.dirty_ids.insert(*id);
        }

        self.items
            .retain(|_, item| item.status != QueueItemStatus::Completed);
    }
}
