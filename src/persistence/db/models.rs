//! Model database operations.

use super::core::{MODELS_TABLE, deserialize, key_to_uuid, serialize, uuid_to_key};
use crate::persistence::{ModelDto, db::Db};
use redb::ReadableTable;

/// Model operations
impl Db {
    /// Saves a model to the database.
    pub fn save_model(
        &self,
        id: uuid::Uuid,
        data: &ModelDto,
    ) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        let mut table = t.open_table(MODELS_TABLE)?;
        let bytes = serialize(data)?;
        table.insert(&uuid_to_key(id), &bytes)?;
        Ok(())
    }

    /// Loads a model from the database.
    pub fn load_model(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<ModelDto>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(MODELS_TABLE)?;
        if let Some(bytes) = table.get(&uuid_to_key(id))?.map(|v| v.value()) {
            Ok(Some(deserialize(&bytes)?))
        } else {
            Ok(None)
        }
    }

    /// Deletes a model from the database.
    pub fn delete_model(&self, id: uuid::Uuid) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        let mut table = t.open_table(MODELS_TABLE)?;
        table.remove(&uuid_to_key(id))?;
        Ok(())
    }

    /// Iterates over all models.
    pub fn iter_models(
        &self,
    ) -> Result<impl Iterator<Item = (uuid::Uuid, ModelDto)>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(MODELS_TABLE)?;
        let items: Vec<(uuid::Uuid, ModelDto)> = table
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
