//! Provider database operations.

use super::core::{PROVIDERS_TABLE, deserialize, key_to_uuid, serialize, uuid_to_key};
use crate::persistence::{ProviderDto, db::Db};
use redb::ReadableTable;

/// Provider operations
impl Db {
    /// Saves a provider to the database.
    pub fn save_provider(
        &self,
        id: uuid::Uuid,
        data: &ProviderDto,
    ) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        let mut table = t.open_table(PROVIDERS_TABLE)?;
        let bytes = serialize(data)?;
        table.insert(&uuid_to_key(id), &bytes)?;
        Ok(())
    }

    /// Loads a provider from the database.
    pub fn load_provider(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<ProviderDto>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(PROVIDERS_TABLE)?;
        if let Some(bytes) = table.get(&uuid_to_key(id))?.map(|v| v.value()) {
            Ok(Some(deserialize(&bytes)?))
        } else {
            Ok(None)
        }
    }

    /// Deletes a provider from the database.
    pub fn delete_provider(&self, id: uuid::Uuid) -> Result<(), crate::persistence::DbError> {
        let t = self.write()?;
        let mut table = t.open_table(PROVIDERS_TABLE)?;
        table.remove(&uuid_to_key(id))?;
        Ok(())
    }

    /// Iterates over all providers.
    pub fn iter_providers(
        &self,
    ) -> Result<impl Iterator<Item = (uuid::Uuid, ProviderDto)>, crate::persistence::DbError> {
        let t = self.read()?;
        let table = t.open_table(PROVIDERS_TABLE)?;
        let items: Vec<(uuid::Uuid, ProviderDto)> = table
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
