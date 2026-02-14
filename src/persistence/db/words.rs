//! Word database operations.

use super::core::{WORDS_TABLE, deserialize, key_to_uuid, serialize, uuid_to_key};
use crate::persistence::{WordDto, db::Db};
use redb::{ReadableTable, ReadableTableMetadata};

/// Word operations
impl Db {
    /// Saves a word to the database.
    pub fn save_word(
        &self,
        id: uuid::Uuid,
        data: &WordDto,
    ) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        {
            let mut table = t.open_table(WORDS_TABLE)?;
            let bytes = serialize(data)?;
            table.insert(&uuid_to_key(id), &bytes)?;
        }
        t.commit()?;
        Ok(())
    }

    /// Loads a word from the database.
    pub fn load_word(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<WordDto>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(WORDS_TABLE)?;
        if let Some(bytes) = table.get(&uuid_to_key(id))?.map(|v| v.value()) {
            Ok(Some(deserialize(&bytes)?))
        } else {
            Ok(None)
        }
    }

    /// Deletes a word from the database.
    pub fn delete_word(&self, id: uuid::Uuid) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        {
            let mut table = t.open_table(WORDS_TABLE)?;
            table.remove(&uuid_to_key(id))?;
        }
        t.commit()?;
        Ok(())
    }

    /// Iterates over all words.
    pub fn iter_words(&self) -> Result<impl Iterator<Item = WordDto>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(WORDS_TABLE)?;
        let items: Vec<WordDto> = table
            .iter()?
            .filter_map(|r| r.ok())
            .filter_map(|(key, bytes)| {
                let id = key_to_uuid(key.value());
                let mut dto: WordDto = deserialize(&bytes.value()).ok()?;
                dto.id = id;
                Some(dto)
            })
            .collect();
        Ok(items.into_iter())
    }

    /// Returns the number of words.
    pub fn count_words(&self) -> Result<u64, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(WORDS_TABLE)?;
        Ok(table.len()?)
    }
}
