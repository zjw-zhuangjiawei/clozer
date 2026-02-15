//! Meaning database operations.

use super::core::{MEANINGS_TABLE, deserialize, key_to_uuid, serialize, uuid_to_key};
use crate::persistence::{MeaningDto, db::Db};
use redb::{ReadableTable, ReadableTableMetadata};

/// Meaning operations
impl Db {
    /// Saves a meaning to the database.
    pub fn save_meaning(
        &self,
        id: uuid::Uuid,
        data: &MeaningDto,
    ) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        {
            let mut table = t.open_table(MEANINGS_TABLE)?;
            let bytes = serialize(data)?;
            table.insert(&uuid_to_key(id), &bytes)?;
        }
        t.commit()?;
        tracing::debug!("Saved meaning {}", id);
        Ok(())
    }

    /// Loads a meaning from the database.
    pub fn load_meaning(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<MeaningDto>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(MEANINGS_TABLE)?;
        if let Some(bytes) = table.get(&uuid_to_key(id))?.map(|v| v.value()) {
            Ok(Some(deserialize(&bytes)?))
        } else {
            Ok(None)
        }
    }

    /// Deletes a meaning from the database.
    pub fn delete_meaning(&self, id: uuid::Uuid) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        {
            let mut table = t.open_table(MEANINGS_TABLE)?;
            table.remove(&uuid_to_key(id))?;
        }
        t.commit()?;
        tracing::debug!("Deleted meaning {}", id);
        Ok(())
    }

    /// Iterates over all meanings.
    pub fn iter_meanings(
        &self,
    ) -> Result<impl Iterator<Item = MeaningDto>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(MEANINGS_TABLE)?;
        let items: Vec<MeaningDto> = table
            .iter()?
            .filter_map(|r| r.ok())
            .filter_map(|(key, bytes)| {
                let id = key_to_uuid(key.value());
                let mut dto: MeaningDto = deserialize(&bytes.value()).ok()?;
                dto.id = id;
                Some(dto)
            })
            .collect();
        tracing::debug!("Loaded {} meanings", items.len());
        Ok(items.into_iter())
    }

    /// Returns the number of meanings.
    pub fn count_meanings(&self) -> Result<u64, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(MEANINGS_TABLE)?;
        Ok(table.len()?)
    }
}
