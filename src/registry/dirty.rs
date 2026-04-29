use crate::persistence::db::{Db, DbError};
use redb::TableDefinition;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub(crate) struct DirtyTracker<K: Ord + Copy> {
    ids: BTreeSet<K>,
}

impl<K: Ord + Copy> DirtyTracker<K> {
    pub fn new() -> Self {
        Self {
            ids: BTreeSet::new(),
        }
    }

    pub fn mark(&mut self, id: K) {
        self.ids.insert(id);
    }

    pub fn clean(&mut self, id: &K) -> bool {
        self.ids.remove(id)
    }

    pub fn has_dirty(&self) -> bool {
        !self.ids.is_empty()
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn snapshot(&self) -> Vec<K> {
        self.ids.iter().copied().collect()
    }
}

pub(crate) fn flush_registry<K, V, D>(
    entities: &BTreeMap<K, V>,
    dirty: &mut DirtyTracker<K>,
    db: &Db,
    table: TableDefinition<[u8; 16], Vec<u8>>,
    to_dto: impl Fn(&V) -> D,
    label: &str,
) -> Result<(), DbError>
where
    K: Ord + Copy + fmt::Display + Into<Uuid>,
    D: serde::Serialize,
{
    let dirty_count = dirty.len();
    if dirty_count == 0 {
        return Ok(());
    }
    tracing::info!(
        count = dirty_count,
        label,
        "Flushing {} dirty entities",
        label
    );

    let mut errors = 0;
    for id in dirty.snapshot() {
        if let Some(entity) = entities.get(&id) {
            let dto = to_dto(entity);
            match db.save_entity(table, id, &dto, label) {
                Ok(_) => {
                    tracing::debug!(entity_id = %id, "Saved {}", label);
                    dirty.clean(&id);
                }
                Err(e) => {
                    errors += 1;
                    tracing::error!(entity_id = %id, error = %e, "Failed to save {}", label);
                }
            }
        } else {
            match db.delete_entity(table, id, label) {
                Ok(_) => {
                    tracing::debug!(entity_id = %id, "Deleted {}", label);
                    dirty.clean(&id);
                }
                Err(e) => {
                    errors += 1;
                    tracing::error!(entity_id = %id, error = %e, "Failed to delete {}", label);
                }
            }
        }
    }
    if errors > 0 {
        tracing::warn!(errors, label, "Some {} failed to persist", label);
    } else {
        tracing::info!(
            count = dirty_count,
            label,
            "Flushed {} {} successfully",
            dirty_count,
            label
        );
    }
    Ok(())
}
