//! Database operations using redb.
//!
//! Provides generic CRUD operations for all entity types.

use std::path::Path;

use redb::{Database, ReadableDatabase, ReadableTable, ReadableTableMetadata};
use serde::Serialize;
use serde::de::DeserializeOwned;
use uuid::Uuid;

mod core;
pub use core::*;

/// Database errors.
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Redb(#[from] redb::DatabaseError),

    #[error("Serialization error (decode): {0}")]
    SerializationDecode(#[from] rmp_serde::decode::Error),

    #[error("Serialization error (encode): {0}")]
    SerializationEncode(#[from] rmp_serde::encode::Error),

    #[error("Transaction error: {0}")]
    Transaction(#[from] redb::TransactionError),

    #[error("Table error: {0}")]
    Table(#[from] redb::TableError),

    #[error("Storage error: {0}")]
    Storage(#[from] redb::StorageError),

    #[error("Commit error: {0}")]
    Commit(#[from] redb::CommitError),
}

/// Main database wrapper.
#[derive(Debug)]
pub struct Db {
    db: Database,
}

impl Db {
    /// Opens or creates the database at the given path.
    pub fn new(path: &Path) -> Result<Self, DbError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let db = Database::create(path)?;

        Ok(Self { db })
    }

    fn read(&self) -> Result<redb::ReadTransaction, redb::TransactionError> {
        self.db.begin_read()
    }

    fn write(&self) -> Result<redb::WriteTransaction, redb::TransactionError> {
        self.db.begin_write()
    }

    /// Saves an entity to the database (generic over table definition and DTO type).
    pub fn save_entity<T: Serialize>(
        &self,
        table: redb::TableDefinition<[u8; 16], Vec<u8>>,
        id: impl Into<Uuid>,
        data: &T,
        label: &str,
    ) -> Result<(), DbError> {
        let uuid = id.into();
        let t = self.write()?;
        {
            let mut t = t.open_table(table)?;
            let bytes = serialize(data)?;
            t.insert(&uuid_to_key(uuid), &bytes)?;
        }
        t.commit()?;
        tracing::debug!(entity_id = %uuid, "Saved {}", label);
        Ok(())
    }

    /// Loads a single entity from the database.
    pub fn load_entity<T: DeserializeOwned>(
        &self,
        table: redb::TableDefinition<[u8; 16], Vec<u8>>,
        id: impl Into<Uuid>,
    ) -> Result<Option<T>, DbError> {
        let uuid = id.into();
        let t = self.read()?;
        let table = t.open_table(table)?;
        if let Some(bytes) = table.get(&uuid_to_key(uuid))?.map(|v| v.value()) {
            Ok(Some(deserialize(&bytes)?))
        } else {
            Ok(None)
        }
    }

    /// Deletes an entity from the database.
    pub fn delete_entity(
        &self,
        table: redb::TableDefinition<[u8; 16], Vec<u8>>,
        id: impl Into<Uuid>,
        label: &str,
    ) -> Result<(), DbError> {
        let uuid = id.into();
        let t = self.write()?;
        {
            let mut t = t.open_table(table)?;
            t.remove(&uuid_to_key(uuid))?;
        }
        t.commit()?;
        tracing::debug!(entity_id = %uuid, "Deleted {}", label);
        Ok(())
    }

    /// Iterates over all entities in a table, returning (Uuid, T) pairs.
    pub fn iter_entities<T: DeserializeOwned>(
        &self,
        table: redb::TableDefinition<[u8; 16], Vec<u8>>,
    ) -> Result<Vec<(Uuid, T)>, DbError> {
        let t = self.read()?;
        let table = t.open_table(table)?;
        let items: Vec<(Uuid, T)> = table
            .iter()?
            .filter_map(|r| r.ok())
            .filter_map(|(key, bytes)| {
                let id = key_to_uuid(key.value());
                let dto: T = match deserialize(&bytes.value()) {
                    Ok(dto) => dto,
                    Err(e) => {
                        tracing::error!(entity_id = %id, error = %e, "Failed to deserialize entity");
                        return None;
                    }
                };
                Some((id, dto))
            })
            .collect();
        tracing::debug!(count = items.len(), "Loaded entities from database");
        Ok(items)
    }

    /// Returns the number of entities in a table.
    pub fn count_entities(
        &self,
        table: redb::TableDefinition<[u8; 16], Vec<u8>>,
    ) -> Result<u64, DbError> {
        let t = self.read()?;
        let table = t.open_table(table)?;
        Ok(table.len()?)
    }
}
