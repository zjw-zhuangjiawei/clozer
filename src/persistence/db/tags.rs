//! Tag database operations.

use super::core::{TAGS_TABLE, deserialize, key_to_uuid, serialize, uuid_to_key};
use crate::persistence::{TagDto, db::Db};
use redb::{ReadableTable, ReadableTableMetadata};

/// Tag operations
impl Db {
    /// Saves a tag to the database.
    pub fn save_tag(
        &self,
        id: uuid::Uuid,
        data: &TagDto,
    ) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        {
            let mut table = t.open_table(TAGS_TABLE)?;
            let bytes = serialize(data)?;
            table.insert(&uuid_to_key(id), &bytes)?;
        }
        t.commit()?;
        Ok(())
    }

    /// Loads a tag from the database.
    pub fn load_tag(&self, id: uuid::Uuid) -> Result<Option<TagDto>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(TAGS_TABLE)?;
        if let Some(bytes) = table.get(&uuid_to_key(id))?.map(|v| v.value()) {
            Ok(Some(deserialize(&bytes)?))
        } else {
            Ok(None)
        }
    }

    /// Deletes a tag from the database.
    pub fn delete_tag(&self, id: uuid::Uuid) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        {
            let mut table = t.open_table(TAGS_TABLE)?;
            table.remove(&uuid_to_key(id))?;
        }
        t.commit()?;
        Ok(())
    }

    /// Iterates over all tags.
    pub fn iter_tags(&self) -> Result<impl Iterator<Item = TagDto>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(TAGS_TABLE)?;
        let items: Vec<TagDto> = table
            .iter()?
            .filter_map(|r| r.ok())
            .filter_map(|(key, bytes)| {
                let id = key_to_uuid(key.value());
                let mut dto: TagDto = deserialize(&bytes.value()).ok()?;
                dto.id = id;
                Some(dto)
            })
            .collect();
        Ok(items.into_iter())
    }

    /// Returns the number of tags.
    pub fn count_tags(&self) -> Result<u64, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(TAGS_TABLE)?;
        Ok(table.len()?)
    }
}
