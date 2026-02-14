//! Cloze database operations.

use super::core::{CLOZES_TABLE, deserialize, key_to_uuid, serialize, uuid_to_key};
use crate::persistence::{ClozeDto, db::Db};
use redb::{ReadableTable, ReadableTableMetadata};

/// Cloze operations
impl Db {
    /// Saves a cloze to the database.
    pub fn save_cloze(
        &self,
        id: uuid::Uuid,
        data: &ClozeDto,
    ) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        {
            let mut table = t.open_table(CLOZES_TABLE)?;
            let bytes = serialize(data)?;
            table.insert(&uuid_to_key(id), &bytes)?;
        }
        t.commit()?;
        Ok(())
    }

    /// Loads a cloze from the database.
    pub fn load_cloze(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<ClozeDto>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(CLOZES_TABLE)?;
        if let Some(bytes) = table.get(&uuid_to_key(id))?.map(|v| v.value()) {
            Ok(Some(deserialize(&bytes)?))
        } else {
            Ok(None)
        }
    }

    /// Deletes a cloze from the database.
    pub fn delete_cloze(&self, id: uuid::Uuid) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        {
            let mut table = t.open_table(CLOZES_TABLE)?;
            table.remove(&uuid_to_key(id))?;
        }
        t.commit()?;
        Ok(())
    }

    /// Iterates over all clozes.
    pub fn iter_clozes(
        &self,
    ) -> Result<impl Iterator<Item = ClozeDto>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(CLOZES_TABLE)?;
        let items: Vec<ClozeDto> = table
            .iter()?
            .filter_map(|r| r.ok())
            .filter_map(|(key, bytes)| {
                let id = key_to_uuid(key.value());
                let mut dto: ClozeDto = deserialize(&bytes.value()).ok()?;
                dto.id = id;
                Some(dto)
            })
            .collect();
        Ok(items.into_iter())
    }

    /// Returns the number of clozes.
    pub fn count_clozes(&self) -> Result<u64, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(CLOZES_TABLE)?;
        Ok(table.len()?)
    }
}
