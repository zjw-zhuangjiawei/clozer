// use crate::persistence::DbError;
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum QueueItemStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct QueueItem {
    pub id: Uuid,
    pub meaning_id: Uuid,
    pub status: QueueItemStatus,
    pub selected: bool,
}

impl QueueItem {
    pub fn new(meaning_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            meaning_id,
            status: QueueItemStatus::Pending,
            selected: true,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct QueueRegistry {
    items: BTreeMap<Uuid, QueueItem>,
    dirty_ids: BTreeSet<Uuid>,
}

impl QueueRegistry {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
            dirty_ids: BTreeSet::new(),
        }
    }

    pub fn enqueue(&mut self, meaning_id: Uuid) {
        let item = QueueItem::new(meaning_id);
        self.items.insert(item.id, item.clone());
        self.dirty_ids.insert(item.id);
    }

    pub fn get_item(&self, id: Uuid) -> Option<&QueueItem> {
        self.items.get(&id)
    }

    pub fn get_item_mut(&mut self, id: Uuid) -> Option<&mut QueueItem> {
        self.items.get_mut(&id)
    }

    pub fn contains(&self, meaning_id: Uuid) -> bool {
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

    pub fn get_items(&self) -> impl Iterator<Item = &QueueItem> {
        self.items.values()
    }

    pub fn remove(&mut self, id: Uuid) {
        if self.items.remove(&id).is_some() {
            self.dirty_ids.insert(id);
        }
    }

    pub fn select(&mut self, id: Uuid) {
        if let Some(item) = self.items.get_mut(&id) {
            item.selected = true
        }
    }

    pub fn deselect(&mut self, id: Uuid) {
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

    pub fn set_processing(&mut self, id: Uuid) {
        if let Some(item) = self.items.get_mut(&id) {
            item.status = QueueItemStatus::Processing;
            self.dirty_ids.insert(id);
        }
    }

    pub fn set_completed(&mut self, id: Uuid) {
        if let Some(item) = self.items.get_mut(&id) {
            item.status = QueueItemStatus::Completed;
            item.selected = false;
            self.dirty_ids.insert(id);
        }
    }

    pub fn set_failed(&mut self, id: Uuid, error: String) {
        if let Some(item) = self.items.get_mut(&id) {
            item.status = QueueItemStatus::Failed(error);
            item.selected = false;
            self.dirty_ids.insert(id);
        }
    }

    pub fn clear_completed(&mut self) {
        let completed_ids: Vec<Uuid> = self
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
