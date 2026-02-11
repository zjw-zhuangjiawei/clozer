//! Queue database operations.

use super::core::{QUEUE_TABLE, deserialize, key_to_uuid, serialize, uuid_to_key};
use crate::persistence::{QueueItemDto, db::Db};
use redb::ReadableTable;

/// Queue operations
impl Db {
    /// Saves a queue item to the database.
    pub fn save_queue_item(
        &self,
        id: uuid::Uuid,
        data: &QueueItemDto,
    ) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        let mut table = t.open_table(QUEUE_TABLE)?;
        let bytes = serialize(data)?;
        table.insert(&uuid_to_key(id), &bytes)?;
        Ok(())
    }

    /// Loads a queue item from the database.
    pub fn load_queue_item(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<QueueItemDto>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(QUEUE_TABLE)?;
        if let Some(bytes) = table.get(&uuid_to_key(id))?.map(|v| v.value()) {
            Ok(Some(deserialize(&bytes)?))
        } else {
            Ok(None)
        }
    }

    /// Deletes a queue item from the database.
    pub fn delete_queue_item(&self, id: uuid::Uuid) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        let mut table = t.open_table(QUEUE_TABLE)?;
        table.remove(&uuid_to_key(id))?;
        Ok(())
    }

    /// Iterates over all queue items.
    pub fn iter_queue(
        &self,
    ) -> Result<impl Iterator<Item = (uuid::Uuid, QueueItemDto)>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(QUEUE_TABLE)?;
        let items: Vec<(uuid::Uuid, QueueItemDto)> = table
            .iter()?
            .filter_map(|r| r.ok())
            .filter_map(|(id, bytes)| {
                let data = deserialize(&bytes.value()).ok()?;
                Some((key_to_uuid(id.value()), data))
            })
            .collect();
        Ok(items.into_iter())
    }
}
