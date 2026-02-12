use crate::persistence::DbError;
use std::collections::{HashMap, HashSet};
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
    items: HashMap<Uuid, QueueItem>,
    dirty_ids: HashSet<Uuid>,
}

impl QueueRegistry {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            dirty_ids: HashSet::new(),
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

    // Persistence
    /// Load all queue items from database
    pub fn load_all(&mut self, db: &crate::persistence::Db) {
        if let Ok(items) = db.iter_queue() {
            for (id, dto) in items {
                let status = match dto.status {
                    crate::persistence::QueueItemStatusDto::Pending => QueueItemStatus::Pending,
                    crate::persistence::QueueItemStatusDto::Processing => {
                        QueueItemStatus::Processing
                    }
                    crate::persistence::QueueItemStatusDto::Completed => QueueItemStatus::Completed,
                    crate::persistence::QueueItemStatusDto::Failed => {
                        QueueItemStatus::Failed(String::new())
                    }
                };
                let item = QueueItem {
                    id,
                    meaning_id: dto.meaning_id,
                    status,
                    selected: false, // DTO doesn't store selected
                };
                self.items.insert(id, item);
            }
        }
    }

    /// Flush all dirty entities to the database
    pub fn flush_dirty(&mut self, db: &crate::persistence::Db) -> Result<(), DbError> {
        for id in &self.dirty_ids {
            if let Some(item) = self.items.get(id) {
                let status = match item.status {
                    QueueItemStatus::Pending => crate::persistence::QueueItemStatusDto::Pending,
                    QueueItemStatus::Processing => {
                        crate::persistence::QueueItemStatusDto::Processing
                    }
                    QueueItemStatus::Completed => crate::persistence::QueueItemStatusDto::Completed,
                    QueueItemStatus::Failed(_) => crate::persistence::QueueItemStatusDto::Failed,
                };
                let dto = crate::persistence::QueueItemDto {
                    meaning_id: item.meaning_id,
                    status,
                };
                db.save_queue_item(*id, &dto)?;
            }
        }
        self.dirty_ids.clear();
        Ok(())
    }

    /// Check if there are any dirty entities
    pub fn has_dirty(&self) -> bool {
        !self.dirty_ids.is_empty()
    }
}
